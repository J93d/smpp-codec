use crate::common::{
    get_status_code, get_status_description, read_c_string, write_c_string, PduError,
    CMD_DELIVER_SM_RESP, HEADER_LEN,
};
use std::io::{Cursor, Read, Write};

/// Represents a Deliver SM Response PDU.
///
/// This PDU is sent by the ESME to acknowledge the receipt of a `DeliverSmRequest`.
/// It contains a message_id (C-Octet String) which is typically NULL (empty) in v3.4 / v5.0.
#[derive(Debug, Clone, PartialEq)]
pub struct DeliverSmResponse {
    /// The sequence number of the PDU, matching the request.
    pub sequence_number: u32,
    /// The command status (e.g., 0 for ESME_ROK).
    pub command_status: u32,
    /// The message ID (C-Octet String). Usually empty/null.
    pub message_id: String,
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
    pub fn new(sequence_number: u32, status_name: &str) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            message_id: String::new(), // Default to empty
            status_description: status_name.to_string(),
        }
    }

    /// Encode the struct into raw bytes for the network.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, status = %self.status_description, message_id = %self.message_id)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding DeliverSmResponse");
        let mut body = Vec::new();
        // Always write message_id if status is 0 (OK)
        if self.command_status == 0 {
            write_c_string(&mut body, &self.message_id)?;
        }

        let command_len = (HEADER_LEN + body.len()) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_DELIVER_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        writer.write_all(&body)?;

        Ok(())
    }

    /// Decode raw bytes from the network into the struct.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(buffer), err, fields(seq = tracing::field::Empty, status = tracing::field::Empty, message_id = tracing::field::Empty)))]
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Decoding DeliverSmResponse from {} bytes", buffer.len());
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(8); // Skip Len and ID

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        let status_description = get_status_description(command_status);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("status", &status_description);

        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        // Read Body (message_id) if present
        let message_id = if buffer.len() > HEADER_LEN {
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
