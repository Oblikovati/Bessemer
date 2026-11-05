use super::*;
use std::{collections::BTreeMap, ops::Range};

pub(super) struct PageReader<'a> {
    bytes: &'a [u8],
    remaining: usize,
    pub(super) occupied: BTreeMap<usize, usize>,
}

pub(super) struct InlinePayloads {
    pub(super) key: Box<[u8]>,
    pub(super) value: Box<[u8]>,
}

pub(super) struct StoredCellHeader {
    pub(super) offset: u16,
    pub(super) hash: u32,
    pub(super) key_length: u32,
    pub(super) value_length: u64,
    pub(super) next: u16,
    pub(super) overflow: u64,
}

impl<'a> PageReader<'a> {
    pub(super) fn new(bytes: &'a [u8], remaining: usize) -> Self {
        Self {
            bytes,
            remaining,
            occupied: BTreeMap::new(),
        }
    }
    pub(super) fn take(&self, offset: usize, length: usize) -> Result<&[u8]> {
        let end = offset.checked_add(length).ok_or_else(|| {
            invalid(format!(
                "range {offset}+{length}; expected no usize overflow"
            ))
        })?;
        self.bytes.get(offset..end).ok_or_else(|| {
            invalid(format!(
                "range {offset}..{end}; expected within {} bytes",
                self.bytes.len()
            ))
        })
    }

    pub(super) fn integer<const N: usize>(&self, offset: usize) -> Result<[u8; N]> {
        self.take(offset, N)?
            .try_into()
            .map_err(|_| invalid(format!("word at {offset}; expected {N} bytes")))
    }

    pub(super) fn reserve(&mut self, range: Range<usize>) -> Result<()> {
        self.take(range.start, range.len())?;
        if range.is_empty() {
            return Ok(());
        }
        let previous = self.occupied.range(..=range.start).next_back();
        let following = self.occupied.range(range.start..).next();
        if previous.is_some_and(|(_, end)| *end > range.start)
            || following.is_some_and(|(start, _)| *start < range.end)
        {
            return Err(invalid(format!(
                "span {range:?}; expected disjoint header, free metadata and cells without cycles"
            )));
        }
        self.occupied.insert(range.start, range.end);
        Ok(())
    }

    pub(super) fn charge(&mut self, offset: u16) -> Result<()> {
        self.remaining = self.remaining.checked_sub(1).ok_or_else(|| {
            invalid(format!(
                "record at {offset}, remaining budget 0; expected at least one entry"
            ))
        })?;
        Ok(())
    }

    pub(super) fn header(&self) -> Result<HashBufferHeader> {
        Ok(HashBufferHeader {
            cell_head: u16::from_be_bytes(self.integer(0)?),
            free_head: u16::from_be_bytes(self.integer(2)?),
            next_page: u64::from_be_bytes(self.integer(4)?),
        })
    }

    pub(super) fn free_chain(&mut self, mut offset: u16) -> Result<Vec<HashFreeRecord>> {
        let mut records = Vec::new();
        while offset != 0 {
            self.charge(offset)?;
            let start = usize::from(offset);
            self.reserve(start..start + 4)?;
            let next = u16::from_be_bytes(self.integer(start)?);
            let size = u16::from_be_bytes(self.integer(start + 2)?);
            records.push(HashFreeRecord { offset, next, size });
            offset = next;
        }
        Ok(records)
    }

    pub(super) fn cell_header(&mut self, offset: u16) -> Result<StoredCellHeader> {
        let start = usize::from(offset);
        self.reserve(start..start + 26)?;
        Ok(StoredCellHeader {
            offset,
            hash: u32::from_be_bytes(self.integer(start)?),
            key_length: u32::from_be_bytes(self.integer(start + 4)?),
            value_length: u64::from_be_bytes(self.integer(start + 8)?),
            next: u16::from_be_bytes(self.integer(start + 16)?),
            overflow: u64::from_be_bytes(self.integer(start + 18)?),
        })
    }
    pub(super) fn payloads(
        &mut self,
        start: usize,
        key_length: usize,
        raw_value_length: u64,
    ) -> Result<InlinePayloads> {
        let length = usize::try_from(raw_value_length).map_err(|_| {
            invalid(format!(
                "value length {raw_value_length}; expected <= usize::MAX"
            ))
        })?;
        let key = self.take(start + 26, key_length)?.into();
        let value_start = start + 26 + key_length;
        let value = self.take(value_start, length)?.into();
        self.reserve(start + 26..value_start + length)?;
        Ok(InlinePayloads { key, value })
    }
}
