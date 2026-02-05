//! # Tagged Length Value (TLV) Parameters
//!
//! This module handles the encoding and decoding of SMPP Optional Parameters (TLVs).
//! It provides a registry of standard tags and mechanisms to parse and serialize them.

use crate::common::PduError;
use std::io::{Cursor, Read, Write};

// --- Standard SMPP Optional Parameter Tags ---
/// Standard SMPP Optional Parameter Tags as defined in SMPP Spec (v3.4 / v5.0).
pub mod tags {
    /// Destination Address Subunit
    pub const DEST_ADDR_SUBUNIT: u16 = 0x0005;
    /// Destination Network Type
    pub const DEST_NETWORK_TYPE: u16 = 0x0006;
    /// Destination Bearer Type
    pub const DEST_BEARER_TYPE: u16 = 0x0007;
    /// Destination Telematics ID
    pub const DEST_TELEMATICS_ID: u16 = 0x0008;
    /// Source Address Subunit
    pub const SOURCE_ADDR_SUBUNIT: u16 = 0x000D;
    /// Source Network Type
    pub const SOURCE_NETWORK_TYPE: u16 = 0x000E;
    /// Source Bearer Type
    pub const SOURCE_BEARER_TYPE: u16 = 0x000F;
    /// Source Telematics ID
    pub const SOURCE_TELEMATICS_ID: u16 = 0x0010;
    /// QoS Time To Live
    pub const QOS_TIME_TO_LIVE: u16 = 0x0017;
    /// Payload Type
    pub const PAYLOAD_TYPE: u16 = 0x0019;
    /// Additional Status Info Text
    pub const ADDITIONAL_STATUS_INFO_TEXT: u16 = 0x001D;
    /// Receipted Message ID
    pub const RECEIPTED_MESSAGE_ID: u16 = 0x001E;
    /// MS Message Wait Facilities
    pub const MS_MSG_WAIT_FACILITIES: u16 = 0x0030;
    /// Privacy Indicator
    pub const PRIVACY_INDICATOR: u16 = 0x0201;
    /// Source Subaddress
    pub const SOURCE_SUBADDRESS: u16 = 0x0202;
    /// Destination Subaddress
    pub const DEST_SUBADDRESS: u16 = 0x0203;
    /// User Message Reference
    pub const USER_MESSAGE_REFERENCE: u16 = 0x0204;
    /// User Response Code
    pub const USER_RESPONSE_CODE: u16 = 0x0205;
    /// Source Port
    pub const SOURCE_PORT: u16 = 0x020A;
    /// Destination Port
    pub const DESTINATION_PORT: u16 = 0x020B;
    /// SAR Message Reference Number
    pub const SAR_MSG_REF_NUM: u16 = 0x020C;
    /// Language Indicator
    pub const LANGUAGE_INDICATOR: u16 = 0x020D;
    /// SAR Total Segments
    pub const SAR_TOTAL_SEGMENTS: u16 = 0x020E;
    /// SAR Segment Sequence Number
    pub const SAR_SEGMENT_SEQNUM: u16 = 0x020F;
    /// SC Interface Version
    pub const SC_INTERFACE_VERSION: u16 = 0x0210;
    /// Callback Number Presentation Indicator
    pub const CALLBACK_NUM_PRES_IND: u16 = 0x0302;
    /// Callback Number Alphanumeric Tag
    pub const CALLBACK_NUM_ATAG: u16 = 0x0303;
    /// Number of Messages
    pub const NUMBER_OF_MESSAGES: u16 = 0x0304;
    /// Callback Number
    pub const CALLBACK_NUM: u16 = 0x0381;
    /// DPF Result
    pub const DPF_RESULT: u16 = 0x0420;
    /// Set DPF
    pub const SET_DPF: u16 = 0x0421;
    /// MS Availability Status
    pub const MS_AVAILABILITY_STATUS: u16 = 0x0422;
    /// Network Error Code
    pub const NETWORK_ERROR_CODE: u16 = 0x0423;
    /// Message Payload
    pub const MESSAGE_PAYLOAD: u16 = 0x0424;
    /// Delivery Failure Reason
    pub const DELIVERY_FAILURE_REASON: u16 = 0x0425;
    /// More Messages To Send
    pub const MORE_MESSAGES_TO_SEND: u16 = 0x0426;
    /// Message State
    pub const MESSAGE_STATE: u16 = 0x0427;
    /// Congestion State
    pub const CONGESTION_STATE: u16 = 0x0428;
    /// USSD Service Op
    pub const USSD_SERVICE_OP: u16 = 0x0501;
    /// Display Time
    pub const DISPLAY_TIME: u16 = 0x1201;
    /// SMS Signal
    pub const SMS_SIGNAL: u16 = 0x1203;
    /// MS Validity
    pub const MS_VALIDITY: u16 = 0x1204;
    /// Alert On Message Delivery
    pub const ALERT_ON_MESSAGE_DELIVERY: u16 = 0x130C;
    /// ITS Reply Type
    pub const ITS_REPLY_TYPE: u16 = 0x1380;
    /// ITS Session Info
    pub const ITS_SESSION_INFO: u16 = 0x1383;
    // --- Broadcast / Multicast specific (SMPP v5.0) ---
    /// Broadcast Area Identifier
    pub const BROADCAST_AREA_IDENTIFIER: u16 = 0x0606;
    /// Broadcast Content Type
    pub const BROADCAST_CONTENT_TYPE: u16 = 0x0601;
    /// Broadcast Repetition Number
    pub const BROADCAST_REP_NUM: u16 = 0x0602;
    /// Broadcast Frequency Interval
    pub const BROADCAST_FREQUENCY_INTERVAL: u16 = 0x0603;
    /// Broadcast Area Success
    pub const BROADCAST_AREA_SUCCESS: u16 = 0x0608;
    /// Broadcast End Time
    pub const BROADCAST_END_TIME: u16 = 0x0609;
    /// Broadcast Service Group
    pub const BROADCAST_SERVICE_GROUP: u16 = 0x060A;
    /// Broadcast Channel Indicator
    pub const BROADCAST_CHANNEL_INDICATOR: u16 = 0x0600;
}

/// Helper function to get tag hex code from name (e.g., "SAR_MSG_REF_NUM" -> 0x020C)
/// Returns 0 if not found.
pub fn get_tag_by_name(name: &str) -> u16 {
    match name {
        "dest_addr_subunit" | "DEST_ADDR_SUBUNIT" => tags::DEST_ADDR_SUBUNIT,
        "dest_network_type" | "DEST_NETWORK_TYPE" => tags::DEST_NETWORK_TYPE,
        "dest_bearer_type" | "DEST_BEARER_TYPE" => tags::DEST_BEARER_TYPE,
        "dest_telematics_id" | "DEST_TELEMATICS_ID" => tags::DEST_TELEMATICS_ID,
        "source_addr_subunit" | "SOURCE_ADDR_SUBUNIT" => tags::SOURCE_ADDR_SUBUNIT,
        "source_network_type" | "SOURCE_NETWORK_TYPE" => tags::SOURCE_NETWORK_TYPE,
        "source_bearer_type" | "SOURCE_BEARER_TYPE" => tags::SOURCE_BEARER_TYPE,
        "source_telematics_id" | "SOURCE_TELEMATICS_ID" => tags::SOURCE_TELEMATICS_ID,
        "qos_time_to_live" | "QOS_TIME_TO_LIVE" => tags::QOS_TIME_TO_LIVE,
        "payload_type" | "PAYLOAD_TYPE" => tags::PAYLOAD_TYPE,
        "additional_status_info_text" | "ADDITIONAL_STATUS_INFO_TEXT" => {
            tags::ADDITIONAL_STATUS_INFO_TEXT
        }
        "receipted_message_id" | "RECEIPTED_MESSAGE_ID" => tags::RECEIPTED_MESSAGE_ID,
        "ms_msg_wait_facilities" | "MS_MSG_WAIT_FACILITIES" => tags::MS_MSG_WAIT_FACILITIES,
        "privacy_indicator" | "PRIVACY_INDICATOR" => tags::PRIVACY_INDICATOR,
        "source_subaddress" | "SOURCE_SUBADDRESS" => tags::SOURCE_SUBADDRESS,
        "dest_subaddress" | "DEST_SUBADDRESS" => tags::DEST_SUBADDRESS,
        "user_message_reference" | "USER_MESSAGE_REFERENCE" => tags::USER_MESSAGE_REFERENCE,
        "user_response_code" | "USER_RESPONSE_CODE" => tags::USER_RESPONSE_CODE,
        "source_port" | "SOURCE_PORT" => tags::SOURCE_PORT,
        "destination_port" | "DESTINATION_PORT" => tags::DESTINATION_PORT,
        "sar_msg_ref_num" | "SAR_MSG_REF_NUM" => tags::SAR_MSG_REF_NUM,
        "language_indicator" | "LANGUAGE_INDICATOR" => tags::LANGUAGE_INDICATOR,
        "sar_total_segments" | "SAR_TOTAL_SEGMENTS" => tags::SAR_TOTAL_SEGMENTS,
        "sar_segment_seqnum" | "SAR_SEGMENT_SEQNUM" => tags::SAR_SEGMENT_SEQNUM,
        "sc_interface_version" | "SC_INTERFACE_VERSION" => tags::SC_INTERFACE_VERSION,
        "callback_num_pres_ind" | "CALLBACK_NUM_PRES_IND" => tags::CALLBACK_NUM_PRES_IND,
        "callback_num_atag" | "CALLBACK_NUM_ATAG" => tags::CALLBACK_NUM_ATAG,
        "number_of_messages" | "NUMBER_OF_MESSAGES" => tags::NUMBER_OF_MESSAGES,
        "callback_num" | "CALLBACK_NUM" => tags::CALLBACK_NUM,
        "dpf_result" | "DPF_RESULT" => tags::DPF_RESULT,
        "set_dpf" | "SET_DPF" => tags::SET_DPF,
        "ms_availability_status" | "MS_AVAILABILITY_STATUS" => tags::MS_AVAILABILITY_STATUS,
        "network_error_code" | "NETWORK_ERROR_CODE" => tags::NETWORK_ERROR_CODE,
        "message_payload" | "MESSAGE_PAYLOAD" => tags::MESSAGE_PAYLOAD,
        "delivery_failure_reason" | "DELIVERY_FAILURE_REASON" => tags::DELIVERY_FAILURE_REASON,
        "more_messages_to_send" | "MORE_MESSAGES_TO_SEND" => tags::MORE_MESSAGES_TO_SEND,
        "message_state" | "MESSAGE_STATE" => tags::MESSAGE_STATE,
        "congestion_state" | "CONGESTION_STATE" => tags::CONGESTION_STATE,
        "ussd_service_op" | "USSD_SERVICE_OP" => tags::USSD_SERVICE_OP,
        "display_time" | "DISPLAY_TIME" => tags::DISPLAY_TIME,
        "sms_signal" | "SMS_SIGNAL" => tags::SMS_SIGNAL,
        "ms_validity" | "MS_VALIDITY" => tags::MS_VALIDITY,
        "alert_on_message_delivery" | "ALERT_ON_MESSAGE_DELIVERY" => {
            tags::ALERT_ON_MESSAGE_DELIVERY
        }
        "its_reply_type" | "ITS_REPLY_TYPE" => tags::ITS_REPLY_TYPE,
        "its_session_info" | "ITS_SESSION_INFO" => tags::ITS_SESSION_INFO,
        // --- Broadcast / Multicast specific (SMPP v5.0) ---
        "broadcast_area_identifier" | "BROADCAST_AREA_IDENTIFIER" => {
            tags::BROADCAST_AREA_IDENTIFIER
        }
        "broadcast_content_type" | "BROADCAST_CONTENT_TYPE" => tags::BROADCAST_CONTENT_TYPE,
        "broadcast_rep_num" | "BROADCAST_REP_NUM" => tags::BROADCAST_REP_NUM,
        "broadcast_frequency_interval" | "BROADCAST_FREQUENCY_INTERVAL" => {
            tags::BROADCAST_FREQUENCY_INTERVAL
        }
        "broadcast_area_success" | "BROADCAST_AREA_SUCCESS" => tags::BROADCAST_AREA_SUCCESS,
        "broadcast_end_time" | "BROADCAST_END_TIME" => tags::BROADCAST_END_TIME,
        "broadcast_service_group" | "BROADCAST_SERVICE_GROUP" => tags::BROADCAST_SERVICE_GROUP,
        "broadcast_channel_indicator" | "BROADCAST_CHANNEL_INDICATOR" => {
            tags::BROADCAST_CHANNEL_INDICATOR
        }
        _ => 0,
    }
}

/// Tag-Length-Value (TLV) Parameter
#[derive(Debug, Clone, PartialEq)]
pub struct Tlv {
    /// The tag identifier for the parameter
    pub tag: u16,
    /// The length of the value field
    pub length: u16,
    /// The value of the parameter
    pub value: Vec<u8>,
}

impl Tlv {
    /// Creates a new TLV with the given tag and value.
    pub fn new(tag: u16, value: Vec<u8>) -> Self {
        Self {
            tag,
            length: value.len() as u16,
            value,
        }
    }

    /// Convenience: create a TLV using a string name for the tag
    pub fn new_from_name(name: &str, value: Vec<u8>) -> Self {
        let tag = get_tag_by_name(name);
        Self::new(tag, value)
    }

    /// Creates a new TLV with a single u8 value.
    pub fn new_u8(tag: u16, val: u8) -> Self {
        Self::new(tag, vec![val])
    }

    /// Convenience: create u8 TLV from name
    pub fn new_u8_from_name(name: &str, val: u8) -> Self {
        Self::new(get_tag_by_name(name), vec![val])
    }

    /// Creates a new TLV with a u16 value (Big Endian).
    pub fn new_u16(tag: u16, val: u16) -> Self {
        Self::new(tag, val.to_be_bytes().to_vec())
    }

    /// Convenience: create u16 TLV from name
    pub fn new_u16_from_name(name: &str, val: u16) -> Self {
        Self::new(get_tag_by_name(name), val.to_be_bytes().to_vec())
    }

    /// Creates a new TLV with a C-style string value (null-terminated).
    pub fn new_string(tag: u16, val: &str) -> Self {
        let mut v = val.as_bytes().to_vec();
        v.push(0); // Null terminator
        Self::new(tag, v)
    }

    /// Creates a new TLV with a raw payload.
    pub fn new_payload(tag: u16, val: Vec<u8>) -> Self {
        Self::new(tag, val)
    }

    /// Encodes the TLV into the writer.
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        writer.write_all(&self.tag.to_be_bytes())?;
        writer.write_all(&self.length.to_be_bytes())?;
        writer.write_all(&self.value)?;
        Ok(())
    }

    /// Decodes a TLV from the cursor.
    pub fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Option<Self>, PduError> {
        let pos = cursor.position();
        let len = cursor.get_ref().len() as u64;

        if pos + 4 > len {
            return Ok(None);
        }

        let mut tag_bytes = [0u8; 2];
        cursor.read_exact(&mut tag_bytes)?;
        let tag = u16::from_be_bytes(tag_bytes);

        let mut len_bytes = [0u8; 2];
        cursor.read_exact(&mut len_bytes)?;
        let length = u16::from_be_bytes(len_bytes);

        let current_pos = cursor.position();
        if current_pos + length as u64 > len {
            return Err(PduError::BufferTooShort);
        }

        let mut value = vec![0u8; length as usize];
        cursor.read_exact(&mut value)?;

        Ok(Some(Self { tag, length, value }))
    }

    // --- Getters ---
    /// Returns the value as a u8 (if length is 1).
    pub fn value_as_u8(&self) -> Result<u8, PduError> {
        if self.value.len() != 1 {
            return Err(PduError::InvalidLength);
        } // Assuming InvalidLength is in common
        Ok(self.value[0])
    }

    /// Returns the value as a u16 (if length is 2).
    pub fn value_as_u16(&self) -> Result<u16, PduError> {
        if self.value.len() != 2 {
            return Err(PduError::InvalidLength);
        }
        Ok(u16::from_be_bytes([self.value[0], self.value[1]]))
    }

    /// Returns the value as a String (removes trailing null if present).
    pub fn value_as_string(&self) -> Result<String, PduError> {
        let v = if !self.value.is_empty() && self.value[self.value.len() - 1] == 0 {
            &self.value[..self.value.len() - 1]
        } else {
            &self.value
        };
        String::from_utf8(v.to_vec()).map_err(PduError::Utf8)
    }
}
