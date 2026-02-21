use smpp_codec::common::BindMode;
use smpp_codec::pdus::BindRequest;
use std::sync::{Arc, Mutex};
use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;

struct TestLayer {
    logs: Arc<Mutex<Vec<String>>>,
}

struct FieldVisitor {
    message: String,
}

impl tracing::field::Visit for FieldVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if !self.message.is_empty() {
            self.message.push_str(", ");
        }
        self.message
            .push_str(&format!("{}: {:?}", field.name(), value));
    }
}

impl<S: Subscriber> Layer<S> for TestLayer {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        let mut visitor = FieldVisitor {
            message: String::new(),
        };
        event.record(&mut visitor);

        let mut logs = self.logs.lock().unwrap();
        logs.push(format!("event: {}", visitor.message));
    }

    fn on_enter(&self, _id: &tracing::span::Id, _ctx: Context<'_, S>) {
        // Just capture events for now to be safe
    }
}

#[test]
#[cfg(feature = "tracing")]
fn test_tracing_integration() {
    let logs = Arc::new(Mutex::new(Vec::new()));
    let layer = TestLayer {
        logs: Arc::clone(&logs),
    };
    let subscriber = tracing_subscriber::registry().with(layer);

    let _ = tracing::subscriber::set_global_default(subscriber);

    // 1. Test successful encoding (fields should be present in metadata, but we'll check event)
    let bind_req = BindRequest::new(123, BindMode::Transceiver, "system".into(), "pwd".into());
    let mut buffer = Vec::new();
    bind_req.encode(&mut buffer).unwrap();

    // 2. Test successful decoding (sequence number should be recorded)
    let _ = BindRequest::decode(&buffer).unwrap();

    // 3. Test error logging (e.g., buffer too short)
    let short_buffer = vec![0u8; 5];
    let _ = BindRequest::decode(&short_buffer);

    let captured_logs = logs.lock().unwrap();

    if !captured_logs
        .iter()
        .any(|l| l.contains("Encoding BindRequest"))
        || !captured_logs
            .iter()
            .any(|l| l.contains("Decoding BindRequest"))
        || !captured_logs.iter().any(|l| l.contains("Buffer too short"))
    {
        println!("Captured logs: {:#?}", captured_logs);
    }

    // Check for encoding log
    assert!(captured_logs
        .iter()
        .any(|l| l.contains("Encoding BindRequest")));

    // Check for decoding log
    assert!(captured_logs
        .iter()
        .any(|l| l.contains("Decoding BindRequest")));

    // Check for error log
    // We expect "Buffer too short" because of PduError::BufferTooShort
    assert!(captured_logs
        .iter()
        .any(|l| l.contains("Buffer too short") || l.contains("PduError") || l.contains("error")));
}
