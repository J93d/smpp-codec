use crate::common::{
    get_status_code, get_status_description, read_c_string, PduError, CMD_SUBMIT_SM_RESP,
    HEADER_LEN,
};
use std::io::{Cursor, Read, Write};

/// Represents a Submit SM Response PDU.
#[derive(Debug, Clone, PartialEq)]
pub struct SubmitSmResponse {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Command Status (0 = OK, others = Error)
    pub command_status: u32,
    /// Message ID (C-Octet String)
    pub message_id: String, // C-Octet String (Max 65 chars)
    /// Human-readable description of status
    pub status_description: String,
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
    /// let resp = SubmitSmResponse::new(sequence_number, "ESME_ROK", "MsgID:123".into());
    /// ```
    pub fn new(sequence_number: u32, status_name: &str, message_id: String) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            message_id,
            status_description: status_name.to_string(),
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
    /// # let resp = SubmitSmResponse::new(sequence_number, "ESME_ROK", "ID".into());
    /// let mut buffer = Vec::new();
    /// resp.encode(&mut buffer).expect("Encoding failed");
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, status = %self.status_description, message_id = %self.message_id)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding SubmitSmResponse");
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
    /// # let resp = SubmitSmResponse::new(sequence_number, "ESME_ROK", "ID".into());
    /// # let mut buffer = Vec::new();
    /// # resp.encode(&mut buffer).unwrap();
    /// let decoded = SubmitSmResponse::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.message_id, "ID");
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(buffer), err, fields(seq = tracing::field::Empty, status = tracing::field::Empty, message_id = tracing::field::Empty)))]
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Decoding SubmitSmResponse from {} bytes", buffer.len());
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);

        // Header
        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let _command_len = u32::from_be_bytes(bytes) as usize;
        cursor.read_exact(&mut bytes)?; // Skip ID
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        let status_description = get_status_description(command_status);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("status", &status_description);

        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        // Body
        // 3. Read Body (message_id)
        let message_id: String = if buffer.len() > HEADER_LEN {
            let id = read_c_string(&mut cursor)?;
            #[cfg(feature = "tracing")]
            tracing::Span::current().record("message_id", &id);
            id
        } else {
            String::new()
        };

        Ok(Self {
            sequence_number,
            command_status,
            message_id,
            status_description,
        })
    }
}
