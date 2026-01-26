use smpp_codec::common::{BindMode, Ton, Npi};
use smpp_codec::pdus::BindRequest;

fn main() {
    println!("=== SMPP Bind Request Example ===");

    // 1. Create a BindTransceiver Request
    let bind_req = BindRequest::new(
        BindMode::Transceiver,
        "my_system_id".to_string(),
        "password".to_string(),
        1, // Sequence Number
    ).with_address_range(Ton::International, Npi::Isdn, "12345".to_string());

    println!("Created PDU: {:?}", bind_req);

    // 2. Encode into bytes
    let mut buffer = Vec::new();
    match bind_req.encode(&mut buffer) {
        Ok(_) => {
            println!("Encoded successfully! {} bytes", buffer.len());
            print!("Hex encoded: ");
            for byte in &buffer {
                print!("{:02X} ", byte);
            }
            println!();
        }
        Err(e) => {
            eprintln!("Failed to encode: {:?}", e);
        }
    }
}
