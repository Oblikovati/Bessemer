use super::{ModelDecoder, ModelObjectRole, kernel_frames, model_lists};
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};
use crate::{BessemerError, FailureClass, Result};

/// Retain direct plane, box and body members without inventing object-map slots.
/// Example: `plane_section_geometry::read(reader, map)?` for section-owned geometry.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    double_vector(reader, "plane.root_point")?;
    double_vector(reader, "plane.normal")?;
    super::configuration_scenes::bounds(reader)?;
    for index in 0..reader.signed_count("bodies.count", 1)? {
        reader.scope(&format!("bodies[{index}]"), |reader| {
            reader.scope("body", serialized_body)?;
            model_lists::pointers(reader, map, "attributes", ModelObjectRole::Any)
        })?;
    }
    for index in 0..4 {
        reader.f64(&format!("border[{index}]"))?;
    }
    double_vector(reader, "display_root_point")
}

/// Reject bodies that cannot establish the documented following attribute layout.
/// Example: `serialized_body(reader)?` before reading section attributes.
pub(super) fn serialized_body(reader: &mut ModelDecoder<'_>) -> Result<()> {
    let state = kernel_frames::read_body_state(reader)?;
    if state == kernel_frames::KernelBodyState::Serialized {
        return Ok(());
    }
    reader.checked(Err(BessemerError::new(
        "model archive",
        "plane-section body",
        FailureClass::Unsupported,
        format!(
            "kernel body {state:?}; expected a serialized body with documented section framing"
        ),
    )))
}
