use smpp_codec::pdus::{CancelBroadcastSm, CancelBroadcastSmResp};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CancelBroadcastSm Example ===");

    let cancel_req = CancelBroadcastSm::new(
        3001,
        "CMT".to_string(),
        "bc_msg_002".to_string(),
        "SourceAddr".to_string(),
    );
    println!("Created Request: {:?}", cancel_req);

    let mut buffer = Vec::new();
    cancel_req.encode(&mut buffer)?;

    let decoded_cancel = CancelBroadcastSm::decode(&buffer)?;
    println!("Decoded Cancel Message ID: {}", decoded_cancel.message_id);

    let cancel_resp = CancelBroadcastSmResp::new(3001, "ESME_ROK");
    println!("Created Response: {:?}", cancel_resp);
    Ok(())
}
