use crate::common::{
    get_status_code, get_status_description, read_c_string, write_c_string, Npi, PduError, Ton,
    CMD_QUERY_BROADCAST_SM, CMD_QUERY_BROADCAST_SM_RESP, HEADER_LEN,
};
use crate::tlv::Tlv;
use std::io::{Cursor, Read, Write};

// --- Request ---

/// Represents a Query Broadcast SM PDU.
///
/// This PDU is used to query the status of a previously submitted broadcast message.
#[derive(Debug, Clone, PartialEq)]
pub struct QueryBroadcastSm {
    pub sequence_number: u32,
    pub message_id: String,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: String,
    pub optional_params: Vec<Tlv>,
}

impl QueryBroadcastSm {
    /// Create a new Query Broadcast SM PDU.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::QueryBroadcastSm;
    ///
    /// let pdu = QueryBroadcastSm::new(
    ///     1,
    ///     "bc_msg_123".to_string(),
    ///     "Source".to_string(),
    /// );
    /// ```
    pub fn new(sequence_number: u32, message_id: String, source_addr: String) -> Self {
        Self {
            sequence_number,
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
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let tlvs_len: usize = self
            .optional_params
            .iter()
            .map(|tlv| 4 + tlv.length as usize)
            .sum();
        let body_len = self.message_id.len() + 1 + 1 + 1 + self.source_addr.len() + 1 + tlvs_len;

        let command_len = (HEADER_LEN + body_len) as u32;

        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_QUERY_BROADCAST_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

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
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(12);

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        let message_id = read_c_string(&mut cursor)?;

        let mut u8_buf = [0u8; 1];
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_npi = Npi::from(u8_buf[0]);
        let source_addr = read_c_string(&mut cursor)?;

        let mut optional_params = Vec::new();
        while let Some(tlv) = Tlv::decode(&mut cursor)? {
            optional_params.push(tlv);
        }

        Ok(Self {
            sequence_number,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            optional_params,
        })
    }
}

// --- Response ---

/// Represents a Query Broadcast SM Response PDU.
#[derive(Debug, Clone, PartialEq)]
pub struct QueryBroadcastSmResp {
    pub sequence_number: u32,
    pub command_status: u32,        // 0 = OK, others = Error
    pub status_description: String, // Human-readable description of status
    pub message_id: String,
    pub optional_params: Vec<Tlv>, // Likely contains 'message_state', 'broadcast_area_success'
}

impl QueryBroadcastSmResp {
    /// Create a new Query Broadcast SM Response PDU.
    pub fn new(sequence_number: u32, status_name: &str, message_id: String) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            status_description: status_name.to_string(),
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
        let tlvs_len: usize = self
            .optional_params
            .iter()
            .map(|tlv| 4 + tlv.length as usize)
            .sum();
        let body_len = self.message_id.len() + 1 + tlvs_len;

        let command_len = (HEADER_LEN + body_len) as u32;

        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_QUERY_BROADCAST_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        write_c_string(writer, &self.message_id)?;

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
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(8);

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
