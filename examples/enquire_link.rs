use smpp_codec::pdus::{EnquireLinkRequest, EnquireLinkResponse};

fn main() {
    println!("=== SMPP Enquire Link Example ===");

    // 1. EnquireLink Request
    println!("\n--- Request ---");
    let enquire_link = EnquireLinkRequest::new(100);
    println!("Request: {:?}", enquire_link);
    let mut buf = Vec::new();
    enquire_link.encode(&mut buf).unwrap();
    println!("Encoded {} bytes", buf.len());

    // 2. EnquireLink Response
    println!("\n--- Response ---");
    let resp = EnquireLinkResponse::new(100, "ESME_ROK");
    println!("Response: {:?}", resp);

    let mut buf2 = Vec::new();
    resp.encode(&mut buf2).unwrap();
    println!("Encoded {} bytes", buf2.len());
}
