use super::{Result, SimulationGenArchive, SimulationMasterArchive};
use crate::simulation::{require, wire};
use std::sync::Arc;

/// Complete stored GEN regions for construction without dropping unknown bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationGenParts {
    /// Unassigned 400-byte initial region.
    pub opaque_header: Arc<[u8]>,
    /// Fourteen nonnegative prefix words; record counts must match supplied blocks.
    pub counts: [u32; 14],
    /// Unassigned 140-byte middle prefix region.
    pub opaque_middle: Arc<[u8]>,
    /// Exact late-word pattern.
    pub late_word: u32,
    /// Ten complete blocks in validated layout order, including the trailer.
    pub blocks: [Arc<[u8]>; 10],
    /// Optional auxiliary vectors, retaining present-empty versus absent distinction.
    pub vectors: Arc<[Option<Arc<[u8]>>]>,
}

impl SimulationGenArchive {
    /// Capture all stored regions for construction, e.g. `let parts = image.parts()`.
    pub fn parts(&self) -> SimulationGenParts {
        SimulationGenParts {
            opaque_header: self.bytes[4..404].into(),
            counts: self.layout.counts,
            opaque_middle: self.bytes[460..600].into(),
            late_word: u32::from_le_bytes(self.bytes[600..604].as_chunks::<4>().0[0]),
            blocks: self
                .layout
                .blocks
                .map(|(start, length)| self.bytes[start..start + length].into()),
            vectors: self
                .layout
                .vectors
                .iter()
                .map(|span| span.map(|(start, length)| self.bytes[start..start + length].into()))
                .collect(),
        }
    }

    /// Construct and validate a complete paired image, e.g. `SimulationGenArchive::from_parts(parts, &master)?`.
    pub fn from_parts(parts: SimulationGenParts, master: &SimulationMasterArchive) -> Result<Self> {
        let bytes = encode(&parts)?;
        let image = Self::from_bytes(&bytes, master)?;
        require(
            image.parts() == parts,
            "GEN parts",
            "supplied regions disagree with decoded count, stride or optional-vector boundaries; expected exact region reconstruction",
        )?;
        Ok(image)
    }
}

fn encode(parts: &SimulationGenParts) -> Result<Vec<u8>> {
    validate_prefix_parts(parts)?;
    let mut bytes = vec![0; 4];
    bytes.extend_from_slice(&parts.opaque_header);
    wire::append_patterns(&mut bytes, &parts.counts);
    bytes.extend_from_slice(&parts.opaque_middle);
    wire::append_patterns(&mut bytes, &[parts.late_word]);
    for block in &parts.blocks[..9] {
        bytes.extend_from_slice(block);
    }
    for vector in parts.vectors.iter().flatten() {
        bytes.extend_from_slice(vector);
    }
    bytes.extend_from_slice(&parts.blocks[9]);
    finalize_transfer(bytes)
}

fn finalize_transfer(mut bytes: Vec<u8>) -> Result<Vec<u8>> {
    let transfers = (bytes.len() - 5) / 0xa00000 + 1;
    require(
        transfers <= i32::MAX as usize,
        "GEN transfer",
        format!("count {transfers}; expected signed 32-bit count"),
    )?;
    bytes[..4].copy_from_slice(&(transfers as u32).to_le_bytes());
    Ok(bytes)
}

fn validate_prefix_parts(parts: &SimulationGenParts) -> Result<()> {
    require(
        parts.opaque_header.len() == 400 && parts.opaque_middle.len() == 140,
        "GEN prefix parts",
        format!(
            "header {} and middle {} bytes; expected 400 and 140",
            parts.opaque_header.len(),
            parts.opaque_middle.len()
        ),
    )?;
    Ok(())
}
