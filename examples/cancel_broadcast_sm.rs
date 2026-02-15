use smpp_codec::pdus::{CancelBroadcastSmRequest, CancelBroadcastSmResponse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CancelBroadcastSmRequest Example ===");

    let cancel_req = CancelBroadcastSmRequest::new(
        3001,
        "CMT".to_string(),
        "bc_msg_002".to_string(),
        "SourceAddr".to_string(),
    );
    println!("Created Request: {:?}", cancel_req);

    let mut buffer = Vec::new();
    cancel_req.encode(&mut buffer)?;

    let decoded_cancel = CancelBroadcastSmRequest::decode(&buffer)?;
    println!("Decoded Cancel Message ID: {}", decoded_cancel.message_id);

    let cancel_resp = CancelBroadcastSmResponse::new(3001, "ESME_ROK");
    println!("Created Response: {:?}", cancel_resp);
    Ok(())
}
