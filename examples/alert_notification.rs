use smpp_codec::common::{Npi, Ton};
use smpp_codec::pdus::AlertNotification;
use smpp_codec::tlv::Tlv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SMPP Alert Notification Example ===");

    // 1. Create Alert Notification
    let mut alert = AlertNotification::new(
        200, // Sequence number
        "source_addr".to_string(),
        "esme_addr".to_string(),
    )
    .with_source_addr(Ton::International, Npi::Isdn, "123".to_string())
    .with_esme_addr(Ton::National, Npi::Telex, "456".to_string());

    // 2. Add optional TLV (e.g., ms_availability_status)
    // Tag 0x0402 is ms_availability_status
    let tlv = Tlv::new_u8(0x0402, 1); // 1 = Available
    alert.add_tlv(tlv);

    println!("Alert Notification: {:?}", alert);

    // 3. Encode
    let mut buf = Vec::new();
    alert.encode(&mut buf)?;
    println!("Encoded {} bytes", buf.len());
    Ok(())
}
