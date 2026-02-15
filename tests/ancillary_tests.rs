use smpp_codec::common::{Npi, Ton};
use smpp_codec::pdus::{
    CancelBroadcastSmRequest, CancelBroadcastSmResponse, CancelSmRequest, CancelSmResponse,
    QueryBroadcastSmRequest, QueryBroadcastSmResponse, QuerySmRequest, QuerySmResponse,
    ReplaceSmRequest, ReplaceSmResponse,
};

#[test]
fn test_replace_sm() {
    let req = ReplaceSmRequest::new(
        100,
        "msg_123".to_string(),
        "source".to_string(),
        b"New Message Content".to_vec(),
    );

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = ReplaceSmRequest::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 100);
    assert_eq!(decoded.message_id, "msg_123");
    assert_eq!(decoded.short_message, b"New Message Content");
}

#[test]
fn test_replace_sm_resp() {
    let resp = ReplaceSmResponse::new(100, "ESME_ROK");

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = ReplaceSmResponse::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 100);
    assert_eq!(decoded.command_status, 0);
}

#[test]
fn test_query_broadcast_sm() {
    let req = QueryBroadcastSmRequest::new(200, "bc_msg_1".to_string(), "source".to_string());

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = QueryBroadcastSmRequest::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 200);
    assert_eq!(decoded.message_id, "bc_msg_1");
}

#[test]
fn test_query_broadcast_sm_resp() {
    let resp = QueryBroadcastSmResponse::new(200, "ESME_ROK", "bc_msg_1".to_string());

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = QueryBroadcastSmResponse::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 200);
    assert_eq!(decoded.message_id, "bc_msg_1");
}

#[test]
fn test_cancel_broadcast_sm() {
    let req = CancelBroadcastSmRequest::new(
        300,
        "CMT".to_string(),
        "bc_msg_2".to_string(),
        "source".to_string(),
    );

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = CancelBroadcastSmRequest::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 300);
    assert_eq!(decoded.message_id, "bc_msg_2");
    assert_eq!(decoded.service_type, "CMT");
}

#[test]
fn test_cancel_broadcast_sm_resp() {
    let resp = CancelBroadcastSmResponse::new(300, "ESME_ROK");

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = CancelBroadcastSmResponse::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 300);
    assert_eq!(decoded.command_status, 0);
}

#[test]
fn test_replace_sm_full_fields() {
    let mut req = ReplaceSmRequest::new(
        101,
        "msg_original".to_string(),
        "source_addr".to_string(),
        b"New Content".to_vec(),
    );

    req.source_addr_ton = Ton::International;
    req.source_addr_npi = Npi::Isdn;
    req.schedule_delivery_time = "231201000000000R".to_string();
    req.validity_period = "231202000000000R".to_string();
    req.registered_delivery = 1;
    req.sm_default_msg_id = 5;

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = ReplaceSmRequest::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 101);
    assert_eq!(decoded.source_addr_ton, Ton::International);
    assert_eq!(decoded.source_addr_npi, Npi::Isdn);
    assert_eq!(decoded.schedule_delivery_time, "231201000000000R");
    assert_eq!(decoded.validity_period, "231202000000000R");
    assert_eq!(decoded.registered_delivery, 1);
    assert_eq!(decoded.sm_default_msg_id, 5);
}

#[test]
fn test_cancel_sm() {
    let mut req = CancelSmRequest::new(
        400,
        "msg_to_cancel".to_string(),
        "src".to_string(),
        "dst".to_string(),
    );

    req.service_type = "CMT".to_string();
    req.source_addr_ton = Ton::National;
    req.source_addr_npi = Npi::Telex;
    req.dest_addr_ton = Ton::SubscriberNumber;
    req.dest_addr_npi = Npi::LandMobile;

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = CancelSmRequest::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 400);
    assert_eq!(decoded.service_type, "CMT");
    assert_eq!(decoded.source_addr_ton, Ton::National);
    assert_eq!(decoded.source_addr_npi, Npi::Telex);
    assert_eq!(decoded.dest_addr_ton, Ton::SubscriberNumber);
    assert_eq!(decoded.dest_addr_npi, Npi::LandMobile);
}

#[test]
fn test_cancel_sm_response() {
    let resp = CancelSmResponse::new(400, "ESME_ROK");

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = CancelSmResponse::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 400);
    assert_eq!(decoded.command_status, 0);
    assert_eq!(decoded.status_description, "ESME_ROK");
}

#[test]
fn test_query_sm() {
    let mut req = QuerySmRequest::new(500, "msg_query".to_string(), "src".to_string());

    req.source_addr_ton = Ton::Abbreviated;
    req.source_addr_npi = Npi::Private;

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = QuerySmRequest::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 500);
    assert_eq!(decoded.message_id, "msg_query");
    assert_eq!(decoded.source_addr_ton, Ton::Abbreviated);
    assert_eq!(decoded.source_addr_npi, Npi::Private);
}

#[test]
fn test_query_sm_validation_error() {
    // Test Message ID too long (max 64)
    let req = QuerySmRequest::new(500, "A".repeat(65), "src".to_string());
    let mut buffer = Vec::new();
    assert!(req.encode(&mut buffer).is_err());
}

#[test]
fn test_query_sm_response() {
    // Success response with body
    let resp = QuerySmResponse::new(
        500,
        "ESME_ROK",
        "msg_query".to_string(),
        "230101120000".to_string(), // Final Date
        2,                          // Delivered
        0,                          // Error Code
    );

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = QuerySmResponse::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 500);
    assert_eq!(decoded.command_status, 0);
    assert_eq!(decoded.message_id, "msg_query");
    assert_eq!(decoded.final_date, "230101120000");
    assert_eq!(decoded.message_state, 2);
    assert_eq!(decoded.error_code, 0);

    // Error response (No body)
    let resp = QuerySmResponse::new(500, "ESME_RQUERYFAIL", "".to_string(), "".to_string(), 0, 0);
    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    // Body length is 0 for error
    assert_eq!(buffer.len(), 16);

    let decoded = QuerySmResponse::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.command_status, 0x00000067);
}
