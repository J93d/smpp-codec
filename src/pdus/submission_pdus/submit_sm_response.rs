use crate::common::{PduError, HEADER_LEN, CMD_SUBMIT_SM_RESP};
use std::io::{Read, Write, Cursor};

#[derive(Debug, Clone)]
pub struct SubmitSmResponse {
    pub sequence_number: u32,
    pub command_status: u32,
    pub message_id: String, // C-Octet String (Max 65 chars)
}

impl SubmitSmResponse {
    /// Create a new SubmitSm Response.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::SubmitSmResponse;
    ///
    /// let sequence_number: u32 = 1;
    /// let resp = SubmitSmResponse::new(sequence_number, 0, "MsgID:123".into());
    /// ```
    pub fn new(sequence_number: u32, command_status: u32, message_id: String) -> Self {
        Self {
            sequence_number,
            command_status,
            message_id,
        }
    }

    /// Encode the struct into raw bytes.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if I/O fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::SubmitSmResponse;
    /// # let sequence_number: u32 = 1;
    /// # let resp = SubmitSmResponse::new(sequence_number, 0, "ID".into());
    /// let mut buffer = Vec::new();
    /// resp.encode(&mut buffer).expect("Encoding failed");
    /// ```
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let mut body = Vec::new();

        // Only write message_id if status is OK
        if self.command_status == 0 {
            body.write_all(self.message_id.as_bytes())?;
            body.write_all(&[0])?;
        }

        let command_len = (HEADER_LEN + body.len()) as u32;

        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_SUBMIT_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        writer.write_all(&body)?;
        
        Ok(())
    }

    /// Decode raw bytes from the network.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if buffer is too short.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::SubmitSmResponse;
    /// # let sequence_number: u32 = 1;
    /// # let resp = SubmitSmResponse::new(sequence_number, 0, "ID".into());
    /// # let mut buffer = Vec::new();
    /// # resp.encode(&mut buffer).unwrap();
    /// let decoded = SubmitSmResponse::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.message_id, "ID");
    /// ```
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN { return Err(PduError::BufferTooShort); }
        let mut cursor = Cursor::new(buffer);
        
        // Header
        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let command_len = u32::from_be_bytes(bytes) as usize;
        cursor.read_exact(&mut bytes)?; // Skip ID
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        // Body
        let mut message_id = String::new();
        if command_status == 0 && command_len > HEADER_LEN {
             message_id = read_c_string(&mut cursor)?;
        }

        Ok(Self {
            sequence_number,
            command_status,
            message_id,
        })
    }
}

// Helper (Assuming this is standard now, can also import from common/utils if you made one)
fn read_c_string(cursor: &mut Cursor<&[u8]>) -> Result<String, PduError> {
    let mut bytes = Vec::new();
    let mut buf = [0u8; 1];
    loop {
        if cursor.read(&mut buf)? == 0 { break; }
        if buf[0] == 0 { break; }
        bytes.push(buf[0]);
    }
    String::from_utf8(bytes).map_err(|e| PduError::Utf8(e))
}