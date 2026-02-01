use smpp_codec::common::CMD_BIND_TRANSCEIVER_RESP;
use smpp_codec::pdus::{
    BindResponse, CancelSmResponse, DeliverSmResponse, EnquireLinkResponse, QuerySmResponse,
    SubmitSmResponse, UnbindResponse,
};

#[test]
fn test_query_sm_response_error() {
    let resp = QuerySmResponse::new(
        3,
        "ESME_RQUERYFAIL", // 0x00000067
        "".to_string(),
        "".to_string(),
        0,
        0,
    );

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encoding failed");

    assert_eq!(buffer.len(), 16); // Header only for error

    let decoded = QuerySmResponse::decode(&buffer).expect("Decoding failed");
    assert_eq!(decoded.sequence_number, 3);
    assert_eq!(decoded.command_status, 0x00000067);
    assert_eq!(decoded.status_description, "ESME_RQUERYFAIL");
    assert_eq!(decoded.message_id, "");
}

#[test]
fn test_cancel_sm_response_error() {
    let resp = CancelSmResponse::new(2, "ESME_RCANCELFAIL"); // 0x00000011

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encoding failed");

    assert_eq!(buffer.len(), 16);

    let decoded = CancelSmResponse::decode(&buffer).expect("Decoding failed");
    assert_eq!(decoded.sequence_number, 2);
    assert_eq!(decoded.command_status, 0x00000011);
    assert_eq!(decoded.status_description, "ESME_RCANCELFAIL");
}

#[test]
fn test_unbind_response_error() {
    let resp = UnbindResponse::new(4, "ESME_RINVSYSID"); // 0x0000000F

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encoding failed");

    assert_eq!(buffer.len(), 16);

    let decoded = UnbindResponse::decode(&buffer).expect("Decoding failed");
    assert_eq!(decoded.sequence_number, 4);
    assert_eq!(decoded.command_status, 0x0000000F);
    assert_eq!(decoded.status_description, "ESME_RINVSYSID");
}

#[test]
fn test_enquire_link_response_error() {
    let resp = EnquireLinkResponse::new(5, "ESME_RSYSERR"); // 0x00000008

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encoding failed");

    assert_eq!(buffer.len(), 16);

    let decoded = EnquireLinkResponse::decode(&buffer).expect("Decoding failed");
    assert_eq!(decoded.sequence_number, 5);
    assert_eq!(decoded.command_status, 0x00000008);
    assert_eq!(decoded.status_description, "ESME_RSYSERR");
}

#[test]
fn test_submit_sm_response_error() {
    // Note: SubmitSmResponse::new can be used here instead of struct init if available,
    // but using struct init to match what was in the unit test,
    // Wait, struct init is NOT allowed if fields are private!
    // I should use `new` if available or check visibility.
    // `SubmitSmResponse` fields are public.
    let resp = SubmitSmResponse::new(2, "ESME_RSYSERR", "".to_string());

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encoding failed");

    assert_eq!(buffer.len(), 16);

    let decoded = SubmitSmResponse::decode(&buffer).expect("Decoding failed");
    assert_eq!(decoded.sequence_number, 2);
    assert_eq!(decoded.command_status, 0x00000008);
    assert_eq!(decoded.message_id, "");
}

#[test]
fn test_bind_response_error() {
    let resp = BindResponse::new(
        3,
        CMD_BIND_TRANSCEIVER_RESP,
        "ESME_RBINDFAIL",
        "".to_string(),
    );

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encoding failed");

    // Header only on error
    assert_eq!(buffer.len(), 16);

    let decoded = BindResponse::decode(&buffer).expect("Decoding failed");
    assert_eq!(decoded.sequence_number, 3);
    assert_eq!(decoded.command_status, 0x0000000D);
    assert_eq!(decoded.system_id, "");
}

#[test]
fn test_deliver_sm_response_error() {
    let resp = DeliverSmResponse::new(4, "ESME_RSYSERR");

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encoding failed");

    assert_eq!(buffer.len(), 16);

    let decoded = DeliverSmResponse::decode(&buffer).expect("Decoding failed");
    assert_eq!(decoded.sequence_number, 4);
    assert_eq!(decoded.command_status, 0x00000008);
}
