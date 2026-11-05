use super::page_reader::PageReader;
use super::*;
use std::collections::BTreeMap;

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

impl PageReader<'_> {
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
        let header = self.cell_header(offset)?;
        let start = usize::from(header.offset);
        self.require_inline(start, header.overflow)?;
        let payload = self.payloads(start, header.key_length as usize, header.value_length)?;
        Ok(HashInlineCell {
            offset: header.offset,
            hash: header.hash,
            next: header.next,
            key: payload.key,
            value: payload.value,
        })
    }

    fn require_inline(&self, start: usize, overflow: u64) -> Result<()> {
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
