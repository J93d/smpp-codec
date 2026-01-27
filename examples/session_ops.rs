use smpp_codec::common::{Ton, Npi};
use smpp_codec::pdus::{EnquireLinkRequest, EnquireLinkResponse, AlertNotification, GenericNack};
use smpp_codec::tlv::Tlv;

fn main() {
    println!("=== SMPP Session Operations Example ===");

    // 1. EnquireLink
    println!("\n--- EnquireLink ---");
    let enquire_link = EnquireLinkRequest::new(100);
    println!("Request: {:?}", enquire_link);
    let mut buf = Vec::new();
    enquire_link.encode(&mut buf).unwrap();
    println!("Encoded {} bytes", buf.len());

    let resp = EnquireLinkResponse::new(100, 0x80000015, "ESME_ROK");
    println!("Response: {:?}", resp);

    // 2. AlertNotification
    println!("\n--- AlertNotification ---");
    let mut alert = AlertNotification::new(
        200,
        "source_addr".to_string(),
        "esme_addr".to_string(),
    );
    // Add a TLV using the helper
    alert.add_tlv(Tlv::new_u16_from_name("sar_msg_ref_num", 1234));
    println!("Alert: {:?}", alert);
    
    let mut buf2 = Vec::new();
    alert.encode(&mut buf2).unwrap();
    println!("Encoded {} bytes", buf2.len());

    // 3. GenericNack
    println!("\n--- GenericNack ---");
    let nack = GenericNack::new("ESME_RINVCMDID", 300);
    println!("Nack: {:?}", nack);
    
    let mut buf3 = Vec::new();
    nack.encode(&mut buf3).unwrap();
    println!("Encoded {} bytes", buf3.len());
}
