use super::{
    DisplayVersion, StoredDouble,
    output::DisplayOutput,
    view_support::{ViewBudget, require_presence},
};
use crate::{
    Result,
    archive_codec::{ArchiveCursor, read_point},
};

/// A stored section plane and its version-gated unknown fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewSectionPlane {
    /// Stored plane root point.
    pub root_point: [StoredDouble; 3],
    /// Stored normal, without normalization.
    pub normal: [StoredDouble; 3],
    /// Unknown stored double.
    pub unknown_double: StoredDouble,
    /// Unknown first integer.
    pub unknown_i0: i32,
    /// Conditional integer pair; the owning serializer selects its version gate.
    pub unknown_pair: Option<[i32; 2]>,
    /// Conditional unsigned word; the owning serializer selects its version gate.
    pub unknown_word: Option<u32>,
}

#[derive(Clone, Copy)]
pub(super) struct PlaneGates {
    pub pair_after: u32,
    pub word_after: u32,
    pub pair_label: &'static str,
    pub word_label: &'static str,
}

pub(super) fn read_list(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    budget: &mut ViewBudget,
    gates: PlaneGates,
    label: &str,
) -> Result<Vec<ViewSectionPlane>> {
    let count = cursor.nonnegative_count(60, label)?;
    budget.nested(count, label)?;
    (0..count)
        .map(|_| read_plane(cursor, version, gates))
        .collect()
}

fn read_plane(
    cursor: &mut ArchiveCursor<'_>,
    version: DisplayVersion,
    gates: PlaneGates,
) -> Result<ViewSectionPlane> {
    Ok(ViewSectionPlane {
        root_point: read_point(cursor)?,
        normal: read_point(cursor)?,
        unknown_double: StoredDouble::from_bits(cursor.u64()?),
        unknown_i0: cursor.i32()?,
        unknown_pair: if version.value() > gates.pair_after {
            Some([cursor.i32()?, cursor.i32()?])
        } else {
            None
        },
        unknown_word: if version.value() > gates.word_after {
            Some(cursor.u32()?)
        } else {
            None
        },
    })
}

pub(super) fn write_list(
    output: &mut DisplayOutput,
    planes: &[ViewSectionPlane],
    version: DisplayVersion,
    budget: &mut ViewBudget,
    gates: PlaneGates,
    label: &str,
) -> Result<()> {
    budget.nested(planes.len(), label)?;
    output.count(planes.len(), label)?;
    for plane in planes {
        write_plane(output, plane, version, gates)?;
    }
    Ok(())
}

fn write_plane(
    output: &mut DisplayOutput,
    plane: &ViewSectionPlane,
    version: DisplayVersion,
    gates: PlaneGates,
) -> Result<()> {
    validate_plane(plane, version, gates)?;
    output.point(&plane.root_point)?;
    output.point(&plane.normal)?;
    output.double(plane.unknown_double)?;
    output.i32(plane.unknown_i0)?;
    if let Some(pair) = plane.unknown_pair {
        for word in pair {
            output.i32(word)?;
        }
    }
    if let Some(word) = plane.unknown_word {
        output.u32(word)?;
    }
    Ok(())
}

fn validate_plane(
    plane: &ViewSectionPlane,
    version: DisplayVersion,
    gates: PlaneGates,
) -> Result<()> {
    require_presence(
        plane.unknown_pair.is_some(),
        version.value() > gates.pair_after,
        gates.pair_label,
    )?;
    require_presence(
        plane.unknown_word.is_some(),
        version.value() > gates.word_after,
        gates.word_label,
    )
}
