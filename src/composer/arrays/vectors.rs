use super::*;
use crate::archive_codec::{ArchiveCursor, ArchiveRevision};
use std::sync::Arc;

/// Complete V2/V3 packed vector, preserving every stored pattern and code.
#[derive(Debug, Clone)]
pub struct ComposerVectorBlock {
    layout: ComposerVectorLayout,
    packing: ComposerVectorPacking,
    axes: Arc<[ComposerVectorAxis]>,
    packed: Arc<[u8]>,
    revision: ComposerArrayRevision,
    maximum_bytes: usize,
}

impl ComposerVectorBlock {
    /// Reconstruct every lane under an explicit profile, e.g. `block.reconstruct(ComposerVectorProfile::NearestEvenGradual79V1, 3000)?`.
    pub fn reconstruct(
        &self,
        profile: ComposerVectorProfile,
        maximum_components: usize,
    ) -> Result<ComposerVectorValues> {
        values::reconstruct_vectors(self, profile, maximum_components).map_err(|mut error| {
            error.operation = "reconstruct Composer vectors";
            error
        })
    }

    /// Parse with explicit caller shape, e.g. `ComposerVectorBlock::from_bytes(bytes, layout, 1024)?`.
    pub fn from_bytes(
        bytes: &[u8],
        layout: ComposerVectorLayout,
        maximum_bytes: usize,
    ) -> Result<Self> {
        require_budget(bytes.len(), maximum_bytes)?;
        let (block, consumed) = read_vector_prefix(bytes, layout, maximum_bytes)?;
        require_length(consumed, bytes.len(), "complete vector span")?;
        require_reconstruction(&block.to_bytes(), bytes)?;
        Ok(block)
    }

    /// Construct explicit stored lanes/codes, e.g. `ComposerVectorBlock::from_parts(layout, packing, axes, codes, 1024)?`.
    pub fn from_parts(
        layout: ComposerVectorLayout,
        packing: ComposerVectorPacking,
        axes: &[ComposerVectorAxis],
        packed: &[u8],
        maximum_bytes: usize,
    ) -> Result<Self> {
        validate_packed(layout, packing, axes, packed.len(), maximum_bytes)?;
        Ok(Self {
            layout,
            packing,
            axes: axes.into(),
            packed: packed.into(),
            maximum_bytes,
            revision: ComposerArrayRevision(ArchiveRevision::default()),
        })
    }

    /// Original caller shape, e.g. `block.layout().count`.
    pub fn layout(&self) -> ComposerVectorLayout {
        self.layout
    }

    /// Stored scalar representation, e.g. `block.packing()`.
    pub fn packing(&self) -> ComposerVectorPacking {
        self.packing
    }

    /// Immutable lane headers in component order, e.g. `block.axes()[0]`.
    pub fn axes(&self) -> &[ComposerVectorAxis] {
        &self.axes
    }

    /// Exact row-interleaved bytes with constant lanes omitted, e.g. `block.packed_bytes()`.
    pub fn packed_bytes(&self) -> &[u8] {
        &self.packed
    }

    /// Unsigned codes/direct patterns in stored order, e.g. `block.codes().collect::<Vec<_>>()`.
    pub fn codes(&self) -> impl ExactSizeIterator<Item = u32> + '_ {
        packed_values(&self.packed, self.packing.width())
    }

    /// Immutable revision for edit provenance, e.g. `block.revision().clone()`.
    pub fn revision(&self) -> &ComposerArrayRevision {
        &self.revision
    }

    /// Reconstruct the complete typed header and codes, e.g. `block.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = encode_header(self.layout.dimensions, self.packing, &self.axes);
        for code in self.codes() {
            bytes.extend_from_slice(&code.to_le_bytes()[..self.packing.width()]);
        }
        bytes
    }

    /// Exact wire coverage, e.g. `block.coverage().wire_bytes`.
    pub fn coverage(&self) -> ComposerArrayCoverage {
        let header = encode_header(self.layout.dimensions, self.packing, &self.axes);
        ComposerArrayCoverage {
            mode: ComposerArrayCheckMode::ExactPackedArray,
            wire_bytes: header.len() + self.packed.len(),
            stored_values: self.codes().len(),
        }
    }

    /// Replace packed codes on a copy, e.g. `block.edit_packed(block.packed_bytes())?`.
    pub fn edit_packed(&self, packed: &[u8]) -> Result<ComposerArrayEdit<Self>> {
        let mut block = Self::from_parts(
            self.layout,
            self.packing,
            &self.axes,
            packed,
            self.maximum_bytes,
        )?;
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

pub(in crate::composer) fn read_vector_prefix(
    bytes: &[u8],
    layout: ComposerVectorLayout,
    maximum: usize,
) -> Result<(ComposerVectorBlock, usize)> {
    let mut cursor = ArchiveCursor::new(bytes);
    let packing = decode_packing(cursor.u8().map_err(read_error)?)?;
    let axes = decode_axes(&mut cursor, packing, layout.dimensions).map_err(read_error)?;
    let varying = axes
        .iter()
        .filter(|axis| !matches!(axis, ComposerVectorAxis::Constant(_)))
        .count();
    let size = bounded_size(layout.count as usize, varying * packing.width(), maximum)?;
    let packed = cursor.take(size).map_err(read_error)?;
    let block = ComposerVectorBlock::from_parts(layout, packing, &axes, packed, maximum)?;
    Ok((block, bytes.len() - cursor.remaining()))
}

fn validate_packed(
    layout: ComposerVectorLayout,
    packing: ComposerVectorPacking,
    axes: &[ComposerVectorAxis],
    actual: usize,
    maximum: usize,
) -> Result<()> {
    validate_axes(layout.dimensions, packing, axes)?;
    let header = encode_header(layout.dimensions, packing, axes);
    let varying = axes
        .iter()
        .filter(|axis| !matches!(axis, ComposerVectorAxis::Constant(_)))
        .count();
    let expected = bounded_size(layout.count as usize, varying * packing.width(), maximum)?;
    bounded_total(expected, header.len(), maximum)?;
    require_length(actual, expected, format!("{layout:?}, {packing:?}"))
}

fn decode_packing(tag: u8) -> Result<ComposerVectorPacking> {
    match tag {
        b'a' => Ok(ComposerVectorPacking::Direct32),
        b'b' => Ok(ComposerVectorPacking::Quantized16),
        b'c' => Ok(ComposerVectorPacking::Quantized8),
        _ => Err(BessemerError::new(
            "composer packed array",
            "packing mode",
            FailureClass::Unsupported,
            format!("mode {tag}; expected 97, 98 or 99"),
        )),
    }
}

fn decode_axes(
    cursor: &mut ArchiveCursor<'_>,
    packing: ComposerVectorPacking,
    dimensions: ComposerVectorDimensions,
) -> Result<Vec<ComposerVectorAxis>> {
    let flags = if dimensions == ComposerVectorDimensions::Three {
        [cursor.u8()?, cursor.u8()?, cursor.u8()?]
    } else {
        [0; 3]
    };
    let mut axes = flags
        .into_iter()
        .take(dimensions.lanes())
        .map(|flag| decode_constant(cursor, flag))
        .collect::<Result<Vec<_>>>()?;
    decode_endpoints(cursor, packing, &mut axes)?;
    Ok(axes)
}

fn decode_constant(cursor: &mut ArchiveCursor<'_>, flag: u8) -> Result<ComposerVectorAxis> {
    match flag {
        0 => Ok(ComposerVectorAxis::Direct),
        1 => Ok(ComposerVectorAxis::Constant(cursor.u32()?)),
        _ => Err(invalid(format!("axis flag {flag}; expected 0 or 1"))),
    }
}

fn decode_endpoints(
    cursor: &mut ArchiveCursor<'_>,
    packing: ComposerVectorPacking,
    axes: &mut [ComposerVectorAxis],
) -> Result<()> {
    if packing == ComposerVectorPacking::Direct32 {
        return Ok(());
    }
    for axis in axes {
        if matches!(axis, ComposerVectorAxis::Direct) {
            *axis = ComposerVectorAxis::Quantized {
                first: cursor.u32()?,
                second: cursor.u32()?,
            };
        }
    }
    Ok(())
}

fn validate_axes(
    dimensions: ComposerVectorDimensions,
    packing: ComposerVectorPacking,
    axes: &[ComposerVectorAxis],
) -> Result<()> {
    require_length(
        axes.len(),
        dimensions.lanes(),
        format!("axes for {dimensions:?}"),
    )?;
    for axis in axes {
        require_axis(dimensions, packing, *axis)?;
    }
    Ok(())
}

fn require_axis(
    dimensions: ComposerVectorDimensions,
    packing: ComposerVectorPacking,
    axis: ComposerVectorAxis,
) -> Result<()> {
    let valid = match axis {
        ComposerVectorAxis::Constant(_) => dimensions == ComposerVectorDimensions::Three,
        ComposerVectorAxis::Direct => packing == ComposerVectorPacking::Direct32,
        ComposerVectorAxis::Quantized { .. } => packing != ComposerVectorPacking::Direct32,
    };
    if valid {
        return Ok(());
    }
    Err(invalid(format!(
        "axis {axis:?}; expected {dimensions:?} / {packing:?} header"
    )))
}

fn encode_header(
    dimensions: ComposerVectorDimensions,
    packing: ComposerVectorPacking,
    axes: &[ComposerVectorAxis],
) -> Vec<u8> {
    let mut bytes = vec![packing.tag()];
    if dimensions == ComposerVectorDimensions::Three {
        bytes.extend(
            axes.iter()
                .map(|axis| u8::from(matches!(axis, ComposerVectorAxis::Constant(_)))),
        );
    }
    append_axis_words(&mut bytes, axes);
    bytes
}

fn append_axis_words(bytes: &mut Vec<u8>, axes: &[ComposerVectorAxis]) {
    for axis in axes {
        if let ComposerVectorAxis::Constant(word) = axis {
            bytes.extend_from_slice(&word.to_le_bytes());
        }
    }
    for axis in axes {
        if let ComposerVectorAxis::Quantized { first, second } = axis {
            bytes.extend_from_slice(&first.to_le_bytes());
            bytes.extend_from_slice(&second.to_le_bytes());
        }
    }
}
