use crate::{Result, archive_codec::ArchiveCursor, simulation::invalid};
use std::sync::Arc;

/// Atomic nonnegative signed-byte-count frame, distinct from character text and persisted BSTR text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostDialogBytes {
    payload: Arc<[u8]>,
}

impl SimulationPostDialogBytes {
    /// Own an arbitrary bounded byte payload; e.g. `Self::from_payload(&[1, 0, 255], 1000)?`.
    pub fn from_payload(payload: &[u8], maximum: usize) -> Result<Self> {
        if payload.len() > i32::MAX as usize || payload.len() > maximum {
            return Err(invalid(
                "dialog counted bytes",
                format!(
                    "{} bytes; expected <= signed32 length and <= {maximum}",
                    payload.len()
                ),
            ));
        }
        Ok(Self {
            payload: payload.into(),
        })
    }

    /// Borrow the exact stored bytes, including empty or odd-length payloads; e.g. `frame.payload()`.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Reconstruct the byte count and its payload together; e.g. `frame.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = (self.payload.len() as i32).to_le_bytes().to_vec();
        bytes.extend_from_slice(self.payload());
        bytes
    }

    pub(super) fn from_frame(bytes: &[u8], maximum: usize) -> Result<Self> {
        let mut cursor = ArchiveCursor::new(bytes);
        let count = cursor.i32()?;
        let count = usize::try_from(count).map_err(|_| {
            invalid(
                "dialog counted bytes",
                format!("{count}; expected nonnegative signed32 byte count"),
            )
        })?;
        let payload = cursor.take(count)?;
        cursor.finish()?;
        Self::from_payload(payload, maximum)
    }
}
