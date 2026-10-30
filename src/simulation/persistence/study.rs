use super::{
    Result, SimulationPersistCheckMode, SimulationPersistCoverage, SimulationPersistText,
    coverage::coverage, invalid, require, text::text_extent,
};
use crate::archive_codec::ArchiveRevision;
use crate::records::dword;
use std::sync::Arc;

/// Immutable identity of one complete study-order image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationStudyOrderRevision(pub(super) ArchiveRevision);

/// Once-normalized leading or item position bound to its original stream revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationStudyTextId {
    pub(super) revision: SimulationStudyOrderRevision,
    pub(super) ordinal: usize,
}

/// Complete lossless StudyListOrder framing, without active-study interpretation.
#[derive(Debug, Clone)]
pub struct SimulationStudyOrder {
    pub(super) bytes: Arc<[u8]>,
    pub(super) texts: Arc<[SimulationPersistText]>,
    pub(super) count_offset: usize,
    pub(super) revision: SimulationStudyOrderRevision,
    pub(super) maximum_items: usize,
    pub(super) maximum_text_bytes: usize,
}

impl SimulationStudyOrder {
    /// Validate count, all texts and EOF, e.g. `SimulationStudyOrder::from_bytes(bytes, 10000, 0x100000)?`.
    pub fn from_bytes(
        bytes: &[u8],
        maximum_items: usize,
        maximum_text_bytes: usize,
    ) -> Result<Self> {
        let (count_offset, offsets) = study_offsets(bytes, maximum_items, maximum_text_bytes)?;
        let parent: Arc<[u8]> = bytes.into();
        let texts = offsets
            .into_iter()
            .map(|offset| {
                SimulationPersistText::read_shared(parent.clone(), offset, maximum_text_bytes)
            })
            .collect::<Result<Arc<[_]>>>()?;
        Ok(Self {
            bytes: parent,
            texts,
            count_offset,
            revision: SimulationStudyOrderRevision(ArchiveRevision::default()),
            maximum_items,
            maximum_text_bytes,
        })
    }

    /// Construct exact raw framing preserving duplicate/null items, e.g. `SimulationStudyOrder::from_parts(&leading, &items, 10000, 0x100000)?`.
    pub fn from_parts(
        leading: &SimulationPersistText,
        items: &[SimulationPersistText],
        maximum_items: usize,
        maximum_text_bytes: usize,
    ) -> Result<Self> {
        let count = validate_parts(leading, items, maximum_items, maximum_text_bytes)?;
        let size = parts_size(leading, items)?;
        let mut bytes = super::reserve_bytes(size)?;
        bytes.extend_from_slice(leading.encoded());
        bytes.extend_from_slice(&count.to_le_bytes());
        for item in items {
            bytes.extend_from_slice(item.encoded());
        }
        Self::from_bytes(&bytes, maximum_items, maximum_text_bytes)
    }

    /// Exact first text without assigning selection semantics, e.g. `order.leading()`.
    pub fn leading(&self) -> &SimulationPersistText {
        &self.texts[0]
    }
    /// All ordered items, including duplicates and nulls, e.g. `order.items()`.
    pub fn items(&self) -> &[SimulationPersistText] {
        &self.texts[1..]
    }
    /// Exact stored count header, e.g. `order.count_header()`.
    pub fn count_header(&self) -> &[u8] {
        &self.bytes[self.count_offset..self.count_offset + 4]
    }
    /// Original raw count position, e.g. `order.count_offset()`.
    pub fn count_offset(&self) -> usize {
        self.count_offset
    }
    /// Complete exact encoded image, e.g. `order.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Immutable image identity, e.g. `order.revision()`.
    pub fn revision(&self) -> &SimulationStudyOrderRevision {
        &self.revision
    }
    /// Revision-bound leading-text identity, e.g. `order.leading_id()`.
    pub fn leading_id(&self) -> SimulationStudyTextId {
        self.text_id(0)
    }
    /// Normalize an actual zero-based item position, e.g. `order.item_id(0)?`.
    pub fn item_id(&self, index: usize) -> Result<SimulationStudyTextId> {
        require(
            index < self.items().len(),
            "study item ordinal",
            format!("{index}; expected an index below {}", self.items().len()),
        )?;
        Ok(self.text_id(index + 1))
    }
    /// Resolve one normalized identity on its original revision, e.g. `order.text(&id)?`.
    pub fn text(&self, id: &SimulationStudyTextId) -> Result<&SimulationPersistText> {
        require(
            id.revision == self.revision,
            "study text revision",
            format!(
                "foreign revision at ordinal {}; expected this study-order revision",
                id.ordinal
            ),
        )?;
        self.texts.get(id.ordinal).ok_or_else(|| {
            invalid(
                "study text ordinal",
                format!("{}; expected below {}", id.ordinal, self.texts.len()),
            )
        })
    }
    /// Exact complete list framing scope, e.g. `order.coverage()`.
    pub fn coverage(&self) -> SimulationPersistCoverage {
        coverage(
            SimulationPersistCheckMode::ExactStudyOrder,
            self.bytes.len(),
            self.bytes.len(),
            0,
            false,
        )
    }
    pub(super) fn text_id(&self, ordinal: usize) -> SimulationStudyTextId {
        SimulationStudyTextId {
            revision: self.revision.clone(),
            ordinal,
        }
    }
}

fn study_offsets(
    bytes: &[u8],
    maximum_items: usize,
    maximum_text: usize,
) -> Result<(usize, Vec<usize>)> {
    let count_offset = text_extent(bytes, 0, maximum_text)?.1;
    let count = study_count(bytes, count_offset, maximum_items)?;
    let mut cursor = count_offset + 4;
    let mut offsets = position_storage(count)?;
    offsets.push(0);
    for _ in 0..count {
        offsets.push(cursor);
        cursor = text_extent(bytes, cursor, maximum_text)?.1;
    }
    require(
        cursor == bytes.len(),
        "study-order endpoint",
        format!("end {cursor}, {} bytes; expected exact EOF", bytes.len()),
    )?;
    Ok((count_offset, offsets))
}

fn study_count(bytes: &[u8], count_offset: usize, maximum_items: usize) -> Result<usize> {
    let count = dword(bytes, count_offset)? as usize;
    let cursor = count_offset + 4;
    require(
        count <= maximum_items && count <= (bytes.len() - cursor) / 4,
        "study item count",
        format!(
            "{count} items with {} remaining bytes; expected at most {maximum_items} items and four available header bytes per item",
            bytes.len() - cursor
        ),
    )?;
    Ok(count)
}

fn position_storage(count: usize) -> Result<Vec<usize>> {
    let mut offsets = Vec::new();
    offsets.try_reserve_exact(count + 1).map_err(|error| {
        invalid(
            "study text positions",
            format!("{count} items, {error}; expected available complete position storage"),
        )
    })?;
    Ok(offsets)
}

fn validate_parts(
    leading: &SimulationPersistText,
    items: &[SimulationPersistText],
    maximum_items: usize,
    maximum_text: usize,
) -> Result<u32> {
    require(
        items.len() <= maximum_items,
        "study item count",
        format!("{} items; expected at most {maximum_items}", items.len()),
    )?;
    for text in std::iter::once(leading).chain(items.iter()) {
        text.require_limit(maximum_text)?;
    }
    u32::try_from(items.len()).map_err(|_| {
        invalid(
            "study item count",
            format!("{} items; expected a u32 count", items.len()),
        )
    })
}

fn parts_size(leading: &SimulationPersistText, items: &[SimulationPersistText]) -> Result<usize> {
    std::iter::once(leading)
        .chain(items.iter())
        .try_fold(4usize, |size, text| {
            size.checked_add(text.encoded().len()).ok_or_else(|| {
                invalid(
                    "study-order extent",
                    format!(
                        "{size}+{} bytes; expected representable complete image",
                        text.encoded().len()
                    ),
                )
            })
        })
}
