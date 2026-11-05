use super::*;
use std::{collections::BTreeMap, ops::Range};

struct PageReader<'a> {
    bytes: &'a [u8],
    remaining: usize,
    occupied: BTreeMap<usize, usize>,
}

struct InlinePayloads {
    key: Box<[u8]>,
    value: Box<[u8]>,
}

pub(super) fn buffer(bytes: &[u8], context: HashBufferContext) -> Result<HashPageBuffer> {
    check_extent(bytes.len(), context)?;
    let mut reader = PageReader::new(bytes, context.maximum_entries);
    reader.reserve(0..12)?;
    let header = reader.header()?;
    let free = reader.free_chain(header.free_head)?;
    let cells = reader.cell_chain(header.cell_head)?;
    let coverage = buffer_coverage(bytes.len(), &reader.occupied);
    Ok(HashPageBuffer {
        context,
        bytes: bytes.into(),
        header,
        free,
        cells,
        coverage,
    })
}

fn buffer_coverage(length: usize, occupied: &BTreeMap<usize, usize>) -> HashBufferCoverage {
    let interpreted = occupied.iter().map(|(start, end)| end - start).sum();
    HashBufferCoverage {
        mode: HashBufferCheckMode::ExactInlineBuffer,
        buffer_bytes: length,
        interpreted_bytes: interpreted,
        uninterpreted_bytes: length - interpreted,
        achieved_byte_error: 0,
    }
}

fn check_extent(length: usize, context: HashBufferContext) -> Result<()> {
    if length < 12 || length != context.engine_extent || length > context.maximum_bytes {
        return Err(invalid(format!(
            "buffer length {length}, engine extent {}, budget {}; expected equal extent >= 12 and within budget",
            context.engine_extent, context.maximum_bytes
        )));
    }
    Ok(())
}

impl<'a> PageReader<'a> {
    fn new(bytes: &'a [u8], remaining: usize) -> Self {
        Self {
            bytes,
            remaining,
            occupied: BTreeMap::new(),
        }
    }
    fn take(&self, offset: usize, length: usize) -> Result<&[u8]> {
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

    fn integer<const N: usize>(&self, offset: usize) -> Result<[u8; N]> {
        self.take(offset, N)?
            .try_into()
            .map_err(|_| invalid(format!("word at {offset}; expected {N} bytes")))
    }

    fn reserve(&mut self, range: Range<usize>) -> Result<()> {
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

    fn charge(&mut self, offset: u16) -> Result<()> {
        self.remaining = self.remaining.checked_sub(1).ok_or_else(|| {
            invalid(format!(
                "record at {offset}, remaining budget 0; expected at least one entry"
            ))
        })?;
        Ok(())
    }

    fn header(&self) -> Result<HashBufferHeader> {
        Ok(HashBufferHeader {
            cell_head: u16::from_be_bytes(self.integer(0)?),
            free_head: u16::from_be_bytes(self.integer(2)?),
            next_page: u64::from_be_bytes(self.integer(4)?),
        })
    }

    fn free_chain(&mut self, mut offset: u16) -> Result<Vec<HashFreeRecord>> {
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

    fn cell_chain(&mut self, mut offset: u16) -> Result<Vec<HashInlineCell>> {
        let mut cells = Vec::new();
        while offset != 0 {
            self.charge(offset)?;
            let cell = self.inline_cell(offset)?;
            offset = cell.next;
            cells.push(cell);
        }
        Ok(cells)
    }

    fn inline_cell(&mut self, offset: u16) -> Result<HashInlineCell> {
        let start = usize::from(offset);
        self.reserve(start..start + 26)?;
        let hash = u32::from_be_bytes(self.integer(start)?);
        let key_length = u32::from_be_bytes(self.integer(start + 4)?) as usize;
        let raw_value_length = u64::from_be_bytes(self.integer(start + 8)?);
        let next = u16::from_be_bytes(self.integer(start + 16)?);
        self.require_inline(start)?;
        let payload = self.payloads(start, key_length, raw_value_length)?;
        Ok(HashInlineCell {
            offset,
            hash,
            next,
            key: payload.key,
            value: payload.value,
        })
    }

    fn payloads(
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

    fn require_inline(&self, start: usize) -> Result<()> {
        let overflow = u64::from_be_bytes(self.integer(start + 18)?);
        if overflow == 0 {
            return Ok(());
        }
        Err(BessemerError::new(
            "database hash buffer",
            "inline profile",
            FailureClass::Unsupported,
            format!(
                "overflow identifier {overflow} at {}; expected zero for complete inline payloads",
                start + 18
            ),
        ))
    }
}
