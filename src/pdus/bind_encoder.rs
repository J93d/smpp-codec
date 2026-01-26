use crate::common::{BindMode, PduError, HEADER_LEN};
use std::io::{Read, Write, Cursor};

#[derive(Debug, Clone)]
pub struct BindRequest {
    // Header Info
    pub sequence_number: u32,
    pub mode: BindMode,

    // Body Fields
    pub system_id: String,      // Max 16
    pub password: String,       // Max 9
    pub system_type: String,    // Max 13
    pub interface_version: u8,  // Usually 0x34
    pub addr_ton: u8,
    pub addr_npi: u8,
    pub address_range: String,  // Max 41
}

impl BindRequest {
    /// Create a new Bind Request with defaults
    pub fn new(
        mode: BindMode,
        system_id: String,
        password: String,
        sequence_number: u32,
    ) -> Self {
        Self {
            sequence_number,
            mode,
            system_id,
            password,
            system_type: String::new(),
            interface_version: 0x34, // SMPP 3.4
            addr_ton: 0,
            addr_npi: 0,
            address_range: String::new(),
        }
    }

    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        // 1. Validate Constraints
        if self.system_id.len() > 16 {
            return Err(PduError::StringTooLong("system_id".into(), 16));
        }
        if self.password.len() > 9 {
            return Err(PduError::StringTooLong("password".into(), 9));
        }
        if self.system_type.len() > 13 {
            return Err(PduError::StringTooLong("system_type".into(), 13));
        }

        // 2. Buffer the body first to calculate length
        let mut body = Vec::new();
        
        write_c_string(&mut body, &self.system_id)?;
        write_c_string(&mut body, &self.password)?;
        write_c_string(&mut body, &self.system_type)?;
        body.write_all(&[self.interface_version])?;
        body.write_all(&[self.addr_ton, self.addr_npi])?;
        write_c_string(&mut body, &self.address_range)?;

        // 3. Write Header
        let command_len = (HEADER_LEN + body.len()) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&self.mode.command_id().to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?; // Command Status (Always 0 for requests)
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        // 4. Write Body
        writer.write_all(&body)?;

        Ok(())
    }
}

// Helper to write Null-Terminated Strings (C-Strings)
fn write_c_string(w: &mut impl Write, s: &str) -> std::io::Result<()> {
    w.write_all(s.as_bytes())?;
    w.write_all(&[0])
}