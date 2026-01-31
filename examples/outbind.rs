use smpp_codec::pdus::OutbindRequest;

fn main() {
    println!("=== SMPP Outbind Example ===");

    // 1. Create Outbind Request
    // Outbind is sent by the MC to the ESME to request the ESME to bind.
    let outbind = OutbindRequest::new(
        100, // Sequence Number
        "system_id".to_string(),
        "password".to_string(),
    );

    println!("Outbind Request: {:?}", outbind);

    // 2. Encode
    let mut buffer = Vec::new();
    outbind.encode(&mut buffer).unwrap();
    println!("Encoded {} bytes", buffer.len());
}
