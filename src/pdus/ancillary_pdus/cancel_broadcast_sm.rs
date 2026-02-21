use crate::common::{
    get_status_code, get_status_description, read_c_string, write_c_string, Npi, PduError, Ton,
    CMD_CANCEL_BROADCAST_SM, CMD_CANCEL_BROADCAST_SM_RESP, HEADER_LEN,
};
use crate::tlv::Tlv;
use std::io::{Cursor, Read, Write};

// --- Request ---

/// Represents a Cancel Broadcast SM PDU.
///
/// This PDU is used to cancel a previously submitted broadcast message.
#[derive(Debug, Clone, PartialEq)]
pub struct CancelBroadcastSmRequest {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Service Type
    pub service_type: String,
    /// Message ID of the message to cancel
    pub message_id: String,
    /// Source Address Type of Number
    pub source_addr_ton: Ton,
    /// Source Address Numbering Plan Indicator
    pub source_addr_npi: Npi,
    /// Source Address
    pub source_addr: String,
    /// Optional TLVs
    pub optional_params: Vec<Tlv>,
}

impl CancelBroadcastSmRequest {
    /// Create a new Cancel Broadcast SM PDU.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::CancelBroadcastSmRequest;
    ///
    /// let pdu = CancelBroadcastSmRequest::new(
    ///     1,
    ///     "CMT".to_string(),
    ///     "bc_msg_123".to_string(),
    ///     "Source".to_string(),
    /// );
    /// ```
    pub fn new(
        sequence_number: u32,
        service_type: String,
        message_id: String,
        source_addr: String,
    ) -> Self {
        Self {
            sequence_number,
            service_type,
            message_id,
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
            optional_params: Vec::new(),
        }
    }

    /// Encode the PDU into the writer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the write fails.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, message_id = %self.message_id, src = %self.source_addr)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding CancelBroadcastSmRequest");
        let tlvs_len: usize = self
            .optional_params
            .iter()
            .map(|tlv| 4 + tlv.length as usize)
            .sum();
        let body_len = self.service_type.len()
            + 1
            + self.message_id.len()
            + 1
            + 1
            + 1
            + self.source_addr.len()
            + 1
            + tlvs_len;

        let command_len = (HEADER_LEN + body_len) as u32;

        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_CANCEL_BROADCAST_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        write_c_string(writer, &self.service_type)?;
        write_c_string(writer, &self.message_id)?;
        writer.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(writer, &self.source_addr)?;

        for tlv in &self.optional_params {
            tlv.encode(writer)?;
        }
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
        tracing::debug!(
            "Decoding CancelBroadcastSmRequest from {} bytes",
            buffer.len()
        );
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

        let service_type = read_c_string(&mut cursor)?;
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

        let mut optional_params = Vec::new();
        while let Some(tlv) = Tlv::decode(&mut cursor)? {
            optional_params.push(tlv);
        }

        Ok(Self {
            sequence_number,
            service_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            optional_params,
        })
    }
}

// --- Response ---

/// Represents a Cancel Broadcast SM Response PDU.
#[derive(Debug, Clone, PartialEq)]
pub struct CancelBroadcastSmResponse {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Command Status (0 = OK, others = Error)
    pub command_status: u32, // 0 = OK, others = Error
    /// Human-readable description of status
    pub status_description: String, // Human-readable description of status
                                    // Body is empty (header only)
}

impl CancelBroadcastSmResponse {
    /// Create a new Cancel Broadcast SM Response PDU.
    /// Create a new Cancel Broadcast SM Response PDU.
    pub fn new(sequence_number: u32, status_name: &str) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            status_description: status_name.to_string(),
        }
    }

    /// Encode the PDU into the writer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the write fails.
    /// Encode the PDU into the writer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the write fails.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, status = %self.status_description)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding CancelBroadcastSmResponse");
        let command_len = HEADER_LEN as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_CANCEL_BROADCAST_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        Ok(())
    }

    /// Decode the PDU from the buffer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the buffer is too short or malformed.
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
        cursor.set_position(8);

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

        Ok(Self {
            sequence_number,
            command_status,
            status_description,
        })
    }
}
