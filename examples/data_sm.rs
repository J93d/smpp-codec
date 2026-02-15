use smpp_codec::pdus::DataSmRequest;

fn main() {
    println!("=== SMPP Data SM Example ===");

    // 1. Create DataSmRequest PDU
    // DataSmRequest is used for session-based data transfer, offering an alternative to Submit/Deliver.
    let mut req = DataSmRequest::new(
        3001,
        "AppServer".to_string(),
        "MobileClient".to_string(),
        b"Session Data Payload".to_vec(),
    );

    // Set options
    req.service_type = "WAP".to_string();
    req.registered_delivery = 1;

    // 2. Encode
    let mut buffer = Vec::new();
    match req.encode(&mut buffer) {
        Ok(_) => println!("Successfully encoded DataSmRequest PDU: {} bytes", buffer.len()),
        Err(e) => eprintln!("Failed to encode PDU: {:?}", e),
    }

    // 3. Decode
    match DataSmRequest::decode(&buffer) {
        Ok(decoded) => {
            println!("Successfully decoded PDU.");
            println!("  Sequence: {}", decoded.sequence_number);
            println!("  Service Type: {}", decoded.service_type);
            println!("  Registered Delivery: {}", decoded.registered_delivery);
            // Payload is in Optional Parameters
            println!("  Optional Params Count: {}", decoded.optional_params.len());
        }
        Err(e) => eprintln!("Failed to decode PDU: {:?}", e),
    }
}
