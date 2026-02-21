use crate::common::{get_status_code, get_status_description, PduError, GENERIC_NACK, HEADER_LEN};
use std::io::{Cursor, Read, Write};

/// Represents a Generic NACK PDU.
///
/// Sent when a PDU cannot be identified or is malformed (e.g., invalid Command ID).
#[derive(Debug, Clone, PartialEq)]
pub struct GenericNack {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// The error code explaining why the NACK was sent
    pub command_status: u32, // The error code explaining why the NACK was sent
    /// Human-readable status name
    pub status_name: String,
}

impl GenericNack {
    /// Create a new Generic NACK.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::GenericNack;
    ///
    /// let sequence_number: u32 = 1;
    /// let nack = GenericNack::new("ESME_RINVCMDID", sequence_number);
    /// ```
    pub fn new(status_name: &str, sequence_number: u32) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            status_name: status_name.to_string(),
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
    /// # use smpp_codec::pdus::GenericNack;
    /// # let sequence_number: u32 = 1;
    /// # let nack = GenericNack::new("ESME_RINVCMDID", sequence_number);
    /// let mut buffer = Vec::new();
    /// nack.encode(&mut buffer).expect("Encoding failed");
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, status = %self.status_name)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding GenericNack");
        let command_len = HEADER_LEN as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&GENERIC_NACK.to_be_bytes())?;
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
    /// # use smpp_codec::pdus::GenericNack;
    /// # let sequence_number: u32 = 1;
    /// # let nack = GenericNack::new("ESME_RINVCMDID", sequence_number);
    /// # let mut buffer = Vec::new();
    /// # nack.encode(&mut buffer).unwrap();
    /// let decoded = GenericNack::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.status_name, "ESME_RINVCMDID");
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(buffer), err, fields(seq = tracing::field::Empty, status = tracing::field::Empty)))]
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Decoding GenericNack from {} bytes", buffer.len());
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);

        // Skip Length (4) and ID (4)
        cursor.set_position(8);

        let mut bytes = [0u8; 4];

        // Status
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        let status_name = get_status_description(command_status);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("status", &status_name);

        // Sequence
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        Ok(Self {
            sequence_number,
            command_status,
            status_name,
        })
    }
}
