use crate::common::{PduError, HEADER_LEN, CMD_ALERT_NOTIFICATION, Ton, Npi};
use crate::tlv::Tlv;
use std::io::{Write, Read, Cursor};

/// Represents an Alert Notification PDU.
///
/// Sent by the SMSC to the ESME to provide information about a message state (e.g., delivered).
#[derive(Debug, Clone)]
pub struct AlertNotification {
    pub sequence_number: u32,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: String, // Max 65
    pub esme_addr_ton: Ton,
    pub esme_addr_npi: Npi,
    pub esme_addr: String,   // Max 65
    pub optional_params: Vec<Tlv>,
}

impl AlertNotification {
    /// Create a new Alert Notification.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::AlertNotification;
    ///
    /// let alert = AlertNotification::new(
    ///     1,
    ///     "source_addr".to_string(),
    ///     "esme_addr".to_string()
    /// );
    /// ```
    pub fn new(
        sequence_number: u32,
        source_addr: String,
        esme_addr: String,
    ) -> Self {
        Self {
            sequence_number,
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
            esme_addr_ton: Ton::Unknown,
            esme_addr_npi: Npi::Unknown,
            esme_addr,
            optional_params: Vec::new(),
        }
    }
    
    // Builder for TON/NPI
    pub fn with_source_addr(mut self, ton: Ton, npi: Npi, addr: String) -> Self {
        self.source_addr_ton = ton;
        self.source_addr_npi = npi;
        self.source_addr = addr;
        self
    }
    
    pub fn with_esme_addr(mut self, ton: Ton, npi: Npi, addr: String) -> Self {
        self.esme_addr_ton = ton;
        self.esme_addr_npi = npi;
        self.esme_addr = addr;
        self
    }

    /// Add a generic TLV
    pub fn add_tlv(&mut self, tlv: Tlv) {
        self.optional_params.push(tlv);
    }

    /// Encode the struct into raw bytes for the network.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if:
    /// * `source_addr` exceeds 65 characters.
    /// * `esme_addr` exceeds 65 characters.
    /// * An I/O error occurs while writing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::AlertNotification;
    /// # let alert = AlertNotification::new(1, "src".into(), "dst".into());
    /// let mut buffer = Vec::new();
    /// alert.encode(&mut buffer).expect("Encoding failed");
    /// ```
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        // Validate lengths
        if self.source_addr.len() > 65 { return Err(PduError::StringTooLong("source_addr".into(), 65)); }
        if self.esme_addr.len() > 65 { return Err(PduError::StringTooLong("esme_addr".into(), 65)); }

        let mut body = Vec::new();
        
        // Source Address
        body.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(&mut body, &self.source_addr)?;

        // ESME Address
        body.write_all(&[self.esme_addr_ton as u8, self.esme_addr_npi as u8])?;
        write_c_string(&mut body, &self.esme_addr)?;

        // Optional Params
        // [Change] Simplified TLV Encoding
        for tlv in &self.optional_params {
            tlv.encode(&mut body)?;
        }

        // Header
        let command_len = (HEADER_LEN + body.len()) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_ALERT_NOTIFICATION.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        
        // Body
        writer.write_all(&body)?;
        
        Ok(())
    }

    /// Decode raw bytes from the network into the struct.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if:
    /// * The buffer is too short.
    /// * The buffer data is malformed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::AlertNotification;
    /// # let alert = AlertNotification::new(1, "src".into(), "dst".into());
    /// # let mut buffer = Vec::new();
    /// # alert.encode(&mut buffer).unwrap();
    /// let decoded = AlertNotification::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.source_addr, "src");
    /// ```
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN { return Err(PduError::BufferTooShort); }
        
        let mut cursor = Cursor::new(buffer);
        // Skip Header
        cursor.set_position(12);
        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        
        // Body
        let mut u8_buf = [0u8; 1];

        // Source
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_npi = Npi::from(u8_buf[0]);
        let source_addr = read_c_string(&mut cursor)?;

        // ESME
        cursor.read_exact(&mut u8_buf)?;
        let esme_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let esme_addr_npi = Npi::from(u8_buf[0]);
        let esme_addr = read_c_string(&mut cursor)?;

        // [Change] Simplified TLV Decoding
        let mut optional_params = Vec::new();
        while let Some(tlv) = Tlv::decode(&mut cursor)? {
            optional_params.push(tlv);
        }

        Ok(Self {
            sequence_number,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            esme_addr_ton,
            esme_addr_npi,
            esme_addr,
            optional_params
        })
    }
}

// Helpers (Import/Copy these as needed)
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