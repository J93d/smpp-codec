use crate::pdus::submission_pdus::submit_sm_request::SubmitSmRequest;
use crate::tlv::{Tlv, tags};
use crate::encoding;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EncodingType {
    Gsm7Bit,
    Latin1,
    Ucs2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplitMode {
    Udh,
    Sar,
    Payload,
}

pub struct MessageSplitter;

impl MessageSplitter {
    /// Split a long message into multiple `SubmitSmRequest` PDUs.
    ///
    /// Supports 3 modes:
    /// * `SplitMode::Udh`: Uses User Data Header (Concatenated SMS).
    /// * `SplitMode::Sar`: Uses SAR Optional Parameters.
    /// * `SplitMode::Payload`: Uses `message_payload` TLV (no splitting, single large PDU).
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::MessageSplitter;
    /// use smpp_codec::pdus::submission_pdus::splitter::{SplitMode, EncodingType};
    ///
    /// let parts = MessageSplitter::split(
    ///     "src".to_string(),
    ///     "dst".to_string(),
    ///     "Hello World".to_string(),
    ///     EncodingType::Gsm7Bit,
    ///     SplitMode::Udh
    /// ).unwrap();
    /// assert_eq!(parts.len(), 1);
    /// ```
    pub fn split(
        source_addr: String,
        dest_addr: String,
        text: String,
        encoding: EncodingType,
        mode: SplitMode,
    ) -> Result<Vec<SubmitSmRequest>, String> {
        
        // 1. Encode Text
        let (encoded_bytes, data_coding) = match encoding {
            EncodingType::Gsm7Bit => (encoding::gsm_7bit_encode(&text)?, 0x00),
            EncodingType::Latin1 => (encoding::encode_8bit(&text), 0x03),
            EncodingType::Ucs2 => (encoding::encode_16bit(&text), 0x08),
        };

        // 2. Determine Limits
        let (single_max, multipart_max) = match mode {
            SplitMode::Udh => match encoding {
                EncodingType::Gsm7Bit => (160, 153), 
                _ => (140, 134),
            },
            SplitMode::Sar => match encoding {
                _ => (254, 254), 
            },
            SplitMode::Payload => (65535, 65535),
        };

        // 3. Simple Case: Fits in one message?
        if encoded_bytes.len() <= single_max || mode == SplitMode::Payload {
            // ... (Same single message logic as before) ...
            let mut pdu = SubmitSmRequest::new(0, source_addr, dest_addr, Vec::new());
            pdu.data_coding = data_coding;
            if mode == SplitMode::Payload && encoded_bytes.len() > single_max {
                pdu.add_tlv(Tlv::new_payload(tags::MESSAGE_PAYLOAD, encoded_bytes));
                pdu.short_message = Vec::new();
            } else {
                pdu.short_message = encoded_bytes;
            }
            return Ok(vec![pdu]);
        }

        // 4. PRE-CALCULATE CHUNKS (The Simplification)
        // Instead of building PDUs inside the loop, we just collect byte slices.
        let mut chunks = Vec::new();
        let mut offset = 0;

        while offset < encoded_bytes.len() {
            let remaining = encoded_bytes.len() - offset;
            let mut chunk_len = std::cmp::min(multipart_max, remaining);

            // Handle GSM 7-bit Escape Splitting
            if encoding == EncodingType::Gsm7Bit && chunk_len < remaining {
                let last_byte_index = offset + chunk_len - 1;
                if encoded_bytes[last_byte_index] == 0x1B {
                    chunk_len -= 1; // Back off
                }
            }

            chunks.push(&encoded_bytes[offset..offset + chunk_len]);
            offset += chunk_len;
        }

        // 5. Build PDUs
        // Now we know the exact count without any guessing logic.
        let total_segments = chunks.len() as u8;
        let ref_num: u8 = rand::thread_rng().random_range(1..=254);
        let mut pdus = Vec::with_capacity(chunks.len());

        for (i, chunk) in chunks.iter().enumerate() {
            let seq_num = (i + 1) as u8;
            let mut pdu = SubmitSmRequest::new(0, source_addr.clone(), dest_addr.clone(), Vec::new());
            pdu.data_coding = data_coding;

            match mode {
                SplitMode::Udh => {
                    pdu.esm_class = 0x40; // UDHI
                    let mut udh = vec![0x05, 0x00, 0x03, ref_num, total_segments, seq_num];
                    udh.extend_from_slice(chunk);
                    pdu.short_message = udh;
                },
                SplitMode::Sar => {
                    pdu.short_message = chunk.to_vec();
                    pdu.add_tlv(Tlv::new_u16(tags::SAR_MSG_REF_NUM, ref_num as u16));
                    pdu.add_tlv(Tlv::new_u16(tags::SAR_TOTAL_SEGMENTS, total_segments as u16));
                    pdu.add_tlv(Tlv::new_u16(tags::SAR_SEGMENT_SEQNUM, seq_num as u16));
                },
                SplitMode::Payload => unreachable!(),
            }
            pdus.push(pdu);
        }

        Ok(pdus)
    }
}