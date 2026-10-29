use super::*;
use crate::archive_codec::ArchiveRevision;
use std::sync::Arc;

/// Complete unsigned index/length block with caller-supplied count and domain.
#[derive(Debug, Clone)]
pub struct ComposerIndexBlock {
    count: u32,
    domain: u32,
    packed: Arc<[u8]>,
    revision: ComposerArrayRevision,
    maximum_bytes: usize,
}

impl ComposerIndexBlock {
    /// Validate exact EOF, e.g. `ComposerIndexBlock::from_bytes(&[255], 1, 0, 1024)?`.
    pub fn from_bytes(bytes: &[u8], count: u32, domain: u32, maximum_bytes: usize) -> Result<Self> {
        let expected = bounded_size(count as usize, index_width(domain), maximum_bytes)?;
        require_length(
            bytes.len(),
            expected,
            format!("index count {count}, domain {domain}"),
        )?;
        let block = Self {
            count,
            domain,
            packed: bytes.into(),
            maximum_bytes,
            revision: ComposerArrayRevision(ArchiveRevision::default()),
        };
        require_reconstruction(&block.to_bytes(), bytes)?;
        Ok(block)
    }

    /// Construct using the domain-selected width, e.g. `ComposerIndexBlock::from_values(&[256], 256, 1024)?`.
    pub fn from_values(values: &[u32], domain: u32, maximum_bytes: usize) -> Result<Self> {
        let count = u32::try_from(values.len())
            .map_err(|_| invalid(format!("index count {}; expected u32", values.len())))?;
        let width = index_width(domain);
        let mut bytes = Vec::with_capacity(bounded_size(values.len(), width, maximum_bytes)?);
        for value in values {
            require_index_width(*value, width)?;
            bytes.extend_from_slice(&value.to_le_bytes()[..width]);
        }
        Self::from_bytes(&bytes, count, domain, maximum_bytes)
    }

    /// Caller-supplied count, e.g. `assert_eq!(block.count(), 3)`.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Original width-selection parameter, e.g. `block.domain()`; it is not a value bound.
    pub fn domain(&self) -> u32 {
        self.domain
    }

    /// Decode exact stored unsigned values, e.g. `block.values().collect::<Vec<_>>()`.
    pub fn values(&self) -> impl ExactSizeIterator<Item = u32> + '_ {
        packed_values(&self.packed, index_width(self.domain))
    }

    /// Immutable revision for edit provenance, e.g. `block.revision().clone()`.
    pub fn revision(&self) -> &ComposerArrayRevision {
        &self.revision
    }

    /// Serialize every typed value, e.g. `block.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let width = index_width(self.domain);
        let mut bytes = Vec::with_capacity(self.packed.len());
        for value in self.values() {
            bytes.extend_from_slice(&value.to_le_bytes()[..width]);
        }
        bytes
    }

    /// Exact wire coverage, e.g. `block.coverage().stored_values`.
    pub fn coverage(&self) -> ComposerArrayCoverage {
        ComposerArrayCoverage {
            mode: ComposerArrayCheckMode::ExactPackedArray,
            wire_bytes: self.packed.len(),
            stored_values: self.count as usize,
        }
    }

    /// Replace on a copy without changing count/domain, e.g. `block.edit_values(&[1, 2, 3])?`.
    pub fn edit_values(&self, values: &[u32]) -> Result<ComposerArrayEdit<Self>> {
        if values.len() != self.count as usize {
            return Err(invalid(format!(
                "replacement count {}; expected {}",
                values.len(),
                self.count
            )));
        }
        let mut block = Self::from_values(values, self.domain, self.maximum_bytes)?;
        if block.packed == self.packed {
            block = self.clone();
        }
        Ok(edit_result(
            &self.revision,
            &block.revision,
            block.clone(),
            block.coverage(),
        ))
    }
}

fn index_width(domain: u32) -> usize {
    match domain {
        0..=255 => 1,
        256..=65535 => 2,
        _ => 4,
    }
}

fn require_index_width(value: u32, width: usize) -> Result<()> {
    let maximum = match width {
        1 => 255,
        2 => 65535,
        _ => u32::MAX,
    };
    if value <= maximum {
        return Ok(());
    }
    Err(invalid(format!(
        "index value {value}; expected 0..={maximum} for {width}-byte storage"
    )))
}
