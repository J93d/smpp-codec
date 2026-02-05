use crate::common::{PduError, CMD_ENQUIRE_LINK, HEADER_LEN};
use std::io::{Cursor, Read, Write};

// --- EnquireLink Request ---
/// Represents an Enquire Link Request PDU.
///
/// Used to check the health of the connection.
#[derive(Debug, Clone, PartialEq)]
pub struct EnquireLinkRequest {
    /// Sequence number of the PDU
    pub sequence_number: u32,
}

impl EnquireLinkRequest {
    /// Create a new Enquire Link Request.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::EnquireLinkRequest;
    ///
    /// let sequence_number: u32 = 1;
    /// let enquire_link = EnquireLinkRequest::new(sequence_number);
    /// ```
    pub fn new(sequence_number: u32) -> Self {
        Self { sequence_number }
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
    /// # use smpp_codec::pdus::EnquireLinkRequest;
    /// # let sequence_number: u32 = 1;
    /// # let enquire_link = EnquireLinkRequest::new(sequence_number);
    /// let mut buffer = Vec::new();
    /// enquire_link.encode(&mut buffer).expect("Encoding failed");
    /// ```
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let command_len = HEADER_LEN as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_ENQUIRE_LINK.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?; // Status always 0
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
    /// # use smpp_codec::pdus::EnquireLinkRequest;
    /// # let sequence_number: u32 = 1;
    /// # let enquire_link = EnquireLinkRequest::new(sequence_number);
    /// # let mut buffer = Vec::new();
    /// # enquire_link.encode(&mut buffer).unwrap();
    /// let decoded = EnquireLinkRequest::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.sequence_number, 1);
    /// ```
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(12); // Skip len, id, status

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        Ok(Self { sequence_number })
    }
}
