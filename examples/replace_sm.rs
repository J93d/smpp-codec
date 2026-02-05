use smpp_codec::pdus::{ReplaceSm, ReplaceSmResp};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ReplaceSm Example ===");

    let replace_req = ReplaceSm::new(
        1001,
        "msg_123".to_string(),
        "MyService".to_string(),
        b"New Message Content".to_vec(),
    );
    println!("Created Request: {:?}", replace_req);

    let mut buffer = Vec::new();
    replace_req.encode(&mut buffer)?;
    println!("Encoded length: {} bytes", buffer.len());

    let decoded_req = ReplaceSm::decode(&buffer)?;
    println!("Decoded Request Sequence: {}", decoded_req.sequence_number);

    let replace_resp = ReplaceSmResp::new(1001, "ESME_ROK");
    println!("Created Response: {:?}", replace_resp);
    Ok(())
}
