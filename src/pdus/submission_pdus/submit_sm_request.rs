use crate::common::{read_c_string, write_c_string, Npi, PduError, Ton, CMD_SUBMIT_SM, HEADER_LEN};
use crate::encoding::MessageBody;
use crate::tlv::{tags, Tlv};
use std::io::{Cursor, Read, Write};

#[derive(Debug, Clone, PartialEq)]
/// Request to submit a short message (SubmitSm)
pub struct SubmitSmRequest {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Service Type (e.g., "CMT", "CPT")
    pub service_type: String, // Max 6 chars
    /// Source Address Type of Number
    pub source_addr_ton: Ton,
    /// Source Address Numbering Plan Indicator
    pub source_addr_npi: Npi,
    /// Source Address
    pub source_addr: String, // Max 21 chars
    /// Destination Address Type of Number
    pub dest_addr_ton: Ton,
    /// Destination Address Numbering Plan Indicator
    pub dest_addr_npi: Npi,
    /// Destination Address
    pub dest_addr: String, // Max 21 chars
    /// ESM Class (Message Mode, Message Type, GSM Features)
    pub esm_class: u8,
    /// Protocol Identifier
    pub protocol_id: u8,
    /// Priority Level
    pub priority_flag: u8,
    /// Scheduled Delivery Time (YYMMDDhhmmsstn00)
    pub schedule_delivery_time: String, // Max 17 chars
    /// Validity Period (YYMMDDhhmmsstn00)
    pub validity_period: String, // Max 17 chars
    /// Registered Delivery (Delivery Receipt Request)
    pub registered_delivery: u8,
    /// Replace If Present Flag
    pub replace_if_present_flag: u8,
    /// Data Coding Scheme (DCS)
    pub data_coding: u8,
    /// SMSC Default Message ID
    pub sm_default_msg_id: u8,
    /// Short Message Data
    pub short_message: Vec<u8>, // Max 254 octets
    /// Optional Parameters (TLVs)
    pub optional_params: Vec<Tlv>,
}

#[derive(Debug, Clone, PartialEq)]
/// Information about message segmentation/concatenation
pub struct SegmentationInfo {
    /// Reference Number (to group segments)
    pub ref_num: u16,
    /// Total number of segments
    pub total_segments: u8,
    /// Sequence number of this segment (1-based)
    pub seq_num: u8,
}

impl SubmitSmRequest {
    /// Create a new SubmitSm Request with mandatory fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::SubmitSmRequest;
    ///
    /// let sequence_number: u32 = 1;
    /// let submit = SubmitSmRequest::new(
    ///     sequence_number,
    ///     "source".to_string(),
    ///     "dest".to_string(),
    ///     b"Hello".to_vec()
    /// );
    /// ```
    pub fn new(
        sequence_number: u32,
        source_addr: String,
        dest_addr: String,
        short_message: Vec<u8>,
    ) -> Self {
        Self {
            sequence_number,
            service_type: String::new(),
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
            dest_addr_ton: Ton::Unknown,
            dest_addr_npi: Npi::Unknown,
            dest_addr,
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

    /// Helper to add a TLV (Optional Parameter)
    pub fn add_tlv(&mut self, tlv: Tlv) {
        self.optional_params.push(tlv);
    }

    /// Encode the struct into raw bytes for the network.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if:
    /// * `service_type` > 6 chars
    /// * `source_addr` or `dest_addr` > 21 chars
    /// * `schedule_delivery_time` or `validity_period` > 17 chars
    /// * `short_message` > 254 octets (use `message_payload` TLV for longer messages)
    /// * Usage of invalid characters when validating C-Strings
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::SubmitSmRequest;
    /// # let sequence_number: u32 = 1;
    /// # let submit = SubmitSmRequest::new(sequence_number, "src".into(), "dst".into(), b"Hi".to_vec());
    /// let mut buffer = Vec::new();
    /// submit.encode(&mut buffer).expect("Encoding failed");
    /// ```
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        // 1. Validation
        if self.service_type.len() > 6 {
            return Err(PduError::StringTooLong("service_type".into(), 6));
        }
        if self.source_addr.len() > 21 {
            return Err(PduError::StringTooLong("source_addr".into(), 21));
        }
        if self.dest_addr.len() > 21 {
            return Err(PduError::StringTooLong("dest_addr".into(), 21));
        }
        if self.schedule_delivery_time.len() > 17 {
            return Err(PduError::StringTooLong("schedule_delivery_time".into(), 17));
        }
        if self.validity_period.len() > 17 {
            return Err(PduError::StringTooLong("validity_period".into(), 17));
        }
        if self.short_message.len() > 254 {
            return Err(PduError::InvalidLength);
        } // Use Payload TLV for longer msgs

        // 2. Calculate Length Upfront
        let tlvs_len: usize = self
            .optional_params
            .iter()
            .map(|tlv| 4 + tlv.length as usize)
            .sum();

        // Fixed fields overhead:
        // Src(Ton1+Npi1) + Dst(Ton1+Npi1) + Flags(3) + Reg/Rep/DC/Id(4) + SmLen(1) = 12 bytes
        let body_len = self.service_type.len()
            + 1
            + (self.source_addr.len() + 1)
            + (self.dest_addr.len() + 1)
            + (self.schedule_delivery_time.len() + 1)
            + (self.validity_period.len() + 1)
            + self.short_message.len()
            + 12
            + tlvs_len;

        // 3. Write Header
        let command_len = (HEADER_LEN + body_len) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_SUBMIT_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?; // Status always 0
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        // 4. Write Body
        write_c_string(writer, &self.service_type)?;

        // Source Address
        writer.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(writer, &self.source_addr)?;

        // Destination Address
        writer.write_all(&[self.dest_addr_ton as u8, self.dest_addr_npi as u8])?;
        write_c_string(writer, &self.dest_addr)?;

        // Flags & settings
        writer.write_all(&[self.esm_class, self.protocol_id, self.priority_flag])?;

        write_c_string(writer, &self.schedule_delivery_time)?;
        write_c_string(writer, &self.validity_period)?;

        writer.write_all(&[
            self.registered_delivery,
            self.replace_if_present_flag,
            self.data_coding,
            self.sm_default_msg_id,
        ])?;

        // Short Message (Length + Content)
        writer.write_all(&[self.short_message.len() as u8])?;
        writer.write_all(&self.short_message)?;

        // Optional Parameters (TLVs)
        for tlv in &self.optional_params {
            tlv.encode(writer)?;
        }

        Ok(())
    }

    /// Decode raw bytes from the network into the struct.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the buffer is too short or malformed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::SubmitSmRequest;
    /// # let sequence_number: u32 = 1;
    /// # let submit = SubmitSmRequest::new(sequence_number, "src".into(), "dst".into(), b"Hi".to_vec());
    /// # let mut buffer = Vec::new();
    /// # submit.encode(&mut buffer).unwrap();
    /// let decoded = SubmitSmRequest::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.short_message, b"Hi");
    /// ```
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }

        let mut cursor = Cursor::new(buffer);
        cursor.set_position(12); // Skip header (len, id, status)
        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        // Body Parsing
        let mut u8_buf = [0u8; 1];

        let service_type = read_c_string(&mut cursor)?;

        // Source
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_npi = Npi::from(u8_buf[0]);
        let source_addr = read_c_string(&mut cursor)?;

        // Dest
        cursor.read_exact(&mut u8_buf)?;
        let dest_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let dest_addr_npi = Npi::from(u8_buf[0]);
        let dest_addr = read_c_string(&mut cursor)?;

        // Flags
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

        // Short Message
        cursor.read_exact(&mut u8_buf)?;
        let sm_length = u8_buf[0] as usize;
        let mut short_message = vec![0u8; sm_length];
        cursor.read_exact(&mut short_message)?;

        // Optional Params (TLVs)
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

    /// Retrieve segmentation information if present (via SAR headers or UDH).
    pub fn get_segmentation_info(&self) -> Option<SegmentationInfo> {
        // STRATEGY 1: Check SAR TLVs (Simpler)
        // We need all three SAR tags to be present.
        let sar_ref = self.get_tlv_u16(tags::SAR_MSG_REF_NUM);
        let sar_total = self.get_tlv_u16(tags::SAR_TOTAL_SEGMENTS);
        let sar_seq = self.get_tlv_u16(tags::SAR_SEGMENT_SEQNUM);

        if let (Some(ref_num), Some(total), Some(seq)) = (sar_ref, sar_total, sar_seq) {
            return Some(SegmentationInfo {
                ref_num,
                total_segments: total as u8, // SAR uses u16 storage but value is u8
                seq_num: seq as u8,
            });
        }

        // STRATEGY 2: Check UDH (User Data Header)
        // Only look if the ESM Class "UDHI" bit (0x40) is set.
        if (self.esm_class & 0x40) != 0 && self.short_message.len() > 5 {
            let udh_len = self.short_message[0] as usize;

            // Safety check: header must be fully contained in message
            if self.short_message.len() > udh_len {
                let header_bytes = &self.short_message[1..=udh_len];

                // Parse UDH Information Elements (IEs)
                let mut cursor = 0;
                while cursor < header_bytes.len() {
                    let ie_id = header_bytes[cursor];
                    let ie_len = header_bytes[cursor + 1] as usize;
                    let ie_data = &header_bytes[cursor + 2..cursor + 2 + ie_len];

                    // 0x00: Concatenated short messages, 8-bit reference number
                    if ie_id == 0x00 && ie_len == 3 {
                        return Some(SegmentationInfo {
                            ref_num: ie_data[0] as u16,
                            total_segments: ie_data[1],
                            seq_num: ie_data[2],
                        });
                    }
                    // 0x08: Concatenated short messages, 16-bit reference number
                    else if ie_id == 0x08 && ie_len == 4 {
                        let ref_num = u16::from_be_bytes([ie_data[0], ie_data[1]]);
                        return Some(SegmentationInfo {
                            ref_num,
                            total_segments: ie_data[2],
                            seq_num: ie_data[3],
                        });
                    }

                    cursor += 2 + ie_len;
                }
            }
        }

        // No segmentation found (Single Message)
        None
    }

    /// Parse the message body based on the `data_coding` and `esm_class` (UDHI).
    pub fn parse_message(&self) -> MessageBody {
        let has_udh = (self.esm_class & 0x40) != 0;
        crate::encoding::process_body(&self.short_message, self.data_coding, has_udh)
    }

    /// Helper to find a TLV and return it as u16 (handles both u8 and u16 storage)
    fn get_tlv_u16(&self, tag: u16) -> Option<u16> {
        self.optional_params
            .iter()
            .find(|t| t.tag == tag)
            .and_then(|t| {
                if t.length == 1 {
                    Some(t.value[0] as u16)
                } else if t.length == 2 {
                    Some(u16::from_be_bytes([t.value[0], t.value[1]]))
                } else {
                    None
                }
            })
    }
}
