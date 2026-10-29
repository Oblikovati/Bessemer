use super::{ModelDecoder, ModelObjectRole, node};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn absent_payload(
    reader: &mut ModelDecoder<'_>,
    label: &str,
    present: i32,
) -> Result<()> {
    if present == 0 {
        return Ok(());
    }
    let error = BessemerError::new(
        "model archive",
        label,
        FailureClass::Unsupported,
        format!(
            "{label} presence {present}; expected zero until its complete payload is implemented"
        ),
    );
    reader.checked(Err(error))
}

/// Retain saved component references without resolving an external document.
/// Example: `part_objects::unit_component(reader, map)?`.
pub(super) fn unit_component(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    let external_owner = unit_owner_is_external(reader, map)?;
    let present = reader.u8("has_external_model")?;
    if external_owner || present != 0 {
        reader.require_verified_part_context()?;
    }
    if present != 0 {
        reader.required_object(
            "external_model",
            map,
            ModelObjectRole::Declared(&["moExtObject_c"]),
        )?;
    }
    Ok(())
}

fn unit_owner_is_external(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<bool> {
    let owner = reader.object_record(
        "owner",
        map,
        ModelObjectRole::Declared(&["moPart_c", "moExtObject_c"]),
    )?;
    Ok(matches!(owner,
        crate::archive_codec::ObjectRecord::New { class, .. }
        | crate::archive_codec::ObjectRecord::Reference { class, .. }
        if class.name == "moExtObject_c"))
}

pub(super) fn hatch(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.text("name")?;
    reader.f64s(&["angle", "scale"])?;
    reader.i32s(&[
        "override",
        "solid",
        "id",
        "layer_id",
        "command_layer_color",
        "randomize_scale",
    ])
}

pub(super) fn material(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object(
        "density_parameter",
        map,
        ModelObjectRole::Declared(&["moDensityParameter_c"]),
    )?;
    if reader.version() > 0x2b90 && reader.version() < 0x2bab {
        reader.f64s(&[
            "unknown_10",
            "unknown_18",
            "unknown_20",
            "unknown_28",
            "unknown_30",
            "unknown_38",
            "unknown_40",
        ])?;
    }
    Ok(())
}

pub(super) fn density_parameter(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    parameter_node(reader, map)?;
    reader.f64("value")?;
    if reader.u8("has_alternate_value")? != 0 {
        reader.f64("alternate_value")?;
    }
    Ok(())
}

pub(super) fn parameter_node(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("node", |reader| node::lightweight(reader, map))?;
    reader.object(
        "parameter_name",
        map,
        ModelObjectRole::Declared(&["moNodeName_c"]),
    )
}

pub(super) fn model_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.object("scene", map, ModelObjectRole::Declared(&["uoScene"]))?;
    reader.i32("unknown_10")?;
    Ok(())
}
