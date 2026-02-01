use smpp_codec::pdus::{DeliverSmRequest, DeliverSmResponse};

#[test]
fn test_deliver_sm_encoding_decoding() {
    let sequence_number = 999;
    let mut req = DeliverSmRequest::new(
        sequence_number,
        "source".to_string(),
        "dest".to_string(),
        b"Delivery Content".to_vec(),
    );

    // Set optional fields
    req.service_type = "CMT".to_string();
    req.esm_class = 0x40; // Simulate UDHI

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = DeliverSmRequest::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, sequence_number);
    assert_eq!(decoded.source_addr, "source");
    assert_eq!(decoded.short_message, b"Delivery Content");
    assert_eq!(decoded.esm_class, 0x40);
}

#[test]
fn test_deliver_sm_validation_errors() {
    let mut req = DeliverSmRequest::new(1, "src".into(), "dst".into(), vec![]);

    // Service type too long
    req.service_type = "1234567".into();
    let mut buf = Vec::new();
    assert!(req.encode(&mut buf).is_err());

    // Source addr too long
    let mut req = DeliverSmRequest::new(1, "A".repeat(22), "dst".into(), vec![]);
    assert!(req.encode(&mut buf).is_err());

    // Dest addr too long
    let req = DeliverSmRequest::new(1, "src".into(), "B".repeat(22), vec![]);
    assert!(req.encode(&mut buf).is_err());

    // Message too long
    let mut req = DeliverSmRequest::new(1, "src".into(), "dst".into(), vec![0; 255]);
    assert!(req.encode(&mut buf).is_err());
}

#[test]
fn test_deliver_sm_receipt_parsing() {
    // Manually construct a receipt string
    let receipt_text = "id:1234567890 sub:001 dlvrd:001 submit date:2301011000 done date:2301011000 stat:DELIVRD err:000 text:Hello";

    let mut req = DeliverSmRequest::new(
        1,
        "SMSC".into(),
        "ESME".into(),
        receipt_text.as_bytes().to_vec(),
    );
    // Important: Set ESM Class for SMSC Delivery Receipt (0x04)
    req.esm_class = 0x04;

    let receipt = req.parse_receipt().expect("Should parse receipt");

    assert_eq!(receipt.message_id, "1234567890");
    assert_eq!(receipt.submitted_count, 1);
    assert_eq!(receipt.delivered_count, 1);
    assert_eq!(receipt.status, "DELIVRD");
    assert_eq!(receipt.text, "Hello");

    // Test to_string / Display
    let display_str = receipt.to_string();
    assert!(display_str.contains("id:1234567890"));
    assert!(display_str.contains("stat:DELIVRD"));
}

#[test]
fn test_deliver_sm_receipt_parsing_failure() {
    // Missing ID
    let receipt_text = "sub:001 stat:DELIVRD";
    let mut req = DeliverSmRequest::new(
        1,
        "SMSC".into(),
        "ESME".into(),
        receipt_text.as_bytes().to_vec(),
    );
    req.esm_class = 0x04;

    assert!(req.parse_receipt().is_none());

    // Wrong ESM Class
    let receipt_text = "id:1234567890 sub:001 stat:DELIVRD";
    // ESM class 0 (default) -> Not a receipt
    let req = DeliverSmRequest::new(
        1,
        "SMSC".into(),
        "ESME".into(),
        receipt_text.as_bytes().to_vec(),
    );

    assert!(req.parse_receipt().is_none());
}

#[test]
fn test_deliver_sm_resp_encoding_decoding() {
    let resp = DeliverSmResponse::new(999, "ESME_ROK");

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = DeliverSmResponse::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, 999);
    assert_eq!(decoded.command_status, 0);
    assert_eq!(decoded.status_description, "ESME_ROK");
}
