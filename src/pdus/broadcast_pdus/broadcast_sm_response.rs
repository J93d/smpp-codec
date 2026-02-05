use crate::common::{
    get_status_code, get_status_description, read_c_string, write_c_string, PduError,
    CMD_BROADCAST_SM_RESP, HEADER_LEN,
};
use crate::tlv::Tlv;
use std::io::{Cursor, Read, Write};

// --- Response ---
/// Represents a Broadcast SM Response PDU.
///
/// Sent by the SMSC in response to a Broadcast SM Request.
#[derive(Debug, Clone, PartialEq)]
pub struct BroadcastSmResp {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Command Status (0 = OK, others = Error)
    pub command_status: u32, // 0 = OK, others = Error
    /// Human-readable description of status
    pub status_description: String, // Human-readable description of status
    /// Message ID
    pub message_id: String,
    /// Optional TLVs (Can return 'broadcast_area_success')
    pub optional_params: Vec<Tlv>, // Can return 'broadcast_area_success'
}

impl BroadcastSmResp {
    /// Create a new Broadcast SM Response.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::BroadcastSmResp;
    ///
    /// let resp = BroadcastSmResp::new(
    ///     1,
    ///     "ESME_ROK",
    ///     "MessageID".to_string(),
    /// );
    /// ```
    pub fn new(sequence_number: u32, status_name: &str, message_id: String) -> Self {
        let command_status = get_status_code(status_name);
        let status_description = status_name.to_string();
        Self {
            sequence_number,
            command_status,
            status_description,
            message_id,
            optional_params: Vec::new(),
        }
    }

    /// Encode the PDU into the writer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the write fails.
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let mut body = Vec::new();

        // BroadcastResp body is: MessageID + Optional TLVs
        write_c_string(&mut body, &self.message_id)?;

        for tlv in &self.optional_params {
            tlv.encode(&mut body)?;
        }

        let command_len = (HEADER_LEN + body.len()) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_BROADCAST_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        writer.write_all(&body)?;
        Ok(())
    }

    /// Decode the PDU from the buffer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the buffer is too short or malformed.
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(8); // Skip Len, ID

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        let message_id = read_c_string(&mut cursor)?;

        let mut optional_params = Vec::new();
        while let Some(tlv) = Tlv::decode(&mut cursor)? {
            optional_params.push(tlv);
        }

        let status_description = get_status_description(command_status);

        Ok(Self {
            sequence_number,
            command_status,
            status_description,
            message_id,
            optional_params,
        })
    }
}
