use crate::common::{PduError, HEADER_LEN, CMD_QUERY_SM, CMD_QUERY_SM_RESP, Ton, Npi, get_status_code, get_status_description};
use std::io::{Read, Write, Cursor};

// --- Request ---
#[derive(Debug, Clone)]
pub struct QuerySm {
    pub sequence_number: u32,
    pub message_id: String,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: String,
}

impl QuerySm {
    pub fn new(
        sequence_number: u32,
        message_id: String,
        source_addr: String,
    ) -> Self {
        Self {
            sequence_number,
            message_id,
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
        }
    }

    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let mut body = Vec::new();
        write_c_string(&mut body, &self.message_id)?;
        body.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(&mut body, &self.source_addr)?;

        let command_len = (HEADER_LEN + body.len()) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_QUERY_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        writer.write_all(&body)?;
        Ok(())
    }

    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN { return Err(PduError::BufferTooShort); }
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

// --- Response ---
#[derive(Debug, Clone)]
pub struct QuerySmResp {
    pub sequence_number: u32,
    pub command_status: u32,
    pub message_id: String,
    pub final_date: String, // format: "YYMMDDhhmm"
    pub message_state: u8,  // See MessageState enum below
    pub error_code: u8,
    pub status_description: String,
}

// Message States (SMPP v3.4 Spec section 5.2.28)
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

impl QuerySmResp {
    pub fn new(sequence_number: u32, status_name: &str, message_id: String, state: u8) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            message_id,
            final_date: String::new(), // Optional in many implementations
            message_state: state,
            error_code: 0,
            status_description: status_name.to_string(),
        }
    }

    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let mut body = Vec::new();
        // Only write body if success
        if self.command_status == 0 {
            write_c_string(&mut body, &self.message_id)?;
            write_c_string(&mut body, &self.final_date)?;
            body.write_all(&[self.message_state, self.error_code])?;
        }

        let command_len = (HEADER_LEN + body.len()) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_QUERY_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        writer.write_all(&body)?;
        Ok(())
    }

    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN { return Err(PduError::BufferTooShort); }
        let mut cursor = Cursor::new(buffer);
        
        // Skip Length(4) + ID(4)
        cursor.set_position(8);
        
        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        let mut message_id = String::new();
        let mut final_date = String::new();
        let mut message_state = 0;
        let mut error_code = 0;

        if command_status == 0 {
             message_id = read_c_string(&mut cursor)?;
             final_date = read_c_string(&mut cursor)?;
             
             let mut u8_buf = [0u8; 1];
             cursor.read_exact(&mut u8_buf)?;
             message_state = u8_buf[0];
             cursor.read_exact(&mut u8_buf)?;
             error_code = u8_buf[0];
        }

        let status_description = get_status_description(command_status);

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

// Private Helpers (Duplicate for simplicity to avoid import cycles)
fn write_c_string(w: &mut impl Write, s: &str) -> std::io::Result<()> {
    w.write_all(s.as_bytes())?;
    w.write_all(&[0])
}

fn read_c_string(cursor: &mut Cursor<&[u8]>) -> Result<String, PduError> {
    let mut bytes = Vec::new();
    let mut buf = [0u8; 1];
    loop {
        if cursor.read(&mut buf)? == 0 { break; }
        if buf[0] == 0 { break; }
        bytes.push(buf[0]);
    }
    String::from_utf8(bytes).map_err(|e| PduError::Utf8(e))
}