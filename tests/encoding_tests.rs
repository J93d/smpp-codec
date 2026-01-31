use smpp_codec::encoding::{gsm_7bit_encode, gsm_7bit_decode, encode_16bit, decode_16bit};


#[test]
fn test_gsm_7bit_basic() {
    let text = "Hello World";
    let encoded = gsm_7bit_encode(text).expect("Encode failed");
    let decoded = gsm_7bit_decode(&encoded);
    assert_eq!(decoded, text);
}

#[test]
fn test_gsm_7bit_extended() {
    let text = "Hello € World";
    let encoded = gsm_7bit_encode(text).expect("Encode failed");
    // € is 0x1B, 0x65 in GSM 7-bit
    assert!(encoded.contains(&0x1B));
    assert!(encoded.contains(&0x65));
    
    let decoded = gsm_7bit_decode(&encoded);
    assert_eq!(decoded, text);
}

#[test]
fn test_ucs2_encoding() {
    let text = "Hello 🌍"; // Earth emoji requires surrogate pair in UCS-2? No, SMPP uses UCS-2 (BMP only usually) but often UTF-16 BE. 
    // Rust strings are UTF-8. 
    // encode_16bit uses encode_utf16 which produces u16 values.
    
    let encoded = encode_16bit(text);
    let decoded = decode_16bit(&encoded);
    assert_eq!(decoded, text);
}
