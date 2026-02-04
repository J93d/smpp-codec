use smpp_codec::pdus::UnbindRequest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SMPP Unbind Example ===");

    let unbind_req = UnbindRequest::new(99);
    println!("Unbind Request: {:?}", unbind_req);

    let mut buffer = Vec::new();
    unbind_req.encode(&mut buffer)?;

    println!("Encoded {} bytes", buffer.len());
    Ok(())
}
