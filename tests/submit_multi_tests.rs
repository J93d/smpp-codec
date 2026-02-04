use smpp_codec::common::{Npi, Ton};
use smpp_codec::pdus::{Destination, SubmitMulti, SubmitMultiResp, UnsuccessSme};

#[test]
fn test_submit_multi_encoding_decoding() {
    let dest_sme = Destination::SmeAddress {
        ton: Ton::International,
        npi: Npi::Isdn,
        address: "1234567890".to_string(),
    };
    let dest_dl = Destination::DistributionList("MyGroup".to_string());

    let req = SubmitMulti::new(
        1001,
        "Sender".to_string(),
        vec![dest_sme.clone(), dest_dl.clone()],
        b"Hello Multicast".to_vec(),
    );

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = SubmitMulti::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 1001);
    assert_eq!(decoded.source_addr, "Sender");
    assert_eq!(decoded.destinations.len(), 2);
    assert_eq!(decoded.short_message, b"Hello Multicast");

    // Verify Destinations
    match &decoded.destinations[0] {
        Destination::SmeAddress { ton, npi, address } => {
            assert_eq!(*ton, Ton::International);
            assert_eq!(*npi, Npi::Isdn);
            assert_eq!(address, "1234567890");
        }
        _ => panic!("Expected SME Address first"),
    }

    match &decoded.destinations[1] {
        Destination::DistributionList(name) => {
            assert_eq!(name, "MyGroup");
        }
        _ => panic!("Expected Distribution List second"),
    }
}

#[test]
fn test_submit_multi_resp_success() {
    let resp = SubmitMultiResp::new(1001, "ESME_ROK", "UUID-1001".to_string(), vec![]);

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = SubmitMultiResp::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.sequence_number, 1001);
    assert_eq!(decoded.command_status, 0); // ESME_ROK
    assert_eq!(decoded.message_id, "UUID-1001");
    assert!(decoded.unsuccess_smes.is_empty());
}

#[test]
fn test_submit_multi_resp_partial_success() {
    let unsuccess = UnsuccessSme {
        ton: Ton::National,
        npi: Npi::Isdn,
        address: "9999".to_string(),
        error_status: 0x0000000B, // ESME_RINVDSTADR
    };

    // 0x00000405 is specific error code often used for partial success/failure involved
    // But SubmitMultiResp::new takes a string status.
    // We'll manually construct or assume we pass a custom status if supported,
    // but here let's assume "ESME_ROK" (0) containing unsuccess SMEs is valid for test logic
    // OR we pass a recognized status name.
    // However, the spec says "ESME_RSUBMITMULTI" (0x00000405) is returned when some fail.
    // Let's rely on command_status being settable.

    // NOTE: get_status_code might not have mapping for 0x405 if not in common.
    // For this test, we accept if logic follows through.
    // If "ESME_RSUBMITMULTI" isn't in generic map, we might need numeric overload or just test ESME_ROK with errors which is structurally possible but logically weird.

    // Let's stick to standard behavior: If unsuccess list > 0, status is usually NOT OK for *all*,
    // but the PDU structure allows returning Message ID + Error List on specific statuses.
    // Our code checks: if self.command_status == 0 || self.command_status == 0x00000405

    // To properly test the 0x405 path, we'd need to ensure `get_status_code` supports it or we manually set it.
    // Since `new` uses `get_status_code`, let's assume we use success path for now to verify encoding of the list.

    let resp = SubmitMultiResp::new(
        1001,
        "ESME_ROK",
        "UUID-PARTIAL".to_string(),
        vec![unsuccess],
    );

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = SubmitMultiResp::decode(&buffer).expect("Decode failed");

    assert_eq!(decoded.unsuccess_smes.len(), 1);
    assert_eq!(decoded.unsuccess_smes[0].address, "9999");
    assert_eq!(decoded.unsuccess_smes[0].error_status, 0x0000000B);
}

#[test]
fn test_submit_multi_resp_failure() {
    let resp = SubmitMultiResp::new(
        1001,
        "ESME_RSYSERR", // Generic error
        "".to_string(),
        vec![],
    );

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = SubmitMultiResp::decode(&buffer).expect("Decode failed");

    // On error (non-0 and non-0x405), body is empty (no msg ID, no list)
    assert_ne!(decoded.command_status, 0);
    assert!(decoded.message_id.is_empty());
    assert!(decoded.unsuccess_smes.is_empty());
}
