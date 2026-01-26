use crate::common::{PduError, HEADER_LEN, CMD_ENQUIRE_LINK};
use std::io::{Write, Read, Cursor};

// --- EnquireLink Request ---
#[derive(Debug, Clone)]
pub struct EnquireLink {
    pub sequence_number: u32,
}

impl EnquireLink {
    pub fn new(sequence_number: u32) -> Self {
        Self { sequence_number }
    }

    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let command_len = HEADER_LEN as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_ENQUIRE_LINK.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?; // Status always 0
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        Ok(())
    }

    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(12); // Skip len, id, status
        
        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        Ok(Self { sequence_number })
    }
}

