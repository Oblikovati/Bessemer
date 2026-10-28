use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, component_references, part_objects};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

/// Preserve the complete single-bend body, direct handles and bounded raw-body stops.
/// Example: `one_bends::read(reader, map)?` for the native single-bend class.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("common_bend_800", |reader| {
        super::sheet_payload::common(reader, map)
    })?;
    reader.i32s(&["raw_840", "raw_844", "raw_848"])?;
    let has_sketch = first_sketch(reader, map)?;
    prefix(reader, map)?;
    ordinary_handle(reader, map)?;
    middle(reader, map)?;
    reader.object("reference_a98", map, ModelObjectRole::Any)?;
    components(reader, map, &["ab0", "ab8"])?;
    if has_sketch {
        reader.scope("direct_handle_878", super::sketch_references::handle)?;
    }
    reader.scope("body_tail", tail)
}

fn first_sketch(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<bool> {
    let record = reader.object_record("sketch_878", map, ModelObjectRole::Any)?;
    Ok(!matches!(record, crate::archive_codec::ObjectRecord::Null))
}

fn prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object("reference_860", map, ModelObjectRole::Any)?;
    declared(
        reader,
        map,
        "angular_dimension_868",
        &["moDisplayAngularDim_c"],
    )?;
    declared(reader, map, "dimension_870", DIMENSIONS)?;
    reader.u16("raw_858")?;
    reader.i32("raw_8f8")?;
    reader.f64("raw_8b8")?;
    reader.i32s(&["raw_8c0", "raw_8c8", "raw_8cc", "raw_8c4", "raw_8d0"])?;
    for offset in ["8d8", "8e0"] {
        declared(reader, map, &format!("dimension_{offset}"), DIMENSIONS)?;
    }
    reader.i32("raw_8e8")?;
    reader.u8("raw_a10")?;
    Ok(())
}

fn ordinary_handle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.sheet_metal_extended() {
        return Ok(());
    }
    let sketch = reader.object_record("sketch_880", map, ModelObjectRole::Any)?;
    if !matches!(sketch, crate::archive_codec::ObjectRecord::Null) {
        reader.scope("direct_handle_880", super::sketch_references::handle)?;
    }
    declared(reader, map, "list_888", &["suObList"])
}

fn middle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object("reference_a90", map, ModelObjectRole::Any)?;
    components(reader, map, &["aa0", "aa8"])?;
    reader.i32("raw_ac8")?;
    declared(reader, map, "conic_ad0", &["moConicBendData_c"])?;
    reader.transform("transform_900")?;
    reader.transform("transform_988")?;
    reader.i32s(&["raw_ad8", "raw_adc", "raw_a30", "raw_b64", "raw_a34"])
}

fn components(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    offsets: &[&str],
) -> Result<()> {
    for offset in offsets {
        declared(
            reader,
            map,
            &format!("component_{offset}"),
            component_references::FEATURE_CLASSES,
        )?;
    }
    Ok(())
}

fn tail(reader: &mut ModelDecoder<'_>) -> Result<()> {
    nonpositive_count(reader, "raw_body_count_ae0")?;
    if reader.version() > 0x335a {
        nonpositive_count(reader, "raw_body_count_b00")?;
    }
    for (offset, gate) in [("b40", 0x334c), ("b50", 0x334f)] {
        if reader.version() >= gate {
            let label = format!("body_presence_{offset}");
            let present = reader.i32(&label)?;
            part_objects::absent_payload(reader, &label, present)?;
        }
    }
    if reader.version() > 0x3734 {
        reader.i32("raw_a88")?;
    }
    reader.i32s(&["raw_a58", "raw_a5c", "raw_d54"])
}

fn nonpositive_count(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    let count = reader.i32(label)?;
    if count <= 0 {
        return Ok(());
    }
    reader.checked(Err(BessemerError::new(
        "model archive",
        "single-bend raw body",
        FailureClass::Unsupported,
        format!(
            "{label} count {count}; expected <= 0 until positive raw-kernel bodies are verified"
        ),
    )))
}
