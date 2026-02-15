use smpp_codec::common::{Npi, Ton};
use smpp_codec::pdus::{Destination, SubmitMultiRequest};

fn main() {
    println!("=== SMPP Submit Multi Example ===");

    // 1. Create Destinations
    // Destination 1: SME Address
    let dest_sme = Destination::SmeAddress {
        ton: Ton::International,
        npi: Npi::Isdn,
        address: "1234567890".to_string(),
    };

    // Destination 2: Distribution List
    let dest_dl = Destination::DistributionList("MyGroup".to_string());

    let destinations = vec![dest_sme, dest_dl];

    println!("Targeting {} destinations.", destinations.len());

    // 2. Create SubmitMultiRequest PDU
    let mut req = SubmitMultiRequest::new(
        1001,
        "Sender".to_string(),
        destinations,
        b"Hello Multicast World!".to_vec(),
    );

    // Set optional parameters if needed
    req.priority_flag = 1;
    req.registered_delivery = 1;

    // 3. Encode
    let mut buffer = Vec::new();
    match req.encode(&mut buffer) {
        Ok(_) => println!(
            "Successfully encoded SubmitMultiRequest PDU: {} bytes",
            buffer.len()
        ),
        Err(e) => eprintln!("Failed to encode PDU: {:?}", e),
    }

    // 4. (Simulated) Decode
    match SubmitMultiRequest::decode(&buffer) {
        Ok(decoded) => {
            println!("Successfully decoded PDU.");
            println!("  Sequence: {}", decoded.sequence_number);
            println!("  Destinations: {}", decoded.destinations.len());
            println!(
                "  Message: {:?}",
                String::from_utf8_lossy(&decoded.short_message)
            );
        }
        Err(e) => eprintln!("Failed to decode PDU: {:?}", e),
    }
}
