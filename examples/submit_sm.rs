use smpp_codec::pdus::{EncodingType, MessageSplitter, SplitMode, SubmitSmRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SMPP Submit SM Example (with Splitter) ===");

    let text = "This is a very long message that will need to be split into multiple parts because it exceeds the standard SMS length limit of 160 characters for GSM 7-bit encoding. The MessageSplitter utility handles this automatically.".to_string();

    println!("Original Text Length: {}", text.len());

    // 1. Split message (handles encoding and valid chunking with UDH)
    let (parts, data_coding) = MessageSplitter::split(text, EncodingType::Gsm7Bit, SplitMode::Udh)?;

    println!("Split into {} parts using UDH concatenation.", parts.len());
    let parts_len = parts.len();

    // 2. Iterate over parts and create/encode PDUs
    for (i, part) in parts.into_iter().enumerate() {
        let sequence_number = (i + 1) as u32;
        let mut submit_req = SubmitSmRequest::new(
            sequence_number,
            "source_addr".to_string(),
            "dest_addr".to_string(),
            part,
        );
        submit_req.data_coding = data_coding;

        // If UDH is present, set the UDHI bit (0x40) in esm_class
        if parts_len > 1 {
            submit_req.esm_class |= 0x40;
        }

        let mut buffer = Vec::new();
        submit_req.encode(&mut buffer)?;

        println!(
            "Part {}: Sequence {}, Encoded {} bytes",
            i + 1,
            sequence_number,
            buffer.len()
        );
    }
    Ok(())
}
