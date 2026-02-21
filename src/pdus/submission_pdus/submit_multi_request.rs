use crate::common::{
    read_c_string, write_c_string, Npi, PduError, Ton, CMD_SUBMIT_MULTI_SM, HEADER_LEN,
};
use crate::tlv::Tlv;
use std::io::Read;
use std::io::{Cursor, Write};

#[derive(Debug, Clone, PartialEq)]
/// Destination for Submit Multi (SME Address or Distribution List)
pub enum Destination {
    /// Normal SME Address
    SmeAddress {
        /// Type of Number
        ton: Ton,
        /// Numbering Plan Indicator
        npi: Npi,
        /// Address
        address: String,
    },
    /// Distribution List Name
    DistributionList(String),
}

/// Represents a Submit Multi PDU.
///
/// Used to submit a short message to multiple recipients (SME addresses or Distribution Lists).
#[derive(Debug, Clone, PartialEq)]
pub struct SubmitMultiRequest {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Service Type
    pub service_type: String,
    /// Source Address Type of Number
    pub source_addr_ton: Ton,
    /// Source Address Numbering Plan Indicator
    pub source_addr_npi: Npi,
    /// Source Address
    pub source_addr: String,
    /// List of Destinations (Max 255)
    pub destinations: Vec<Destination>,
    /// ESM Class
    pub esm_class: u8,
    /// Protocol Identifier
    pub protocol_id: u8,
    /// Priority Level
    pub priority_flag: u8,
    /// Scheduled Delivery Time
    pub schedule_delivery_time: String,
    /// Validity Period
    pub validity_period: String,
    /// Registered Delivery
    pub registered_delivery: u8,
    /// Replace If Present Flag
    pub replace_if_present_flag: u8,
    /// Data Coding Scheme
    pub data_coding: u8,
    /// SMSC Default Message ID
    pub sm_default_msg_id: u8,
    /// Short Message Data
    pub short_message: Vec<u8>,
    /// Optional Parameters (TLVs)
    pub optional_params: Vec<Tlv>,
}

impl SubmitMultiRequest {
    /// Create a new Submit Multi PDU.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::{SubmitMultiRequest, Destination};
    /// use smpp_codec::common::{Ton, Npi};
    ///
    /// let dest1 = Destination::SmeAddress {
    ///     ton: Ton::International,
    ///     npi: Npi::Isdn,
    ///     address: "1234567890".to_string(),
    /// };
    /// let dest2 = Destination::DistributionList("MyList".to_string());
    ///
    /// let pdu = SubmitMultiRequest::new(
    ///     1,
    ///     "Source".to_string(),
    ///     vec![dest1, dest2],
    ///     b"Hello World".to_vec(),
    /// );
    /// ```
    pub fn new(
        sequence_number: u32,
        source_addr: String,
        destinations: Vec<Destination>,
        short_message: Vec<u8>,
    ) -> Self {
        Self {
            sequence_number,
            service_type: String::new(),
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
            destinations,
            esm_class: 0,
            protocol_id: 0,
            priority_flag: 0,
            schedule_delivery_time: String::new(),
            validity_period: String::new(),
            registered_delivery: 0, // Default: Don't request delivery receipt
            replace_if_present_flag: 0,
            data_coding: 0, // Default: SMSC Default
            sm_default_msg_id: 0,
            short_message,
            optional_params: Vec::new(),
        }
    }

    /// Returns the number of destinations as a u8.
    fn dest_count(&self) -> u8 {
        self.destinations.len() as u8
    }

    /// Encode the PDU into the writer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the write fails or fields are invalid.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, src = %self.source_addr, dests = self.dest_count())))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding SubmitMultiRequest");
        // Calculate length of destinations
        let mut dest_len = 0;
        for dest in &self.destinations {
            match dest {
                Destination::SmeAddress { address, .. } => {
                    // Flag(1) + Ton(1) + Npi(1) + Address(N+1)
                    dest_len += 1 + 1 + 1 + address.len() + 1;
                }
                Destination::DistributionList(name) => {
                    // Flag(1) + Name(N+1)
                    dest_len += 1 + name.len() + 1;
                }
            }
        }

        // Calculate length of TLVs
        let tlvs_len: usize = self
            .optional_params
            .iter()
            .map(|tlv| 4 + tlv.length as usize)
            .sum();

        // Calculate Body Length
        // ServiceType(N+1)
        // SrcTon(1) + SrcNpi(1) + SrcAddr(N+1)
        // DestCount(1) + DestLen
        // Flags(3)
        // Sched(N+1) + Validity(N+1)
        // Reg(1) + Rep(1) + DC(1) + MsgId(1)
        // SmLen(1) + SmData(N)
        // TLVs
        let body_len = self.service_type.len() + 1 +
                       1 + 1 + self.source_addr.len() + 1 +
                       1 + dest_len +
                       1 + 1 + 1 + // esm_class, protocol_id, priority_flag
                       self.schedule_delivery_time.len() + 1 +
                       self.validity_period.len() + 1 +
                       1 + 1 + 1 + 1 + // reg, rep, dc, id
                       1 + self.short_message.len() +
                       tlvs_len;

        let command_len = (HEADER_LEN + body_len) as u32;

        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_SUBMIT_MULTI_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        write_c_string(writer, &self.service_type)?;
        writer.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(writer, &self.source_addr)?;

        // Standard: "number_of_dests"
        // We calculate it dynamically from the Vector length
        if self.destinations.len() > 255 {
            // Optional: Return error if > 255, as spec limits it.
            // For now, casting to u8 is the simple approach.
        }
        writer.write_all(&[self.destinations.len() as u8])?;
        for dest in &self.destinations {
            match dest {
                Destination::SmeAddress { ton, npi, address } => {
                    writer.write_all(&[1])?; // Dest Flag 1 = SME Address
                    writer.write_all(&[*ton as u8, *npi as u8])?;
                    write_c_string(writer, address)?;
                }
                Destination::DistributionList(name) => {
                    writer.write_all(&[2])?; // Dest Flag 2 = Distribution List
                    write_c_string(writer, name)?;
                }
            }
        }

        writer.write_all(&[self.esm_class, self.protocol_id, self.priority_flag])?;
        write_c_string(writer, &self.schedule_delivery_time)?;
        write_c_string(writer, &self.validity_period)?;
        writer.write_all(&[
            self.registered_delivery,
            self.replace_if_present_flag,
            self.data_coding,
            self.sm_default_msg_id,
            self.short_message.len() as u8,
        ])?;
        writer.write_all(&self.short_message)?;

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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(buffer), err, fields(seq = tracing::field::Empty, src = tracing::field::Empty, dests = tracing::field::Empty)))]
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Decoding SubmitMultiRequest from {} bytes", buffer.len());
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(12); // Skip Header fields (Len, ID, Status)

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        let service_type = read_c_string(&mut cursor)?;

        let mut u8_buf = [0u8; 1];
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_npi = Npi::from(u8_buf[0]);
        let source_addr = read_c_string(&mut cursor)?;
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("src", &source_addr);

        // Decode Destination List
        cursor.read_exact(&mut u8_buf)?;
        let dest_count = u8_buf[0];
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("dests", dest_count);
        let mut destinations = Vec::with_capacity(dest_count as usize);

        for _ in 0..dest_count {
            cursor.read_exact(&mut u8_buf)?;
            let dest_flag = u8_buf[0];
            if dest_flag == 1 {
                // SME Address
                cursor.read_exact(&mut u8_buf)?;
                let ton = Ton::from(u8_buf[0]);
                cursor.read_exact(&mut u8_buf)?;
                let npi = Npi::from(u8_buf[0]);
                let address = read_c_string(&mut cursor)?;
                destinations.push(Destination::SmeAddress { ton, npi, address });
            } else {
                // Distribution List
                let dl_name = read_c_string(&mut cursor)?;
                destinations.push(Destination::DistributionList(dl_name));
            }
        }

        cursor.read_exact(&mut u8_buf)?;
        let esm_class = u8_buf[0];
        cursor.read_exact(&mut u8_buf)?;
        let protocol_id = u8_buf[0];
        cursor.read_exact(&mut u8_buf)?;
        let priority_flag = u8_buf[0];

        let schedule_delivery_time = read_c_string(&mut cursor)?;
        let validity_period = read_c_string(&mut cursor)?;

        cursor.read_exact(&mut u8_buf)?;
        let registered_delivery = u8_buf[0];
        cursor.read_exact(&mut u8_buf)?;
        let replace_if_present_flag = u8_buf[0];
        cursor.read_exact(&mut u8_buf)?;
        let data_coding = u8_buf[0];
        cursor.read_exact(&mut u8_buf)?;
        let sm_default_msg_id = u8_buf[0];

        cursor.read_exact(&mut u8_buf)?;
        let sm_length = u8_buf[0] as usize;
        let mut short_message = vec![0u8; sm_length];
        cursor.read_exact(&mut short_message)?;

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
            destinations,
            esm_class,
            protocol_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            short_message,
            optional_params,
        })
    }
}
