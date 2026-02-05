//! # SMPP Codec
//!
//! `smpp-codec` is a Rust library for encoding and decoding SMPP 3.4 / 5.0 PDUs (Protocol Data Units).
//! It provides a type-safe and efficient way to handle SMPP messages, suitable for building SMSCs (Short Message Service Centers)
//! or ESMEs (External Short Message Entities).
//!
//! ## Features
//!
//! *   Full support for SMPP 3.4 / 5.0 PDUs.
//! *   Strongly typed structures for all standard operations (Bind, SubmitSm, DeliverSm, etc.).
//! *   Support for TLVs (Tagged Length Values) / Optional Parameters.
//! *   Easy-to-use API for encoding and decoding.
//!
//! ## Performance Tips
//!
//! this library is designed to be compatible with any `std::io::Write` implementation.
//! When encoding PDUs directly to a network stream, **always use buffering**.
//!
//! *   **Recommended**: Encode to a `Vec<u8>` first, then write the vector to the stream.
//! *   **Alternative**: Wrap your `TcpStream` in a `std::io::BufWriter`.
//!
//! *Passing a raw `TcpStream` to `encode` will result in many small system calls, significantly reducing throughput.*
//!
//! See [docs/PERFORMANCE.md](docs/PERFORMANCE.md) for a detailed guide.
//!
//! ## Example
//!
//! ```rust
//! use smpp_codec::pdus::BindRequest;
//! use smpp_codec::common::BindMode;
//!
//! let mut bind_req = BindRequest::new(
//!     1, // Sequence number
//!     BindMode::Transmitter,
//!     "my_system_id".to_string(),
//!     "password".to_string(),
//! );
//!
//! // Encode to bytes
//! let mut buffer = Vec::new();
//! bind_req.encode(&mut buffer).unwrap();
//! ```

#![warn(missing_docs)]
// Expose the common module (constants, errors, enums)
/// Common constants, enums, and error types.
pub mod common;
/// Encoding and decoding utilities (GSM 7-bit, etc.).
pub mod encoding;
/// Tag-Length-Value (TLV) support.
pub mod tlv;

// Expose the PDUs module (the structs for specific operations)
/// PDU definitions and submodules.
pub mod pdus;
/// Message splitting and concatenation utilities.
pub mod splitter;

#[cfg(test)]
mod tests {
    doc_comment::doctest!("../README.md");
}
