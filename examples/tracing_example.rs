use smpp_codec::common::BindMode;
use smpp_codec::pdus::{BindRequest, SubmitSmRequest};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    // 1. Initialize tracing subscriber
    // In a real app, you might use tracing-subscriber to log to stdout/stderr.
    tracing_subscriber::fmt()
        .with_env_filter("tracing_example=debug,smpp_codec=debug")
        .with_span_events(FmtSpan::CLOSE)
        .init();

    tracing::info!("Starting tracing example");

    // 2. Demonstrate BindRequest encoding with tracing
    let bind_req = BindRequest::new(
        1,
        BindMode::Transceiver,
        "my_system".to_string(),
        "password".to_string(),
    );

    let mut buffer = Vec::new();
    tracing::info!("Encoding BindRequest...");
    bind_req.encode(&mut buffer).expect("Failed to encode");

    // 3. Demonstrate SubmitSmRequest decoding with tracing
    let submit_req = SubmitSmRequest::new(
        2,
        "source".to_string(),
        "dest".to_string(),
        b"Hello with tracing!".to_vec(),
    );

    let mut submit_buffer = Vec::new();
    submit_req
        .encode(&mut submit_buffer)
        .expect("Failed to encode submit");

    tracing::info!("Decoding SubmitSmRequest...");
    let _decoded = SubmitSmRequest::decode(&submit_buffer).expect("Failed to decode");

    tracing::info!("Tracing example finished. Check logs above for smpp_codec spans.");
}
