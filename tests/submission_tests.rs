use smpp_codec::pdus::{
    EncodingType, MessageSplitter, SplitMode, SubmitSmRequest, SubmitSmResponse,
};

#[test]
fn test_submit_sm_encoding_decoding() {
    let sequence_number = 12345;
    let mut req = SubmitSmRequest::new(
        sequence_number,
        "src".to_string(),
        "dst".to_string(),
        b"Hello".to_vec(),
    );

    // Set some fields
    req.service_type = "CMT".to_string();
    req.registered_delivery = 1;

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = SubmitSmRequest::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, sequence_number);
    assert_eq!(decoded.service_type, "CMT");
    assert_eq!(decoded.short_message, b"Hello");
}

#[test]
fn test_submit_sm_resp_encoding_decoding() {
    let resp = SubmitSmResponse::new(55, "ESME_ROK", "UUID-1234".to_string());

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = SubmitSmResponse::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, 55);
    assert_eq!(decoded.message_id, "UUID-1234");
}

#[test]
fn test_splitter_udh() {
    // Generate a long string > 160 chars
    let text = "A".repeat(200);

    let (parts, _) = MessageSplitter::split(text.clone(), EncodingType::Gsm7Bit, SplitMode::Udh)
        .expect("Split failed");

    assert_eq!(parts.len(), 2);

    // Verify first part has UDH
    let pdu1_body = &parts[0];
    // UDH header is typically 6 bytes (len(1) + ie(5)) = 05 00 03 AA TT 01
    assert!(pdu1_body.len() > 6);
    assert_eq!(pdu1_body[0], 0x05); // UDH Len
}

#[test]
fn test_splitter_sar() {
    let text = "B".repeat(300);

    let (parts, _) = MessageSplitter::split(text.clone(), EncodingType::Gsm7Bit, SplitMode::Sar)
        .expect("Split failed");

    assert_eq!(parts.len(), 2);

    // Verify chunks are within limits
    let pdu2_body = &parts[1];
    assert!(pdu2_body.len() <= 254);
    // Note: SAR TLVs are added by the caller, so we can't test them here.
}
