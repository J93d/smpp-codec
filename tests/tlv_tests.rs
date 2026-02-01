use smpp_codec::common::PduError;
use smpp_codec::tlv::{tags, Tlv};
use std::io::Cursor;

#[test]
fn test_tlv_u8_constructors() {
    let tlv = Tlv::new_u8(tags::DEST_ADDR_SUBUNIT, 0x01);
    assert_eq!(tlv.tag, tags::DEST_ADDR_SUBUNIT);
    assert_eq!(tlv.length, 1);
    assert_eq!(tlv.value, vec![0x01]);
    assert_eq!(tlv.value_as_u8().unwrap(), 0x01);

    let tlv_named = Tlv::new_u8_from_name("dest_addr_subunit", 0x02);
    assert_eq!(tlv_named.tag, tags::DEST_ADDR_SUBUNIT);
    assert_eq!(tlv_named.value_as_u8().unwrap(), 0x02);
}

#[test]
fn test_tlv_u16_constructors() {
    let tlv = Tlv::new_u16(tags::SAR_MSG_REF_NUM, 0x1234);
    assert_eq!(tlv.tag, tags::SAR_MSG_REF_NUM);
    assert_eq!(tlv.length, 2);
    // Big Endian check because SMPP is BE
    assert_eq!(tlv.value, vec![0x12, 0x34]);
    assert_eq!(tlv.value_as_u16().unwrap(), 0x1234);

    let tlv_named = Tlv::new_u16_from_name("sar_msg_ref_num", 0x5678);
    assert_eq!(tlv_named.tag, tags::SAR_MSG_REF_NUM);
    assert_eq!(tlv_named.value_as_u16().unwrap(), 0x5678);
}

#[test]
fn test_tlv_string_constructors() {
    let text = "Hello";
    let tlv = Tlv::new_string(tags::ADDITIONAL_STATUS_INFO_TEXT, text);
    assert_eq!(tlv.tag, tags::ADDITIONAL_STATUS_INFO_TEXT);
    // Length should be text len + 1 (null terminator)
    assert_eq!(tlv.length, 6);
    assert_eq!(tlv.value, b"Hello\0".to_vec());
    assert_eq!(tlv.value_as_string().unwrap(), "Hello");
}

#[test]
fn test_tlv_payload_constructor() {
    let data = vec![0x01, 0x02, 0x03];
    let tlv = Tlv::new_payload(tags::MESSAGE_PAYLOAD, data.clone());
    assert_eq!(tlv.tag, tags::MESSAGE_PAYLOAD);
    assert_eq!(tlv.length, 3);
    assert_eq!(tlv.value, data);
}

#[test]
fn test_tlv_getters_errors() {
    // U8 mismatch
    let tlv_u16 = Tlv::new_u16(tags::SAR_MSG_REF_NUM, 123);
    assert!(tlv_u16.value_as_u8().is_err()); // Expect InvalidLength

    // U16 mismatch
    let tlv_u8 = Tlv::new_u8(tags::DEST_ADDR_SUBUNIT, 1);
    assert!(tlv_u8.value_as_u16().is_err()); // Expect InvalidLength

    // String utf8 error
    let bad_utf8 = vec![0xFF, 0xFF, 0x00];
    let tlv_bad = Tlv::new(tags::ADDITIONAL_STATUS_INFO_TEXT, bad_utf8);
    assert!(match tlv_bad.value_as_string() {
        Err(PduError::Utf8(_)) => true,
        _ => false,
    });
}

#[test]
fn test_tlv_encoding_decoding() {
    let tlv = Tlv::new_u16(0x1234, 0xABCD);
    let mut buffer = Vec::new();
    tlv.encode(&mut buffer).expect("Encode failed");

    // 2 bytes tag, 2 bytes len, 2 bytes value
    assert_eq!(buffer.len(), 6);
    assert_eq!(buffer, vec![0x12, 0x34, 0x00, 0x02, 0xAB, 0xCD]);

    // Fix: create cursor over slice explicitly
    let decoded = Tlv::decode(&mut Cursor::new(&buffer[..]))
        .expect("Decode failed")
        .expect("Should return some");
    assert_eq!(decoded, tlv);
}

#[test]
fn test_tlv_decode_partial() {
    // Only tag (2 bytes)
    let buffer = vec![0x12, 0x34];
    let mut cursor = Cursor::new(&buffer[..]);
    let result = Tlv::decode(&mut cursor).expect("Should not error on EOF check, just None");
    assert!(result.is_none());

    // Tag + Len but no value
    let buffer = vec![0x12, 0x34, 0x00, 0x05];
    let mut cursor = Cursor::new(&buffer[..]);
    // decode checks length upfront
    let err = Tlv::decode(&mut cursor).unwrap_err();
    match err {
        PduError::BufferTooShort => (),
        _ => panic!("Expected BufferTooShort, got {:?}", err),
    }
}

#[test]
fn test_get_tag_by_name() {
    use smpp_codec::tlv::{get_tag_by_name, tags};

    assert_eq!(
        get_tag_by_name("dest_addr_subunit"),
        tags::DEST_ADDR_SUBUNIT
    );
    assert_eq!(
        get_tag_by_name("DEST_ADDR_SUBUNIT"),
        tags::DEST_ADDR_SUBUNIT
    );
    assert_eq!(get_tag_by_name("sar_msg_ref_num"), tags::SAR_MSG_REF_NUM);
    assert_eq!(get_tag_by_name("message_payload"), tags::MESSAGE_PAYLOAD);
    assert_eq!(
        get_tag_by_name("alert_on_message_delivery"),
        tags::ALERT_ON_MESSAGE_DELIVERY
    );

    // Case sensitivity check (impl handles both usually)
    assert_eq!(get_tag_by_name("SAR_MSG_REF_NUM"), tags::SAR_MSG_REF_NUM);

    // Unknown tag
    assert_eq!(get_tag_by_name("unknown_tag_xyz"), 0);
}
