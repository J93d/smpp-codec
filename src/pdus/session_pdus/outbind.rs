use crate::common::{read_c_string, write_c_string, PduError, CMD_OUTBIND, HEADER_LEN};
use std::io::{Cursor, Read, Write};

/// Represents an Outbind PDU.
///
/// Sent by the SMSC to the ESME to request the ESME to initiate a Bind.
#[derive(Debug, Clone, PartialEq)]
pub struct OutbindRequest {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// System ID identifying the ESME
    pub system_id: String,
    /// Password for authentication
    pub password: String,
}

impl OutbindRequest {
    /// Create a new Outbind Request.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::OutbindRequest;
    ///
    /// let sequence_number: u32 = 1;
    /// let outbind = OutbindRequest::new(
    ///     sequence_number, // Sequence number
    ///     "my_system_id".to_string(),
    ///     "password".to_string(),
    /// );
    /// ```
    pub fn new(sequence_number: u32, system_id: String, password: String) -> Self {
        Self {
            sequence_number,
            system_id,
            password,
        }
    }

    /// Encode the struct into raw bytes for the network.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if:
    /// * `system_id` exceeds 16 characters.
    /// * `password` exceeds 9 characters.
    /// * An I/O error occurs while writing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::OutbindRequest;
    /// # let sequence_number: u32 = 1;
    /// # let outbind = OutbindRequest::new(sequence_number, "id".into(), "pwd".into());
    /// let mut buffer = Vec::new();
    /// outbind.encode(&mut buffer).expect("Encoding failed");
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, system_id = %self.system_id)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding OutbindRequest");
        // Validate
        if self.system_id.len() > 16 {
            return Err(PduError::StringTooLong("system_id".into(), 16));
        }
        if self.password.len() > 9 {
            return Err(PduError::StringTooLong("password".into(), 9));
        }

        // Calculate body length
        let body_len = self.system_id.len() + 1 + self.password.len() + 1;
        let command_len = (HEADER_LEN + body_len) as u32;

        // Header
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_OUTBIND.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?; // Status is always 0
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        // Body
        write_c_string(writer, &self.system_id)?;
        write_c_string(writer, &self.password)?;

        Ok(())
    }

    /// Decode raw bytes from the network into the struct.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if:
    /// * The buffer is too short.
    /// * The buffer data is malformed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::OutbindRequest;
    /// # let sequence_number: u32 = 1;
    /// # let outbind = OutbindRequest::new(sequence_number, "id".into(), "pwd".into());
    /// # let mut buffer = Vec::new();
    /// # outbind.encode(&mut buffer).unwrap();
    /// let decoded = OutbindRequest::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.system_id, "id");
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(buffer), err, fields(seq = tracing::field::Empty, system_id = tracing::field::Empty)))]
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Decoding OutbindRequest from {} bytes", buffer.len());
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }

        let mut cursor = Cursor::new(buffer);

        // Skip Header (assuming caller checked ID, or we just consume it)
        cursor.set_position(12); // Skip len, id, status
        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        // Body
        let system_id = read_c_string(&mut cursor)?;
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("system_id", &system_id);

        let password = read_c_string(&mut cursor)?;

        Ok(Self {
            sequence_number,
            system_id,
            password,
        })
    }
}
