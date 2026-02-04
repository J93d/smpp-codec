use smpp_codec::common::*;
use smpp_codec::pdus::*;
use std::time::Instant;

macro_rules! bench_all {
    ($type:ty, $name:expr, $setup:expr, $iters:expr) => {{
        let req: $type = $setup;

        // Measure Encoding
        let mut buffer = Vec::with_capacity(2048);
        let start_encode = Instant::now();
        for _ in 0..$iters {
            buffer.clear();
            req.encode(&mut buffer).unwrap();
        }
        let duration_encode = start_encode.elapsed();
        let ops_encode = $iters as f64 / duration_encode.as_secs_f64();

        // Measure Decoding
        let start_decode = Instant::now();
        for _ in 0..$iters {
            let _ = <$type>::decode(&buffer).unwrap();
        }
        let duration_decode = start_decode.elapsed();
        let ops_decode = $iters as f64 / duration_decode.as_secs_f64();

        println!(
            "| {:<25} | {:<13.2?} | {:<15.2} | {:<13.2?} | {:<15.2} |",
            $name, duration_encode, ops_encode, duration_decode, ops_decode
        );
    }};
}

#[test]
fn benchmark_suite() {
    let iterations = 100_000;

    println!(
        "\n=== SMPP PDU Performance Benchmarks ({} iterations) ===",
        iterations
    );
    println!("Running on: {}", std::env::consts::OS);
    println!();
    println!(
        "| {:<25} | {:<13} | {:<15} | {:<13} | {:<15} |",
        "Request Name", "Encoding Time", "Enc Rate (op/s)", "Decoding Time", "Dec Rate (op/s)"
    );
    println!(
        "|{:-<27}|{:-<15}|{:-<17}|{:-<15}|{:-<17}|",
        "", "", "", "", ""
    );

    // --- Session PDUs ---

    bench_all!(
        BindRequest,
        "BindRequest",
        BindRequest::new(
            1,
            BindMode::Transceiver,
            "system_id".into(),
            "password".into()
        )
        .with_address_range(Ton::International, Npi::Isdn, "12345".into()),
        iterations
    );

    bench_all!(
        BindResponse,
        "BindResponse",
        BindResponse::new(2, CMD_BIND_TRANSCEIVER_RESP, "ESME_ROK", "sys_id".into()),
        iterations
    );

    bench_all!(
        OutbindRequest,
        "OutbindRequest",
        OutbindRequest::new(3, "sys_id".into(), "pwd".into()),
        iterations
    );

    bench_all!(
        UnbindRequest,
        "UnbindRequest",
        UnbindRequest::new(4),
        iterations
    );

    bench_all!(
        UnbindResponse,
        "UnbindResponse",
        UnbindResponse::new(4, "ESME_ROK"),
        iterations
    );

    bench_all!(
        EnquireLinkRequest,
        "EnquireLinkRequest",
        EnquireLinkRequest::new(5),
        iterations
    );

    bench_all!(
        EnquireLinkResponse,
        "EnquireLinkResponse",
        EnquireLinkResponse::new(5, "ESME_ROK"),
        iterations
    );

    bench_all!(
        GenericNack,
        "GenericNack",
        GenericNack::new("ESME_RINVCMDID", 6),
        iterations
    );

    bench_all!(
        AlertNotification,
        "AlertNotification",
        AlertNotification::new(7, "source".into(), "esme".into()),
        iterations
    );

    // --- Submission PDUs ---

    bench_all!(
        SubmitSmRequest,
        "SubmitSmRequest",
        SubmitSmRequest::new(
            100,
            "source".into(),
            "dest".into(),
            b"Short Message Content".to_vec()
        ),
        iterations
    );

    {
        let name = "SubmitSmRequest (Multi)";
        // Create a message long enough to be split (GSM 7-bit limit is 160 septets)
        // With UDH (6 bytes), the limit is lower (153 septets).
        // 320 chars guarantees multiple parts.
        let text = "A".repeat(320);

        let (parts, _) =
            MessageSplitter::split(text, EncodingType::Gsm7Bit, SplitMode::Udh).unwrap();

        let requests: Vec<SubmitSmRequest> = parts
            .into_iter()
            .enumerate()
            .map(|(i, part)| {
                let mut req =
                    SubmitSmRequest::new(100 + i as u32, "source".into(), "dest".into(), part);
                req.data_coding = 10; // Requested DCS
                req.protocol_id = 10; // Requested PID
                req.esm_class = 0x40; // UDHI
                req
            })
            .collect();

        // Measure Encoding
        let mut buffer = Vec::with_capacity(2048);
        let start_encode = Instant::now();
        let mut total_encoded = 0;
        for _ in 0..iterations {
            for req in &requests {
                buffer.clear();
                req.encode(&mut buffer).unwrap();
                total_encoded += 1;
            }
        }
        let duration_encode = start_encode.elapsed();
        let ops_encode = total_encoded as f64 / duration_encode.as_secs_f64();

        // Measure Decoding
        // Pre-encode to simulate receiving
        let encoded_buffers: Vec<Vec<u8>> = requests
            .iter()
            .map(|r| {
                let mut b = Vec::new();
                r.encode(&mut b).unwrap();
                b
            })
            .collect();

        let start_decode = Instant::now();
        let mut total_decoded = 0;
        for _ in 0..iterations {
            for buf in &encoded_buffers {
                let _ = SubmitSmRequest::decode(buf).unwrap();
                total_decoded += 1;
            }
        }
        let duration_decode = start_decode.elapsed();
        let ops_decode = total_decoded as f64 / duration_decode.as_secs_f64();

        println!(
            "| {:<25} | {:<13.2?} | {:<15.2} | {:<13.2?} | {:<15.2} |",
            name, duration_encode, ops_encode, duration_decode, ops_decode
        );
    }

    bench_all!(
        SubmitSmResponse,
        "SubmitSmResponse",
        SubmitSmResponse::new(100, "ESME_ROK", "msg_id_123".into()),
        iterations
    );

    bench_all!(
        SubmitMulti,
        "SubmitMulti",
        SubmitMulti::new(
            1001,
            "Source".to_string(),
            vec![
                Destination::SmeAddress {
                    ton: Ton::International,
                    npi: Npi::Isdn,
                    address: "123".to_string()
                },
                Destination::DistributionList("List".to_string())
            ],
            b"Payload".to_vec()
        ),
        iterations
    );

    bench_all!(
        SubmitMultiResp,
        "SubmitMultiResp",
        SubmitMultiResp::new(1001, "ESME_ROK", "msg_id".into(), vec![]),
        iterations
    );

    // --- Delivery PDUs ---

    bench_all!(
        DeliverSmRequest,
        "DeliverSmRequest",
        DeliverSmRequest::new(
            200,
            "source".into(),
            "dest".into(),
            b"Delivery Receipt or Msg".to_vec()
        ),
        iterations
    );

    bench_all!(
        DeliverSmResponse,
        "DeliverSmResponse",
        DeliverSmResponse::new(200, "ESME_ROK"),
        iterations
    );

    bench_all!(
        DataSm,
        "DataSm",
        DataSm::new(
            3001,
            "Source".to_string(),
            "Dest".to_string(),
            b"Data Payload".to_vec()
        ),
        iterations
    );

    bench_all!(
        DataSmResp,
        "DataSmResp",
        DataSmResp::new(3001, "ESME_ROK", "msg_id".into()),
        iterations
    );

    // --- Ancillary PDUs ---

    bench_all!(
        CancelSmRequest,
        "CancelSmRequest",
        CancelSmRequest::new(300, "msg_id".into(), "src".into(), "dst".into()),
        iterations
    );

    bench_all!(
        CancelSmResponse,
        "CancelSmResponse",
        CancelSmResponse::new(300, "ESME_ROK"),
        iterations
    );

    bench_all!(
        QuerySmRequest,
        "QuerySmRequest",
        QuerySmRequest::new(400, "msg_id".into(), "src".into()),
        iterations
    );

    bench_all!(
        QuerySmResponse,
        "QuerySmResponse",
        QuerySmResponse::new(
            400,
            "ESME_ROK",
            "msg_id".into(),
            String::new(),
            MessageState::Delivered as u8,
            0
        ),
        iterations
    );

    bench_all!(
        BroadcastSm,
        "BroadcastSm",
        BroadcastSm::new(
            2001,
            "ServiceAlert".to_string(),
            b"Payload".to_vec(),
            smpp_codec::tlv::Tlv::new(0x0606, vec![0x00])
        ),
        iterations
    );

    bench_all!(
        BroadcastSmResp,
        "BroadcastSmResp",
        BroadcastSmResp::new(2001, "ESME_ROK", "msg_id".into()),
        iterations
    );

    bench_all!(
        ReplaceSm,
        "ReplaceSm",
        ReplaceSm::new(
            4001,
            "msg_123".to_string(),
            "Source".to_string(),
            b"New Content".to_vec()
        ),
        iterations
    );

    bench_all!(
        ReplaceSmResp,
        "ReplaceSmResp",
        ReplaceSmResp::new(4001, "ESME_ROK"),
        iterations
    );

    bench_all!(
        QueryBroadcastSm,
        "QueryBroadcastSm",
        QueryBroadcastSm::new(2002, "bc_msg_1".to_string(), "Source".to_string()),
        iterations
    );

    bench_all!(
        QueryBroadcastSmResp,
        "QueryBroadcastSmResp",
        QueryBroadcastSmResp::new(2002, "ESME_ROK", "bc_msg_1".to_string()),
        iterations
    );

    bench_all!(
        CancelBroadcastSm,
        "CancelBroadcastSm",
        CancelBroadcastSm::new(
            3002,
            "CMT".to_string(),
            "bc_msg_2".to_string(),
            "Source".to_string()
        ),
        iterations
    );

    bench_all!(
        CancelBroadcastSmResp,
        "CancelBroadcastSmResp",
        CancelBroadcastSmResp::new(3002, "ESME_ROK"),
        iterations
    );

    println!(
        "|{:-<27}|{:-<15}|{:-<17}|{:-<15}|{:-<17}|",
        "", "", "", "", ""
    );
    println!("\nBenchmark suite completed successfully.");
}
