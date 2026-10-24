use super::invalid;
use crate::Result;
use crate::archive_codec::{ArchiveString, HeaderBox};

pub(super) struct FacetOutput {
    bytes: Option<Vec<u8>>,
}

impl FacetOutput {
    pub fn encoded() -> Self {
        Self {
            bytes: Some(Vec::new()),
        }
    }
    pub fn validation() -> Self {
        Self { bytes: None }
    }
    pub fn finish(self) -> Vec<u8> {
        self.bytes.unwrap_or_default()
    }

    pub fn raw(&mut self, bytes: &[u8]) {
        if let Some(output) = &mut self.bytes {
            output.extend_from_slice(bytes);
        }
    }
    pub fn u8(&mut self, value: u8) {
        self.raw(&[value]);
    }
    pub fn u32(&mut self, value: u32) {
        self.raw(&value.to_le_bytes());
    }
    pub fn i32(&mut self, value: i32) {
        self.u32(value as u32);
    }
    pub fn u64(&mut self, value: u64) {
        self.raw(&value.to_le_bytes());
    }
    pub fn high_words(&mut self, value: u64) {
        self.u32((value >> 32) as u32);
        self.u32(value as u32);
    }

    pub fn text(&mut self, text: &ArchiveString) -> Result<()> {
        if let Some(bytes) = &mut self.bytes {
            text.encode(bytes)?;
        }
        Ok(())
    }
    pub fn bounds(&mut self, bounds: &HeaderBox) {
        if let Some(bytes) = &mut self.bytes {
            crate::archive_codec::write_box(bounds, bytes);
        }
    }
    pub fn count(&mut self, count: usize) -> Result<()> {
        let stored = i32::try_from(count).map_err(|_| {
            invalid(format!(
                "collection length {count}; expected 0..={}",
                i32::MAX
            ))
        })?;
        self.i32(stored);
        Ok(())
    }
}
