use super::arrays::{ArrayElementSize, DynamicArray};
use super::codec::FacetCodec;
use super::context::FacetState;
use super::output::FacetOutput;
use super::record::archive_record;
use super::{StoredFloat, invalid};
use crate::Result;
use crate::archive_codec::ArchiveCursor;

archive_record! {
    /// Raw colour/flag words and the arrays discarded by the native runtime.
    FacetStyle(context, 28, Element) {
        /// Stored face colour.
        face_color: required u32 => true;
        /// Stored edge colour.
        edge_color: required u32 => true;
        /// All prune bits, including bits the native loader masks.
        prune_flags: required u32 => true;
        /// All render bits, including bits the native loader masks.
        render_flags: required u32 => true;
        /// Uninterpreted native member +0x18.
        flags_18: required u32 => true;
        /// Complete direct integer array, even though the native runtime discards it.
        ignored_ints: required Vec<i32> => true;
        /// Complete direct float array with retained IEEE bits.
        ignored_floats: required Vec<StoredFloat> => true;
    }
}

/// Vertex-number framing selected by archive version and the stored header mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FacetVertexLayout {
    /// Through version 0x3EE7, no mode or style words precede this ordinary array.
    Legacy(DynamicArray),
    /// Mode 0, with no vertex-number array on the wire.
    NoVertexNumbers,
    /// Mode 4, with an implied four-byte element width.
    Compact(DynamicArray),
    /// Every other mode carries the complete style and ordinary vertex-number array.
    Styled {
        /// Original mode word; must differ from 0 and 4.
        mode: i32,
        /// Raw style fields.
        style: FacetStyle,
        /// Nullable array of strip lengths.
        vertex_numbers: DynamicArray,
    },
}

impl FacetCodec for FacetVertexLayout {
    const MINIMUM: usize = 4;
    fn read(cursor: &mut ArchiveCursor<'_>, context: &mut FacetState) -> Result<Self> {
        if context.version <= 0x3ee7 {
            return DynamicArray::read(cursor, context).map(Self::Legacy);
        }
        match cursor.i32()? {
            0 => Ok(Self::NoVertexNumbers),
            4 => DynamicArray::read_implied(cursor, context, ArrayElementSize::new(4)?)
                .map(Self::Compact),
            mode => Ok(Self::Styled {
                mode,
                style: FacetStyle::read(cursor, context)?,
                vertex_numbers: DynamicArray::read(cursor, context)?,
            }),
        }
    }
    fn write(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        if context.version <= 0x3ee7 {
            let Self::Legacy(array) = self else {
                return Err(invalid(format!(
                    "modern facet vertex layout; expected legacy array at version {:#x}",
                    context.version
                )));
            };
            return array.write(output, context);
        }
        self.write_modern(output, context)
    }
}

impl FacetVertexLayout {
    fn write_modern(&self, output: &mut FacetOutput, context: &mut FacetState) -> Result<()> {
        match self {
            Self::NoVertexNumbers => output.i32(0),
            Self::Compact(array) => {
                output.i32(4);
                array.write_implied(output, 4)?;
            }
            Self::Styled {
                mode,
                style,
                vertex_numbers,
            } => write_style(*mode, style, vertex_numbers, output, context)?,
            Self::Legacy(_) => {
                return Err(invalid(format!(
                    "legacy vertex layout; expected modern layout at version {:#x}",
                    context.version
                )));
            }
        }
        Ok(())
    }
}

fn write_style(
    mode: i32,
    style: &FacetStyle,
    vertex_numbers: &DynamicArray,
    output: &mut FacetOutput,
    context: &mut FacetState,
) -> Result<()> {
    require_styled_mode(mode)?;
    output.i32(mode);
    style.write(output, context)?;
    vertex_numbers.write(output, context)
}

fn require_styled_mode(mode: i32) -> Result<()> {
    if mode != 0 && mode != 4 {
        return Ok(());
    }
    Err(invalid(format!(
        "styled mode {mode}; expected a mode other than 0 or 4"
    )))
}
