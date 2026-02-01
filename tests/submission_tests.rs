use smpp_codec::pdus::{
    EncodingType, MessageSplitter, SplitMode, SubmitSmRequest, SubmitSmResponse,
};

#[test]
fn test_submit_sm_encoding_decoding() {
    let sequence_number = 12345;
    let mut req = SubmitSmRequest::new(
        sequence_number,
        "src".to_string(),
        "dst".to_string(),
        b"Hello".to_vec(),
    );

    // Set some fields
    req.service_type = "CMT".to_string();
    req.registered_delivery = 1;

    let mut buffer = Vec::new();
    req.encode(&mut buffer).expect("Encode failed");

    let decoded = SubmitSmRequest::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, sequence_number);
    assert_eq!(decoded.service_type, "CMT");
    assert_eq!(decoded.short_message, b"Hello");
}

#[test]
fn test_submit_sm_resp_encoding_decoding() {
    let resp = SubmitSmResponse::new(55, "ESME_ROK", "UUID-1234".to_string());

    let mut buffer = Vec::new();
    resp.encode(&mut buffer).expect("Encode failed");

    let decoded = SubmitSmResponse::decode(&buffer).expect("Decode failed");
    assert_eq!(decoded.sequence_number, 55);
    assert_eq!(decoded.message_id, "UUID-1234");
}

#[test]
fn test_splitter_udh() {
    // Generate a long string > 160 chars
    let text = "A".repeat(200);

    let (parts, _) = MessageSplitter::split(text.clone(), EncodingType::Gsm7Bit, SplitMode::Udh)
        .expect("Split failed");

    assert_eq!(parts.len(), 2);

    // Verify first part has UDH
    let pdu1_body = &parts[0];
    // UDH header is typically 6 bytes (len(1) + ie(5)) = 05 00 03 AA TT 01
    assert!(pdu1_body.len() > 6);
    assert_eq!(pdu1_body[0], 0x05); // UDH Len
}

#[test]
fn test_splitter_sar() {
    let text = "B".repeat(300);

    let (parts, _) = MessageSplitter::split(text.clone(), EncodingType::Gsm7Bit, SplitMode::Sar)
        .expect("Split failed");

    assert_eq!(parts.len(), 2);

    // Verify chunks are within limits
    let pdu2_body = &parts[1];
    assert!(pdu2_body.len() <= 254);
    // Note: SAR TLVs are added by the caller, so we can't test them here.
}

#[test]
fn test_submit_sm_validation_errors() {
    // Service type too long
    let mut req = SubmitSmRequest::new(1, "src".into(), "dst".into(), vec![]);
    req.service_type = "1234567".to_string(); // 7 chars (max 6)
    let mut buf = Vec::new();
    assert!(req.encode(&mut buf).is_err());

    // Source addr too long
    let req = SubmitSmRequest::new(1, "A".repeat(22), "dst".into(), vec![]);
    assert!(req.encode(&mut buf).is_err());

    // Dest addr too long
    let req = SubmitSmRequest::new(1, "src".into(), "B".repeat(22), vec![]);
    assert!(req.encode(&mut buf).is_err());

    // Schedule time too long
    let mut req = SubmitSmRequest::new(1, "src".into(), "dst".into(), vec![]);
    req.schedule_delivery_time = "A".repeat(18);
    assert!(req.encode(&mut buf).is_err());

    // Validity period too long
    let mut req = SubmitSmRequest::new(1, "src".into(), "dst".into(), vec![]);
    req.validity_period = "A".repeat(18);
    assert!(req.encode(&mut buf).is_err());

    // Message too long (should use payload TLV instead, but encode just errors for now)
    let mut req = SubmitSmRequest::new(1, "src".into(), "dst".into(), vec![0; 255]);
    assert!(req.encode(&mut buf).is_err());
}

#[test]
fn test_submit_sm_tlvs_and_segmentation_info() {
    // Test that we can extract segmentation info when using TLVs
    let mut req = SubmitSmRequest::new(1, "src".into(), "dst".into(), b"Part2".to_vec());

    // Manually add SAR TLVs
    req.add_tlv(smpp_codec::tlv::Tlv::new_u16(
        smpp_codec::tlv::tags::SAR_MSG_REF_NUM,
        0x1234,
    ));
    req.add_tlv(smpp_codec::tlv::Tlv::new_u16(
        smpp_codec::tlv::tags::SAR_TOTAL_SEGMENTS,
        2,
    ));
    req.add_tlv(smpp_codec::tlv::Tlv::new_u16(
        smpp_codec::tlv::tags::SAR_SEGMENT_SEQNUM,
        2,
    ));

    let info = req
        .get_segmentation_info()
        .expect("Should find segmentation info");
    assert_eq!(info.ref_num, 0x1234);
    assert_eq!(info.total_segments, 2);
    assert_eq!(info.seq_num, 2);
}

#[test]
fn test_submit_sm_segmentation_info_udh_16bit_ref() {
    // Test 16-bit reference number UDH (IE ID 0x08)
    // UDH: Len(6) 08 04 Ref1 Ref2 Tot Seq
    let mut udh = vec![0x06, 0x08, 0x04, 0xAB, 0xCD, 0x02, 0x02];
    udh.extend_from_slice(b"Content");

    let mut req = SubmitSmRequest::new(1, "src".into(), "dst".into(), udh);
    req.esm_class = 0x40; // UDHI Set

    let info = req
        .get_segmentation_info()
        .expect("Should find 16-bit UDH info");
    assert_eq!(info.ref_num, 0xABCD);
    assert_eq!(info.total_segments, 2);
    assert_eq!(info.seq_num, 2);
}
