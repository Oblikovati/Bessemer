use super::*;
use std::collections::BTreeSet;

struct ChainReader<'a> {
    pages: &'a [OverflowPageImage],
    current: usize,
    cursor: usize,
    visited: BTreeSet<NonZeroU64>,
}

pub(super) fn payload(
    pages: &[OverflowPageImage],
    first: NonZeroU64,
    key_length: u32,
    value_length: u64,
    context: OverflowStorageContext,
) -> Result<OverflowPayload> {
    let (key_count, value_count, total) = check_inputs(pages, key_length, value_length, context)?;
    let mut owned = pages.to_vec();
    owned.sort_by_key(|page| page.identifier);
    check_unique(&owned)?;
    let mut reader = ChainReader::new(&owned, first)?;
    let (key, _) = reader.take(key_count)?;
    reader.check_value_start(first)?;
    let (value, value_spans) = reader.take(value_count)?;
    reader.finish()?;
    let coverage = coverage(total, key_count + value_count, owned.len());
    Ok(OverflowPayload {
        context,
        first,
        pages: owned,
        key: key.into(),
        value: value.into(),
        value_spans,
        coverage,
    })
}

fn check_inputs(
    pages: &[OverflowPageImage],
    key: u32,
    value: u64,
    context: OverflowStorageContext,
) -> Result<(usize, usize, usize)> {
    let total = supplied_bytes(pages, context)?;
    let capacity = total - 8 * pages.len() - 10;
    let count = u64::from(key).checked_add(value);
    if !count.is_some_and(|count| {
        count <= context.maximum_payload_bytes as u64 && count <= capacity as u64
    }) {
        return Err(invalid(
            "payload lengths",
            format!(
                "key {key}, value {value}; expected sum <= {} and physical capacity {capacity}",
                context.maximum_payload_bytes
            ),
        ));
    }
    Ok((key as usize, value as usize, total))
}

fn supplied_bytes(pages: &[OverflowPageImage], context: OverflowStorageContext) -> Result<usize> {
    check_extent_count(pages.len(), context)?;
    let total = pages
        .len()
        .checked_mul(context.engine_extent)
        .filter(|total| *total <= context.maximum_bytes)
        .ok_or_else(|| {
            invalid(
                "page set",
                format!(
                    "pages {}, extent {}; expected addressable aggregate <= {}",
                    pages.len(),
                    context.engine_extent,
                    context.maximum_bytes
                ),
            )
        })?;
    check_page_lengths(pages, context.engine_extent)?;
    Ok(total)
}

fn check_extent_count(count: usize, context: OverflowStorageContext) -> Result<()> {
    if !(512..=65536).contains(&context.engine_extent)
        || count == 0
        || count > context.maximum_pages
    {
        return Err(invalid(
            "page set",
            format!(
                "extent {}, pages {count}; expected extent512..=65536 and1..={} pages",
                context.engine_extent, context.maximum_pages
            ),
        ));
    }
    Ok(())
}

fn check_page_lengths(pages: &[OverflowPageImage], extent: usize) -> Result<()> {
    if let Some(page) = pages.iter().find(|page| page.bytes.len() != extent) {
        return Err(invalid(
            page.identifier,
            format!(
                "page length {}; expected complete extent {extent}",
                page.bytes.len()
            ),
        ));
    }
    Ok(())
}

fn check_unique(pages: &[OverflowPageImage]) -> Result<()> {
    if let Some(pair) = pages
        .windows(2)
        .find(|pair| pair[0].identifier == pair[1].identifier)
    {
        return Err(invalid(
            pair[0].identifier,
            "duplicate page identity; expected one complete image per ID",
        ));
    }
    Ok(())
}

fn coverage(total: usize, payload: usize, pages: usize) -> OverflowCoverage {
    let interpreted = 8 * pages + 10 + payload;
    OverflowCoverage {
        mode: OverflowCheckMode::ExactSuppliedWriterChain,
        buffer_bytes: total,
        interpreted_bytes: interpreted,
        uninterpreted_bytes: total - interpreted,
        pages,
        achieved_byte_error: 0,
    }
}

impl<'a> ChainReader<'a> {
    fn new(pages: &'a [OverflowPageImage], first: NonZeroU64) -> Result<Self> {
        let current = locate(pages, first)?;
        Ok(Self {
            pages,
            current,
            cursor: 18,
            visited: BTreeSet::from([first]),
        })
    }
    fn take(&mut self, length: usize) -> Result<(Vec<u8>, Vec<types::PayloadSpan>)> {
        let mut bytes = Vec::with_capacity(length);
        let mut spans = Vec::new();
        if length == 0 {
            spans.push(self.span(self.cursor));
        }
        while bytes.len() < length {
            self.advance_if_full()?;
            let page = &self.pages[self.current];
            let end = self.cursor + (length - bytes.len()).min(page.bytes.len() - self.cursor);
            bytes.extend_from_slice(&page.bytes[self.cursor..end]);
            spans.push(self.span(end));
            self.cursor = end;
        }
        Ok((bytes, spans))
    }
    fn span(&self, end: usize) -> types::PayloadSpan {
        types::PayloadSpan {
            page: self.pages[self.current].identifier,
            range: self.cursor..end,
        }
    }
    fn advance_if_full(&mut self) -> Result<()> {
        if self.cursor < self.pages[self.current].bytes.len() {
            return Ok(());
        }
        let current = &self.pages[self.current];
        let raw = read_u64(&current.bytes[..8]);
        let next = NonZeroU64::new(raw).ok_or_else(|| {
            invalid(
                current.identifier,
                "next0 before complete payload; expected supplied continuation page",
            )
        })?;
        if !self.visited.insert(next) {
            return Err(invalid(
                next,
                "cyclic page link; expected each page visited once",
            ));
        }
        self.current = locate(self.pages, next)?;
        self.cursor = 8;
        Ok(())
    }
    fn check_value_start(&self, first: NonZeroU64) -> Result<()> {
        let bytes = &self.pages[locate(self.pages, first)?].bytes;
        let page = read_u64(&bytes[8..16]);
        let cursor = usize::from(u16::from_be_bytes([bytes[16], bytes[17]]));
        let current = self.pages[self.current].identifier;
        if page != current.get() || cursor != self.cursor {
            return Err(invalid(
                first,
                format!(
                    "value checkpoint {page}:{cursor}; expected exact key end {current}:{} without cursor truncation",
                    self.cursor
                ),
            ));
        }
        Ok(())
    }
    fn finish(&self) -> Result<()> {
        let current = &self.pages[self.current];
        let following = read_u64(&current.bytes[..8]);
        if following != 0 || self.visited.len() != self.pages.len() {
            return Err(invalid(
                current.identifier,
                format!(
                    "next {following}, visited {}, supplied {}; expected terminal0 and all supplied pages visited",
                    self.visited.len(),
                    self.pages.len()
                ),
            ));
        }
        Ok(())
    }
}

fn locate(pages: &[OverflowPageImage], identifier: NonZeroU64) -> Result<usize> {
    pages
        .binary_search_by_key(&identifier, |page| page.identifier)
        .map_err(|_| {
            BessemerError::new(
                "database overflow",
                &identifier.to_string(),
                FailureClass::Missing,
                format!("page {identifier}; expected a supplied complete page image"),
            )
        })
}

fn read_u64(bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .fold(0u64, |value, byte| (value << 8) | u64::from(*byte))
}
