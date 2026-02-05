use crate::common::{
    get_status_code, get_status_description, PduError, CMD_UNBIND, CMD_UNBIND_RESP, HEADER_LEN,
};
use std::io::{Cursor, Read, Write};

// --- Unbind Request ---
/// Represents an Unbind Request PDU.
///
/// Used to request the termination of a session.
#[derive(Debug, Clone, PartialEq)]
pub struct UnbindRequest {
    /// Sequence number of the PDU
    pub sequence_number: u32,
}

impl UnbindRequest {
    /// Create a new Unbind Request.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::UnbindRequest;
    ///
    /// let sequence_number: u32 = 1;
    /// let unbind_req = UnbindRequest::new(sequence_number);
    /// ```
    pub fn new(sequence_number: u32) -> Self {
        Self { sequence_number }
    }

    /// Encode the struct into raw bytes for the network.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if an I/O error occurs while writing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::UnbindRequest;
    /// # let sequence_number: u32 = 1;
    /// # let unbind_req = UnbindRequest::new(sequence_number);
    /// let mut buffer = Vec::new();
    /// unbind_req.encode(&mut buffer).expect("Encoding failed");
    /// ```
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let command_len = HEADER_LEN as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_UNBIND.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?; // Status always 0 for requests
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        Ok(())
    }

    /// Decode raw bytes from the network into the struct.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the buffer is too short.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::UnbindRequest;
    /// # let sequence_number: u32 = 1;
    /// # let unbind_req = UnbindRequest::new(sequence_number);
    /// # let mut buffer = Vec::new();
    /// # unbind_req.encode(&mut buffer).unwrap();
    /// let decoded = UnbindRequest::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.sequence_number, 1);
    /// ```
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(12); // Skip len, id, status

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        Ok(Self { sequence_number })
    }
}

// --- Unbind Response ---
/// Represents an Unbind Response PDU.
///
/// Sent by the SMSC in response to an Unbind Request.
#[derive(Debug, Clone, PartialEq)]
pub struct UnbindResponse {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Command Status (0 = OK, others = Error)
    pub command_status: u32,
    /// Human-readable description of status
    pub status_description: String,
}

impl UnbindResponse {
    /// Create a new Unbind Response.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::UnbindResponse;
    ///
    /// let sequence_number: u32 = 1;
    /// let unbind_resp = UnbindResponse::new(sequence_number, "ESME_ROK");
    /// ```
    pub fn new(sequence_number: u32, status_name: &str) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            status_description: status_name.to_string(),
        }
    }

    /// Encode the struct into raw bytes for the network.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if an I/O error occurs while writing.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::UnbindResponse;
    /// # let sequence_number: u32 = 1;
    /// # let unbind_resp = UnbindResponse::new(sequence_number, "ESME_ROK");
    /// let mut buffer = Vec::new();
    /// unbind_resp.encode(&mut buffer).expect("Encoding failed");
    /// ```
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let command_len = HEADER_LEN as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_UNBIND_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        Ok(())
    }

    /// Decode raw bytes from the network into the struct.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the buffer is too short.
    ///
    /// # Examples
    ///
    /// ```
    /// # use smpp_codec::pdus::UnbindResponse;
    /// # let sequence_number: u32 = 1;
    /// # let unbind_resp = UnbindResponse::new(sequence_number, "ESME_ROK");
    /// # let mut buffer = Vec::new();
    /// # unbind_resp.encode(&mut buffer).unwrap();
    /// let decoded = UnbindResponse::decode(&buffer).expect("Decoding failed");
    /// assert_eq!(decoded.sequence_number, 1);
    /// ```
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);

        // Skip Length (4) + ID (4)
        cursor.set_position(8);

        let mut bytes = [0u8; 4];

        // Read Status
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);

        // Read Sequence
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        Ok(Self {
            sequence_number,
            command_status,
            status_description: get_status_description(command_status),
        })
    }
}
