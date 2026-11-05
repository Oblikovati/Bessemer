use super::super::page_reader::{PageReader, StoredCellHeader};
use super::*;
use std::collections::{BTreeMap, BTreeSet};

struct OverflowInputs<'a> {
    pages: BTreeMap<NonZeroU64, &'a OverflowPageImage>,
    used: BTreeSet<NonZeroU64>,
    remaining_payload: usize,
    context: OverflowStorageContext,
}

struct BucketParts {
    header: HashBufferHeader,
    free: Vec<HashFreeRecord>,
    cells: Vec<HashWriterCell>,
    interpreted: usize,
}

pub(super) fn bucket(
    bytes: &[u8],
    pages: &[OverflowPageImage],
    context: OverflowStorageContext,
    maximum_entries: usize,
) -> Result<HashWriterBucket> {
    let total = check_images(bytes, pages, context)?;
    let parts = read_parts(bytes, pages, context, maximum_entries)?;
    let coverage = bucket_coverage(&parts, total, pages.len() + 1);
    Ok(HashWriterBucket {
        context,
        maximum_entries,
        bytes: bytes.into(),
        header: parts.header,
        free: parts.free,
        cells: parts.cells,
        coverage,
    })
}

fn read_parts(
    bytes: &[u8],
    pages: &[OverflowPageImage],
    context: OverflowStorageContext,
    maximum_entries: usize,
) -> Result<BucketParts> {
    let mut inputs = OverflowInputs::new(pages, context)?;
    let mut reader = PageReader::new(bytes, maximum_entries);
    reader.reserve(0..12)?;
    let header = reader.header()?;
    let free = reader.free_chain(header.free_head)?;
    let cells = read_cells(&mut reader, &mut inputs, header.cell_head)?;
    inputs.finish(pages.len())?;
    let interpreted =
        reader.occupied.iter().map(|(a, b)| b - a).sum::<usize>() + overflow_interpreted(&cells);
    Ok(BucketParts {
        header,
        free,
        cells,
        interpreted,
    })
}

fn bucket_coverage(parts: &BucketParts, total: usize, pages: usize) -> HashWriterCoverage {
    HashWriterCoverage {
        mode: HashWriterCheckMode::ExactSuppliedWriterBucket,
        buffer_bytes: total,
        interpreted_bytes: parts.interpreted,
        uninterpreted_bytes: total - parts.interpreted,
        pages,
        cells: parts.cells.len(),
        achieved_byte_error: 0,
    }
}

fn check_images(
    bytes: &[u8],
    pages: &[OverflowPageImage],
    context: OverflowStorageContext,
) -> Result<usize> {
    let extent = context.engine_extent;
    let count = pages
        .len()
        .checked_add(1)
        .filter(|n| *n <= context.maximum_pages);
    let total = count
        .and_then(|n| n.checked_mul(extent))
        .filter(|n| *n <= context.maximum_bytes);
    if !(512..=65536).contains(&extent) || bytes.len() != extent || total.is_none() {
        return Err(invalid(format!(
            "bucket bytes {}, extent {extent}, overflow pages {}; expected extent512..=65536, equal images and aggregate within {} bytes/{} pages",
            bytes.len(),
            pages.len(),
            context.maximum_bytes,
            context.maximum_pages
        )));
    }
    check_page_extents(pages, extent)?;
    total.ok_or_else(|| invalid("aggregate unavailable; expected bounded complete supplied images"))
}

fn check_page_extents(pages: &[OverflowPageImage], extent: usize) -> Result<()> {
    if let Some(page) = pages.iter().find(|p| p.bytes.len() != extent) {
        return Err(invalid(format!(
            "page {} length {}; expected {extent}",
            page.identifier,
            page.bytes.len()
        )));
    }
    Ok(())
}

fn overflow_interpreted(cells: &[HashWriterCell]) -> usize {
    cells
        .iter()
        .map(|cell| match &cell.payload {
            WriterPayload::Inline(_) => 0,
            WriterPayload::Overflow { payload, .. } => payload.coverage().interpreted_bytes,
        })
        .sum()
}

fn read_cells(
    reader: &mut PageReader<'_>,
    inputs: &mut OverflowInputs<'_>,
    mut offset: u16,
) -> Result<Vec<HashWriterCell>> {
    let mut cells = Vec::new();
    while offset != 0 {
        reader.charge(offset)?;
        let header = reader.cell_header(offset)?;
        inputs.charge_payload(&header)?;
        offset = header.next;
        let payload = read_payload(reader, inputs, &header)?;
        cells.push(HashWriterCell { header, payload });
    }
    Ok(cells)
}

fn read_payload(
    reader: &mut PageReader<'_>,
    inputs: &mut OverflowInputs<'_>,
    header: &StoredCellHeader,
) -> Result<WriterPayload> {
    let Some(first) = NonZeroU64::new(header.overflow) else {
        return Ok(WriterPayload::Inline(reader.payloads(
            usize::from(header.offset),
            header.key_length as usize,
            header.value_length,
        )?));
    };
    let pages = inputs.chain(first)?;
    let payload = OverflowPayload::from_pages(
        &pages,
        first,
        header.key_length,
        header.value_length,
        inputs.context,
    )?;
    Ok(WriterPayload::Overflow { first, payload })
}

impl<'a> OverflowInputs<'a> {
    fn new(pages: &'a [OverflowPageImage], context: OverflowStorageContext) -> Result<Self> {
        let mut lookup = BTreeMap::new();
        for page in pages {
            if lookup.insert(page.identifier, page).is_some() {
                return Err(invalid(format!(
                    "duplicate page {}; expected unique supplied IDs",
                    page.identifier
                )));
            }
        }
        Ok(Self {
            pages: lookup,
            used: BTreeSet::new(),
            remaining_payload: context.maximum_payload_bytes,
            context,
        })
    }
    fn charge_payload(&mut self, header: &StoredCellHeader) -> Result<()> {
        let count = u64::from(header.key_length)
            .checked_add(header.value_length)
            .and_then(|n| usize::try_from(n).ok());
        self.remaining_payload = count
            .and_then(|n| self.remaining_payload.checked_sub(n))
            .ok_or_else(|| {
                invalid(format!(
                    "cell {} key {}, value {}; expected sum within remaining payload budget {}",
                    header.offset, header.key_length, header.value_length, self.remaining_payload
                ))
            })?;
        Ok(())
    }
    fn finish(&self, expected: usize) -> Result<()> {
        if self.used.len() != expected {
            return Err(invalid(format!(
                "used {}, supplied {expected} overflow pages; expected exact ownership",
                self.used.len()
            )));
        }
        Ok(())
    }
    fn page(&self, identifier: NonZeroU64) -> Result<&'a OverflowPageImage> {
        self.pages.get(&identifier).copied().ok_or_else(|| {
            BessemerError::new(
                "database hash writer",
                &identifier.to_string(),
                FailureClass::Missing,
                format!("page {identifier}; expected supplied complete overflow image"),
            )
        })
    }
    fn chain(&mut self, first: NonZeroU64) -> Result<Vec<OverflowPageImage>> {
        let mut next = Some(first);
        let mut images = Vec::new();
        while let Some(identifier) = next {
            if !self.used.insert(identifier) {
                return Err(invalid(format!(
                    "page {identifier} reused; expected disjoint acyclic cell ownership"
                )));
            }
            let page = self.page(identifier)?;
            next = NonZeroU64::new(
                page.bytes[..8]
                    .iter()
                    .fold(0u64, |n, b| (n << 8) | u64::from(*b)),
            );
            images.push(page.clone());
        }
        Ok(images)
    }
}
