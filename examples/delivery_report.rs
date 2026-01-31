use smpp_codec::pdus::{DeliverSmRequest, DeliveryReceipt};

fn main() {
    println!("=== SMPP Delivery Report Example ===");

    // 1. Create a Delivery Receipt object
    let receipt = DeliveryReceipt {
        message_id: "1234567890".to_string(),
        submitted_count: 1,
        delivered_count: 1,
        submit_date: "2601312300".to_string(), // YYMMDDhhmm
        done_date: "2601312301".to_string(),   // YYMMDDhhmm
        status: "DELIVRD".to_string(),
        error_code: 0,
        text: "Hello World".to_string(),
    };

    println!("Original Receipt: {:?}", receipt);
    println!("Formatted Text: {}", receipt.to_string());

    // 2. Create DeliverSm PDU wrapping the receipt
    let pdu = DeliverSmRequest::new_receipt(
        101, // Sequence number
        "SMSC".to_string(), // Source (SMSC)
        "SystemId".to_string(), // Dest (ESME)
        receipt.clone(),
    );

    println!("\nCreated DeliverSm PDU: {:?}", pdu);
    println!("ESM Class (should be 4): {}", pdu.esm_class);

    // 3. Encode
    let mut buffer = Vec::new();
    pdu.encode(&mut buffer).expect("Failed to encode");
    println!("Encoded PDU size: {} bytes", buffer.len());

    // 4. Decode
    let decoded_pdu = DeliverSmRequest::decode(&buffer).expect("Failed to decode");
    println!("\nDecoded PDU: {:?}", decoded_pdu);

    // 5. Parse Receipt from PDU
    if let Some(parsed_receipt) = decoded_pdu.parse_receipt() {
        println!("Successfully parsed receipt from PDU:");
        println!("  Message ID: {}", parsed_receipt.message_id);
        println!("  Status: {}", parsed_receipt.status);
        println!("  Done Date: {}", parsed_receipt.done_date);
    } else {
        println!("Failed to parse receipt from decoded PDU!");
    }
}
