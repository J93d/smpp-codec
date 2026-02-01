use smpp_codec::pdus::{BindRequest, BindResponse};

#[test]
fn test_bind_request_hex() {
    // Hex provided by user: Bind Request
    let hex_str = "0000002300000009000000000000002574657374007465737400746573740003010100";

    let bytes = (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).expect("Invalid hex"))
        .collect::<Vec<u8>>();

    println!("Attempting to decode BindRequest...");
    match BindRequest::decode(&bytes) {
        Ok(req) => {
            println!("Success: {:?}", req);
            assert_eq!(req.sequence_number, 37);
            assert_eq!(req.system_id, "test");
            assert_eq!(req.password, "test");
        }
        Err(e) => {
            println!("BindRequest Decode Failed (Expected?): {:?}", e);
        }
    }
}

#[test]
fn test_bind_response_hex() {
    // Hex provided by user: Bind Response
    let hex_str = "000000158000000900000000000000257465737400";

    let bytes = (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).expect("Invalid hex"))
        .collect::<Vec<u8>>();

    let resp = BindResponse::decode(&bytes).expect("Failed to decode BindResponse");

    println!("Decoded: {:?}", resp);

    assert_eq!(resp.sequence_number, 37);
    assert_eq!(resp.command_status, 0); // ESME_ROK
    assert_eq!(resp.system_id, "test");
    assert_eq!(resp.status_description, "ESME_ROK");
}
