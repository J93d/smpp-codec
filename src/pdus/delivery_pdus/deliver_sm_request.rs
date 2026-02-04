use crate::common::{
    read_c_string, write_c_string, Npi, PduError, Ton, CMD_DELIVER_SM, HEADER_LEN,
};
use crate::encoding::MessageBody;
use crate::tlv::Tlv;
use std::io::{Cursor, Read, Write};

/// Represents a Deliver SM PDU.
///
/// This PDU is issued by the SMSC to send a short message to the ESME (Client)
/// or to deliver a delivery receipt (DLR).
#[derive(Debug, Clone)]
pub struct DeliverSmRequest {
    /// The sequence number of the PDU.
    pub sequence_number: u32,
    /// The service type (e.g., "CMT", "CPT"). Max 6 chars.
    pub service_type: String,
    /// Type of Number for source address.
    pub source_addr_ton: Ton,
    /// Numbering Plan Indicator for source address.
    pub source_addr_npi: Npi,
    /// Source address (Sender). Max 21 chars.
    pub source_addr: String,
    /// Type of Number for destination address.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination address.
    pub dest_addr_npi: Npi,
    /// Destination address (Receiver/ESME). Max 21 chars.
    pub dest_addr: String,
    /// ESM Class (Message Mode, Message Type, GSM Features).
    pub esm_class: u8,
    /// Protocol Identifier.
    pub protocol_id: u8,
    /// Priority Flag.
    pub priority_flag: u8,
    /// Schedule Delivery Time. Max 17 chars.
    pub schedule_delivery_time: String,
    /// Validity Period. Max 17 chars.
    pub validity_period: String,
    /// Registered Delivery Flag.
    pub registered_delivery: u8,
    /// Replace If Present Flag.
    pub replace_if_present_flag: u8,
    /// Data Coding Scheme.
    pub data_coding: u8,
    /// SMSC Default Message ID.
    pub sm_default_msg_id: u8,
    /// The short message content (or DLR text). Max 254 octets.
    pub short_message: Vec<u8>,
    /// Optional Parameters (TLV).
    pub optional_params: Vec<Tlv>,
}

impl DeliverSmRequest {
    /// Create a new DeliverSm Request with mandatory fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::DeliverSmRequest;
    ///
    /// let deliver = DeliverSmRequest::new(
    ///     1,
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
            registered_delivery: 0,
            replace_if_present_flag: 0,
            data_coding: 0,
            sm_default_msg_id: 0,
            short_message,
            optional_params: Vec::new(),
        }
    }

    /// Encode the struct into raw bytes for the network.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::DeliverSmRequest;
    /// # let deliver = DeliverSmRequest::new(1, "src".into(), "dst".into(), b"Hi".to_vec());
    /// let mut buffer = Vec::new();
    /// deliver.encode(&mut buffer).expect("Encoding failed");
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
        }

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
        writer.write_all(&CMD_DELIVER_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
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
            self.short_message.len() as u8,
        ])?;
        writer.write_all(&self.short_message)?;

        // Optional Parameters
        for tlv in &self.optional_params {
            tlv.encode(writer)?;
        }

        Ok(())
    }

    /// Decode raw bytes from the network into the struct.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::DeliverSmRequest;
    /// # let deliver = DeliverSmRequest::new(1, "src".into(), "dst".into(), b"Hi".to_vec());
    /// # let mut buffer = Vec::new();
    /// # deliver.encode(&mut buffer).unwrap();
    /// let decoded = DeliverSmRequest::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.short_message, b"Hi");
    /// ```
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(12);

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

    pub fn parse_message(&self) -> MessageBody {
        let has_udh = (self.esm_class & 0x40) != 0;
        crate::encoding::process_body(&self.short_message, self.data_coding, has_udh)
    }

    pub fn new_receipt(
        sequence_number: u32,
        source: String,
        dest: String,
        receipt: DeliveryReceipt,
    ) -> Self {
        let receipt_text = receipt.to_string();
        let mut pdu = Self::new(sequence_number, source, dest, receipt_text.into_bytes());

        // 0x04 indicates this is an SMSC Delivery Receipt
        pdu.esm_class = 0x04;
        pdu.data_coding = 0x00; // Receipts are usually GSM 7-bit/Default

        pdu
    }

    pub fn parse_receipt(&self) -> Option<DeliveryReceipt> {
        // Check if the ESM Class has the receipt bit (0x04) set
        if (self.esm_class & 0x04) != 0 {
            // Decode bytes to string first using our existing encoding helpers
            let text = crate::encoding::gsm_7bit_decode(&self.short_message);
            text.parse::<DeliveryReceipt>().ok()
        } else {
            None
        }
    }
}

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DeliveryReceipt {
    pub message_id: String,
    pub submitted_count: u32,
    pub delivered_count: u32,
    pub submit_date: String, // Format: YYMMDDhhmm
    pub done_date: String,   // Format: YYMMDDhhmm
    pub status: String,      // e.g., "DELIVRD", "EXPIRED", "UNDELIV"
    pub error_code: u32,     // Network specific error
    pub text: String,        // First 20 chars of original msg
}

impl fmt::Display for DeliveryReceipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "id:{} sub:{:03} dlvrd:{:03} submit date:{} done date:{} stat:{} err:{:03} text:{}",
            self.message_id,
            self.submitted_count,
            self.delivered_count,
            self.submit_date,
            self.done_date,
            self.status,
            self.error_code,
            self.text
        )
    }
}

impl FromStr for DeliveryReceipt {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut receipt = DeliveryReceipt {
            message_id: String::new(),
            submitted_count: 0,
            delivered_count: 0,
            submit_date: String::new(),
            done_date: String::new(),
            status: String::new(),
            error_code: 0,
            text: String::new(),
        };

        // Helper to normalize the input string for easier parsing
        // "submit date" -> "submit_date", "done date" -> "done_date"
        let s_clean = s
            .replace("submit date", "submit_date")
            .replace("done date", "done_date");

        // Standard sequence: id, sub, dlvrd, submit_date, done_date, stat, err, text
        // 8 fields typical. parsing 'text' last allows it to contain spaces.
        let parts: Vec<&str> = s_clean.splitn(8, ' ').collect();

        for part in parts {
            let kv: Vec<&str> = part.splitn(2, ':').collect();
            if kv.len() < 2 {
                continue;
            }

            let key = kv[0];
            let val = kv[1];

            match key {
                "id" => receipt.message_id = val.to_string(),
                "sub" => receipt.submitted_count = val.parse().unwrap_or(0),
                "dlvrd" => receipt.delivered_count = val.parse().unwrap_or(0),
                "submit date" | "sdate" => receipt.submit_date = val.to_string(),
                "done date" | "ddate" => receipt.done_date = val.to_string(),
                "stat" => receipt.status = val.to_string(),
                "err" => receipt.error_code = val.parse().unwrap_or(0),
                "text" => receipt.text = val.to_string(),
                _ => {} // Ignore unknown keys
            }
        }

        if receipt.message_id.is_empty() {
            return Err("Invalid DLR format: missing message_id".into());
        }

        Ok(receipt)
    }
}
