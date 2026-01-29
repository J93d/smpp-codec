use smpp_codec::pdus::{SubmitSmRequest, SubmitSmResponse, MessageSplitter, SplitMode, EncodingType};
use smpp_codec::tlv::tags;

#[test]
fn test_submit_sm_encoding_decoding() {
    let sequence_number = 12345;
    let mut req = SubmitSmRequest::new(
        sequence_number,
        "src".to_string(),
        "dst".to_string(), 
        b"Hello".to_vec()
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
    let resp = SubmitSmResponse::new(55, 0, "UUID-1234".to_string());
    
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
    
    let parts = MessageSplitter::split(
        "src".into(),
        "dst".into(),
        text.clone(),
        EncodingType::Gsm7Bit,
        SplitMode::Udh
    ).expect("Split failed");

    assert_eq!(parts.len(), 2);
    
    // Verify first part has UDH
    let pdu1 = &parts[0];
    assert_eq!(pdu1.esm_class, 0x40); // UDHI set
    // UDH header is typically 6 bytes (len(1) + ie(5)) = 05 00 03 AA TT 01
    // src/pdus/submission_pdus/splitter.rs line 99: vec![0x05, 0x00, 0x03, ref_num, total_segments, seq_num];
    assert!(pdu1.short_message.len() > 6);
    assert_eq!(pdu1.short_message[0], 0x05); // UDH Len
}

#[test]
fn test_splitter_sar() {
    let text = "B".repeat(300);
    
    let parts = MessageSplitter::split(
        "src".into(),
        "dst".into(),
        text.clone(),
        EncodingType::Gsm7Bit,
        SplitMode::Sar
    ).expect("Split failed");

    assert_eq!(parts.len(), 2);
    
    // Verify SAR tags present
    let pdu2 = &parts[1];
    // We expect 3 TLVs for SAR
    assert!(pdu2.optional_params.len() >= 3);
    
    let has_sar_total = pdu2.optional_params.iter().any(|t| t.tag == tags::SAR_TOTAL_SEGMENTS);
    assert!(has_sar_total);
}
