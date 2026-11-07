use super::{ModelDecoder, ModelObjectRole, model_lists, node};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("node", |reader| node::lightweight(reader, map))?;
    if reader.i32("has_merged_list")? != 0 {
        reader.scope("merged_list", |reader| model_lists::nodes(reader, map))?;
    }
    reader.object(
        "visual_properties",
        map,
        ModelObjectRole::Declared(&["moVisualProperties_c"]),
    )?;
    reader.scope("feature_list", |reader| model_lists::nodes(reader, map))?;
    reader.i32("atom_id")?;
    reader.u32("unknown_13c")?;
    reader.i32s(&["unknown_128", "unknown_124"])?;
    reader.u32("unknown_150")?;
    body_mode(reader)?;
    reader.i32s(ATOM_TAIL)
}

const ATOM_TAIL: &[&str] = &[
    "unknown_168",
    "unknown_184",
    "mold_body_type",
    "unknown_178",
    "unknown_180",
    "unknown_1a8",
    "unknown_194",
];

fn body_mode(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let mode = reader.i32("body_mode")?;
    if mode == 1 {
        let error = BessemerError::new(
            "model archive",
            "atom body mode",
            FailureClass::Unsupported,
            format!(
                "atom body mode {mode}; expected an implemented body mode without embedded kernel data"
            ),
        );
        return reader.checked(Err(error));
    }
    if mode == 2 {
        reader.i32("body_reference")?;
    }
    Ok(())
}
