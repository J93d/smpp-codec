use smpp_codec::pdus::{QueryBroadcastSmRequest, QueryBroadcastSmResponse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== QueryBroadcastSmRequest Example ===");

    let query_req =
        QueryBroadcastSmRequest::new(2001, "bc_msg_001".to_string(), "SourceAddr".to_string());
    println!("Created Request: {:?}", query_req);

    let mut buffer = Vec::new();
    query_req.encode(&mut buffer)?;

    let decoded_query = QueryBroadcastSmRequest::decode(&buffer)?;
    println!("Decoded Query Message ID: {}", decoded_query.message_id);

    let query_resp = QueryBroadcastSmResponse::new(2001, "ESME_ROK", "bc_msg_001".to_string());
    println!("Created Response: {:?}", query_resp);
    Ok(())
}
