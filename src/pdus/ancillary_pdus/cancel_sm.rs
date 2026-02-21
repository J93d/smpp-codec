use crate::common::{
    get_status_code, get_status_description, read_c_string, write_c_string, Npi, PduError, Ton,
    CMD_CANCEL_SM, CMD_CANCEL_SM_RESP, HEADER_LEN,
};
use std::io::{Cursor, Read, Write};

// --- Request ---
/// Represents a Cancel SM Request PDU.
#[derive(Debug, Clone, PartialEq)]
pub struct CancelSmRequest {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Service Type
    pub service_type: String,
    /// Message ID of the message to cancel
    pub message_id: String,
    /// Source Address Type of Number
    pub source_addr_ton: Ton,
    /// Source Address Numbering Plan Indicator
    pub source_addr_npi: Npi,
    /// Source Address
    pub source_addr: String,
    /// Destination Address Type of Number
    pub dest_addr_ton: Ton,
    /// Destination Address Numbering Plan Indicator
    pub dest_addr_npi: Npi,
    /// Destination Address
    pub dest_addr: String,
}

impl CancelSmRequest {
    /// Create a new Cancel SM Request.
    pub fn new(
        sequence_number: u32,
        message_id: String,
        source_addr: String,
        dest_addr: String,
    ) -> Self {
        Self {
            sequence_number,
            service_type: String::new(),
            message_id,
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr,
            dest_addr_ton: Ton::Unknown,
            dest_addr_npi: Npi::Unknown,
            dest_addr,
        }
    }

    /// Encode the PDU into the writer.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, message_id = %self.message_id, src = %self.source_addr, dst = %self.dest_addr)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding CancelSmRequest");
        // Calculate body length upfront to avoid double buffering
        let body_len = self.service_type.len() + 1 + // C-String
                       self.message_id.len() + 1 +   // C-String
                       1 + 1 +                       // source ton + npi
                       self.source_addr.len() + 1 +  // C-String
                       1 + 1 +                       // dest ton + npi
                       self.dest_addr.len() + 1; // C-String

        let command_len = (HEADER_LEN + body_len) as u32;

        writer.write_all(&command_len.to_be_bytes())?;
        writer.write_all(&CMD_CANCEL_SM.to_be_bytes())?;
        writer.write_all(&0u32.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;

        write_c_string(writer, &self.service_type)?;
        write_c_string(writer, &self.message_id)?;
        writer.write_all(&[self.source_addr_ton as u8, self.source_addr_npi as u8])?;
        write_c_string(writer, &self.source_addr)?;
        writer.write_all(&[self.dest_addr_ton as u8, self.dest_addr_npi as u8])?;
        write_c_string(writer, &self.dest_addr)?;

        Ok(())
    }

    /// Decode the PDU from the buffer.
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(12); // Skip Header

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        let service_type = read_c_string(&mut cursor)?;
        let message_id = read_c_string(&mut cursor)?;
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("message_id", &message_id);

        let mut u8_buf = [0u8; 1];
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let source_addr_npi = Npi::from(u8_buf[0]);
        let source_addr = read_c_string(&mut cursor)?;
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("src", &source_addr);

        cursor.read_exact(&mut u8_buf)?;
        let dest_addr_ton = Ton::from(u8_buf[0]);
        cursor.read_exact(&mut u8_buf)?;
        let dest_addr_npi = Npi::from(u8_buf[0]);
        let dest_addr = read_c_string(&mut cursor)?;
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("dst", &dest_addr);

        Ok(Self {
            sequence_number,
            service_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            dest_addr,
        })
    }
}

// --- Response ---
// CancelSmResp has NO BODY. It is just a header.
/// Represents a Cancel SM Response PDU.
#[derive(Debug, Clone, PartialEq)]
pub struct CancelSmResponse {
    /// Sequence number of the PDU
    pub sequence_number: u32,
    /// Command Status
    pub command_status: u32,
    /// Status Description
    pub status_description: String,
}

impl CancelSmResponse {
    /// Create a new CancelSm response.
    ///
    /// # Examples
    ///
    /// ```
    /// use smpp_codec::pdus::CancelSmResponse;
    ///
    /// let message = CancelSmResponse::new(1, "ESME_ROK");
    /// ```
    pub fn new(sequence_number: u32, status_name: &str) -> Self {
        let command_status = get_status_code(status_name);
        Self {
            sequence_number,
            command_status,
            status_description: status_name.to_string(),
        }
    }

    /// Encode the PDU into the writer.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(writer), err, fields(seq = self.sequence_number, status = %self.status_description)))]
    pub fn encode(&self, writer: &mut impl Write) -> Result<(), PduError> {
        #[cfg(feature = "tracing")]
        tracing::debug!("Encoding CancelSmResponse");
        writer.write_all(&(HEADER_LEN as u32).to_be_bytes())?;
        writer.write_all(&CMD_CANCEL_SM_RESP.to_be_bytes())?;
        writer.write_all(&self.command_status.to_be_bytes())?;
        writer.write_all(&self.sequence_number.to_be_bytes())?;
        Ok(())
    }

    /// Decode the PDU from the buffer.
    pub fn decode(buffer: &[u8]) -> Result<Self, PduError> {
        if buffer.len() < HEADER_LEN {
            return Err(PduError::BufferTooShort);
        }
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(8); // Skip Len, ID

        let mut bytes = [0u8; 4];
        cursor.read_exact(&mut bytes)?;
        let command_status = u32::from_be_bytes(bytes);
        let status_description = get_status_description(command_status);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("status", &status_description);

        cursor.read_exact(&mut bytes)?;
        let sequence_number = u32::from_be_bytes(bytes);
        #[cfg(feature = "tracing")]
        tracing::Span::current().record("seq", sequence_number);

        Ok(Self {
            sequence_number,
            command_status,
            status_description,
        })
    }
}
