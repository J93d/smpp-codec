use std::fmt;

// Command IDs
pub const CMD_BIND_RECEIVER: u32 = 0x00000001;
pub const CMD_BIND_RECEIVER_RESP: u32 = 0x80000001;

pub const CMD_BIND_TRANSMITTER: u32 = 0x00000002;
pub const CMD_BIND_TRANSMITTER_RESP: u32 = 0x80000002;

pub const CMD_BIND_TRANSCEIVER: u32 = 0x00000009;
pub const CMD_BIND_TRANSCEIVER_RESP: u32 = 0x80000009;

pub const CMD_ENQUIRE_LINK: u32 = 0x00000015;
pub const CMD_ENQUIRE_LINK_RESP: u32 = 0x80000015;

pub const CMD_SUBMIT_SM: u32 = 0x00000004;
pub const CMD_SUBMIT_SM_RESP: u32 = 0x80000004;

pub const CMD_DELIVER_SM: u32 = 0x00000005;
pub const CMD_DELIVER_SM_RESP: u32 = 0x80000005;

pub const CMD_UNBIND: u32 = 0x00000006;
pub const CMD_UNBIND_RESP: u32 = 0x80000006;

pub const CMD_SUBMIT_MULTI_SM: u32 = 0x00000021;
pub const CMD_SUBMIT_MULTI_SM_RESP: u32 = 0x80000021;

pub const CMD_QUERY_SM: u32 = 0x00000022;
pub const CMD_QUERY_SM_RESP: u32 = 0x80000022;

pub const CMD_CANCEL_SM: u32 = 0x00000023;
pub const CMD_CANCEL_SM_RESP: u32 = 0x80000023;

pub const CMD_REPLACE_SM: u32 = 0x00000024;
pub const CMD_REPLACE_SM_RESP: u32 = 0x80000024;

pub const CMD_SUBMIT_SM_MULTI: u32 = 0x00000025;
pub const CMD_SUBMIT_SM_MULTI_RESP: u32 = 0x80000025;

pub const CMD_DATA_SM: u32 = 0x00000103;
pub const CMD_DATA_SM_RESP: u32 = 0x80000103;

pub const CMD_ALERT_NOTIFICATION: u32 = 0x00000102;
pub const CMD_ALERT_NOTIFICATION_RESP: u32 = 0x80000102;

pub const GENERIC_NACK: u32 = 0x80000000;

// Standard Header Length
pub const HEADER_LEN: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BindMode {
    Receiver,
    Transmitter,
    Transceiver,
}

impl BindMode {
    pub fn command_id(&self) -> u32 {
        match self {
            BindMode::Receiver => CMD_BIND_RECEIVER,
            BindMode::Transmitter => CMD_BIND_TRANSMITTER,
            BindMode::Transceiver => CMD_BIND_TRANSCEIVER,
        }
    }
}

// Custom Error for PDU operations
#[derive(Debug)]
pub enum PduError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    BufferTooShort,
    InvalidCommandId(u32),
    StringTooLong(String, usize), // Field name, Max len
}

// Convert IO errors to PduError
impl From<std::io::Error> for PduError {
    fn from(err: std::io::Error) -> Self { PduError::Io(err) }
}