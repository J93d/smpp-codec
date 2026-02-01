use smpp_codec::pdus::{DeliverSmRequest, EncodingType, MessageSplitter, SplitMode};

fn main() {
    println!("=== SMPP Deliver SM Example (Incoming Message) ===");

    // Simulate a long incoming message content
    let text = "This is a simulated long incoming message that behaves exactly like SubmitSM. The MessageSplitter is used to chunk it, and the DeliverSmRequest struct is used to represent each chunk sent to the ESME.".to_string();

    println!("Incoming Text Length: {}", text.len());

    // 1. Split message
    let (parts, data_coding) =
        MessageSplitter::split(text, EncodingType::Gsm7Bit, SplitMode::Udh).unwrap();

    println!("Split into {} parts.", parts.len());
    let parts_len = parts.len();

    // 2. Create DeliverSm PDUs for each part
    for (i, part) in parts.into_iter().enumerate() {
        let sequence_number = (i + 100) as u32;
        let mut deliver_req = DeliverSmRequest::new(
            sequence_number,
            "sender_number".to_string(),
            "my_shortcode".to_string(),
            part,
        );
        deliver_req.data_coding = data_coding;

        // CRITICAL: Set UDHI bit (0x40) in esm_class if UDH is present (parts > 1)
        if parts_len > 1 {
            deliver_req.esm_class |= 0x40;
        }

        let mut buffer = Vec::new();
        deliver_req.encode(&mut buffer).unwrap();

        println!(
            "Part {}: Sequence {}, Encoded {} bytes. UDHI bit set: {}",
            i + 1,
            sequence_number,
            buffer.len(),
            (deliver_req.esm_class & 0x40) != 0
        );
    }
}
