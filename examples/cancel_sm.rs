use smpp_codec::pdus::{CancelSmRequest, CancelSmResponse};

fn main() {
    println!("=== SMPP Cancel SM Example ===");

    // 1. Request
    let cancel = CancelSmRequest::new(
        200,
        "Msg12345".to_string(), // Message ID to cancel
        "source_addr".to_string(),
        "dest_addr".to_string(),
    );
    println!("Cancel Request: {:?}", cancel);

    let mut buf = Vec::new();
    cancel.encode(&mut buf).unwrap();
    println!("Encoded {} bytes", buf.len());

    // 2. Response
    let resp = CancelSmResponse::new(200, "ESME_ROK");
    println!("Cancel Response: {:?}", resp);
}
