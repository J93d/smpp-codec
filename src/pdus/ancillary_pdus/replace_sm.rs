use crate::common::{
    get_status_code, get_status_description, read_c_string, write_c_string, Npi, PduError, Ton,
    CMD_REPLACE_SM, CMD_REPLACE_SM_RESP, HEADER_LEN,
};
use std::io::{Cursor, Read, Write};

// --- Request ---

/// Represents a Replace SM PDU.
///
/// This PDU is used to replace a previously submitted short message that is still pending delivery.
#[derive(Debug, Clone)]
pub struct ReplaceSm {
    pub sequence_number: u32,
    pub message_id: String, // The ID of the message to replace
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: String,
    pub schedule_delivery_time: String,
    pub validity_period: String,
    pub registered_delivery: u8,
    pub sm_default_msg_id: u8,
    pub short_message: Vec<u8>, // The NEW message body
}

impl ReplaceSm {
    /// Create a new Replace SM PDU.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::ReplaceSm;
    ///
    /// let pdu = ReplaceSm::new(
    ///     1,
    ///     "msg_123".to_string(),
    ///     "Source".to_string(),
    ///     b"New Content".to_vec(),
    /// );
    /// ```
    pub fn new(
        sequence_number: u32,
        message_id: String,
        source_addr: String,
        new_message: Vec<u8>,
    ) -> Self {
        Self {
            sequence_number,
            message_id,
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
            schedule_delivery_time: String::new(),
            validity_period: String::new(),
            registered_delivery: 0,
            sm_default_msg_id: 0,
            short_message: new_message,
        }
    }

    /// Encode the PDU into the writer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the write fails.
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        // Calculate Body Length
        // MessageId(N+1)
        // Source(1+1+N+1)
        // Sched(N+1) + Validity(N+1)
        // Reg(1) + DefMsgId(1) + SmLen(1) + SmData(N)
        let body_len = self.message_id.len()
            + 1
            + 1
            + 1
            + self.source_addr.len()
            + 1
            + self.schedule_delivery_time.len()
            + 1
            + self.validity_period.len()
            + 1
            + 1
            + 1
            + 1
            + self.short_message.len();

        let command_len = (HEADER_LEN + body_len) as u32;

        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_REPLACE_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        write_c_string(writer, &self.message_id)?;
        writer.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(writer, &self.source_addr)?;
        write_c_string(writer, &self.schedule_delivery_time)?;
        write_c_string(writer, &self.validity_period)?;

        writer.write_all(&[
            self.registered_delivery,
            self.sm_default_msg_id,
            self.short_message.len() as u8,
        ])?;

        writer.write_all(&self.short_message)?;

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

        let schedule_delivery_time = read_c_string(&mut cursor)?;
        let validity_period = read_c_string(&mut cursor)?;

        cursor.read_exact(&mut u8_buf)?;
        let registered_delivery = u8_buf[0];
        cursor.read_exact(&mut u8_buf)?;
        let sm_default_msg_id = u8_buf[0];

        cursor.read_exact(&mut u8_buf)?;
        let sm_length = u8_buf[0] as usize;
        let mut short_message = vec![0u8; sm_length];
        cursor.read_exact(&mut short_message)?;

        Ok(Self {
            sequence_number,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            sm_default_msg_id,
            short_message,
        })
    }
}

// --- Response ---

/// Represents a Replace SM Response PDU.
#[derive(Debug, Clone)]
pub struct ReplaceSmResp {
    pub sequence_number: u32,
    pub command_status: u32,        // 0 = OK, others = Error
    pub status_description: String, // Human-readable description of status
}

impl ReplaceSmResp {
    /// Create a new Replace SM Response PDU.
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
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        // Header Only PDU (Body Length = 0)
        let command_len = HEADER_LEN as u32;

        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_REPLACE_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
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
        cursor.set_position(8); // Skip Len, ID

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
