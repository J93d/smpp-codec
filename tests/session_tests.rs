use smpp_codec::common::{Ton, Npi};
use smpp_codec::pdus::{EnquireLinkRequest, EnquireLinkResponse, AlertNotification, GenericNack};
use smpp_codec::tlv::Tlv;

#[test]

fn test_enquire_link_req() {
    let req = EnquireLinkRequest::new(123);
    
    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");
    
    let decoded = EnquireLinkRequest::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, 123);
}

#[test]
#[test]
fn test_enquire_link_resp() {
    let resp = EnquireLinkResponse::new(123, "ESME_ROK");
    
    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");
    
    let decoded = EnquireLinkResponse::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, 123);
    assert_eq!(decoded.command_status, 0);
    assert_eq!(decoded.status_description, "ESME_ROK");
}

#[test]
fn test_alert_notification() {
    let mut alert = AlertNotification::new(
        100, 
        "123".to_string(), 
        "456".to_string()
    )
    .with_source_addr(Ton::International, Npi::Isdn, "123".to_string())
    .with_esme_addr(Ton::National, Npi::Telex, "456".to_string());

    // Add a TLV
    alert.add_tlv(Tlv::new_u16(0x020C, 999)); // sar_msg_ref_num = 999

    let mut buffer = Vec::new();
    alert.encode(&mut buffer).expect("Encode failed");

    let decoded = AlertNotification::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, 100);
    assert_eq!(decoded.source_addr, "123");
    assert_eq!(decoded.source_addr_ton, Ton::International);
    assert_eq!(decoded.esme_addr, "456");
    assert_eq!(decoded.esme_addr_npi, Npi::Telex);
    
    assert_eq!(decoded.optional_params.len(), 1);
    let tlv = &decoded.optional_params[0];
    assert_eq!(tlv.tag, 0x020C);
    assert_eq!(tlv.value_as_u16().unwrap(), 999);
}

#[test]
fn test_generic_nack() {
    let nack = GenericNack::new("ESME_RINVCMDID", 500);
    
    let mut buffer = Vec::new();
    nack.encode(&mut buffer).expect("Encode failed");
    
    let decoded = GenericNack::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, 500);
    assert_eq!(decoded.command_status, 0x00000003); // ESME_RINVCMDID
    assert_eq!(decoded.status_name, "ESME_RINVCMDID");
}
