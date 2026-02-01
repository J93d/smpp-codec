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
fn test_deliver_sm_resp_encoding_decoding() {
    let resp = DeliverSmResponse::new(999, "ESME_ROK");

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = DeliverSmResponse::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, 999);
    assert_eq!(decoded.command_status, 0);
    assert_eq!(decoded.status_description, "ESME_ROK");
}
