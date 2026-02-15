use smpp_codec::common::{
    BindMode, Npi, Ton, CMD_BIND_TRANSCEIVER_RESP, CMD_BROADCAST_SM_RESP,
    CMD_CANCEL_BROADCAST_SM_RESP, CMD_CANCEL_SM_RESP, CMD_DATA_SM_RESP, CMD_DELIVER_SM_RESP,
    CMD_QUERY_BROADCAST_SM_RESP, CMD_QUERY_SM_RESP, CMD_REPLACE_SM_RESP, CMD_SUBMIT_MULTI_SM_RESP,
    CMD_SUBMIT_SM_RESP, CMD_UNBIND_RESP, HEADER_LEN,
};
use smpp_codec::pdus::{
    AlertNotification, BindRequest, BroadcastSmRequest, CancelBroadcastSmRequest, CancelSmRequest,
    DataSmRequest, DeliverSmRequest, Destination, QueryBroadcastSmRequest, QuerySmRequest,
    ReplaceSmRequest, SubmitMultiRequest, SubmitSmRequest, UnbindRequest,
};
use smpp_codec::tlv::{tags, Tlv};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to server...");
    let mut stream = TcpStream::connect("127.0.0.1:2775")?;
    println!("Connected!");

    // 1. Bind
    let bind_req = BindRequest::new(
        1,
        BindMode::Transceiver,
        "my_system_id".to_string(),
        "password".to_string(),
    );
    send_pdu(&mut stream, &bind_req)?;
    read_response(&mut stream, CMD_BIND_TRANSCEIVER_RESP)?;

    // 2. SubmitSm with ALL fields and some TLVs
    let mut submit_req = SubmitSmRequest::new(
        2,
        "source_addr".to_string(),
        "dest_addr".to_string(),
        b"Hello SubmitSm".to_vec(),
    );
    submit_req.service_type = "CMT".to_string();
    submit_req.source_addr_ton = Ton::International;
    submit_req.source_addr_npi = Npi::Isdn;
    submit_req.dest_addr_ton = Ton::National;
    submit_req.dest_addr_npi = Npi::Isdn;
    submit_req.esm_class = 0x00; // Default
    submit_req.protocol_id = 0x00;
    submit_req.priority_flag = 1;
    submit_req.schedule_delivery_time = "231201000000000R".to_string(); // Example format
    submit_req.validity_period = "231202000000000R".to_string(); // Example format
    submit_req.registered_delivery = 1;
    submit_req.replace_if_present_flag = 0;
    submit_req.data_coding = 0x00; // Default GSM
    submit_req.sm_default_msg_id = 0;

    // Add TLVs
    submit_req.add_tlv(Tlv::new(tags::USER_MESSAGE_REFERENCE, vec![0x00, 0x01]));
    submit_req.add_tlv(Tlv::new(tags::SAR_MSG_REF_NUM, vec![0x00, 0x01]));
    submit_req.add_tlv(Tlv::new(tags::SAR_TOTAL_SEGMENTS, vec![0x02]));
    submit_req.add_tlv(Tlv::new(tags::SAR_SEGMENT_SEQNUM, vec![0x01]));

    send_pdu(&mut stream, &submit_req)?;
    read_response(&mut stream, CMD_SUBMIT_SM_RESP)?;

    // 3. DeliverSm with ALL fields and TLVs
    let mut deliver_req = DeliverSmRequest::new(
        3,
        "source_addr".to_string(),
        "dest_addr".to_string(),
        b"Hello DeliverSm".to_vec(),
    );
    deliver_req.service_type = "CMT".to_string();
    deliver_req.source_addr_ton = Ton::International;
    deliver_req.source_addr_npi = Npi::Isdn;
    deliver_req.dest_addr_ton = Ton::National;
    deliver_req.dest_addr_npi = Npi::Isdn;
    deliver_req.esm_class = 0x00;
    deliver_req.protocol_id = 0x00;
    deliver_req.priority_flag = 0;
    deliver_req.schedule_delivery_time = "".to_string(); // Often empty in DeliverSm
    deliver_req.validity_period = "".to_string(); // Often empty in DeliverSm
    deliver_req.registered_delivery = 0;
    deliver_req.replace_if_present_flag = 0;
    deliver_req.data_coding = 0x00;
    deliver_req.sm_default_msg_id = 0;

    // Add TLVs
    deliver_req
        .optional_params
        .push(Tlv::new(tags::NETWORK_ERROR_CODE, vec![0x03, 0x00, 0x00])); // Example TLV

    send_pdu(&mut stream, &deliver_req)?;
    read_response(&mut stream, CMD_DELIVER_SM_RESP)?;

    // 4. BroadcastSmRequest with ALL fields
    let area_tlv = Tlv::new(tags::BROADCAST_AREA_IDENTIFIER, vec![0x01, 0x02, 0x03]); // Dummy area
    let mut broadcast_req = BroadcastSmRequest::new(
        4,
        "source_addr".to_string(),
        b"Hello Broadcast".to_vec(),
        area_tlv,
    );
    broadcast_req.service_type = "CMT".to_string();
    broadcast_req.source_addr_ton = Ton::International;
    broadcast_req.source_addr_npi = Npi::Isdn;
    broadcast_req.message_id = "".to_string(); // Empty in request
    broadcast_req.priority_flag = 2;
    broadcast_req.schedule_delivery_time = "".to_string();
    broadcast_req.validity_period = "".to_string();
    broadcast_req.replace_if_present_flag = 0;
    broadcast_req.data_coding = 0x00;
    broadcast_req.sm_default_msg_id = 0;

    // Additional TLVs
    broadcast_req.add_tlv(Tlv::new(tags::BROADCAST_REP_NUM, vec![0x00, 0x01]));

    send_pdu(&mut stream, &broadcast_req)?;
    read_response(&mut stream, CMD_BROADCAST_SM_RESP)?;

    // 5. SubmitMultiRequest
    let dest1 = Destination::SmeAddress {
        ton: Ton::International,
        npi: Npi::Isdn,
        address: "111111".to_string(),
    };
    let dest2 = Destination::DistributionList("MyList".to_string());
    let mut multi_req = SubmitMultiRequest::new(
        5,
        "source_addr".to_string(),
        vec![dest1, dest2],
        b"Hello SubmitMultiRequest".to_vec(),
    );
    multi_req.service_type = "CMT".to_string();
    send_pdu(&mut stream, &multi_req)?;
    read_response(&mut stream, CMD_SUBMIT_MULTI_SM_RESP)?;

    // 6. QuerySm
    let query_req = QuerySmRequest::new(6, "MsgID_12345".to_string(), "source_addr".to_string());
    send_pdu(&mut stream, &query_req)?;
    read_response(&mut stream, CMD_QUERY_SM_RESP)?;

    // 7. CancelSm
    let cancel_req = CancelSmRequest::new(
        7,
        "MsgID_12345".to_string(),
        "source_addr".to_string(),
        "dest_addr".to_string(),
    );
    send_pdu(&mut stream, &cancel_req)?;
    read_response(&mut stream, CMD_CANCEL_SM_RESP)?;

    // 8. ReplaceSmRequest
    let replace_req = ReplaceSmRequest::new(
        8,
        "MsgID_12345".to_string(),
        "source_addr".to_string(),
        b"New Content".to_vec(),
    );
    send_pdu(&mut stream, &replace_req)?;
    read_response(&mut stream, CMD_REPLACE_SM_RESP)?;

    // 9. DataSmRequest
    let data_req = DataSmRequest::new(
        9,
        "source_addr".to_string(),
        "dest_addr".to_string(),
        b"Hello DataSmRequest".to_vec(),
    );
    send_pdu(&mut stream, &data_req)?;
    read_response(&mut stream, CMD_DATA_SM_RESP)?;

    // 10. AlertNotification
    let alert_req = AlertNotification::new(10, "source_addr".to_string(), "esme_addr".to_string());
    send_pdu(&mut stream, &alert_req)?;
    // No response expected

    // 11. QueryBroadcastSmRequest
    let query_bc_req =
        QueryBroadcastSmRequest::new(11, "BcastID_999".to_string(), "source_addr".to_string());
    send_pdu(&mut stream, &query_bc_req)?;
    read_response(&mut stream, CMD_QUERY_BROADCAST_SM_RESP)?;

    // 12. CancelBroadcastSmRequest
    let cancel_bc_req = CancelBroadcastSmRequest::new(
        12,
        "CMT".to_string(),
        "BcastID_999".to_string(),
        "source_addr".to_string(),
    );
    send_pdu(&mut stream, &cancel_bc_req)?;
    read_response(&mut stream, CMD_CANCEL_BROADCAST_SM_RESP)?;

    // 13. Unbind
    let unbind_req = UnbindRequest::new(13);
    send_pdu(&mut stream, &unbind_req)?;
    read_response(&mut stream, CMD_UNBIND_RESP)?;

    println!("Done!");
    Ok(())
}

fn send_pdu(stream: &mut TcpStream, pdu: &impl Encode) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    pdu.encode(&mut buffer)?;
    stream.write_all(&buffer)?;
    println!("Sent PDU");
    Ok(())
}

fn read_response(
    stream: &mut TcpStream,
    expected_id: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut header_buf = [0u8; HEADER_LEN];
    stream.read_exact(&mut header_buf)?;

    let command_len =
        u32::from_be_bytes([header_buf[0], header_buf[1], header_buf[2], header_buf[3]]);
    let command_id =
        u32::from_be_bytes([header_buf[4], header_buf[5], header_buf[6], header_buf[7]]);
    let command_status =
        u32::from_be_bytes([header_buf[8], header_buf[9], header_buf[10], header_buf[11]]);
    let sequence_number = u32::from_be_bytes([
        header_buf[12],
        header_buf[13],
        header_buf[14],
        header_buf[15],
    ]);

    println!(
        "Response: Len={}, ID=0x{:08X}, Status={}, Seq={}",
        command_len, command_id, command_status, sequence_number
    );

    if command_id != expected_id {
        return Err(format!(
            "Expected command ID 0x{:08X}, got 0x{:08X}",
            expected_id, command_id
        )
        .into());
    }

    // Read body
    let body_len = command_len as usize - HEADER_LEN;
    if body_len > 0 {
        let mut body_buf = vec![0u8; body_len];
        stream.read_exact(&mut body_buf)?;
    }

    Ok(())
}

trait Encode {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError>;
}

// Implement Encode for all PDU types used
impl Encode for BindRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for SubmitSmRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for DeliverSmRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for BroadcastSmRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for UnbindRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for SubmitMultiRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for QuerySmRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for CancelSmRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for ReplaceSmRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for DataSmRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for AlertNotification {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for QueryBroadcastSmRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
impl Encode for CancelBroadcastSmRequest {
    fn encode(&self, writer: &mut impl Write) -> Result<(), smpp_codec::common::PduError> {
        self.encode(writer)
    }
}
