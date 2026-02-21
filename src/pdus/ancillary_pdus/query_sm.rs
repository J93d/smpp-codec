use crate::common::{
    get_status_code, read_c_string, write_c_string, Npi, PduError, Ton, CMD_QUERY_SM, HEADER_LEN,
};
use std::io::{Cursor, Read, Write};

// --- Request ---
/// Represents a Query SM Request PDU.
///
/// Used to query the status of a previously submitted message.
#[derive(Debug, Clone, PartialEq)]
pub struct QuerySmRequest {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Message ID of the message to query
    pub message_id: String,
    /// Source Address Type of Number
    pub source_addr_ton: Ton,
    /// Source Address Numbering Plan Indicator
    pub source_addr_npi: Npi,
    /// Source Address
    pub source_addr: String,
}

impl QuerySmRequest {
    /// Create a new Query SM Request.
    pub fn new(sequence_number: u32, message_id: String, source_addr: String) -> Self {
        Self {
            sequence_number,
            message_id,
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
        }
    }

    /// Encode the PDU into the writer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the write fails or strings are too long.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, message_id = %self.message_id, src = %self.source_addr)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding QuerySmRequest");
        // 1. Validation
        if self.message_id.len() > 64 {
            return Err(PduError::StringTooLong("message_id".into(), 64));
        }
        if self.source_addr.len() > 21 {
            return Err(PduError::StringTooLong("source_addr".into(), 21));
        }

        // 2. Calculate Length Upfront
        let body_len = self.message_id.len() + 1 +
                       1 + 1 + // source_addr_ton, source_addr_npi
                       self.source_addr.len() + 1;

        // 3. Write Header
        let command_len = (HEADER_LEN + body_len) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_QUERY_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        // 4. Write Body
        write_c_string(writer, &self.message_id)?;
        writer.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(writer, &self.source_addr)?;

        Ok(())
    }

    /// Decode the PDU from the buffer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the buffer is too short or malformed.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(buffer), err, fields(seq = tracing::field::Empty, message_id = tracing::field::Empty, src = tracing::field::Empty)))]
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Decoding QuerySmRequest from {} bytes", buffer.len());
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }

        let mut cursor = Cursor::new(buffer);
        cursor.set_position(12);

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        let message_id = read_c_string(&mut cursor)?;
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("message_id", &message_id);

        let mut u8_buf = [0u8; 1];
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_npi = Npi::from(u8_buf[0]);

        let source_addr = read_c_string(&mut cursor)?;
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("src", &source_addr);

        Ok(Self {
            sequence_number,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
        })
    }
}

/// Represents a Query SM Response PDU.
#[derive(Debug, Clone, PartialEq)]
pub struct QuerySmResponse {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Command Status
    pub command_status: u32,
    /// Message ID
    pub message_id: String,
    /// Final Date (Format: YYMMDDhhmmsstn00)
    pub final_date: String,
    /// Message State (See [`MessageState`])
    pub message_state: u8,
    /// Error Code (Network specific error code)
    pub error_code: u8,
    /// Status Description
    pub status_description: String,
}

/// Message States (as per SMPP v3.4 Spec section 5.2.28)
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MessageState {
    /// Message is in enroute state
    Enroute = 1,
    /// Message is delivered
    Delivered = 2,
    /// Message validity period has expired
    Expired = 3,
    /// Message has been deleted
    Deleted = 4,
    /// Message is undeliverable
    Undeliverable = 5,
    /// Message is in accepted state
    Accepted = 6,
    /// Message is in invalid state
    Unknown = 7,
    /// Message is in rejected state
    Rejected = 8,
}

impl QuerySmResponse {
    /// Create a new Query SM Response.
    pub fn new(
        sequence_number: u32,
        status_name: &str,
        message_id: String,
        final_date: String,
        message_state: u8,
        error_code: u8,
    ) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            message_id,
            final_date,
            message_state,
            error_code,
            status_description: status_name.to_string(),
        }
    }

    /// Encode the PDU into the writer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the write fails.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, status = %self.status_description, message_id = %self.message_id)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding QuerySmResponse");
        let body_len = if self.command_status == 0 {
            self.message_id.len() + 1 + self.final_date.len() + 1 + 1 + 1 // message_state, error_code
        } else {
            0
        };

        let command_len = (HEADER_LEN + body_len) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&crate::common::CMD_QUERY_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        if self.command_status == 0 {
            write_c_string(writer, &self.message_id)?;
            write_c_string(writer, &self.final_date)?;
            writer.write_all(&[self.message_state, self.error_code])?;
        }

        Ok(())
    }

    /// Decode the PDU from the buffer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the buffer is too short or malformed.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(buffer), err, fields(seq = tracing::field::Empty, status = tracing::field::Empty, message_id = tracing::field::Empty)))]
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Decoding QuerySmResponse from {} bytes", buffer.len());
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(8);

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        let status_description = crate::common::get_status_description(command_status);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("status", &status_description);

        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        let message_id: String;
        let final_date: String;
        let message_state: u8;
        let error_code: u8;

        if command_status == 0 && buffer.len() > HEADER_LEN {
            let id = read_c_string(&mut cursor)?;
            #[cfg(feature = "tracing")]
            tracing::Span::current().record("message_id", &id);
            message_id = id;
            final_date = read_c_string(&mut cursor)?;

            let mut u8_buf = [0u8; 1];
            cursor.read_exact(&mut u8_buf)?;
            message_state = u8_buf[0];
            cursor.read_exact(&mut u8_buf)?;
            error_code = u8_buf[0];
        } else {
            message_id = String::new();
            final_date = String::new();
            message_state = 0;
            error_code = 0;
        }

        Ok(Self {
            sequence_number,
            command_status,
            message_id,
            final_date,
            message_state,
            error_code,
            status_description,
        })
    }
}
