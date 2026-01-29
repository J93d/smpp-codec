//! # SMPP Codec
//!
//! `smpp-codec` is a Rust library for encoding and decoding SMPP 3.4 PDUs (Protocol Data Units).
//! It provides a type-safe and efficient way to handle SMPP messages, suitable for building SMSCs (Short Message Service Centers)
//! or ESMEs (External Short Message Entities).
//!
//! ## Features
//!
//! *   Full support for SMPP 3.4 PDUs.
//! *   Strongly typed structures for all standard operations (Bind, SubmitSm, DeliverSm, etc.).
//! *   Support for TLVs (Tagged Length Values) / Optional Parameters.
//! *   Easy-to-use API for encoding and decoding.
//!
//! ## Example
//!
//! ```rust
//! use smpp_codec::pdus::BindRequest;
//! use smpp_codec::common::BindMode;
//!
//! let mut bind_req = BindRequest::new(
//!     BindMode::Transmitter,
//!     "my_system_id".to_string(),
//!     "password".to_string(),
//!     1 // Sequence number
//! );
//!
//! // Encode to bytes
//! let mut buffer = Vec::new();
//! bind_req.encode(&mut buffer).unwrap();
//! ```

// Expose the common module (constants, errors, enums)
pub mod common;
pub mod tlv;
pub mod encoding;

// Expose the PDUs module (the structs for specific operations)
pub mod pdus;