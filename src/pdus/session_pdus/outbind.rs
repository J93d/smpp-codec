use crate::common::{PduError, HEADER_LEN, CMD_OUTBIND};
use std::io::{Read, Write, Cursor};

/// Represents an Outbind PDU.
///
/// Sent by the SMSC to the ESME to request the ESME to initiate a Bind.
#[derive(Debug, Clone)]
pub struct OutbindRequest {
    pub sequence_number: u32,
    pub system_id: String,
    pub password: String,
}

impl OutbindRequest {
    pub fn new(system_id: String, password: String, sequence_number: u32) -> Self {
        Self {
            sequence_number,
            system_id,
            password,
        }
    }

    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        // Validate
        if self.system_id.len() > 16 {
            return Err(PduError::StringTooLong("system_id".into(), 16));
        }
        if self.password.len() > 9 {
            return Err(PduError::StringTooLong("password".into(), 9));
        }

        let mut body = Vec::new();
        write_c_string(&mut body, &self.system_id)?;
        write_c_string(&mut body, &self.password)?;

        let command_len = (HEADER_LEN + body.len()) as u32;

        // Header
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_OUTBIND.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?; // Status is always 0
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        // Body
        writer.write_all(&body)?;

        Ok(())
    }

    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }

        let mut cursor = Cursor::new(buffer);
        
        // Skip Header (assuming caller checked ID, or we just consume it)
        cursor.set_position(12); // Skip len, id, status
        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        // Body
        let system_id = read_c_string(&mut cursor)?;
        let password = read_c_string(&mut cursor)?;

        Ok(Self {
            sequence_number,
            system_id,
            password,
        })
    }
}

// Helper (copy/import this)
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