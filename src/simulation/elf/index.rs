use super::{Result, SimulationElfScalarRecord, require, wire};
use crate::archive_codec::ArchiveRevision;
use crate::records::{dword, region};
use crate::simulation::invalid;
use std::sync::Arc;

/// Explicit budgets for complete source bytes and selected record visits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElfIndexLimits {
    /// Maximum wrapped source bytes retained, including unvisited content.
    pub maximum_file_bytes: usize,
    /// Maximum records actually visited; a negative identifier can stop before the declared count.
    pub maximum_visited_records: usize,
}

/// Validated one-based selector in the observed nominal 100-slot directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElfSelector(u8);

impl SimulationElfSelector {
    /// Validate a one-based selector once, e.g. `SimulationElfSelector::new(100)?`.
    pub fn new(one_based: usize) -> Result<Self> {
        require(
            (1..=100).contains(&one_based),
            "ELF selector",
            format!("{one_based}; expected 1..=100"),
        )?;
        Ok(Self(one_based as u8))
    }

    /// The normalized one-based selector, e.g. `selector.one_based()`.
    pub fn one_based(self) -> usize {
        usize::from(self.0)
    }

    pub(super) fn pair_body_offset(self) -> usize {
        244 + (self.one_based() - 1) * 8
    }
}

/// Identity of one complete preserved source snapshot, without whole-populated-file acceptance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationElfIndexRevision(ArchiveRevision);

/// Validated physical record address bound to one source snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationElfRecordId {
    pub(super) revision: SimulationElfIndexRevision,
    pub(super) body_offset: usize,
}

impl SimulationElfRecordId {
    /// Original wrapped stream address, e.g. `id.stream_offset()`.
    pub fn stream_offset(&self) -> usize {
        self.body_offset + 4
    }
}

/// Immutable complete source bytes with validated observed header/index context only.
#[derive(Debug, Clone)]
pub struct SimulationElfIndexContext {
    pub(super) bytes: Arc<[u8]>,
    pub(super) header: [u32; 7],
    pub(super) revision: SimulationElfIndexRevision,
    pub(super) limits: SimulationElfIndexLimits,
}

impl SimulationElfIndexContext {
    /// Validate copied framing and header/index context, e.g. `SimulationElfIndexContext::from_bytes(elf, limits)?`.
    pub fn from_bytes(bytes: &[u8], limits: SimulationElfIndexLimits) -> Result<Self> {
        validate_context(bytes, limits.maximum_file_bytes)?;
        let header = wire::patterns(region(bytes, 4, 28)?, "ELF index header")?;
        require(
            header[1] == 244 && header[2] == 100,
            "ELF index profile",
            format!(
                "record size {} and capacity {}; expected 244 and 100",
                header[1], header[2]
            ),
        )?;
        Ok(Self {
            bytes: bytes.into(),
            header,
            revision: SimulationElfIndexRevision(ArchiveRevision::default()),
            limits,
        })
    }

    /// Preserve every source byte, including unvisited records and unknown spans, e.g. `context.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    /// All seven raw header patterns, e.g. `context.header_words()[5]`.
    pub fn header_words(&self) -> [u32; 7] {
        self.header
    }

    /// Immutable source identity, e.g. `context.revision()`.
    pub fn revision(&self) -> &SimulationElfIndexRevision {
        &self.revision
    }

    /// Resolve a previously validated physical record identity, e.g. `context.record(&id)?`.
    pub fn record(&self, id: &SimulationElfRecordId) -> Result<SimulationElfScalarRecord> {
        require(
            id.revision == self.revision,
            "ELF record identity",
            format!(
                "stream offset {} from foreign revision; expected this source snapshot",
                id.stream_offset()
            ),
        )?;
        SimulationElfScalarRecord::from_bytes(region(&self.bytes, id.stream_offset(), 244)?)
    }
}

fn validate_context(bytes: &[u8], maximum: usize) -> Result<()> {
    require(
        bytes.len() >= 1224 && bytes.len() <= maximum,
        "ELF index extent",
        format!(
            "{} bytes; expected complete header/index and at most {maximum} wrapped bytes",
            bytes.len()
        ),
    )?;
    let expected = (bytes.len() - 5) / 0xa00000 + 1;
    let actual = dword(bytes, 0)? as i32;
    require(
        actual > 0 && actual as usize == expected,
        "ELF transfer count",
        format!("{actual}; expected {expected} for complete copied source"),
    )
}

pub(super) fn record_start(first: i32, count: u32) -> Result<Option<usize>> {
    if count == 0 {
        return Ok(None);
    }
    let offset = addressable_data_offset(first)?;
    require(
        offset >= 1220,
        "ELF selected record address",
        format!(
            "record {first} begins at body offset {offset}; expected at least 1220 beyond the header/index area"
        ),
    )?;
    Ok(Some(offset))
}

fn addressable_data_offset(first: i32) -> Result<usize> {
    usize::try_from(first).ok().and_then(|number| number.checked_sub(1))
        .and_then(|index| index.checked_mul(244))
        .ok_or_else(|| invalid("ELF selected record address", format!("first record {first}; expected a positive one-based addressable 244-byte record offset")))
}
