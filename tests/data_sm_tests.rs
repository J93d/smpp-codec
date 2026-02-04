use smpp_codec::pdus::{DataSm, DataSmResp};
use smpp_codec::tlv::tags;

#[test]
fn test_data_sm_encoding_decoding() {
    let mut req = DataSm::new(
        3001,
        "Source".to_string(),
        "Dest".to_string(),
        b"Data Payload".to_vec(),
    );

    // Set some options
    req.service_type = "CMT".to_string();
    req.registered_delivery = 1;

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = DataSm::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 3001);
    assert_eq!(decoded.service_type, "CMT");
    assert_eq!(decoded.source_addr, "Source");
    assert_eq!(decoded.dest_addr, "Dest");
    assert_eq!(decoded.registered_delivery, 1);

    // Verify Payload TLV
    let mut found_payload = false;
    for tlv in decoded.optional_params {
        if tlv.tag == tags::MESSAGE_PAYLOAD {
            assert_eq!(tlv.value, b"Data Payload");
            found_payload = true;
        }
    }
    assert!(found_payload, "Message Payload TLV missing");
}

#[test]
fn test_data_sm_resp_success() {
    let resp = DataSmResp::new(3001, "ESME_ROK", "DATA-ID-123".to_string());

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = DataSmResp::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 3001);
    assert_eq!(decoded.command_status, 0);
    assert_eq!(decoded.message_id, "DATA-ID-123");
}

#[test]
fn test_data_sm_resp_failure() {
    let resp = DataSmResp::new(3001, "ESME_RSYSERR", "".to_string());

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = DataSmResp::decode(&buffer).expect("Decode failed");

    assert_ne!(decoded.command_status, 0);
    // On failure body might be empty or partial, message_id empty
    assert!(decoded.message_id.is_empty());
}
