use super::malformed;
use crate::{Result, records};

pub(crate) struct ArchiveCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> ArchiveCursor<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub(crate) fn take(&mut self, size: usize) -> Result<&'a [u8]> {
        let span = records::region(self.bytes, self.offset, size).map_err(|mut error| {
            error.operation = "decode archive";
            error.entity = format!("archive at byte {}", self.offset);
            error
        })?;
        self.offset += size;
        Ok(span)
    }

    pub(crate) fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    pub(crate) fn u16(&mut self) -> Result<u16> {
        let span = self.take(2)?;
        Ok(u16::from_le_bytes([span[0], span[1]]))
    }

    pub(crate) fn u32(&mut self) -> Result<u32> {
        let span = self.take(4)?;
        Ok(u32::from_le_bytes([span[0], span[1], span[2], span[3]]))
    }

    pub(crate) fn i32(&mut self) -> Result<i32> {
        Ok(self.u32()? as i32)
    }

    pub(crate) fn u64(&mut self) -> Result<u64> {
        let low = u64::from(self.u32()?);
        Ok(low | (u64::from(self.u32()?) << 32))
    }

    pub(crate) fn collection_count(&mut self, minimum_size: usize) -> Result<usize> {
        let short = self.u16()?;
        let count = if short == u16::MAX {
            self.u32()?
        } else {
            u32::from(short)
        };
        let limit = (self.bytes.len() - self.offset) / minimum_size;
        if count as usize > limit {
            return Err(malformed(format!(
                "collection count {count}; expected 0..={limit} for remaining bytes"
            )));
        }
        Ok(count as usize)
    }

    pub(crate) fn count(&mut self, minimum_size: usize, label: &str) -> Result<usize> {
        let count = self.i32()?;
        let available = self.bytes.len() - self.offset;
        if count < 1 || count as usize > available / minimum_size {
            return Err(malformed(format!(
                "{label} count {count}; expected 1..={} in {available} bytes",
                available / minimum_size
            )));
        }
        Ok(count as usize)
    }

    pub(crate) fn finish(&self) -> Result<()> {
        if self.offset == self.bytes.len() {
            return Ok(());
        }
        Err(malformed(format!(
            "consumed {} of {} bytes; expected complete standalone archive",
            self.offset,
            self.bytes.len()
        )))
    }
}
