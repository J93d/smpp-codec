use crate::common::{
    get_status_code, get_status_description, read_c_string, write_c_string, Npi, PduError, Ton,
    CMD_SUBMIT_MULTI_SM_RESP, HEADER_LEN,
};

use std::io::{Cursor, Read, Write};

/// Represents an unsuccessful SME in a Submit Multi Response.
#[derive(Debug, Clone, PartialEq)]
pub struct UnsuccessfulDelivery {
    pub ton: Ton,
    pub npi: Npi,
    pub address: String,
    pub error_status: u32,
}

/// Represents a Submit Multi Response PDU.
///
/// Sent by the SMSC in response to a Submit Multi Request.
#[derive(Debug, Clone, PartialEq)]
pub struct SubmitMultiResp {
    pub sequence_number: u32,
    pub command_status: u32,        // 0 = OK, others = Error
    pub status_description: String, // Human-readable description of status
    pub message_id: String,
    pub unsuccess_smes: Vec<UnsuccessfulDelivery>,
}

impl SubmitMultiResp {
    /// Create a new Submit Multi Response.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::{SubmitMultiResp, UnsuccessfulDelivery};
    ///
    /// let resp = SubmitMultiResp::new(
    ///     1,
    ///     "ESME_ROK",
    ///     "MessageID".to_string(),
    ///     vec![],
    /// );
    /// ```
    pub fn new(
        sequence_number: u32,
        status_name: &str,
        message_id: String,
        unsuccess_smes: Vec<UnsuccessfulDelivery>,
    ) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            status_description: status_name.to_string(),
            message_id,
            unsuccess_smes,
        }
    }

    /// Encode the PDU into the writer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the write fails.
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        let mut body = Vec::new();

        // Body is only present if we have a Message ID to return
        // (Usually on success 0x00 or partial success 0x405)
        if self.command_status == 0 || self.command_status == 0x00000405 {
            write_c_string(&mut body, &self.message_id)?;
            body.write_all(&[self.unsuccess_smes.len() as u8])?;

            for sme in &self.unsuccess_smes {
                body.write_all(&[sme.ton as u8, sme.npi as u8])?;
                write_c_string(&mut body, &sme.address)?;
                body.write_all(&sme.error_status.to_be_bytes())?;
            }
        }

        let command_len = (HEADER_LEN + body.len()) as u32;
        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_SUBMIT_MULTI_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        writer.write_all(&body)?;

        Ok(())
    }

    /// Decode the PDU from the buffer.
    ///
    /// # Errors
    ///
    /// Returns a [`PduError`] if the buffer is too short or malformed.
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(8);

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);

        let mut message_id = String::new();
        let mut unsuccess_smes = Vec::new();

        if command_status == 0 || command_status == 0x00000405 {
            // Partial success
            message_id = read_c_string(&mut cursor)?;
            let mut u8_buf = [0u8; 1];
            cursor.read_exact(&mut u8_buf)?;
            let count = u8_buf[0];

            for _ in 0..count {
                cursor.read_exact(&mut u8_buf)?;
                let ton = Ton::from(u8_buf[0]);
                cursor.read_exact(&mut u8_buf)?;
                let npi = Npi::from(u8_buf[0]);
                let address = read_c_string(&mut cursor)?;

                let mut err_bytes = [0u8; 4];
                cursor.read_exact(&mut err_bytes)?;
                let error_status = u32::from_be_bytes(err_bytes);

                unsuccess_smes.push(UnsuccessfulDelivery {
                    ton,
                    npi,
                    address,
                    error_status,
                });
            }
        }

        let status_description = get_status_description(command_status);

        Ok(Self {
            sequence_number,
            command_status,
            status_description,
            message_id,
            unsuccess_smes,
        })
    }
}
