use crate::common::{
    get_status_code, get_status_description, PduError, CMD_ENQUIRE_LINK_RESP, HEADER_LEN,
};
use std::io::{Cursor, Read, Write};

// --- EnquireLink Response ---
/// Represents an Enquire Link Response PDU.
///
/// Sent in response to an Enquire Link Request.
#[derive(Debug, Clone, PartialEq)]
pub struct EnquireLinkResponse {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Command Status (0 = OK, others = Error)
    pub command_status: u32, // 0 = OK, others = Error
    /// Human-readable description of status
    pub status_description: String, // Human-readable description of status
}

impl EnquireLinkResponse {
    /// Create a new Enquire Link Response.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::EnquireLinkResponse;
    ///
    /// let sequence_number: u32 = 1;
    /// let resp = EnquireLinkResponse::new(sequence_number, "ESME_ROK");
    /// ```
    pub fn new(sequence_number: u32, status_name: &str) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            status_description: status_name.to_string(),
        }
    }

    /// Encode the struct into raw bytes for the network.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if an I/O error occurs while writing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::EnquireLinkResponse;
    /// # use smpp_codec::common::CMD_ENQUIRE_LINK_RESP;
    /// # let sequence_number: u32 = 1;
    /// # let resp = EnquireLinkResponse::new(sequence_number, "ESME_ROK");
    /// let mut buffer = Vec::new();
    /// resp.encode(&mut buffer).expect("Encoding failed");
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, status = %self.status_description)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding EnquireLinkResponse");
        let command_len = HEADER_LEN as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_ENQUIRE_LINK_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        Ok(())
    }

    /// Decode raw bytes from the network into the struct.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the buffer is too short.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::EnquireLinkResponse;
    /// # use smpp_codec::common::CMD_ENQUIRE_LINK_RESP;
    /// # let sequence_number: u32 = 1;
    /// # let resp = EnquireLinkResponse::new(sequence_number, "ESME_ROK");
    /// # let mut buffer = Vec::new();
    /// # resp.encode(&mut buffer).unwrap();
    /// let decoded = EnquireLinkResponse::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.sequence_number, 1);
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(buffer), err, fields(seq = tracing::field::Empty, status = tracing::field::Empty)))]
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Decoding EnquireLinkResponse from {} bytes", buffer.len());
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);

        // Skip Length (4) and ID (4)
        cursor.set_position(8);

        let mut bytes = [0u8; 4];

        // [Change] Read Status
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        let status_description = get_status_description(command_status);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("status", &status_description);

        // Read Sequence
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        Ok(Self {
            sequence_number,
            command_status,
            status_description,
        })
    }
}
