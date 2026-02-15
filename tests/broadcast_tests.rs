use smpp_codec::pdus::{BroadcastSmRequest, BroadcastSmResponse};
use smpp_codec::tlv::{tags, Tlv};

#[test]
fn test_broadcast_sm_encoding_decoding() {
    let area_tlv = Tlv::new(tags::BROADCAST_AREA_IDENTIFIER, vec![0x01, 0x02, 0x03]);
    let payload = b"Broadcast Message".to_vec();

    let req = BroadcastSmRequest::new(2001, "Source".to_string(), payload.clone(), area_tlv);

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = BroadcastSmRequest::decode(&buffer).expect("Decode failed");

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
    let resp = BroadcastSmResponse::new(2001, "ESME_ROK", "BC-12345".to_string());

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = BroadcastSmResponse::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 2001);
    assert_eq!(decoded.command_status, 0);
    assert_eq!(decoded.message_id, "BC-12345");
}

#[test]
fn test_broadcast_sm_resp_failure() {
    let resp = BroadcastSmResponse::new(
        2001,
        "ESME_RINVBCASTAREA", // Invalid broadcast area
        "".to_string(),
    );

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = BroadcastSmResponse::decode(&buffer).expect("Decode failed");

    // Even on failure, BroadcastResp might return MessageID if partial, but here we passed empty.
    assert!(decoded.message_id.is_empty());
}

#[test]
fn test_broadcast_sm_full_fields() {
    use smpp_codec::common::{Npi, Ton};

    let area_tlv = Tlv::new(tags::BROADCAST_AREA_IDENTIFIER, vec![0x01]);
    let payload = b"Full Payload".to_vec();

    let mut req = BroadcastSmRequest::new(555, "SrcOrigin".to_string(), payload, area_tlv);

    // Set all fields
    req.service_type = "CMT".to_string();
    req.source_addr_ton = Ton::International;
    req.source_addr_npi = Npi::Isdn;
    req.message_id = "MsgID-99".to_string();
    req.priority_flag = 1;
    req.schedule_delivery_time = "231201000000000R".to_string();
    req.validity_period = "231202000000000R".to_string();
    req.replace_if_present_flag = 1;
    req.data_coding = 0x01;
    req.sm_default_msg_id = 99;

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = BroadcastSmRequest::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 555);
    assert_eq!(decoded.service_type, "CMT");
    assert_eq!(decoded.source_addr, "SrcOrigin");
    assert_eq!(decoded.source_addr_ton, Ton::International);
    assert_eq!(decoded.source_addr_npi, Npi::Isdn);
    assert_eq!(decoded.message_id, "MsgID-99");
    assert_eq!(decoded.priority_flag, 1);
    assert_eq!(decoded.schedule_delivery_time, "231201000000000R");
    assert_eq!(decoded.validity_period, "231202000000000R");
    assert_eq!(decoded.replace_if_present_flag, 1);
    assert_eq!(decoded.data_coding, 0x01);
    assert_eq!(decoded.sm_default_msg_id, 99);
}
