use crate::common::{read_c_string, write_c_string, Npi, PduError, Ton, CMD_DATA_SM, HEADER_LEN};
use crate::tlv::{tags, Tlv};
use std::io::{Cursor, Read, Write};

/// Represents a Data SM Request PDU.
///
/// Used to transfer data between the SMSC and the ESME.
/// It is an alternative to `SubmitSm` and `DeliverSm`.
#[derive(Debug, Clone, PartialEq)]
pub struct DataSm {
    pub sequence_number: u32,
    pub service_type: String,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: String,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub dest_addr: String,
    pub esm_class: u8,
    pub registered_delivery: u8,
    pub data_coding: u8,
    pub optional_params: Vec<Tlv>, // Payload goes here via 'message_payload'
}

impl DataSm {
    /// Create a new Data SM Request.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::DataSm;
    ///
    /// let pdu = DataSm::new(
    ///     1,
    ///     "Source".to_string(),
    ///     "Dest".to_string(),
    ///     b"Payload".to_vec(),
    /// );
    /// ```
    pub fn new(
        sequence_number: u32,
        source_addr: String,
        dest_addr: String,
        payload: Vec<u8>,
    ) -> Self {
        let mut pdu = Self {
            sequence_number,
            service_type: String::new(),
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
            dest_addr_ton: Ton::Unknown,
            dest_addr_npi: Npi::Unknown,
            dest_addr,
            esm_class: 0,
            registered_delivery: 0,
            data_coding: 0,
            optional_params: Vec::new(),
        };

        // DataSm relies on TLV for the body
        pdu.add_tlv(Tlv::new(tags::MESSAGE_PAYLOAD, payload));
        pdu
    }

    /// Add a TLV to the optional parameters.
    pub fn add_tlv(&mut self, tlv: Tlv) {
        self.optional_params.push(tlv);
    }

    /// Encode the PDU into the writer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the write fails.
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        // Calculate Body Length
        // ServiceType(N+1)
        // Src(1+1+N+1)
        // Dest(1+1+N+1)
        // Flags(1+1+1)
        // TLVs
        let tlvs_len: usize = self
            .optional_params
            .iter()
            .map(|tlv| 4 + tlv.length as usize)
            .sum();

        let body_len = self.service_type.len() + 1 +
                       1 + 1 + self.source_addr.len() + 1 +
                       1 + 1 + self.dest_addr.len() + 1 +
                       1 + 1 + 1 + // esm_class, reg_del, data_coding
                       tlvs_len;

        let command_len = (HEADER_LEN + body_len) as u32;

        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_DATA_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        write_c_string(writer, &self.service_type)?;

        writer.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(writer, &self.source_addr)?;

        writer.write_all(&[self.dest_addr_ton as u8, self.dest_addr_npi as u8])?;
        write_c_string(writer, &self.dest_addr)?;

        writer.write_all(&[self.esm_class, self.registered_delivery, self.data_coding])?;

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

        let service_type = read_c_string(&mut cursor)?;

        let mut u8_buf = [0u8; 1];

        // Source
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_npi = Npi::from(u8_buf[0]);
        let source_addr = read_c_string(&mut cursor)?;

        // Destination
        cursor.read_exact(&mut u8_buf)?;
        let dest_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let dest_addr_npi = Npi::from(u8_buf[0]);
        let dest_addr = read_c_string(&mut cursor)?;

        // Flags
        cursor.read_exact(&mut u8_buf)?;
        let esm_class = u8_buf[0];
        cursor.read_exact(&mut u8_buf)?;
        let registered_delivery = u8_buf[0];
        cursor.read_exact(&mut u8_buf)?;
        let data_coding = u8_buf[0];

        let mut optional_params = Vec::new();
        while let Some(tlv) = Tlv::decode(&mut cursor)? {
            optional_params.push(tlv);
        }

        Ok(Self {
            sequence_number,
            service_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            dest_addr,
            esm_class,
            registered_delivery,
            data_coding,
            optional_params,
        })
    }
}
