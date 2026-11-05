use super::display_dimension::CLASSES as DIMENSIONS;
use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, component_references, kernel_frames};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Preserve the complete single-bend body, direct handles and framed raw bodies.
/// Example: `one_bends::read(reader, map)?` for the native single-bend class.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    read_kind(reader, map).map(|_| ())
}

/// Return the retained bend kind for a concrete subclass's continuation.
/// Example: `let kind = one_bends::read_kind(reader, map)?;`.
pub(super) fn read_kind(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<i32> {
    let has_sketch = body_prefix(reader, map)?;
    ordinary_handle(reader, map)?;
    let kind = middle(reader, map)?;
    reader.object("reference_a98", map, ModelObjectRole::Any)?;
    components(reader, map, &["ab0", "ab8"])?;
    if has_sketch {
        reader.scope("direct_handle_878", super::sketch_references::handle)?;
    }
    reader.scope("body_tail", tail)?;
    Ok(kind)
}

fn body_prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<bool> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("common_bend_800", |reader| {
        super::sheet_payload::common(reader, map)
    })?;
    reader.i32s(&["raw_840", "raw_844", "raw_848"])?;
    let has_sketch = first_sketch(reader, map)?;
    prefix(reader, map)?;
    Ok(has_sketch)
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

fn middle(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<i32> {
    reader.object("reference_a90", map, ModelObjectRole::Any)?;
    components(reader, map, &["aa0", "aa8"])?;
    let kind = reader.i32("raw_ac8")?;
    declared(reader, map, "conic_ad0", &["moConicBendData_c"])?;
    reader.transform("transform_900")?;
    reader.transform("transform_988")?;
    reader.i32s(&["raw_ad8", "raw_adc", "raw_a30", "raw_b64", "raw_a34"])?;
    Ok(kind)
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
    raw_bodies(reader, "ae0")?;
    if reader.version() > 0x335a {
        raw_bodies(reader, "b00")?;
    }
    for (offset, gate) in [("b40", 0x334c), ("b50", 0x334f)] {
        if reader.version() >= gate {
            optional_body(reader, offset)?;
        }
    }
    if reader.version() > 0x3734 {
        reader.i32("raw_a88")?;
    }
    reader.i32s(&["raw_a58", "raw_a5c", "raw_d54"])
}

fn raw_bodies(reader: &mut ModelDecoder<'_>, offset: &str) -> Result<()> {
    let label = format!("raw_body_count_{offset}");
    let count = super::configuration_io::positive_count(reader, &label, 4)?;
    for index in 0..count {
        reader.scope(
            &format!("raw_bodies_{offset}[{index}]"),
            kernel_frames::read_raw_body,
        )?;
    }
    Ok(())
}

fn optional_body(reader: &mut ModelDecoder<'_>, offset: &str) -> Result<()> {
    if reader.i32(&format!("body_presence_{offset}"))? == 0 {
        return Ok(());
    }
    reader.scope(&format!("body_{offset}"), kernel_frames::read_raw_body)
}
