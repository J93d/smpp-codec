use smpp_codec::common::{BindMode, Npi, Ton};
use smpp_codec::pdus::{
    BindRequest, BindResponse, BroadcastSmRequest, BroadcastSmResponse, CancelBroadcastSmRequest,
    CancelBroadcastSmResponse, DataSmRequest, DataSmResponse, DeliverSmRequest, DeliverSmResponse,
    EnquireLinkRequest, EnquireLinkResponse, QueryBroadcastSmRequest, QueryBroadcastSmResponse,
    ReplaceSmRequest, ReplaceSmResponse, SubmitMultiRequest, SubmitMultiResponse, SubmitSmRequest,
    SubmitSmResponse, UnbindRequest, UnbindResponse,
};
use smpp_codec::tlv::Tlv;

macro_rules! verify_roundtrip {
    ($pdu:expr, $type:ty) => {
        let mut buffer = Vec::new();
        $pdu.encode(&mut buffer).expect("Encoding failed");

        let decoded = <$type>::decode(&buffer).expect("Decoding failed");

        // Custom assertion or just PartialEq check
        assert_eq!($pdu, decoded, "Round-trip failed for {}", stringify!($type));
    };
}

#[test]
fn test_bind_transceiver_roundtrip() {
    let pdu = BindRequest::new(1, BindMode::Transceiver, "sys_id".into(), "pwd".into());
    verify_roundtrip!(pdu, BindRequest);
}

#[test]
fn test_bind_response_roundtrip() {
    let pdu = BindResponse::new(1, 0x80000009, "ESME_ROK", "sys_id".into());
    verify_roundtrip!(pdu, BindResponse);
}

#[test]
fn test_submit_sm_roundtrip() {
    let pdu = SubmitSmRequest::new(1, "src".into(), "dst".into(), b"hello world".to_vec());
    verify_roundtrip!(pdu, SubmitSmRequest);
}

#[test]
fn test_submit_sm_resp_roundtrip() {
    let pdu = SubmitSmResponse::new(1, "ESME_ROK", "msg_id".into());
    verify_roundtrip!(pdu, SubmitSmResponse);
}

#[test]
fn test_deliver_sm_roundtrip() {
    let pdu = DeliverSmRequest::new(1, "src".into(), "dst".into(), b"delivery receipt".to_vec());
    verify_roundtrip!(pdu, DeliverSmRequest);
}

#[test]
fn test_deliver_sm_resp_roundtrip() {
    let pdu = DeliverSmResponse::new(1, "ESME_ROK");
    verify_roundtrip!(pdu, DeliverSmResponse);
}

#[test]
fn test_enquire_link_roundtrip() {
    let pdu = EnquireLinkRequest::new(1);
    verify_roundtrip!(pdu, EnquireLinkRequest);
}

#[test]
fn test_enquire_link_resp_roundtrip() {
    let pdu = EnquireLinkResponse::new(1, "ESME_ROK");
    verify_roundtrip!(pdu, EnquireLinkResponse);
}

#[test]
fn test_unbind_roundtrip() {
    let pdu = UnbindRequest::new(1);
    verify_roundtrip!(pdu, UnbindRequest);
}

#[test]
fn test_unbind_resp_roundtrip() {
    let pdu = UnbindResponse::new(1, "ESME_ROK");
    verify_roundtrip!(pdu, UnbindResponse);
}

#[test]
fn test_submit_multi_roundtrip() {
    use smpp_codec::pdus::Destination;

    let dests = vec![
        Destination::SmeAddress {
            ton: Ton::International,
            npi: Npi::Isdn,
            address: "123456".into(),
        },
        Destination::DistributionList("MyList".into()),
    ];
    let pdu = SubmitMultiRequest::new(1, "src".into(), dests, b"multi msg".to_vec());
    verify_roundtrip!(pdu, SubmitMultiRequest);
}

#[test]
fn test_submit_multi_resp_roundtrip() {
    let pdu = SubmitMultiResponse::new(1, "ESME_ROK", "msg_id".into(), vec![]);
    verify_roundtrip!(pdu, SubmitMultiResponse);
}

#[test]
fn test_broadcast_sm_roundtrip() {
    let area_tlv = Tlv::new_u8(0x0606, 1); // Mock Area ID
    let pdu = BroadcastSmRequest::new(1, "src".into(), b"payload".to_vec(), area_tlv);
    verify_roundtrip!(pdu, BroadcastSmRequest);
}

#[test]
fn test_broadcast_sm_resp_roundtrip() {
    let pdu = BroadcastSmResponse::new(1, "ESME_ROK", "msg_id".into());
    verify_roundtrip!(pdu, BroadcastSmResponse);
}

#[test]
fn test_data_sm_roundtrip() {
    let pdu = DataSmRequest::new(1, "src".into(), "dst".into(), vec![]);
    verify_roundtrip!(pdu, DataSmRequest);
}

#[test]
fn test_data_sm_resp_roundtrip() {
    let pdu = DataSmResponse::new(1, "ESME_ROK", "msg_id".into());
    verify_roundtrip!(pdu, DataSmResponse);
}

#[test]
fn test_replace_sm_roundtrip() {
    let pdu = ReplaceSmRequest::new(1, "msg_id_old".into(), "src".into(), b"new msg".to_vec());
    verify_roundtrip!(pdu, ReplaceSmRequest);
}

#[test]
fn test_replace_sm_resp_roundtrip() {
    let pdu = ReplaceSmResponse::new(1, "ESME_ROK");
    verify_roundtrip!(pdu, ReplaceSmResponse);
}

#[test]
fn test_query_broadcast_sm_roundtrip() {
    let pdu = QueryBroadcastSmRequest::new(1, "msg_id".into(), "src".into());
    verify_roundtrip!(pdu, QueryBroadcastSmRequest);
}

#[test]
fn test_query_broadcast_sm_resp_roundtrip() {
    let pdu = QueryBroadcastSmResponse::new(1, "ESME_ROK", "msg_id".into());
    verify_roundtrip!(pdu, QueryBroadcastSmResponse);
}

#[test]
fn test_cancel_broadcast_sm_roundtrip() {
    let pdu = CancelBroadcastSmRequest::new(1, "CMT".into(), "msg_id".into(), "src".into());
    verify_roundtrip!(pdu, CancelBroadcastSmRequest);
}

#[test]
fn test_cancel_broadcast_sm_resp_roundtrip() {
    let pdu = CancelBroadcastSmResponse::new(1, "ESME_ROK");
    verify_roundtrip!(pdu, CancelBroadcastSmResponse);
}

use smpp_codec::pdus::{CancelSmRequest, CancelSmResponse, QuerySmRequest, QuerySmResponse};

#[test]
fn test_cancel_sm_roundtrip() {
    let pdu = CancelSmRequest::new(1, "msg_id".into(), "src".into(), "dst".into());
    verify_roundtrip!(pdu, CancelSmRequest);
}

#[test]
fn test_cancel_sm_resp_roundtrip() {
    let pdu = CancelSmResponse::new(1, "ESME_ROK");
    verify_roundtrip!(pdu, CancelSmResponse);
}

#[test]
fn test_query_sm_roundtrip() {
    let pdu = QuerySmRequest::new(1, "msg_id".into(), "src".into());
    verify_roundtrip!(pdu, QuerySmRequest);
}

#[test]
fn test_query_sm_resp_roundtrip() {
    let pdu = QuerySmResponse::new(1, "ESME_ROK", "msg_id".into(), "date".into(), 1, 0);
    verify_roundtrip!(pdu, QuerySmResponse);
}
