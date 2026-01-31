use crate::encoding;
// use rand::Rng; // Deprecated/Unused


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
    /// Split a long message into multiple chunks.
    ///
    /// Returns a tuple of (Chunks, DataCoding).
    /// * For `SplitMode::Udh`, chunks include the User Data Header.
    /// * For `SplitMode::Sar`, chunks are raw payload; caller must add SAR TLVs.
    /// * For `SplitMode::Payload`, returns a single chunk (no splitting).
    pub fn split(
        text: String,
        encoding: EncodingType,
        mode: SplitMode,
    ) -> Result<(Vec<Vec<u8>>, u8), String> {
        
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
            SplitMode::Sar =>(254, 254),
            SplitMode::Payload => (65535, 65535),
        };

        // 3. Simple Case: Fits in one message?
        if encoded_bytes.len() <= single_max || matches!(mode, SplitMode::Payload) {
            return Ok((vec![encoded_bytes], data_coding));
        }

        // 4. Calculate Chunks (Payload Only First)
        let mut temp_chunks = Vec::new();
        let mut offset = 0;

        while offset < encoded_bytes.len() {
            let remaining = encoded_bytes.len() - offset;
            let mut chunk_len = std::cmp::min(multipart_max, remaining);

            // Handle GSM 7-bit Escape Splitting
            if encoding == EncodingType::Gsm7Bit && chunk_len < remaining {
                let last_byte_index = offset + chunk_len - 1;
                if encoded_bytes[last_byte_index] == 0x1B {
                    chunk_len -= 1; // Back off to avoid splitting escape sequence
                }
            }

            temp_chunks.push(encoded_bytes[offset..offset + chunk_len].to_vec());
            offset += chunk_len;
        }

        // 5. Finalize Chunks (Add UDH if needed)
        let mut final_chunks = Vec::new();
        let total_segments = temp_chunks.len() as u8;
        let ref_num = rand::random::<u8>();

        for (i, chunk_payload) in temp_chunks.iter().enumerate() {
            let mut chunk = Vec::new();
            
            if mode == SplitMode::Udh {
                // UDH: Len(05) + ID(00) + Len(03) + Ref + Total + Seq
                let seq_num = (i + 1) as u8;
                let udh = vec![0x05, 0x00, 0x03, ref_num, total_segments, seq_num];
                chunk.extend(udh);
            }
            
            chunk.extend_from_slice(chunk_payload);
            final_chunks.push(chunk);
        }

        Ok((final_chunks, data_coding))
    }
}   