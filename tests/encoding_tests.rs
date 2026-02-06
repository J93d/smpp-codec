use smpp_codec::encoding::{
    decode_16bit, decode_8bit, encode_16bit, encode_8bit, gsm_7bit_decode, gsm_7bit_encode,
    process_body, MessageBody,
};

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

#[test]
fn test_gsm_7bit_unsupported_char() {
    let text = "Hello \u{1F600}"; // Emoji not supported in GSM
    let result = gsm_7bit_encode(text);
    assert!(result.is_err());
}

#[test]
fn test_gsm_7bit_all_extended_chars() {
    // ^ { } \ [ ~ ] | €
    let text = "^{}\\[~]|€";
    let encoded = gsm_7bit_encode(text).expect("Encode failed");

    // Each should be 2 bytes (Escape 0x1B + Code)
    assert_eq!(encoded.len(), text.chars().count() * 2);

    let decoded = gsm_7bit_decode(&encoded);
    assert_eq!(decoded, text);
}

#[test]
fn test_8bit_encoding_fallback() {
    // 8-bit supports Latin1, but replaces others with '?'
    let text = "Café \u{1F600}"; // é is ok, emoji is not
    let encoded = encode_8bit(text);
    let decoded = decode_8bit(&encoded);

    assert_eq!(decoded, "Café ?");
}

#[test]
fn test_process_body_logic() {
    // 1. GSM 7-bit (Default DCS 0)
    let text = "Hello";
    let encoded = gsm_7bit_encode(text).unwrap();
    let body = process_body(&encoded, 0x00, false);
    assert_eq!(body, MessageBody::Text(text.to_string()));

    // 2. Latin 1 (DCS 3)
    let text_latin = "Café";
    let encoded_latin = encode_8bit(text_latin);
    let body = process_body(&encoded_latin, 0x03, false);
    assert_eq!(body, MessageBody::Text(text_latin.to_string()));

    // 3. UCS-2 (DCS 8)
    let text_ucs2 = "Hi 🌍";
    let encoded_ucs2 = encode_16bit(text_ucs2);
    let body = process_body(&encoded_ucs2, 0x08, false);
    assert_eq!(body, MessageBody::Text(text_ucs2.to_string()));

    // 4. Binary (DCS 4)
    let data = vec![0x00, 0xFF, 0xAA];
    let body = process_body(&data, 0x04, false);
    assert_eq!(body, MessageBody::Binary(data));
}

#[test]
fn test_process_body_udh() {
    // Construct a payload with UDH: [UDHLen, IEI, Len, Data...] + Message
    // UDH: Len=03, IEI=00, Len=01, Ref=05 (Simulate logic)
    // DCS=00 (GSM7), UDHI=true => strip UDH, decode rest

    let text = "Hello";
    let encoded_text = gsm_7bit_encode(text).unwrap();

    let mut payload = vec![0x03, 0x00, 0x01, 0x05];
    payload.extend_from_slice(&encoded_text);

    let body = process_body(&payload, 0x00, true);
    assert_eq!(body, MessageBody::Text(text.to_string()));
}
