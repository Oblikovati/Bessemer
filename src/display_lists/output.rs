use super::view_support::invalid;
use super::{CompactTransform, StoredDouble, transform_codec};
use crate::{Result, archive_codec::ArchiveString};

pub(super) struct DisplayOutput {
    bytes: Vec<u8>,
    maximum: usize,
}

impl DisplayOutput {
    pub fn new(maximum: usize) -> Self {
        Self {
            bytes: Vec::new(),
            maximum,
        }
    }
    pub fn finish(self) -> Vec<u8> {
        self.bytes
    }

    pub fn remaining(&self) -> usize {
        self.maximum.saturating_sub(self.bytes.len())
    }

    fn reserve(&self, additional: usize) -> Result<()> {
        let length = self.bytes.len().checked_add(additional).ok_or_else(|| {
            invalid(format!(
                "encoded display bytes {} + {additional}; expected usize",
                self.bytes.len()
            ))
        })?;
        super::view_support::require_size(length, self.maximum, "encoded display bytes")
    }

    fn append(&mut self, bytes: &[u8]) -> Result<()> {
        self.reserve(bytes.len())?;
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }

    pub fn u16(&mut self, value: u16) -> Result<()> {
        self.append(&value.to_le_bytes())
    }
    pub fn bytes(&mut self, values: &[u8]) -> Result<()> {
        self.append(values)
    }

    pub fn i32(&mut self, value: i32) -> Result<()> {
        self.append(&value.to_le_bytes())
    }
    pub fn u32(&mut self, value: u32) -> Result<()> {
        self.append(&value.to_le_bytes())
    }
    pub fn double(&mut self, value: StoredDouble) -> Result<()> {
        self.append(&value.bits().to_le_bytes())
    }

    pub fn point(&mut self, values: &[StoredDouble; 3]) -> Result<()> {
        for value in values {
            self.double(*value)?;
        }
        Ok(())
    }

    pub fn string(&mut self, value: &ArchiveString) -> Result<()> {
        self.reserve(value.encoded_size()?)?;
        value.encode(&mut self.bytes)
    }

    pub fn compact(&mut self, value: &CompactTransform) -> Result<()> {
        transform_codec::validate_compact(value)?;
        let mut bytes = Vec::new();
        transform_codec::write_compact(value, &mut bytes);
        self.append(&bytes)
    }

    pub fn count(&mut self, count: usize, label: &str) -> Result<()> {
        let count = i32::try_from(count)
            .map_err(|_| invalid(format!("{label} count {count}; expected nonnegative i32")))?;
        self.i32(count)
    }
}
