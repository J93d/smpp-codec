use crate::common::{PduError, HEADER_LEN, CMD_ENQUIRE_LINK_RESP, get_status_code, get_status_description};
use std::io::{Write, Read, Cursor};

// --- EnquireLink Response ---
#[derive(Debug, Clone)]
pub struct EnquireLinkResp {
    pub sequence_number: u32,
    pub command_status: u32, // 0 = OK, others = Error
    pub status_description: String, // Human-readable description of status
}

impl EnquireLinkResp {
    pub fn new(
        sequence_number: u32,
        command_id: u32,
        status_name: &str,
    ) -> Self {
        let command_status = get_status_code(status_name);
        Self { 
            sequence_number, 
            command_status,
            status_description: status_name.to_string(),
        }
    }

    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let command_len = HEADER_LEN as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_ENQUIRE_LINK_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        Ok(())
    }

    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        
        // Skip Length (4) and ID (4)
        cursor.set_position(8); 

        let mut bytes = [0u8; 4];
        
        // [Change] Read Status
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);

        // Read Sequence
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        Ok(Self { 
            sequence_number, 
            command_status, 
            status_description: get_status_description(command_status),
        })
    }
}