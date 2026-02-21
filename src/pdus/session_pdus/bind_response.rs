// bind_response.rs (Handles both Encoder and Decoder)
use crate::common::{get_status_code, get_status_description, read_c_string, PduError, HEADER_LEN};
use std::io::{Cursor, Read, Write};

/// Represents a Bind Response PDU.
///
/// Sent by the SMSC in response to a Bind Request.
#[derive(Debug, Clone, PartialEq)]
pub struct BindResponse {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Command Status (0 = OK, others = Error)
    pub command_status: u32, // 0 = OK, others = Error
    /// Human-readable description of status
    pub status_description: String, // Human-readable description of status
    /// Command ID (e.g., 0x80000009 for bind_transceiver_resp)
    pub command_id: u32, // e.g., 0x80000009 (bind_transceiver_resp)
    /// System ID identifying the SMSC
    pub system_id: String, // SMSC ID
    /// Optional TLV parameters
    pub optional_params: Vec<u8>, // Rarely used in Bind Resp, but allowed
}

impl BindResponse {
    /// Create a new Bind Response.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::BindResponse;
    /// use smpp_codec::common::{CMD_BIND_TRANSCEIVER_RESP, BindMode};
    ///
    /// let sequence_number: u32 = 1;
    /// let resp = BindResponse::new(
    ///     sequence_number,
    ///     CMD_BIND_TRANSCEIVER_RESP,
    ///     "ESME_ROK",
    ///     "system_id".to_string()
    /// );
    /// ```
    pub fn new(
        sequence_number: u32,
        command_id: u32,
        status_name: &str,
        system_id: String,
    ) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_id,
            command_status,
            status_description: status_name.to_string(),
            system_id,
            optional_params: Vec::new(),
        }
    }

    /// Encode the struct into raw bytes for the network.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if:
    /// * An I/O error occurs while writing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::BindResponse;
    /// # use smpp_codec::common::CMD_BIND_TRANSMITTER_RESP;
    /// # let sequence_number: u32 = 1;
    /// # let bind_resp = BindResponse::new(sequence_number, CMD_BIND_TRANSMITTER_RESP, "ESME_ROK", "id".into());
    /// let mut buffer = Vec::new();
    /// bind_resp.encode(&mut buffer).expect("Encoding failed");
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, id = %format!("{:#010X}", self.command_id), status = %self.status_description)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding BindResponse");
        let mut body = Vec::new();

        // If status is OK, we must include system_id.
        // On error, body can be empty (or just header).
        if self.command_status == 0 {
            body.write_all(self.system_id.as_bytes())?;
            body.write_all(&[0])?; // Null terminator

            // Add optional params if any
            body.write_all(&self.optional_params)?;
        }

        let command_len = (HEADER_LEN + body.len()) as u32;

        // Write Header
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&self.command_id.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        // Write Body
        writer.write_all(&body)?;

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
    /// # use smpp_codec::pdus::BindResponse;
    /// # use smpp_codec::common::CMD_BIND_TRANSCEIVER_RESP;
    /// # let sequence_number: u32 = 1;
    /// # let resp = BindResponse::new(sequence_number, CMD_BIND_TRANSCEIVER_RESP, "ESME_ROK", "id".into());
    /// # let mut buffer = Vec::new();
    /// # resp.encode(&mut buffer).unwrap();
    /// let decoded = BindResponse::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.sequence_number, 1);
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(buffer), err, fields(seq = tracing::field::Empty, id = tracing::field::Empty, status = tracing::field::Empty, system_id = tracing::field::Empty)))]
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Decoding BindResponse from {} bytes", buffer.len());
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }

        let mut cursor = Cursor::new(buffer);

        // 1. Read Header
        let mut bytes = [0u8; 4];

        cursor.read_exact(&mut bytes)?;
        let command_len = u32::from_be_bytes(bytes) as usize;

        cursor.read_exact(&mut bytes)?;
        let command_id = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("id", format!("{:#010X}", command_id));

        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        let status_description = get_status_description(command_status);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("status", &status_description);

        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        // 2. Read Body
        let system_id: String;
        let mut optional_params = Vec::new();

        // Only try to read body if length > header
        if command_len > HEADER_LEN {
            // The cursor is currently at pos 16 (end of header).
            // Read system_id (C-String)
            if command_status == 0 {
                system_id = read_c_string(&mut cursor)?;
                #[cfg(feature = "tracing")]
                tracing::Span::current().record("system_id", &system_id);
            } else {
                system_id = String::new();
            }
        } else {
            system_id = String::new();
        }

        // Optional Params
        // optional_params reused from above
        let current_pos = cursor.position() as usize;
        if current_pos < command_len {
            let remaining_len = command_len - current_pos;
            let mut temp_buf = vec![0u8; remaining_len];
            cursor.read_exact(&mut temp_buf)?;
            optional_params = temp_buf;
        }

        Ok(Self {
            sequence_number,
            command_status,
            status_description,
            command_id,
            system_id,
            optional_params,
        })
    }
}
