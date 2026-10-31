use crate::{Result, archive_codec::ArchiveCursor, simulation::invalid};
use std::sync::Arc;

/// Atomic bounded character-count text, retaining raw UTF-16 code units without a terminator rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostGraphText {
    payload: Arc<[u8]>,
}

impl SimulationPostGraphText {
    /// Construct complete code units without Unicode repair; e.g. `Self::from_payload(&[0, 0xd8], 2048)?`.
    pub fn from_payload(payload: &[u8], maximum_bytes: usize) -> Result<Self> {
        if !payload.len().is_multiple_of(2) || payload.len() > 2048 || payload.len() > maximum_bytes
        {
            return Err(invalid(
                "graph character text",
                format!(
                    "{} payload bytes; expected even length <= 2048 and <= {maximum_bytes}",
                    payload.len()
                ),
            ));
        }
        Ok(Self {
            payload: payload.into(),
        })
    }

    /// Borrow exact code units including NULs and lone surrogates; e.g. `text.payload()`.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// Return the stored character count without a terminator adjustment; e.g. `text.character_count()`.
    pub fn character_count(&self) -> usize {
        self.payload.len() / 2
    }

    /// Encode count and payload as one complete value; e.g. `text.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = (self.character_count() as i32).to_le_bytes().to_vec();
        bytes.extend_from_slice(self.payload());
        bytes
    }

    pub(super) fn from_frame(bytes: &[u8], maximum_bytes: usize) -> Result<Self> {
        let mut cursor = ArchiveCursor::new(bytes);
        let count = cursor.i32()?;
        if !(0..=1024).contains(&count) {
            return Err(invalid(
                "graph character count",
                format!("{count}; expected 0..=1024"),
            ));
        }
        let payload = cursor.take(count as usize * 2)?;
        cursor.finish()?;
        Self::from_payload(payload, maximum_bytes)
    }
}
