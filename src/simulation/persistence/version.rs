use super::{
    Result, SimulationPersistCheckMode, SimulationPersistCoverage, SimulationPersistText,
    coverage::coverage, invalid, require,
};
use crate::records::region;
use std::sync::Arc;

/// Once-normalized explicit caller numeric context; no text conversion is inferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPersistVersion(i32);

impl SimulationPersistVersion {
    pub(super) const COMPATIBILITY: Self = Self(500070);
    /// Require nonnegative signed-i32 context, e.g. `SimulationPersistVersion::new(830032)?`.
    pub fn new(value: i32) -> Result<Self> {
        require(
            value >= 0,
            "primary numeric version",
            format!("{value}; expected a nonnegative signed-i32 value"),
        )?;
        Ok(Self(value))
    }

    /// Exact caller-supplied value, e.g. `version.value()`.
    pub fn value(self) -> i32 {
        self.0
    }
}

/// Explicit reader mode supplied by the caller, never inferred from input length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistReadMode {
    /// Require all version-selected words and exact EOF.
    Full,
    /// Consume only the first conditional word and preserve every remaining byte.
    Partial,
}

/// Documented runtime assignment destination, distinct from a stored byte offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistOwnerSlot {
    /// Runtime owner destination +0x1A8.
    Owner1A8,
    /// Runtime owner destination +0x1C8.
    Owner1C8,
}

/// One ordered modeled assignment; it does not execute or clear other owner fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPersistOwnerUpdate {
    /// Typed runtime destination.
    pub destination: SimulationPersistOwnerSlot,
    /// Exact signed four-byte pattern.
    pub value: i32,
}

/// Immutable primary CosworksVersoin image, including explicit unconsumed bytes.
#[derive(Debug, Clone)]
pub struct SimulationPrimaryVersion {
    text: SimulationPersistText,
    numeric: SimulationPersistVersion,
    mode: SimulationPersistReadMode,
    words: Arc<[i32]>,
    updates: Arc<[SimulationPersistOwnerUpdate]>,
    end: usize,
}

impl SimulationPrimaryVersion {
    /// Decode using explicit caller context, e.g. `SimulationPrimaryVersion::from_bytes(bytes, version, SimulationPersistReadMode::Full, 0x100000)?`.
    pub fn from_bytes(
        bytes: &[u8],
        numeric: SimulationPersistVersion,
        mode: SimulationPersistReadMode,
        maximum_text_bytes: usize,
    ) -> Result<Self> {
        let start = super::text::text_extent(bytes, 0, maximum_text_bytes)?.1;
        let (end, words) = decode_words(bytes, start, numeric, mode)?;
        let updates = owner_updates(numeric, mode, &words);
        let text = SimulationPersistText::read_from(bytes, 0, maximum_text_bytes)?;
        Ok(Self {
            text,
            numeric,
            mode,
            words,
            updates,
            end,
        })
    }

    /// Construct exact stored framing without native CString conversion, e.g. `SimulationPrimaryVersion::from_parts(&text, version, mode, &words, &[], maximum)?`.
    pub fn from_parts(
        text: &SimulationPersistText,
        numeric: SimulationPersistVersion,
        mode: SimulationPersistReadMode,
        words: &[i32],
        remainder: &[u8],
        maximum_text_bytes: usize,
    ) -> Result<Self> {
        validate_word_count(words.len(), numeric, mode)?;
        validate_suffix(remainder, mode)?;
        text.require_limit(maximum_text_bytes)?;
        let size = text.encoded().len().checked_add(words.len() * 4).and_then(|size| size.checked_add(remainder.len())).ok_or_else(|| invalid("primary version extent", format!("{} text bytes, {} words, {} suffix bytes; expected representable complete image", text.encoded().len(), words.len(), remainder.len())))?;
        let mut bytes = super::reserve_bytes(size)?;
        bytes.extend_from_slice(text.encoded());
        bytes.extend(words.iter().flat_map(|word| word.to_le_bytes()));
        bytes.extend_from_slice(remainder);
        Self::from_bytes(&bytes, numeric, mode, maximum_text_bytes)
    }

    /// Counted version text retaining the complete source parent, e.g. `version.text()`.
    pub fn text(&self) -> &SimulationPersistText {
        &self.text
    }
    /// Original explicit numeric context, e.g. `version.numeric_version()`.
    pub fn numeric_version(&self) -> SimulationPersistVersion {
        self.numeric
    }
    /// Original explicit full/partial mode, e.g. `version.read_mode()`.
    pub fn read_mode(&self) -> SimulationPersistReadMode {
        self.mode
    }
    /// Ordered signed four-byte words, including consumed but unassigned words, e.g. `version.words()`.
    pub fn words(&self) -> &[i32] {
        &self.words
    }
    /// Exact consumed tail bytes, e.g. `version.word_payload()`.
    pub fn word_payload(&self) -> &[u8] {
        &self.text.source_parent()[self.text.source_offsets().2..self.end]
    }
    /// Ordered modeled runtime assignments, e.g. `version.owner_updates()`.
    pub fn owner_updates(&self) -> &[SimulationPersistOwnerUpdate] {
        &self.updates
    }
    /// First unconsumed original byte offset, e.g. `version.consumed_end()`.
    pub fn consumed_end(&self) -> usize {
        self.end
    }
    /// Entire unconsumed suffix, e.g. `version.remainder()`.
    pub fn remainder(&self) -> &[u8] {
        &self.text.source_parent()[self.end..]
    }
    /// Complete original image including the explicit partial suffix, e.g. `version.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.text.source_parent().to_vec()
    }
    /// Exact consumed scope and explicit unconsumed completion, e.g. `version.coverage()`.
    pub fn coverage(&self) -> SimulationPersistCoverage {
        coverage(
            SimulationPersistCheckMode::ExactPrimaryVersionMode,
            self.text.source_parent().len(),
            self.end,
            self.remainder().len(),
            !self.remainder().is_empty(),
        )
    }
}

fn word_count(numeric: SimulationPersistVersion, mode: SimulationPersistReadMode) -> usize {
    let version = numeric.value();
    usize::from(version > 800018)
        + usize::from(mode == SimulationPersistReadMode::Full && version > 800076)
        + usize::from(mode == SimulationPersistReadMode::Full && version >= 830032)
}

fn decode_words(
    bytes: &[u8],
    start: usize,
    numeric: SimulationPersistVersion,
    mode: SimulationPersistReadMode,
) -> Result<(usize, Arc<[i32]>)> {
    let raw = region(bytes, start, word_count(numeric, mode) * 4)?;
    let end = start + raw.len();
    require(
        mode == SimulationPersistReadMode::Partial || end == bytes.len(),
        "primary version endpoint",
        format!(
            "end {end}, {} bytes; expected exact EOF in Full mode",
            bytes.len()
        ),
    )?;
    let words = raw
        .as_chunks::<4>()
        .0
        .iter()
        .map(|word| i32::from_le_bytes(*word))
        .collect();
    Ok((end, words))
}

fn validate_word_count(
    count: usize,
    numeric: SimulationPersistVersion,
    mode: SimulationPersistReadMode,
) -> Result<()> {
    require(
        count == word_count(numeric, mode),
        "primary version word count",
        format!(
            "{count} words; expected {} for {} {mode:?}",
            word_count(numeric, mode),
            numeric.value()
        ),
    )
}

fn validate_suffix(remainder: &[u8], mode: SimulationPersistReadMode) -> Result<()> {
    require(
        mode == SimulationPersistReadMode::Partial || remainder.is_empty(),
        "primary version suffix",
        format!(
            "{} bytes; expected empty suffix in Full mode",
            remainder.len()
        ),
    )
}

fn owner_updates(
    numeric: SimulationPersistVersion,
    mode: SimulationPersistReadMode,
    words: &[i32],
) -> Arc<[SimulationPersistOwnerUpdate]> {
    let mut updates = Vec::new();
    if (800019..830032).contains(&numeric.value()) {
        updates.push(SimulationPersistOwnerUpdate {
            destination: SimulationPersistOwnerSlot::Owner1C8,
            value: words[0],
        });
    }
    if mode == SimulationPersistReadMode::Full && numeric.value() > 800076 {
        updates.push(SimulationPersistOwnerUpdate {
            destination: SimulationPersistOwnerSlot::Owner1A8,
            value: words[1],
        });
    }
    if mode == SimulationPersistReadMode::Full && numeric.value() >= 830032 {
        updates.push(SimulationPersistOwnerUpdate {
            destination: SimulationPersistOwnerSlot::Owner1C8,
            value: words[2],
        });
    }
    updates.into()
}
