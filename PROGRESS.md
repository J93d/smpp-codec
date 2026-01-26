# SMPP Implementation Progress

Tracking the implementation status of SMPP v3.4 PDUs.

## Session Management
- [x] `bind_receiver`
- [x] `bind_receiver_resp`
- [x] `bind_transmitter`
- [x] `bind_transmitter_resp`
- [x] `bind_transceiver`
- [x] `bind_transceiver_resp`
- [x] `outbind`
- [x] `unbind` (Pending implementation? Checks needed)
- [x] `unbind_resp` (Pending implementation? Checks needed)
- [x] `enquire_link`
- [x] `enquire_link_resp`
- [x] `alert_notification`
- [x] `generic_nack`

## Message Submission
- [ ] `submit_sm`
- [ ] `submit_sm_resp`
- [ ] `submit_multi`
- [ ] `submit_multi_resp`
- [ ] `data_sm`
- [ ] `data_sm_resp`

## Message Delivery
- [ ] `deliver_sm`
- [ ] `deliver_sm_resp`

## Message Query/Cancel/Replace
- [ ] `query_sm`
- [ ] `query_sm_resp`
- [ ] `cancel_sm`
- [ ] `cancel_sm_resp`
- [ ] `replace_sm`
- [ ] `replace_sm_resp`

## Ancillary Operations
- [ ] Broadcast SM (Optional/Advanced)
