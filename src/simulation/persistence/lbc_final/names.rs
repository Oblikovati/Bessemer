use super::*;
use crate::archive_codec::ArchiveRevision;
use crate::records::dword;

/// Immutable identity of one exact LbcListOrder image and its resource context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLbcNameRevision(pub(super) ArchiveRevision);

/// Once-normalized zero-based name position bound to its original list revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLbcNameId {
    pub(super) revision: SimulationLbcNameRevision,
    pub(super) index: usize,
}

/// Complete signed-count LbcListOrder framing, without selection-string or matching semantics.
#[derive(Debug, Clone)]
pub struct SimulationLbcNameOrder {
    pub(super) bytes: Arc<[u8]>,
    pub(super) items: Arc<[SimulationPersistText]>,
    pub(super) revision: SimulationLbcNameRevision,
    pub(super) maximum_items: usize,
    pub(super) maximum_text_bytes: usize,
}

impl SimulationLbcNameOrder {
    /// Require the complete signed-count profile, e.g. `SimulationLbcNameOrder::from_bytes(bytes, 10000, 0x100000)?`.
    pub fn from_bytes(
        bytes: &[u8],
        maximum_items: usize,
        maximum_text_bytes: usize,
    ) -> Result<Self> {
        let offsets = name_offsets(bytes, maximum_items, maximum_text_bytes)?;
        let parent: Arc<[u8]> = bytes.into();
        let items = offsets
            .into_iter()
            .map(|offset| {
                SimulationPersistText::read_shared(parent.clone(), offset, maximum_text_bytes)
            })
            .collect::<Result<Arc<[_]>>>()?;
        Ok(Self {
            bytes: parent,
            items,
            revision: SimulationLbcNameRevision(ArchiveRevision::default()),
            maximum_items,
            maximum_text_bytes,
        })
    }

    /// Construct complete raw name frames preserving order/nulls/duplicates, e.g. `SimulationLbcNameOrder::from_items(&items, 10000, 0x100000)?`.
    pub fn from_items(
        items: &[SimulationPersistText],
        maximum_items: usize,
        maximum_text_bytes: usize,
    ) -> Result<Self> {
        let count = construction_count(items, maximum_items, maximum_text_bytes)?;
        let size = name_extent(items)?;
        let mut bytes = super::super::reserve_bytes(size)?;
        bytes.extend_from_slice(&count.to_le_bytes());
        for item in items {
            bytes.extend_from_slice(item.encoded());
        }
        Self::from_bytes(&bytes, maximum_items, maximum_text_bytes)
    }

    /// Every ordered text, including null/empty/duplicate/odd frames, e.g. `order.items()`.
    pub fn items(&self) -> &[SimulationPersistText] {
        &self.items
    }
    /// Original exact four-byte count header, e.g. `order.count_header()`.
    pub fn count_header(&self) -> &[u8] {
        &self.bytes[..4]
    }
    /// Entire exact original image, e.g. `order.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Immutable image identity, e.g. `order.revision()`.
    pub fn revision(&self) -> &SimulationLbcNameRevision {
        &self.revision
    }
    /// Normalize one actual zero-based stored position, e.g. `order.item_id(0)?`.
    pub fn item_id(&self, index: usize) -> Result<SimulationLbcNameId> {
        require(
            index < self.items.len(),
            "LBC name ordinal",
            format!("{index}; expected below {}", self.items.len()),
        )?;
        Ok(SimulationLbcNameId {
            revision: self.revision.clone(),
            index,
        })
    }
    /// Resolve an original normalized identity, e.g. `order.item(&id)?`.
    pub fn item(&self, id: &SimulationLbcNameId) -> Result<&SimulationPersistText> {
        require(
            id.revision == self.revision,
            "LBC name revision",
            format!(
                "foreign revision at item {}; expected this exact name-order revision",
                id.index
            ),
        )?;
        self.items.get(id.index).ok_or_else(|| {
            invalid(
                "LBC name ordinal",
                format!("{}; expected below {}", id.index, self.items.len()),
            )
        })
    }
    /// Exact complete name-list scope and exclusions, e.g. `order.coverage()`.
    pub fn coverage(&self) -> SimulationLbcFinalCoverage {
        super::coverage::name_coverage(self.bytes.len(), self.items.len())
    }
}

fn name_offsets(bytes: &[u8], maximum_items: usize, maximum_text: usize) -> Result<Vec<usize>> {
    let count = name_count(bytes, maximum_items)?;
    let mut offsets = name_position_storage(count)?;
    let end = super::super::text_sequence::append_frame_offsets(
        bytes,
        4,
        count,
        maximum_text,
        &mut offsets,
    )?;
    require(
        end == bytes.len(),
        "LBC name-order endpoint",
        format!("end {end}, {} bytes; expected exact EOF", bytes.len()),
    )?;
    Ok(offsets)
}

fn name_position_storage(count: usize) -> Result<Vec<usize>> {
    let mut offsets = Vec::new();
    offsets.try_reserve_exact(count).map_err(|error| {
        invalid(
            "LBC name positions",
            format!("{count} items, {error}; expected available complete position storage"),
        )
    })?;
    Ok(offsets)
}

fn name_count(bytes: &[u8], maximum: usize) -> Result<usize> {
    let count = dword(bytes, 0)? as i32;
    require(
        count >= 0 && count as usize <= maximum && count as usize <= (bytes.len() - 4) / 4,
        "LBC name count",
        format!(
            "{count} items with {} remaining bytes; expected a nonnegative signed-i32 count at most {maximum} with four header bytes per item",
            bytes.len() - 4
        ),
    )?;
    Ok(count as usize)
}

fn construction_count(
    items: &[SimulationPersistText],
    maximum: usize,
    maximum_text: usize,
) -> Result<i32> {
    require(
        items.len() <= maximum,
        "LBC name construction count",
        format!("{}; expected at most {maximum} items", items.len()),
    )?;
    let count = i32::try_from(items.len()).map_err(|_| {
        invalid(
            "LBC name construction count",
            format!("{}; expected a nonnegative signed-i32 count", items.len()),
        )
    })?;
    for item in items {
        item.require_limit(maximum_text)?;
    }
    Ok(count)
}

fn name_extent(items: &[SimulationPersistText]) -> Result<usize> {
    items.iter().try_fold(4usize, |size, item| {
        size.checked_add(item.encoded().len()).ok_or_else(|| {
            invalid(
                "LBC name construction extent",
                format!(
                    "{size}+{} bytes; expected representable complete image",
                    item.encoded().len()
                ),
            )
        })
    })
}
