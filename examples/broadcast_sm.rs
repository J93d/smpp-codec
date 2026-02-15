use smpp_codec::pdus::BroadcastSmRequest;
use smpp_codec::tlv::{tags, Tlv};

fn main() {
    println!("=== SMPP Broadcast SM Example ===");

    // 1. Prepare Mandatory TLVs
    // Broadcast Area Identifier (Tag: 0x0606)
    // Structure depends on network (e.g., Cell ID, Location Area, etc.)
    // Here we use a dummy value.
    let area_tlv = Tlv::new(
        tags::BROADCAST_AREA_IDENTIFIER,
        vec![0x01, 0x00, 0x05], // Format + Data
    );

    let payload = b"Emergency Alert: Weather Warning".to_vec();

    // 2. Create BroadcastSmRequest PDU
    let mut req =
        BroadcastSmRequest::new(2001, "ServiceAlert".to_string(), payload.clone(), area_tlv);

    // Set optional parameters
    req.priority_flag = 3; // Urgent

    // 3. Encode
    let mut buffer = Vec::new();
    match req.encode(&mut buffer) {
        Ok(_) => println!(
            "Successfully encoded BroadcastSmRequest PDU: {} bytes",
            buffer.len()
        ),
        Err(e) => eprintln!("Failed to encode PDU: {:?}", e),
    }

    // 4. Decode
    match BroadcastSmRequest::decode(&buffer) {
        Ok(decoded) => {
            println!("Successfully decoded PDU.");
            println!("  Sequence: {}", decoded.sequence_number);
            println!("  Source: {}", decoded.source_addr);

            // Check for payload in TLVs
            let has_payload = decoded
                .optional_params
                .iter()
                .any(|t| t.tag == tags::MESSAGE_PAYLOAD);
            println!("  Has Payload TLV: {}", has_payload);
        }
        Err(e) => eprintln!("Failed to decode PDU: {:?}", e),
    }
}
