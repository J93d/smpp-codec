//! # Common Definitions
//!
//! This module contains common constants, error types, and helper functions used throughout the library.

// --- Command IDs ---
// These constants define the Command ID for each SMPP PDU.

/// Command ID for Bind Receiver.
pub const CMD_BIND_RECEIVER: u32 = 0x00000001;
/// Command ID for Bind Receiver Response.
pub const CMD_BIND_RECEIVER_RESP: u32 = 0x80000001;

/// Command ID for Bind Transmitter.
pub const CMD_BIND_TRANSMITTER: u32 = 0x00000002;
/// Command ID for Bind Transmitter Response.
pub const CMD_BIND_TRANSMITTER_RESP: u32 = 0x80000002;

/// Command ID for Bind Transceiver.
pub const CMD_BIND_TRANSCEIVER: u32 = 0x00000009;
/// Command ID for Bind Transceiver Response.
pub const CMD_BIND_TRANSCEIVER_RESP: u32 = 0x80000009;

/// Command ID for Outbind.
pub const CMD_OUTBIND: u32 = 0x0000000B;

/// Command ID for Enquire Link.
pub const CMD_ENQUIRE_LINK: u32 = 0x00000015;
/// Command ID for Enquire Link Response.
pub const CMD_ENQUIRE_LINK_RESP: u32 = 0x80000015;

/// Command ID for Submit SM.
pub const CMD_SUBMIT_SM: u32 = 0x00000004;
/// Command ID for Submit SM Response.
pub const CMD_SUBMIT_SM_RESP: u32 = 0x80000004;

/// Command ID for Deliver SM.
pub const CMD_DELIVER_SM: u32 = 0x00000005;
/// Command ID for Deliver SM Response.
pub const CMD_DELIVER_SM_RESP: u32 = 0x80000005;

/// Command ID for Unbind.
pub const CMD_UNBIND: u32 = 0x00000006;
/// Command ID for Unbind Response.
pub const CMD_UNBIND_RESP: u32 = 0x80000006;

/// Command ID for Submit Multi SM.
pub const CMD_SUBMIT_MULTI_SM: u32 = 0x00000021;
/// Command ID for Submit Multi SM Response.
pub const CMD_SUBMIT_MULTI_SM_RESP: u32 = 0x80000021;

/// Command ID for Query SM.
pub const CMD_QUERY_SM: u32 = 0x00000022;
/// Command ID for Query SM Response.
pub const CMD_QUERY_SM_RESP: u32 = 0x80000022;

/// Command ID for Cancel SM.
pub const CMD_CANCEL_SM: u32 = 0x00000023;
/// Command ID for Cancel SM Response.
pub const CMD_CANCEL_SM_RESP: u32 = 0x80000023;

/// Command ID for Replace SM.
pub const CMD_REPLACE_SM: u32 = 0x00000024;
/// Command ID for Replace SM Response.
pub const CMD_REPLACE_SM_RESP: u32 = 0x80000024;

/// Command ID for Submit SM Multi (Reserved).
pub const CMD_SUBMIT_SM_MULTI: u32 = 0x00000025;
/// Command ID for Submit SM Multi Response (Reserved).
pub const CMD_SUBMIT_SM_MULTI_RESP: u32 = 0x80000025;

/// Command ID for Data SM.
pub const CMD_DATA_SM: u32 = 0x00000103;
/// Command ID for Data SM Response.
pub const CMD_DATA_SM_RESP: u32 = 0x80000103;

/// Command ID for Alert Notification.
pub const CMD_ALERT_NOTIFICATION: u32 = 0x00000102;
/// Command ID for Alert Notification Response (Note: Alert Notification does not have a standard response, but some implementations might use one).
pub const CMD_ALERT_NOTIFICATION_RESP: u32 = 0x80000102;

/// Generic NACK.
pub const GENERIC_NACK: u32 = 0x80000000;

/// Command ID for Broadcast SM.
pub const CMD_BROADCAST_SM: u32 = 0x00000111;
/// Command ID for Broadcast SM Response.
pub const CMD_BROADCAST_SM_RESP: u32 = 0x80000112;

/// Command ID for Query Broadcast SM.
pub const CMD_QUERY_BROADCAST_SM: u32 = 0x00000112;
/// Command ID for Query Broadcast SM Response.
pub const CMD_QUERY_BROADCAST_SM_RESP: u32 = 0x80000112;
/// Command ID for Cancel Broadcast SM.
pub const CMD_CANCEL_BROADCAST_SM: u32 = 0x00000113;
/// Command ID for Cancel Broadcast SM Response.
pub const CMD_CANCEL_BROADCAST_SM_RESP: u32 = 0x80000113;

/// Standard Header Length
pub const HEADER_LEN: usize = 16;

/// SMPP Interface Version
pub const SMPP_INTERFACE_VERSION_34: u8 = 0x34;
pub const SMPP_INTERFACE_VERSION_50: u8 = 0x50;

/// Address Type of Number (TON)
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Ton {
    Unknown = 0x00,
    International = 0x01,
    National = 0x02,
    NetworkSpecific = 0x03,
    SubscriberNumber = 0x04,
    Alphanumeric = 0x05,
    Abbreviated = 0x06,
}

impl From<u8> for Ton {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Ton::International,
            0x02 => Ton::National,
            0x03 => Ton::NetworkSpecific,
            0x04 => Ton::SubscriberNumber,
            0x05 => Ton::Alphanumeric,
            0x06 => Ton::Abbreviated,
            _ => Ton::Unknown,
        }
    }
}

/// Address Numbering Plan Indicator (NPI)
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Npi {
    Unknown = 0x00,
    Isdn = 0x01,
    Data = 0x03,
    Telex = 0x04,
    LandMobile = 0x06,
    National = 0x08,
    Private = 0x09,
    Ermes = 0x0A,
    Internet = 0x0E,
    Wap = 0x12,
}

impl From<u8> for Npi {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Npi::Isdn,
            0x03 => Npi::Data,
            0x04 => Npi::Telex,
            0x06 => Npi::LandMobile,
            0x08 => Npi::National,
            0x09 => Npi::Private,
            0x0A => Npi::Ermes,
            0x0E => Npi::Internet,
            0x12 => Npi::Wap,
            _ => Npi::Unknown,
        }
    }
}

/// Command Status
pub const COMMAND_STATUS_OK: u32 = 0x00000000;

/// Helper function to get error description from status code
pub fn get_status_description(status: u32) -> String {
    match status {
        0x00000000 => "ESME_ROK".to_string(),
        0x00000001 => "ESME_RINVMSGLEN".to_string(),
        0x00000002 => "ESME_RINVCMDLEN".to_string(),
        0x00000003 => "ESME_RINVCMDID".to_string(),
        0x00000004 => "ESME_RINVBNDSTS".to_string(),
        0x00000005 => "ESME_RALYBND".to_string(),
        0x00000006 => "ESME_RINVPRTFLG".to_string(),
        0x00000007 => "ESME_RINVREGDLVFLG".to_string(),
        0x00000008 => "ESME_RSYSERR".to_string(),
        0x0000000A => "ESME_RINVSRCADR".to_string(),
        0x0000000B => "ESME_RINVDSTADR".to_string(),
        0x0000000C => "ESME_RINVMSGID".to_string(),
        0x0000000D => "ESME_RBINDFAIL".to_string(),
        0x0000000E => "ESME_RINVPASWD".to_string(),
        0x0000000F => "ESME_RINVSYSID".to_string(),
        0x00000011 => "ESME_RCANCELFAIL".to_string(),
        0x00000013 => "ESME_RREPLACEFAIL".to_string(),
        0x00000014 => "ESME_RMSGQFUL".to_string(),
        0x00000015 => "ESME_RINVSERVICETYPE".to_string(),
        0x00000033 => "ESME_RINVNUMDESTS".to_string(),
        0x00000034 => "ESME_RINVDLNAME".to_string(),
        0x00000040 => "ESME_RINVDESTFLAG".to_string(),
        0x00000042 => "ESME_RINVSUBREP".to_string(),
        0x00000043 => "ESME_RINVESMCLASS".to_string(),
        0x00000044 => "ESME_RCNTSUBDL".to_string(),
        0x00000045 => "ESME_RSUBMITFAIL".to_string(),
        0x00000048 => "ESME_RINVSRCTON".to_string(),
        0x00000049 => "ESME_RINVSRCNPI".to_string(),
        0x00000050 => "ESME_RINVDSTTON".to_string(),
        0x00000051 => "ESME_RINVDSTNPI".to_string(),
        0x00000053 => "ESME_RINVSYSTYP".to_string(),
        0x00000054 => "ESME_RINVREPFLAG".to_string(),
        0x00000055 => "ESME_RINVNUMMSGS".to_string(),
        0x00000058 => "ESME_RTHROTTLED".to_string(),
        0x00000061 => "ESME_RINVSCHED".to_string(),
        0x00000062 => "ESME_RINVEXPIRY".to_string(),
        0x00000063 => "ESME_RINVDFTMSGID".to_string(),
        0x00000064 => "ESME_RX_T_APPN".to_string(),
        0x00000065 => "ESME_RX_P_APPN".to_string(),
        0x00000066 => "ESME_RX_R_APPN".to_string(),
        0x00000067 => "ESME_RQUERYFAIL".to_string(),
        0x000000C0 => "ESME_RINVOPTPARSTREAM".to_string(),
        0x000000C1 => "ESME_ROPTPARNOTALLWD".to_string(),
        0x000000C2 => "ESME_RINVPARLEN".to_string(),
        0x000000C3 => "ESME_RMISSINGOPTPARAM".to_string(),
        0x000000C4 => "ESME_RINVOPTPARAMVAL".to_string(),
        0x000000FE => "ESME_RDELIVERYFAILURE".to_string(),
        0x000000FF => "ESME_RUNKNOWNERR".to_string(),
        _ => format!("Unknown Error: 0x{:08X}", status),
    }
}

/// Helper function to get status code from description (Reverse lookup)
pub fn get_status_code(name: &str) -> u32 {
    match name {
        "ESME_ROK" => 0x00000000,
        "ESME_RINVMSGLEN" => 0x00000001,
        "ESME_RINVCMDLEN" => 0x00000002,
        "ESME_RINVCMDID" => 0x00000003,
        "ESME_RINVBNDSTS" => 0x00000004,
        "ESME_RALYBND" => 0x00000005,
        "ESME_RINVPRTFLG" => 0x00000006,
        "ESME_RINVREGDLVFLG" => 0x00000007,
        "ESME_RSYSERR" => 0x00000008,
        "ESME_RINVSRCADR" => 0x0000000A,
        "ESME_RINVDSTADR" => 0x0000000B,
        "ESME_RINVMSGID" => 0x0000000C,
        "ESME_RBINDFAIL" => 0x0000000D,
        "ESME_RINVPASWD" => 0x0000000E,
        "ESME_RINVSYSID" => 0x0000000F,
        "ESME_RCANCELFAIL" => 0x00000011,
        "ESME_RREPLACEFAIL" => 0x00000013,
        "ESME_RMSGQFUL" => 0x00000014,
        "ESME_RINVSERVICETYPE" => 0x00000015,
        "ESME_RINVNUMDESTS" => 0x00000033,
        "ESME_RINVDLNAME" => 0x00000034,
        "ESME_RINVDESTFLAG" => 0x00000040,
        "ESME_RINVSUBREP" => 0x00000042,
        "ESME_RINVESMCLASS" => 0x00000043,
        "ESME_RCNTSUBDL" => 0x00000044,
        "ESME_RSUBMITFAIL" => 0x00000045,
        "ESME_RINVSRCTON" => 0x00000048,
        "ESME_RINVSRCNPI" => 0x00000049,
        "ESME_RINVDSTTON" => 0x00000050,
        "ESME_RINVDSTNPI" => 0x00000051,
        "ESME_RINVSYSTYP" => 0x00000053,
        "ESME_RINVREPFLAG" => 0x00000054,
        "ESME_RINVNUMMSGS" => 0x00000055,
        "ESME_RTHROTTLED" => 0x00000058,
        "ESME_RINVSCHED" => 0x00000061,
        "ESME_RINVEXPIRY" => 0x00000062,
        "ESME_RINVDFTMSGID" => 0x00000063,
        "ESME_RX_T_APPN" => 0x00000064,
        "ESME_RX_P_APPN" => 0x00000065,
        "ESME_RX_R_APPN" => 0x00000066,
        "ESME_RQUERYFAIL" => 0x00000067,
        "ESME_RINVOPTPARSTREAM" => 0x000000C0,
        "ESME_ROPTPARNOTALLWD" => 0x000000C1,
        "ESME_RINVPARLEN" => 0x000000C2,
        "ESME_RMISSINGOPTPARAM" => 0x000000C3,
        "ESME_RINVOPTPARAMVAL" => 0x000000C4,
        "ESME_RDELIVERYFAILURE" => 0x000000FE,
        "ESME_RUNKNOWNERR" => 0x000000FF,
        _ => 0x000000FF, // Default to Unknown Error if string not found
    }
}

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
    InvalidLength,
}

// Convert IO errors to PduError
impl From<std::io::Error> for PduError {
    fn from(err: std::io::Error) -> Self {
        PduError::Io(err)
    }
}

/// optimized helper to read a C-Style string from a Cursor<&[u8]>
pub fn read_c_string(cursor: &mut std::io::Cursor<&[u8]>) -> Result<String, PduError> {
    let current_pos = cursor.position() as usize;
    let inner = cursor.get_ref();

    if current_pos >= inner.len() {
        return Err(PduError::Io(std::io::Error::from(
            std::io::ErrorKind::UnexpectedEof,
        )));
    }

    let remaining = &inner[current_pos..];

    // Find null byte efficiently using slice iter
    match remaining.iter().position(|&b| b == 0) {
        Some(null_idx) => {
            let s_bytes = &remaining[..null_idx];
            let s = String::from_utf8(s_bytes.to_vec()).map_err(PduError::Utf8)?;
            cursor.set_position((current_pos + null_idx + 1) as u64);
            Ok(s)
        }
        None => Err(PduError::Io(std::io::Error::from(
            std::io::ErrorKind::UnexpectedEof,
        ))),
    }
}

/// Helper to write a C-Style string (null terminated)
pub fn write_c_string(w: &mut impl std::io::Write, s: &str) -> std::io::Result<()> {
    w.write_all(s.as_bytes())?;
    w.write_all(&[0])
}
