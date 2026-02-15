use smpp_codec::pdus::{ReplaceSmRequest, ReplaceSmResponse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ReplaceSmRequest Example ===");

    let replace_req = ReplaceSmRequest::new(
        1001,
        "msg_123".to_string(),
        "MyService".to_string(),
        b"New Message Content".to_vec(),
    );
    println!("Created Request: {:?}", replace_req);

    let mut buffer = Vec::new();
    replace_req.encode(&mut buffer)?;
    println!("Encoded length: {} bytes", buffer.len());

    let decoded_req = ReplaceSmRequest::decode(&buffer)?;
    println!("Decoded Request Sequence: {}", decoded_req.sequence_number);

    let replace_resp = ReplaceSmResponse::new(1001, "ESME_ROK");
    println!("Created Response: {:?}", replace_resp);
    Ok(())
}
