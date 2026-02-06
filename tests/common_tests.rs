use smpp_codec::common::{
    get_status_code, get_status_description, BindMode, Npi, PduError, Ton, CMD_BIND_RECEIVER,
    CMD_BIND_TRANSCEIVER, CMD_BIND_TRANSMITTER,
};

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

#[test]
fn test_bind_mode_command_ids() {
    assert_eq!(BindMode::Receiver.command_id(), CMD_BIND_RECEIVER);
    assert_eq!(BindMode::Transmitter.command_id(), CMD_BIND_TRANSMITTER);
    assert_eq!(BindMode::Transceiver.command_id(), CMD_BIND_TRANSCEIVER);
}

#[test]
fn test_pdu_error_display() {
    let err = PduError::BufferTooShort;
    assert_eq!(format!("{}", err), "Buffer contains insufficient data");

    let err = PduError::InvalidCommandId(0x12345678);
    assert_eq!(format!("{}", err), "Invalid Command ID: 0x12345678");

    let err = PduError::StringTooLong("my_field".to_string(), 10);
    assert_eq!(
        format!("{}", err),
        "String too long for field 'my_field' (max: 10)"
    );

    let err = PduError::InvalidLength;
    assert_eq!(format!("{}", err), "Invalid length for field");

    // Test From<std::io::Error>
    let io_err = std::io::Error::new(std::io::ErrorKind::Other, "oh no");
    let pdu_err = PduError::from(io_err);
    if let PduError::Io(e) = pdu_err {
        assert_eq!(format!("{}", e), "oh no");
    } else {
        panic!("Expected PduError::Io");
    }
}
