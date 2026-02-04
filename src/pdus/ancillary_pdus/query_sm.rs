use crate::common::{
    get_status_code, read_c_string, write_c_string, Npi, PduError, Ton, CMD_QUERY_SM, HEADER_LEN,
};
use std::io::{Cursor, Read, Write};

// --- Request ---
#[derive(Debug, Clone)]
pub struct QuerySmRequest {
    pub sequence_number: u32,
    pub message_id: String,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: String,
}

impl QuerySmRequest {
    pub fn new(sequence_number: u32, message_id: String, source_addr: String) -> Self {
        Self {
            sequence_number,
            message_id,
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
        }
    }

    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        // 1. Validation
        if self.message_id.len() > 64 {
            return Err(PduError::StringTooLong("message_id".into(), 64));
        }
        if self.source_addr.len() > 21 {
            return Err(PduError::StringTooLong("source_addr".into(), 21));
        }

        // 2. Buffer Body to Calculate Length
        let mut body = Vec::new();
        write_c_string(&mut body, &self.message_id)?;
        body.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(&mut body, &self.source_addr)?;

        // 3. Write Header
        let command_len = (HEADER_LEN + body.len()) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_QUERY_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        // 4. Write Body
        writer.write_all(&body)?;

        Ok(())
    }

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

        Ok(Self {
            sequence_number,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
        })
    }
}

/// Query_SM Response
#[derive(Debug, Clone)]
pub struct QuerySmResponse {
    pub sequence_number: u32,
    pub command_status: u32,
    pub message_id: String,
    pub final_date: String,
    pub message_state: u8,
    pub error_code: u8,
    pub status_description: String,
}

// Message States (SMPP v3.4 Spec section 5.2.28)
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MessageState {
    Enroute = 1,
    Delivered = 2,
    Expired = 3,
    Deleted = 4,
    Undeliverable = 5,
    Accepted = 6,
    Unknown = 7,
    Rejected = 8,
}

impl QuerySmResponse {
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

    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let mut body = Vec::new();

        if self.command_status == 0 {
            write_c_string(&mut body, &self.message_id)?;
            write_c_string(&mut body, &self.final_date)?;
            body.write_all(&[self.message_state, self.error_code])?;
        }

        let command_len = (HEADER_LEN + body.len()) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&crate::common::CMD_QUERY_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        writer.write_all(&body)?;

        Ok(())
    }

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

        let message_id: String;
        let final_date: String;
        let message_state: u8;
        let error_code: u8;

        if command_status == 0 && buffer.len() > HEADER_LEN {
            message_id = read_c_string(&mut cursor)?;
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

        let status_description = crate::common::get_status_description(command_status);

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
