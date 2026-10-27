use super::{ModelDecoder, ModelObjectRole, sketch_arrays};
use crate::Result;
use crate::archive_codec::{ObjectReader, ObjectRecord};
use crate::{BessemerError, FailureClass};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    let kind = reader.u16("stored_kind")?;
    if kind == 2 {
        return sketch_arrays::doubles(reader, "plane_18", 6);
    }
    reference(reader, map, kind)
}

fn reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, kind: u16) -> Result<()> {
    let role = if kind == 1 {
        ModelObjectRole::Declared(&["moCompFace_c"])
    } else {
        ModelObjectRole::Any
    };
    let reference = reader.object_record("reference_10", map, role)?;
    if kind == 0 || kind == 1 || matches!(reference, ObjectRecord::Null) {
        return Ok(());
    }
    reader.checked(Err(BessemerError::new(
        "model archive", "dimension-plane tag", FailureClass::Unsupported,
        format!("stored dimension-plane kind {kind} with nonnull reference; expected kind 0, kind 1 face, kind 2 inline plane or a null reference"),
    )))
}
