use smpp_codec::common::{get_status_code, get_status_description, Npi, Ton};

#[test]
fn test_ton_conversion() {
    assert_eq!(Ton::from(0x01), Ton::International);
    assert_eq!(Ton::from(0x02), Ton::National);
    assert_eq!(Ton::from(0x03), Ton::NetworkSpecific);
    assert_eq!(Ton::from(0x04), Ton::SubscriberNumber);
    assert_eq!(Ton::from(0x05), Ton::Alphanumeric);
    assert_eq!(Ton::from(0x06), Ton::Abbreviated);
    assert_eq!(Ton::from(0xFF), Ton::Unknown);
}

#[test]
fn test_npi_conversion() {
    assert_eq!(Npi::from(0x01), Npi::Isdn);
    assert_eq!(Npi::from(0x03), Npi::Data);
    assert_eq!(Npi::from(0x04), Npi::Telex);
    assert_eq!(Npi::from(0x06), Npi::LandMobile);
    assert_eq!(Npi::from(0x08), Npi::National);
    assert_eq!(Npi::from(0x09), Npi::Private);
    assert_eq!(Npi::from(0x0A), Npi::Ermes);
    assert_eq!(Npi::from(0x0E), Npi::Internet);
    assert_eq!(Npi::from(0x12), Npi::Wap);
    assert_eq!(Npi::from(0xFF), Npi::Unknown);
}

#[test]
fn test_status_codes() {
    // Check key status codes description
    assert_eq!(get_status_description(0x00000000), "ESME_ROK");
    assert_eq!(get_status_description(0x00000001), "ESME_RINVMSGLEN");
    assert_eq!(get_status_description(0x00000058), "ESME_RTHROTTLED");
    assert_eq!(get_status_description(0x000000FF), "ESME_RUNKNOWNERR");

    // Check unknown status formatting
    let unknown = get_status_description(0xDEADBEEF);
    assert!(unknown.contains("Unknown Error"));
    assert!(unknown.contains("DEADBEEF"));

    // Check reverse lookup
    assert_eq!(get_status_code("ESME_ROK"), 0x00000000);
    assert_eq!(get_status_code("ESME_RTHROTTLED"), 0x00000058);
    assert_eq!(get_status_code("NON_EXISTENT"), 0x000000FF);
}
