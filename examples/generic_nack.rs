use smpp_codec::pdus::GenericNack;

fn main() {
    println!("=== SMPP Generic Nack Example ===");

    // 1. Create Generic Nack
    // Used when an invalid command_id is received (header decoding fails or unknown command)
    let nack = GenericNack::new(
        "ESME_RINVCMDID", // Status Name
        500,              // Sequence Number of the failed request
    );

    println!("Generic Nack: {:?}", nack);

    // 2. Encode
    let mut buf = Vec::new();
    nack.encode(&mut buf).unwrap();
    println!("Encoded {} bytes", buf.len());
    
    // 3. Decode verification
    let decoded = GenericNack::decode(&buf).unwrap();
    println!("Decoded Command Status: 0x{:08X}", decoded.command_status);
}
