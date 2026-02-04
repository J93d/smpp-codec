use crate::common::{read_c_string, write_c_string, BindMode, Npi, PduError, Ton, HEADER_LEN};
use std::io::{Cursor, Read, Write};

/// Represents a Bind Request PDU (Receiver, Transmitter, or Transceiver).
///
/// This PDU is used to initiate a session with the SMSC.
#[derive(Debug, Clone, PartialEq)]
pub struct BindRequest {
    pub sequence_number: u32,
    pub mode: BindMode,
    pub system_id: String,
    pub password: String,
    pub system_type: String,
    pub interface_version: u8,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: String,
}

impl BindRequest {
    /// Create a new Bind Request with defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::BindRequest;
    /// use smpp_codec::common::BindMode;
    ///
    /// let sequence_number: u32 = 1;
    /// let bind_req = BindRequest::new(
    ///     sequence_number,
    ///     BindMode::Transceiver,
    ///     "system_id".to_string(),
    ///     "password".to_string(),
    /// );
    /// ```
    pub fn new(sequence_number: u32, mode: BindMode, system_id: String, password: String) -> Self {
        Self {
            sequence_number,
            mode,
            system_id,
            password,
            system_type: String::new(),
            interface_version: 0x34, // SMPP 3.4
            addr_ton: Ton::Unknown,
            addr_npi: Npi::Unknown,
            address_range: String::new(),
        }
    }

    pub fn with_address_range(mut self, ton: Ton, npi: Npi, range: String) -> Self {
        self.addr_ton = ton;
        self.addr_npi = npi;
        self.address_range = range;
        self
    }

    /// Encode the struct into raw bytes for the network.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if:
    /// * `system_id` exceeds 16 characters.
    /// * `password` exceeds 9 characters.
    /// * `system_type` exceeds 13 characters.
    /// * `address_range` exceeds 41 characters.
    /// * An I/O error occurs while writing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::BindRequest;
    /// # use smpp_codec::common::BindMode;
    /// # let sequence_number: u32 = 1;
    /// # let bind_req = BindRequest::new(sequence_number, BindMode::Transmitter, "id".into(), "pwd".into());
    /// let mut buffer = Vec::new();
    /// bind_req.encode(&mut buffer).expect("Encoding failed");
    /// ```
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

        if self.address_range.len() > 41 {
            return Err(PduError::StringTooLong("address_range".into(), 41));
        }

        // 2. Calculate Length Upfront
        // Header (16) + SystemID (N+1) + Password (N+1) + SystemType (N+1) + Ver(1) + Ton(1) + Npi(1) + Range(N+1)
        let body_len = self.system_id.len() + 1 +
                       self.password.len() + 1 +
                       self.system_type.len() + 1 +
                       1 + // interface_version
                       1 + // addr_ton
                       1 + // addr_npi
                       self.address_range.len() + 1;

        let command_len = (HEADER_LEN + body_len) as u32;

        // 3. Write Header
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&self.mode.command_id().to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?; // Command Status
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        // 4. Write Body
        write_c_string(writer, &self.system_id)?;
        write_c_string(writer, &self.password)?;
        write_c_string(writer, &self.system_type)?;
        writer.write_all(&[self.interface_version])?;
        writer.write_all(&[self.addr_ton as u8, self.addr_npi as u8])?;
        write_c_string(writer, &self.address_range)?;

        Ok(())
    }

    /// Decode raw bytes from the network into the struct.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if:
    /// * The buffer is too short to contain a valid header.
    /// * The command ID does not correspond to a Bind Request.
    /// * The buffer data is malformed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::BindRequest;
    /// # use smpp_codec::common::BindMode;
    /// # let bind_req = BindRequest::new(1, BindMode::Transmitter, "id".into(), "pwd".into());
    /// # let mut buffer = Vec::new();
    /// # bind_req.encode(&mut buffer).unwrap();
    /// let decoded = BindRequest::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.system_id, "id");
    /// ```
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        // 1. Validate total length
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }

        let mut cursor = Cursor::new(buffer);

        // 2. Read Header
        let mut bytes = [0u8; 4];

        // Command Length
        cursor.read_exact(&mut bytes)?;
        let command_len = u32::from_be_bytes(bytes) as usize;

        if buffer.len() != command_len {
            // We can be strict or loose here. Strict is safer for libraries.
            // return Err(PduError::InvalidLength);
        }

        // Command ID
        cursor.read_exact(&mut bytes)?;
        let command_id = u32::from_be_bytes(bytes);

        // Map Command ID to BindMode
        let mode = match command_id {
            0x00000001 => BindMode::Receiver,
            0x00000002 => BindMode::Transmitter,
            0x00000009 => BindMode::Transceiver,
            _ => return Err(PduError::InvalidCommandId(command_id)),
        };

        // Command Status
        cursor.read_exact(&mut bytes)?;
        let _command_status = u32::from_be_bytes(bytes);

        // Sequence Number
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        // 3. Read Body (C-Strings and u8s)
        let system_id = read_c_string(&mut cursor)?;
        let password = read_c_string(&mut cursor)?;
        let system_type = read_c_string(&mut cursor)?;

        // Simple u8 reads
        let mut u8_buf = [0u8; 1];
        cursor.read_exact(&mut u8_buf)?;
        let interface_version = u8_buf[0];

        cursor.read_exact(&mut u8_buf)?;
        let addr_ton = Ton::from(u8_buf[0]); // Convert byte -> Enum

        cursor.read_exact(&mut u8_buf)?;
        let addr_npi = Npi::from(u8_buf[0]); // Convert byte -> Enum

        let address_range = read_c_string(&mut cursor)?;

        Ok(Self {
            sequence_number,
            mode,
            system_id,
            password,
            system_type,
            interface_version,
            addr_ton,
            addr_npi,
            address_range,
        })
    }
}
