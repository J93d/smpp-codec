use smpp_codec::pdus::{QueryBroadcastSm, QueryBroadcastSmResp};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== QueryBroadcastSm Example ===");

    let query_req = QueryBroadcastSm::new(2001, "bc_msg_001".to_string(), "SourceAddr".to_string());
    println!("Created Request: {:?}", query_req);

    let mut buffer = Vec::new();
    query_req.encode(&mut buffer)?;

    let decoded_query = QueryBroadcastSm::decode(&buffer)?;
    println!("Decoded Query Message ID: {}", decoded_query.message_id);

    let query_resp = QueryBroadcastSmResp::new(2001, "ESME_ROK", "bc_msg_001".to_string());
    println!("Created Response: {:?}", query_resp);
    Ok(())
}
