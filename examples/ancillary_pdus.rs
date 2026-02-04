use smpp_codec::pdus::{
    CancelBroadcastSm, CancelBroadcastSmResp, QueryBroadcastSm, QueryBroadcastSmResp, ReplaceSm,
    ReplaceSmResp,
};
use smpp_codec::tlv::{tags, Tlv};
use std::io::Cursor;

fn main() {
    println!("=== Ancillary PDUs Example ===");

    // --- ReplaceSm ---
    println!("\n--- ReplaceSm ---");
    let replace_req = ReplaceSm::new(
        1001,
        "msg_123".to_string(),
        "MyService".to_string(),
        b"New Message Content".to_vec(),
    );
    println!("Created Request: {:?}", replace_req);

    let mut buffer = Vec::new();
    replace_req.encode(&mut buffer).unwrap();
    println!("Encoded length: {} bytes", buffer.len());

    let decoded_req = ReplaceSm::decode(&buffer).unwrap();
    println!("Decoded Request Sequence: {}", decoded_req.sequence_number);

    let replace_resp = ReplaceSmResp::new(1001, "ESME_ROK");
    println!("Created Response: {:?}", replace_resp);

    // --- QueryBroadcastSm ---
    println!("\n--- QueryBroadcastSm ---");
    let query_req = QueryBroadcastSm::new(2001, "bc_msg_001".to_string(), "SourceAddr".to_string());
    println!("Created Request: {:?}", query_req);

    let mut buffer = Vec::new();
    query_req.encode(&mut buffer).unwrap();

    let decoded_query = QueryBroadcastSm::decode(&buffer).unwrap();
    println!("Decoded Query Message ID: {}", decoded_query.message_id);

    let query_resp = QueryBroadcastSmResp::new(2001, "ESME_ROK", "bc_msg_001".to_string());
    println!("Created Response: {:?}", query_resp);

    // --- CancelBroadcastSm ---
    println!("\n--- CancelBroadcastSm ---");
    let cancel_req = CancelBroadcastSm::new(
        3001,
        "CMT".to_string(),
        "bc_msg_002".to_string(),
        "SourceAddr".to_string(),
    );
    println!("Created Request: {:?}", cancel_req);

    let mut buffer = Vec::new();
    cancel_req.encode(&mut buffer).unwrap();

    let decoded_cancel = CancelBroadcastSm::decode(&buffer).unwrap();
    println!("Decoded Cancel Message ID: {}", decoded_cancel.message_id);

    let cancel_resp = CancelBroadcastSmResp::new(3001, "ESME_ROK");
    println!("Created Response: {:?}", cancel_resp);
}
