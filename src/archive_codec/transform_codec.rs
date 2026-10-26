use super::{
    CompactTransform, FixedTransform, PerspectiveBase, PerspectivePayload, StoredDouble,
    TransformContents, TransformEncoding,
};
use crate::{
    BessemerError, FailureClass, Result,
    archive_codec::{ArchiveCursor, read_point},
};

pub(crate) fn read(
    cursor: &mut ArchiveCursor<'_>,
    encoding: TransformEncoding,
) -> Result<TransformContents> {
    match encoding {
        TransformEncoding::Compact => read_compact(cursor).map(TransformContents::Compact),
        TransformEncoding::Fixed => read_fixed(cursor).map(TransformContents::Fixed),
    }
}

pub(crate) fn read_compact(cursor: &mut ArchiveCursor<'_>) -> Result<CompactTransform> {
    let has_matrix = cursor.u8()?;
    let matrix = read_optional_matrix(cursor, has_matrix)?;
    let translation = read_point(cursor)?;
    let scaling = StoredDouble::from_bits(cursor.u64()?);
    let flags = cursor.u8()?;
    let perspective = read_perspective(cursor, flags)?;
    Ok(CompactTransform {
        has_matrix,
        matrix,
        translation,
        scaling,
        flags,
        perspective,
    })
}

fn read_optional_matrix(
    cursor: &mut ArchiveCursor<'_>,
    presence: u8,
) -> Result<Option<[[StoredDouble; 3]; 3]>> {
    if presence == 0 {
        return Ok(None);
    }
    Ok(Some([
        read_point(cursor)?,
        read_point(cursor)?,
        read_point(cursor)?,
    ]))
}

fn read_fixed(cursor: &mut ArchiveCursor<'_>) -> Result<FixedTransform> {
    Ok(FixedTransform {
        matrix: read_doubles(cursor)?,
        unknown_48: [cursor.i32()?, cursor.i32()?],
        translation: read_point(cursor)?,
        scaling: StoredDouble::from_bits(cursor.u64()?),
        unknown_70: [cursor.i32()?, cursor.i32()?],
    })
}

fn read_doubles<const N: usize>(cursor: &mut ArchiveCursor<'_>) -> Result<[StoredDouble; N]> {
    let mut values = [StoredDouble::from_bits(0); N];
    for value in &mut values {
        *value = StoredDouble::from_bits(cursor.u64()?);
    }
    Ok(values)
}

fn read_base(cursor: &mut ArchiveCursor<'_>) -> Result<PerspectiveBase> {
    Ok(PerspectiveBase {
        eye_point: read_point(cursor)?,
        plane_distance: StoredDouble::from_bits(cursor.u64()?),
    })
}

fn read_perspective(cursor: &mut ArchiveCursor<'_>, flags: u8) -> Result<PerspectivePayload> {
    match flags & 0xf0 {
        0x10 => Ok(PerspectivePayload::Base(read_base(cursor)?)),
        0x20 => Ok(PerspectivePayload::Extended {
            base: read_base(cursor)?,
            minimum_far_clip: StoredDouble::from_bits(cursor.u64()?),
            camera_fov_plane: StoredDouble::from_bits(cursor.u64()?),
        }),
        0x40 => Ok(PerspectivePayload::Matrix5 {
            discarded: cursor.i32()?,
            matrix: Box::new(read_doubles(cursor)?),
        }),
        _ => Ok(PerspectivePayload::None),
    }
}

pub(crate) fn validate(contents: &TransformContents) -> Result<()> {
    let TransformContents::Compact(compact) = contents else {
        return Ok(());
    };
    validate_compact(compact)
}

pub(crate) fn validate_compact(compact: &CompactTransform) -> Result<()> {
    require_matrix(compact)?;
    let selector = match compact.perspective {
        PerspectivePayload::Base(_) => 0x10,
        PerspectivePayload::Extended { .. } => 0x20,
        PerspectivePayload::Matrix5 { .. } => 0x40,
        PerspectivePayload::None => return require_no_perspective(compact.flags),
    };
    if compact.flags & 0xf0 == selector {
        return Ok(());
    }
    Err(invalid(format!(
        "perspective selector {:#04x}; expected {selector:#04x} for {:?}",
        compact.flags & 0xf0,
        compact.perspective
    )))
}

fn require_matrix(compact: &CompactTransform) -> Result<()> {
    if (compact.has_matrix != 0) != compact.matrix.is_some() {
        return Err(invalid(format!(
            "has_matrix {} with matrix presence {}; expected presence for every nonzero flag",
            compact.has_matrix,
            compact.matrix.is_some()
        )));
    }
    Ok(())
}

fn require_no_perspective(flags: u8) -> Result<()> {
    if ![0x10, 0x20, 0x40].contains(&(flags & 0xf0)) {
        return Ok(());
    }
    Err(invalid(format!(
        "flags {flags:#04x} without perspective; expected matching high-nibble payload"
    )))
}

pub(crate) fn encode(contents: &TransformContents) -> Vec<u8> {
    let mut output = Vec::new();
    match contents {
        TransformContents::Compact(compact) => write_compact(compact, &mut output),
        TransformContents::Fixed(fixed) => write_fixed(fixed, &mut output),
    }
    output
}

pub(crate) fn write_compact(compact: &CompactTransform, output: &mut Vec<u8>) {
    output.push(compact.has_matrix);
    if let Some(matrix) = &compact.matrix {
        for row in matrix {
            write_doubles(row, output);
        }
    }
    write_doubles(&compact.translation, output);
    write_doubles(&[compact.scaling], output);
    output.push(compact.flags);
    write_perspective(&compact.perspective, output);
}

fn write_fixed(fixed: &FixedTransform, output: &mut Vec<u8>) {
    write_doubles(&fixed.matrix, output);
    output.extend(
        fixed
            .unknown_48
            .iter()
            .flat_map(|value| value.to_le_bytes()),
    );
    write_doubles(&fixed.translation, output);
    write_doubles(&[fixed.scaling], output);
    output.extend(
        fixed
            .unknown_70
            .iter()
            .flat_map(|value| value.to_le_bytes()),
    );
}

fn write_doubles(values: &[StoredDouble], output: &mut Vec<u8>) {
    for value in values {
        output.extend_from_slice(&value.bits().to_le_bytes());
    }
}

fn write_base(base: &PerspectiveBase, output: &mut Vec<u8>) {
    write_doubles(&base.eye_point, output);
    write_doubles(&[base.plane_distance], output);
}

fn write_perspective(perspective: &PerspectivePayload, output: &mut Vec<u8>) {
    match perspective {
        PerspectivePayload::None => (),
        PerspectivePayload::Base(base) => write_base(base, output),
        PerspectivePayload::Extended {
            base,
            minimum_far_clip,
            camera_fov_plane,
        } => {
            write_base(base, output);
            write_doubles(&[*minimum_far_clip, *camera_fov_plane], output);
        }
        PerspectivePayload::Matrix5 { discarded, matrix } => {
            output.extend_from_slice(&discarded.to_le_bytes());
            write_doubles(matrix.as_ref(), output);
        }
    }
}

fn invalid(cause: String) -> BessemerError {
    BessemerError {
        operation: "display-list transform",
        entity: "mgXform_c".into(),
        class: FailureClass::Invalid,
        cause,
    }
}
