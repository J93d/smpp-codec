//! # Content Encoding/Decoding Module
//!
//! This module contains encoding/decoding logic for message content
use std::collections::HashMap;

const GSM_BASIC_CHARSET: &str = "@£$¥èéùìòÇ\nØø\rÅåΔ_ΦΓΛΩΠΨΣΘΞ\x1bÆæßÉ !\"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ`¿abcdefghijklmnopqrstuvwxyzäöñüà";

/// Encodes text into GSM 7-bit packed format (unpacked representation).
///
/// Returns an error if the text contains characters not supported by GSM 03.38.
///
/// # Examples
///
/// ```
/// use smpp_codec::encoding::gsm_7bit_encode;
/// let encoded = gsm_7bit_encode("Hello").unwrap();
/// assert_eq!(encoded.len(), 5);
/// ```
pub fn gsm_7bit_encode(text: &str) -> Result<Vec<u8>, String> {
    let mut encoded_text = Vec::new();
    let mut gsm_extended_charset = HashMap::new();
    gsm_extended_charset.insert('^', 20);
    gsm_extended_charset.insert('{', 40);
    gsm_extended_charset.insert('}', 41);
    gsm_extended_charset.insert('\\', 47);
    gsm_extended_charset.insert('[', 60);
    gsm_extended_charset.insert('~', 61);
    gsm_extended_charset.insert(']', 62);
    gsm_extended_charset.insert('|', 64);
    gsm_extended_charset.insert('€', 101);

    for char in text.chars() {
        if let Some(index) = GSM_BASIC_CHARSET.chars().position(|c| c == char) {
            encoded_text.push(index as u8);
        } else if let Some(&code) = gsm_extended_charset.get(&char) {
            encoded_text.push(0x1B);
            encoded_text.push(code);
        } else {
            return Err(format!("Character '{}' not supported in GSM 03.38", char));
        }
    }
    Ok(encoded_text)
}

/// Encodes text into 8-bit Latin1 (ISO-8859-1).
/// Replaces unsupported characters with '?'.
pub fn encode_8bit(text: &str) -> Vec<u8> {
    text.chars()
        .map(|c| if (c as u32) <= 0xFF { c as u8 } else { b'?' })
        .collect()
}

/// Encodes text into 16-bit UCS-2 (Big Endian).
pub fn encode_16bit(text: &str) -> Vec<u8> {
    text.encode_utf16().flat_map(|u| u.to_be_bytes()).collect()
}

/// Decodes GSM 7-bit data (unpacked) into a String.
///
/// # Examples
///
/// ```
/// use smpp_codec::encoding::gsm_7bit_decode;
/// let decoded = gsm_7bit_decode(&[0x48, 0x65, 0x6C, 0x6C, 0x6F]);
/// assert_eq!(decoded, "Hello");
/// ```
pub fn gsm_7bit_decode(bytes: &[u8]) -> String {
    let basic_chars: Vec<char> = GSM_BASIC_CHARSET.chars().collect();
    let mut result = String::new();
    let mut i = 0;

    while i < bytes.len() {
        let byte = bytes[i];
        if byte == 0x1B {
            // Handle Extended Character
            if i + 1 < bytes.len() {
                let next_byte = bytes[i + 1];
                let decoded_char = match next_byte {
                    20 => '^',
                    40 => '{',
                    41 => '}',
                    47 => '\\',
                    60 => '[',
                    61 => '~',
                    62 => ']',
                    64 => '|',
                    101 => '€',
                    _ => '?', // Unknown extended char
                };
                result.push(decoded_char);
                i += 2; // Skip escape + char
            } else {
                i += 1; // Trailing escape, ignore
            }
        } else {
            // Handle Basic Character
            if (byte as usize) < basic_chars.len() {
                result.push(basic_chars[byte as usize]);
            } else {
                result.push('?');
            }
            i += 1;
        }
    }
    result
}

/// Decodes 8-bit Latin1 (ISO-8859-1) data into a String.
pub fn decode_8bit(bytes: &[u8]) -> String {
    // Latin1 (ISO-8859-1) maps 1:1 to first 256 Unicode code points
    bytes.iter().map(|&b| b as char).collect()
}

/// Decodes 16-bit UCS-2 (Big Endian) data into a String.
pub fn decode_16bit(bytes: &[u8]) -> String {
    // UCS-2 (Big Endian)
    let u16_vec: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect();
    String::from_utf16_lossy(&u16_vec)
}

// --- SMSC HELPER ---

/// Represents the body of a Short Message, either Text or Binary.
#[derive(Debug, PartialEq, Clone)]
pub enum MessageBody {
    /// Text content (decoded from GSM7, Latin1, or UCS2)
    Text(String), // It was GSM7, UCS2, or Latin1
    /// Binary content (8-bit Data, Class 2, or Unknown)
    Binary(Vec<u8>), // It was Class 2, 8-bit Data, or Unknown
}

#[derive(Debug, Clone, Copy)]
enum RawEncoding {
    Gsm7Bit,
    Latin1,
    Ucs2,
    Binary8Bit, // Pure data (Class 2, OTA, etc)
}

/// distinguishing Latin1 (0x03) from Binary (0x04, Class 2)
fn detect_raw_encoding(dcs: u8) -> RawEncoding {
    match dcs {
        // Standard "Safe" Values
        0x00 | 0x01 => RawEncoding::Gsm7Bit,
        0x03 => RawEncoding::Latin1, // Explicit Latin-1
        0x08 => RawEncoding::Ucs2,
        0x02 | 0x04 => RawEncoding::Binary8Bit, // Explicit 8-bit Data

        // Bitmask / Classes logic
        _ => {
            let group = dcs >> 4;
            match group {
                // Group 00xx: General Data Coding
                0x00..=0x03 => {
                    match (dcs & 0x0C) >> 2 {
                        0x02 => RawEncoding::Ucs2,
                        0x01 => RawEncoding::Binary8Bit, // 8-bit data
                        _ => RawEncoding::Gsm7Bit,
                    }
                }
                // Group 1111: Data Coding / Message Class (OTA often lives here)
                0x0F => {
                    if (dcs & 0x04) != 0 {
                        RawEncoding::Binary8Bit // 8-bit Data
                    } else {
                        RawEncoding::Gsm7Bit
                    }
                }
                _ => RawEncoding::Binary8Bit, // Treat unknown as binary to be safe
            }
        }
    }
}

// The Public Helper
/// Processes the raw message body based on Data Coding Scheme (DCS) and UDHI flag.
/// Returns a `MessageBody` which is either `Text` (if decodable) or `Binary`.
pub fn process_body(body: &[u8], dcs: u8, udhi: bool) -> MessageBody {
    // 1. Strip UDH if present
    let payload = if udhi && !body.is_empty() {
        let udh_len = body[0] as usize;
        if body.len() > udh_len + 1 {
            &body[udh_len + 1..]
        } else {
            // Malformed UDH? Return raw bytes to be safe.
            return MessageBody::Binary(body.to_vec());
        }
    } else {
        body
    };

    // 2. Decode based on detected type
    match detect_raw_encoding(dcs) {
        RawEncoding::Gsm7Bit => MessageBody::Text(gsm_7bit_decode(payload)),
        RawEncoding::Latin1 => MessageBody::Text(decode_8bit(payload)),
        RawEncoding::Ucs2 => MessageBody::Text(decode_16bit(payload)),
        RawEncoding::Binary8Bit => MessageBody::Binary(payload.to_vec()),
    }
}
