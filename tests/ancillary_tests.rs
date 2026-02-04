use smpp_codec::pdus::{
    CancelBroadcastSm, CancelBroadcastSmResp, QueryBroadcastSm, QueryBroadcastSmResp, ReplaceSm,
    ReplaceSmResp,
};

#[test]
fn test_replace_sm() {
    let req = ReplaceSm::new(
        100,
        "msg_123".to_string(),
        "source".to_string(),
        b"New Message Content".to_vec(),
    );

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = ReplaceSm::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 100);
    assert_eq!(decoded.message_id, "msg_123");
    assert_eq!(decoded.short_message, b"New Message Content");
}

#[test]
fn test_replace_sm_resp() {
    let resp = ReplaceSmResp::new(100, "ESME_ROK");

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = ReplaceSmResp::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 100);
    assert_eq!(decoded.command_status, 0);
}

#[test]
fn test_query_broadcast_sm() {
    let req = QueryBroadcastSm::new(200, "bc_msg_1".to_string(), "source".to_string());

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = QueryBroadcastSm::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 200);
    assert_eq!(decoded.message_id, "bc_msg_1");
}

#[test]
fn test_query_broadcast_sm_resp() {
    let resp = QueryBroadcastSmResp::new(200, "ESME_ROK", "bc_msg_1".to_string());

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = QueryBroadcastSmResp::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 200);
    assert_eq!(decoded.message_id, "bc_msg_1");
}

#[test]
fn test_cancel_broadcast_sm() {
    let req = CancelBroadcastSm::new(
        300,
        "CMT".to_string(),
        "bc_msg_2".to_string(),
        "source".to_string(),
    );

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = CancelBroadcastSm::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 300);
    assert_eq!(decoded.message_id, "bc_msg_2");
    assert_eq!(decoded.service_type, "CMT");
}

#[test]
fn test_cancel_broadcast_sm_resp() {
    let resp = CancelBroadcastSmResp::new(300, "ESME_ROK");

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = CancelBroadcastSmResp::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 300);
    assert_eq!(decoded.command_status, 0);
}
