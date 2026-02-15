// 1. Declare the directory modules
/// Ancillary PDUs (Cancel, Replace, Query)
pub mod ancillary_pdus;
/// Broadcast PDUs
pub mod broadcast_pdus;
/// Delivery PDUs (DeliverSm, DataSm)
pub mod delivery_pdus;
/// Session PDUs (Bind, Unbind, EnquireLink)
pub mod session_pdus;
/// Submission PDUs (SubmitSm, SubmitMulti)
pub mod submission_pdus;

pub use crate::splitter::{EncodingType, MessageSplitter, SplitMode};
pub use ancillary_pdus::{CancelBroadcastSmRequest, CancelBroadcastSmResponse};
pub use ancillary_pdus::{CancelSmRequest, CancelSmResponse};
pub use ancillary_pdus::{MessageState, QuerySmRequest, QuerySmResponse};
pub use ancillary_pdus::{QueryBroadcastSmRequest, QueryBroadcastSmResponse};
pub use ancillary_pdus::{ReplaceSmRequest, ReplaceSmResponse};
pub use broadcast_pdus::{BroadcastSmRequest, BroadcastSmResponse};
pub use delivery_pdus::{DataSmRequest, DataSmResponse, DeliverSmRequest, DeliverSmResponse, DeliveryReceipt};
pub use session_pdus::alert_notification::AlertNotification;
pub use session_pdus::bind_request::BindRequest;
pub use session_pdus::bind_response::BindResponse;
pub use session_pdus::enquirelink_request::EnquireLinkRequest;
pub use session_pdus::enquirelink_response::EnquireLinkResponse;
pub use session_pdus::generic_nack::GenericNack;
pub use session_pdus::outbind::OutbindRequest;
pub use session_pdus::unbind::UnbindRequest;
pub use session_pdus::unbind::UnbindResponse;
pub use submission_pdus::SubmitSmRequest;
pub use submission_pdus::SubmitSmResponse;
pub use submission_pdus::{Destination, SubmitMultiRequest};
pub use submission_pdus::{SubmitMultiResponse, UnsuccessfulDelivery};

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `DataSmRequest` for consistency. Please update your code."
)]
pub type DataSm = DataSmRequest;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `DataSmResponse` for consistency."
)]
pub type DataSmResp = DataSmResponse;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `SubmitMultiRequest` for consistency."
)]
pub type SubmitMulti = SubmitMultiRequest;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `SubmitMultiResponse` for consistency."
)]
pub type SubmitMultiResp = SubmitMultiResponse;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `BroadcastSmRequest` for consistency."
)]
pub type BroadcastSm = BroadcastSmRequest;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `BroadcastSmResponse` for consistency."
)]
pub type BroadcastSmResp = BroadcastSmResponse;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `CancelBroadcastSmRequest` for consistency."
)]
pub type CancelBroadcastSm = CancelBroadcastSmRequest;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `CancelBroadcastSmResponse` for consistency."
)]
pub type CancelBroadcastSmResp = CancelBroadcastSmResponse;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `QueryBroadcastSmRequest` for consistency."
)]
pub type QueryBroadcastSm = QueryBroadcastSmRequest;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `QueryBroadcastSmResponse` for consistency."
)]
pub type QueryBroadcastSmResp = QueryBroadcastSmResponse;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `ReplaceSmRequest` for consistency."
)]
pub type ReplaceSm = ReplaceSmRequest;

#[deprecated(
    since = "0.2.2", 
    note = "Renamed to `ReplaceSmResponse` for consistency."
)]
pub type ReplaceSmResp = ReplaceSmResponse;
