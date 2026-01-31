use crate::common::{PduError, HEADER_LEN, CMD_DELIVER_SM_RESP, get_status_code, get_status_description};
use std::io::{Write, Cursor, Read};

/// Represents a Deliver SM Response PDU.
///
/// This PDU is sent by the ESME to acknowledge the receipt of a `DeliverSmRequest`.
/// It typically contains no body other than the message ID (which is often unused or empty in v3.4 for this response).
#[derive(Debug, Clone)]
pub struct DeliverSmResponse {
    /// The sequence number of the PDU, matching the request.
    pub sequence_number: u32,
    /// The command status (e.g., 0 for ESME_ROK).
    pub command_status: u32,
    /// A human-readable description of the status.
    pub status_description: String,
}

impl DeliverSmResponse {
    /// Create a new DeliverSm Response.
    ///
    /// # Arguments
    ///
    /// * `sequence_number` - The sequence number from the request.
    /// * `status_name` - The status name (e.g., "ESME_ROK").
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::DeliverSmResponse;
    ///
    /// let resp = DeliverSmResponse::new(1, "ESME_ROK");
    /// ```
    pub fn new(
        sequence_number: u32, 
        status_name: &str,
    ) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            status_description: status_name.to_string(),
        }
    }

    /// Encode the struct into raw bytes for the network.
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let command_len = HEADER_LEN as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_DELIVER_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        
        // DeliverSmResp typically has no body (message_id is unused in v3.4)
        Ok(())
    }

    /// Decode raw bytes from the network into the struct.
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN { return Err(PduError::BufferTooShort); }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(8); // Skip Len and ID

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        let status_description = get_status_description(command_status);

        Ok(Self {
            sequence_number,
            command_status,
            status_description,
        })
    }
}

