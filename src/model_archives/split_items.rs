use super::{ModelDecoder, ModelObjectRole, kernel_frames, model_lists};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.text("name_08")?;
    super::display_records::declared(reader, map, "visual_10", &["moVisualProperties_c"])?;
    if reader.i32("body_presence")? != 0 {
        body_and_attributes(reader, map)?;
    }
    for label in ["transform_20", "transform_a8"] {
        super::fixed_transform_records::read(reader, label, "split item")?;
    }
    Ok(())
}

fn body_and_attributes(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let state = reader.scope("body_18", kernel_frames::read_body_state)?;
    if state == kernel_frames::KernelBodyState::Suppressed {
        return reader.checked(Err(BessemerError::new(
            "model archive",
            "split item body_18",
            FailureClass::Unsupported,
            "split item body state Suppressed; expected Serialized compressed single-body kernel partition before attributes and transforms",
        )));
    }
    model_lists::pointers(reader, map, "attributes_138", ModelObjectRole::Any)
}
