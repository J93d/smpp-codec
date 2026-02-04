use smpp_codec::pdus::{BroadcastSm, BroadcastSmResp};
use smpp_codec::tlv::{tags, Tlv};

#[test]
fn test_broadcast_sm_encoding_decoding() {
    let area_tlv = Tlv::new(tags::BROADCAST_AREA_IDENTIFIER, vec![0x01, 0x02, 0x03]);
    let payload = b"Broadcast Message".to_vec();

    let req = BroadcastSm::new(2001, "Source".to_string(), payload.clone(), area_tlv);

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = BroadcastSm::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 2001);
    assert_eq!(decoded.source_addr, "Source");

    // Check TLVs
    let mut found_payload = false;
    let mut found_area = false;

    for tlv in decoded.optional_params {
        if tlv.tag == tags::MESSAGE_PAYLOAD {
            assert_eq!(tlv.value, payload);
            found_payload = true;
        } else if tlv.tag == tags::BROADCAST_AREA_IDENTIFIER {
            assert_eq!(tlv.value, vec![0x01, 0x02, 0x03]);
            found_area = true;
        }
    }
    assert!(found_payload, "Payload TLV missing");
    assert!(found_area, "Area TLV missing");
}

#[test]
fn test_broadcast_sm_resp_success() {
    let resp = BroadcastSmResp::new(2001, "ESME_ROK", "BC-12345".to_string());

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = BroadcastSmResp::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 2001);
    assert_eq!(decoded.command_status, 0);
    assert_eq!(decoded.message_id, "BC-12345");
}

#[test]
fn test_broadcast_sm_resp_failure() {
    let resp = BroadcastSmResp::new(
        2001,
        "ESME_RINVBCASTAREA", // Invalid broadcast area
        "".to_string(),
    );

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = BroadcastSmResp::decode(&buffer).expect("Decode failed");

    assert_ne!(decoded.command_status, 0);
    // Even on failure, BroadcastResp might return MessageID if partial, but here we passed empty.
    assert!(decoded.message_id.is_empty());
}
