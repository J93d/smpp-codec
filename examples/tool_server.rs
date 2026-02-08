use smpp_codec::common::{
    get_status_description, BindMode, PduError, CMD_ALERT_NOTIFICATION, CMD_BIND_RECEIVER,
    CMD_BIND_TRANSCEIVER, CMD_BIND_TRANSMITTER, CMD_BROADCAST_SM, CMD_CANCEL_BROADCAST_SM,
    CMD_CANCEL_SM, CMD_DATA_SM, CMD_DELIVER_SM, CMD_ENQUIRE_LINK, CMD_QUERY_BROADCAST_SM,
    CMD_QUERY_SM, CMD_REPLACE_SM, CMD_SUBMIT_MULTI_SM, CMD_SUBMIT_SM, CMD_UNBIND, HEADER_LEN,
};
use smpp_codec::pdus::{
    AlertNotification, BindRequest, BindResponse, BroadcastSm, BroadcastSmResp, CancelBroadcastSm,
    CancelBroadcastSmResp, CancelSmRequest, CancelSmResponse, DataSm, DataSmResp, DeliverSmRequest,
    DeliverSmResponse, EnquireLinkRequest, EnquireLinkResponse, QueryBroadcastSm,
    QueryBroadcastSmResp, QuerySmRequest, QuerySmResponse, ReplaceSm, ReplaceSmResp, SubmitMulti,
    SubmitMultiResp, SubmitSmRequest, SubmitSmResponse, UnbindRequest, UnbindResponse,
};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2775")?;
    println!("Server listening on 127.0.0.1:2775");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {:?}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error connection: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut header_buf = [0u8; HEADER_LEN];
        if let Err(e) = stream.read_exact(&mut header_buf) {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                println!("Client disconnected");
                return Ok(());
            }
            return Err(Box::new(e));
        }

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
            "Header: Len={}, ID=0x{:08X}, Status={}, Seq={}",
            command_len, command_id, command_status, sequence_number
        );

        if command_len < HEADER_LEN as u32 {
            eprintln!("Invalid command length: {}", command_len);
            return Err(Box::new(PduError::InvalidLength));
        }

        let body_len = command_len as usize - HEADER_LEN;
        let mut body_buf = vec![0u8; body_len];
        stream.read_exact(&mut body_buf)?;

        // Reconstruct full buffer for decoding
        let mut full_pdu = Vec::with_capacity(command_len as usize);
        full_pdu.extend_from_slice(&header_buf);
        full_pdu.extend_from_slice(&body_buf);

        match command_id {
            CMD_BIND_RECEIVER | CMD_BIND_TRANSMITTER | CMD_BIND_TRANSCEIVER => {
                let req = BindRequest::decode(&full_pdu)?;
                println!("Received Bind: {:#?}", req);
                let system_id = req.system_id;

                let resp = BindResponse::new(
                    sequence_number,
                    command_id | 0x80000000,
                    "ESME_ROK",
                    system_id,
                );
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent Bind Response");
            }
            CMD_SUBMIT_SM => {
                let req = SubmitSmRequest::decode(&full_pdu)?;
                println!("Received SubmitSm: {:#?}", req);

                let resp =
                    SubmitSmResponse::new(sequence_number, "ESME_ROK", "MsgID_12345".to_string());
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent SubmitSm Response");
            }
            CMD_DELIVER_SM => {
                let req = DeliverSmRequest::decode(&full_pdu)?;
                println!("Received DeliverSm: {:#?}", req);

                let mut resp = DeliverSmResponse::new(sequence_number, "ESME_ROK");
                resp.message_id = "MsgID_DLR".to_string(); // Normally DLR response message_id is empty or specific, but here we just put something
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent DeliverSm Response");
            }
            CMD_BROADCAST_SM => {
                let req = BroadcastSm::decode(&full_pdu)?;
                println!("Received BroadcastSm: {:#?}", req);

                let resp =
                    BroadcastSmResp::new(sequence_number, "ESME_ROK", "BcastID_999".to_string());
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent BroadcastSm Response");
            }
            CMD_ENQUIRE_LINK => {
                let _req = EnquireLinkRequest::decode(&full_pdu)?;
                println!("Received EnquireLink");
                let resp = EnquireLinkResponse::new(sequence_number, "ESME_ROK");
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent EnquireLink Response");
            }
            CMD_SUBMIT_MULTI_SM => {
                let req = SubmitMulti::decode(&full_pdu)?;
                println!("Received SubmitMulti: {:#?}", req);
                let resp = SubmitMultiResp::new(
                    sequence_number,
                    "ESME_ROK",
                    "MsgID_Multi".to_string(),
                    vec![],
                );
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent SubmitMulti Response");
            }
            CMD_QUERY_SM => {
                let req = QuerySmRequest::decode(&full_pdu)?;
                println!("Received QuerySm: {:#?}", req);
                let resp = QuerySmResponse::new(
                    sequence_number,
                    "ESME_ROK",
                    req.message_id,
                    "220101000000000R".to_string(),
                    2, // Delivered
                    0,
                );
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent QuerySm Response");
            }
            CMD_CANCEL_SM => {
                let req = CancelSmRequest::decode(&full_pdu)?;
                println!("Received CancelSm: {:#?}", req);
                let resp = CancelSmResponse::new(sequence_number, "ESME_ROK");
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent CancelSm Response");
            }
            CMD_REPLACE_SM => {
                let req = ReplaceSm::decode(&full_pdu)?;
                println!("Received ReplaceSm: {:#?}", req);
                let resp = ReplaceSmResp::new(sequence_number, "ESME_ROK");
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent ReplaceSm Response");
            }
            CMD_DATA_SM => {
                let req = DataSm::decode(&full_pdu)?;
                println!("Received DataSm: {:#?}", req);
                let resp = DataSmResp::new(sequence_number, "ESME_ROK", "MsgID_Data".to_string());
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent DataSm Response");
            }
            CMD_ALERT_NOTIFICATION => {
                let req = AlertNotification::decode(&full_pdu)?;
                println!("Received AlertNotification: {:#?}", req);
                // No Response for AlertNotification
            }
            CMD_QUERY_BROADCAST_SM => {
                let req = QueryBroadcastSm::decode(&full_pdu)?;
                println!("Received QueryBroadcastSm: {:#?}", req);
                let resp = QueryBroadcastSmResp::new(sequence_number, "ESME_ROK", req.message_id);
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent QueryBroadcastSm Response");
            }
            CMD_CANCEL_BROADCAST_SM => {
                let req = CancelBroadcastSm::decode(&full_pdu)?;
                println!("Received CancelBroadcastSm: {:#?}", req);
                let resp = CancelBroadcastSmResp::new(sequence_number, "ESME_ROK");
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent CancelBroadcastSm Response");
            }
            CMD_UNBIND => {
                let _req = UnbindRequest::decode(&full_pdu)?;
                println!("Received Unbind");
                let resp = UnbindResponse::new(sequence_number, "ESME_ROK");
                let mut resp_buf = Vec::new();
                resp.encode(&mut resp_buf)?;
                stream.write_all(&resp_buf)?;
                println!("Sent Unbind Response");
                return Ok(()); // Close connection after unbind
            }
            _ => {
                println!("Unknown Command ID: 0x{:08X}", command_id);
            }
        }
    }
}
