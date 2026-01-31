use smpp_codec::pdus::{QuerySmRequest, QuerySmResponse, MessageState};

fn main() {
    println!("=== SMPP Query SM Example ===");

    // 1. Request
    let query = QuerySmRequest::new(
        300,
        "Msg12345".to_string(),
        "source_addr".to_string(),
    );
    println!("Query Request: {:?}", query);

    let mut buf = Vec::new();
    query.encode(&mut buf).unwrap();
    println!("Encoded {} bytes", buf.len());

    // 2. Response
    let resp = QuerySmResponse::new(
        300, 
        "ESME_ROK", 
        "Msg12345".to_string(), 
        String::new(), // final_date
        MessageState::Delivered as u8,
        0 // error_code
    );
    println!("Query Response: {:?}", resp);
}
