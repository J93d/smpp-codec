use smpp_codec::common::{BindMode, Ton, Npi};
use smpp_codec::pdus::{BindRequest, BindResponse, OutbindRequest};

#[test]
fn test_bind_transceiver_encode_decode() {
    let bind_req = BindRequest::new(
        12345,
        BindMode::Transceiver,
        "my_system_id".to_string(),
        "secret".to_string(),
    ).with_address_range(Ton::International, Npi::Isdn, "12345".to_string());

    let mut buffer = Vec::new();
    bind_req.encode(&mut buffer).expect("Failed to encode");

    let decoded = BindRequest::decode(&buffer).expect("Failed to decode");

    assert_eq!(decoded.sequence_number, 12345);
    assert_eq!(decoded.system_id, "my_system_id");
    assert_eq!(decoded.password, "secret");
    assert_eq!(decoded.mode, BindMode::Transceiver);
    assert_eq!(decoded.addr_ton, Ton::International);
    assert_eq!(decoded.addr_npi, Npi::Isdn);
    assert_eq!(decoded.address_range, "12345");
}

#[test]
fn test_bind_receiver_defaults() {
    let bind_req = BindRequest::new(
        111, //Sequence Number
        BindMode::Receiver,
        "guest".to_string(),
        "guest".to_string(),
    );

    let mut buffer = Vec::new();
    bind_req.encode(&mut buffer).expect("Failed to encode");

    let decoded = BindRequest::decode(&buffer).expect("Failed to decode");

    assert_eq!(decoded.sequence_number, 111);
    assert_eq!(decoded.mode, BindMode::Receiver);
    // Verify defaults
    assert_eq!(decoded.addr_ton, Ton::Unknown);
    assert_eq!(decoded.addr_npi, Npi::Unknown);
    assert_eq!(decoded.address_range, "");
}

#[test]
fn test_bind_response_success() {
    let bind_resp = BindResponse::new(
        67890,
        0x80000009, // bind_transceiver_resp
        "ESME_ROK",
        "SMSC_ID".to_string(),
    );

    let mut buffer = Vec::new();
    bind_resp.encode(&mut buffer).expect("Failed to encode");

    let decoded = BindResponse::decode(&buffer).expect("Failed to decode");

    assert_eq!(decoded.sequence_number, 67890);
    assert_eq!(decoded.command_status, 0);
    assert_eq!(decoded.system_id, "SMSC_ID");
}

#[test]
fn test_bind_response_failure() {
    let bind_resp = BindResponse::new(
        11111,
        0x80000009,
        "ESME_RINVPASWD",
        String::new(), // No system_id on error
    );

    let mut buffer = Vec::new();
    bind_resp.encode(&mut buffer).expect("Failed to encode");

    let decoded = BindResponse::decode(&buffer).expect("Failed to decode");

    assert_eq!(decoded.sequence_number, 11111);
    assert_ne!(decoded.command_status, 0);
    // On error, system_id might be empty string or null, depends on implementation but here we passed empty.
    // Our decode logic for error status prevents reading system_id from body if status != 0, so it defaults to empty.
    assert_eq!(decoded.system_id, "");
}

#[test]
fn test_outbind_request() {
    let outbind = OutbindRequest::new(
        55555,
        "sys_id".to_string(),
        "password".to_string(),
    );

    let mut buffer = Vec::new();
    outbind.encode(&mut buffer).expect("Failed to encode");

    let decoded = OutbindRequest::decode(&buffer).expect("Failed to decode");

    assert_eq!(decoded.sequence_number, 55555);
    assert_eq!(decoded.system_id, "sys_id");
    assert_eq!(decoded.password, "password");
}
