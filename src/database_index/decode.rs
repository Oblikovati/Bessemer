use super::index::{FileReader, integer};
use super::*;
use crate::database_header::{DatabaseHeaderBuffer, DatabaseHeaderProfile};
use crate::database_overflow::{OverflowPageImage, OverflowStorageContext, OverflowStorageProfile};
use std::{collections::BTreeSet, num::NonZeroU64};

pub(super) fn file(bytes: &[u8], context: HashFileContext) -> Result<HashFileGraph> {
    let mut reader = new_reader(bytes, context)?;
    reader.claim(0, HashFilePageRole::Header)?;
    let index = index::read(&mut reader)?;
    let buckets = read_buckets(&mut reader, &index)?;
    read_free(&mut reader, index.free_head)?;
    let coverage = coverage(&reader, &buckets);
    let roles = complete_roles(reader.roles)?;
    Ok(HashFileGraph {
        context,
        bytes: bytes.into(),
        index,
        roles,
        buckets,
        coverage,
    })
}

fn new_reader(bytes: &[u8], context: HashFileContext) -> Result<FileReader<'_>> {
    check_extent(bytes, context)?;
    Ok(FileReader {
        bytes,
        context,
        roles: vec![None; bytes.len() / context.pager_extent],
        remaining_records: context.maximum_records,
        remaining_payload: context.maximum_payload_bytes,
        interpreted: check_header(bytes, context)?,
    })
}

fn check_extent(bytes: &[u8], context: HashFileContext) -> Result<()> {
    let extent = context.pager_extent;
    if !(512..=65536).contains(&extent) || context.engine_extent != extent {
        return Err(invalid(
            "extents",
            format!(
                "pager {extent}, engine {}; expected equal explicit extents in 512..=65536",
                context.engine_extent
            ),
        ));
    }
    check_length(bytes, context)
}

fn check_length(bytes: &[u8], context: HashFileContext) -> Result<()> {
    let extent = context.pager_extent;
    if bytes.len() < extent * 2 || !bytes.len().is_multiple_of(extent) {
        return Err(invalid(
            "file",
            format!(
                "bytes {}; expected >=2 complete {extent}-byte pages",
                bytes.len()
            ),
        ));
    }
    check_limits(bytes, context)
}

fn check_limits(bytes: &[u8], context: HashFileContext) -> Result<()> {
    if bytes.len() > context.maximum_bytes
        || bytes.len() / context.pager_extent > context.maximum_pages
    {
        return Err(invalid(
            "file",
            format!(
                "bytes {}, pages {}; expected <= {} bytes/{} pages",
                bytes.len(),
                bytes.len() / context.pager_extent,
                context.maximum_bytes,
                context.maximum_pages
            ),
        ));
    }
    Ok(())
}

fn check_header(bytes: &[u8], context: HashFileContext) -> Result<usize> {
    let header =
        DatabaseHeaderBuffer::from_bytes(&bytes[..512], DatabaseHeaderProfile::DefaultWindows512)?;
    let fields = header.fields();
    if fields.page_size as usize != context.pager_extent || fields.engine_name != b"hash" {
        return Err(invalid(
            "header",
            format!(
                "P {}, engine {:02x?}; expected explicit pager {} and exact name hash",
                fields.page_size, fields.engine_name, context.pager_extent
            ),
        ));
    }
    Ok(header.interpreted_prefix_length())
}

fn read_buckets(
    reader: &mut FileReader<'_>,
    index: &index::FileIndex,
) -> Result<BTreeMap<u64, HashWriterBucket>> {
    let mut buckets = BTreeMap::new();
    for entry in &index.entries {
        let mut current = entry.physical;
        if current == 0 {
            return Err(invalid(
                entry.logical,
                "root page 0; expected nonzero complete bucket",
            ));
        }
        while current != 0 {
            reader.claim(current, HashFilePageRole::Bucket(entry.logical))?;
            let bucket = read_bucket(reader, current)?;
            check_hashes(&bucket, entry.logical, index, current)?;
            let next = bucket.header().next_page;
            buckets.insert(current, bucket);
            current = next;
        }
    }
    Ok(buckets)
}

fn read_bucket(reader: &mut FileReader<'_>, physical: u64) -> Result<HashWriterBucket> {
    let pages = collect_overflow(reader, physical)?;
    let context = OverflowStorageContext {
        profile: OverflowStorageProfile::InstalledWriter,
        engine_extent: reader.context.engine_extent,
        maximum_bytes: reader.context.maximum_bytes,
        maximum_pages: reader.context.maximum_pages,
        maximum_payload_bytes: reader.remaining_payload,
    };
    let bucket = HashWriterBucket::from_bytes(
        reader.page(physical)?,
        &pages,
        context,
        reader.remaining_records,
    )
    .map_err(|error| qualify_bucket_error(physical, error))?;
    charge_bucket(reader, &bucket)?;
    Ok(bucket)
}

fn charge_bucket(reader: &mut FileReader<'_>, bucket: &HashWriterBucket) -> Result<()> {
    reader.charge_records(bucket.cells().len() + bucket.free_records().len())?;
    for cell in bucket.cells() {
        reader.charge_payload(cell.key().len())?;
        reader.charge_payload(cell.value().len())?;
    }
    reader.interpreted += bucket.coverage().interpreted_bytes;
    Ok(())
}

fn collect_overflow(reader: &mut FileReader<'_>, physical: u64) -> Result<Vec<OverflowPageImage>> {
    let mut offset = u16::from_be_bytes(integer(reader.page(physical)?, 0)?);
    let mut visited = BTreeSet::new();
    let mut pages = Vec::new();
    while offset != 0 {
        claim_cell(&mut visited, offset, reader.remaining_records, physical)?;
        let page = reader.page(physical)?;
        let next = u16::from_be_bytes(integer(page, usize::from(offset) + 16)?);
        let first = u64::from_be_bytes(integer(page, usize::from(offset) + 18)?);
        append_overflow(reader, first, &mut pages)?;
        offset = next;
    }
    Ok(pages)
}

fn claim_cell(
    visited: &mut BTreeSet<u16>,
    offset: u16,
    maximum: usize,
    physical: u64,
) -> Result<()> {
    if visited.len() >= maximum || !visited.insert(offset) {
        return Err(invalid(
            physical,
            format!(
                "cell offset {offset}, visits {}; expected acyclic within {maximum} remaining records",
                visited.len()
            ),
        ));
    }
    Ok(())
}
fn append_overflow(
    reader: &mut FileReader<'_>,
    first: u64,
    pages: &mut Vec<OverflowPageImage>,
) -> Result<()> {
    let mut current = first;
    while current != 0 {
        reader.claim(current, HashFilePageRole::Overflow)?;
        let page = reader.page(current)?;
        let next = u64::from_be_bytes(integer(page, 0)?);
        let identifier = NonZeroU64::new(current)
            .ok_or_else(|| invalid(current, "overflow page 0; expected positive identity"))?;
        pages.push(OverflowPageImage {
            identifier,
            bytes: page.to_vec(),
        });
        current = next;
    }
    Ok(())
}

fn check_hashes(
    bucket: &HashWriterBucket,
    logical: u64,
    index: &index::FileIndex,
    physical: u64,
) -> Result<()> {
    for cell in bucket.cells() {
        let hash = index::default_hash(cell.key());
        let requested = index::requested_bucket(hash, index);
        if hash != cell.key_hash() || requested != logical {
            return Err(invalid(
                physical,
                format!(
                    "cell {} stored hash {:#x}, logical {logical}; expected hash {hash:#x}, request {requested}",
                    cell.offset(),
                    cell.key_hash()
                ),
            ));
        }
    }
    Ok(())
}

fn read_free(reader: &mut FileReader<'_>, first: u64) -> Result<()> {
    let mut current = first;
    while current != 0 {
        reader.claim(current, HashFilePageRole::Free)?;
        current = u64::from_be_bytes(integer(reader.page(current)?, 0)?);
        reader.interpreted += 8;
    }
    Ok(())
}

fn complete_roles(roles: Vec<Option<HashFilePageRole>>) -> Result<Vec<HashFilePageRole>> {
    roles
        .into_iter()
        .enumerate()
        .map(|(identity, role)| {
            role.ok_or_else(|| {
                invalid(
                    identity,
                    "unclassified page; expected exactly one reachable role",
                )
            })
        })
        .collect()
}

fn coverage(
    reader: &FileReader<'_>,
    buckets: &BTreeMap<u64, HashWriterBucket>,
) -> HashFileCoverage {
    HashFileCoverage {
        mode: HashFileCheckMode::ExactSuppliedFileGraph,
        buffer_bytes: reader.bytes.len(),
        interpreted_bytes: reader.interpreted,
        uninterpreted_bytes: reader.bytes.len() - reader.interpreted,
        pages: reader.roles.len(),
        cells: buckets.values().map(|bucket| bucket.cells().len()).sum(),
        achieved_byte_error: 0,
    }
}
