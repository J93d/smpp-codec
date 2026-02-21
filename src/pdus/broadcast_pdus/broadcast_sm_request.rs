use crate::common::{
    read_c_string, write_c_string, Npi, PduError, Ton, CMD_BROADCAST_SM, HEADER_LEN,
};
use crate::tlv::{tags, Tlv};
use std::io::{Cursor, Read, Write};

// --- Request ---
/// Represents a Broadcast SM Request PDU.
///
/// Used to broadcast a message to multiple recipients in a specific area.
#[derive(Debug, Clone, PartialEq)]
pub struct BroadcastSmRequest {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Service Type (e.g., "CMT", "CPT")
    pub service_type: String,
    /// Source Address Type of Number
    pub source_addr_ton: Ton,
    /// Source Address Numbering Plan Indicator
    pub source_addr_npi: Npi,
    /// Source Address (Sender)
    pub source_addr: String,
    /// Message ID (Usually empty in Request, used in Response)
    pub message_id: String, // Usually empty in Request, used in Response
    /// Priority Flag
    pub priority_flag: u8,
    /// Scheduled Delivery Time
    pub schedule_delivery_time: String,
    /// Validity Period
    pub validity_period: String,
    /// Replace If Present Flag
    pub replace_if_present_flag: u8,
    /// Data Coding Scheme
    pub data_coding: u8,
    /// SMSC Default Message ID
    pub sm_default_msg_id: u8,
    /// Optional Parameters (TLV). Must contain 'broadcast_area_identifier'
    pub optional_params: Vec<Tlv>, // Critical: Must contain 'broadcast_area_identifier'
}

impl BroadcastSmRequest {
    /// Create a new Broadcast SM Request.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::BroadcastSmRequest;
    /// use smpp_codec::tlv::{Tlv, tags};
    ///
    /// let area_tlv = Tlv::new(tags::BROADCAST_AREA_IDENTIFIER, vec![0x01]);
    /// let pdu = BroadcastSmRequest::new(
    ///     1,
    ///     "Source".to_string(),
    ///     b"Hello".to_vec(),
    ///     area_tlv,
    /// );
    /// ```
    pub fn new(
        sequence_number: u32,
        source_addr: String,
        payload: Vec<u8>,
        area_tlv: Tlv, // Mandatory param for Broadcast
    ) -> Self {
        let mut pdu = Self {
            sequence_number,
            service_type: String::new(),
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
            message_id: String::new(),
            priority_flag: 0,
            schedule_delivery_time: String::new(),
            validity_period: String::new(),
            replace_if_present_flag: 0,
            data_coding: 0,
            sm_default_msg_id: 0,
            optional_params: Vec::new(),
        };

        // Broadcast requires payload in TLV, not body
        pdu.add_tlv(Tlv::new(tags::MESSAGE_PAYLOAD, payload));
        pdu.add_tlv(area_tlv);

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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, src = %self.source_addr, message_id = %self.message_id)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding BroadcastSmRequest");
        // Calculate length upfront
        let tlvs_len: usize = self
            .optional_params
            .iter()
            .map(|t| 4 + t.length as usize)
            .sum();

        let body_len = self.service_type.len() + 1 +
                       1 + 1 + // source ton + npi
                       self.source_addr.len() + 1 +
                       self.message_id.len() + 1 +
                       1 + // priority_flag
                       self.schedule_delivery_time.len() + 1 +
                       self.validity_period.len() + 1 +
                       1 + 1 + 1 + // replace, data_coding, default_msg_id
                       tlvs_len;

        let command_len = (HEADER_LEN + body_len) as u32;

        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_BROADCAST_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        write_c_string(writer, &self.service_type)?;
        writer.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(writer, &self.source_addr)?;
        write_c_string(writer, &self.message_id)?;

        writer.write_all(&[self.priority_flag])?;
        write_c_string(writer, &self.schedule_delivery_time)?;
        write_c_string(writer, &self.validity_period)?;
        writer.write_all(&[
            self.replace_if_present_flag,
            self.data_coding,
            self.sm_default_msg_id,
        ])?;

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
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(buffer), err, fields(seq = tracing::field::Empty, src = tracing::field::Empty, message_id = tracing::field::Empty)))]
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Decoding BroadcastSmRequest from {} bytes", buffer.len());
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(12);

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
        let message_id = read_c_string(&mut cursor)?;
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("message_id", &message_id);

        cursor.read_exact(&mut u8_buf)?;
        let priority_flag = u8_buf[0];
        let schedule_delivery_time = read_c_string(&mut cursor)?;
        let validity_period = read_c_string(&mut cursor)?;

        cursor.read_exact(&mut u8_buf)?;
        let replace_if_present_flag = u8_buf[0];
        cursor.read_exact(&mut u8_buf)?;
        let data_coding = u8_buf[0];
        cursor.read_exact(&mut u8_buf)?;
        let sm_default_msg_id = u8_buf[0];

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
            message_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            optional_params,
        })
    }
}
