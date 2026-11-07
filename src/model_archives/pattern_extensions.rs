use super::display_records::declared;
use super::{
    ModelDecoder, ModelObjectRole, body_features, configuration_io, linear_patterns,
    model_references,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Retain the complete common pattern before its table-specific direct members.
/// Example: `pattern_extensions::table(reader, map)?` for `moTablePattern_c`.
pub(super) fn table(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("pattern", |reader| linear_patterns::common(reader, map))?;
    reader.scope("payload", |reader| table_payload(reader, map))
}

fn table_payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "coordinate_1140", &["moCoordSysRef_w"])?;
    declared(
        reader,
        map,
        "reference_1148",
        model_references::WRAPPER_CLASSES,
    )?;
    reader.scope("array_1150", |reader| {
        body_features::nullable_array(reader, map)
    })?;
    double_vector(reader, "point_1130")
}

/// Select the sketch map from the containing node, without searching nested fields.
/// Example: `pattern_extensions::sketch(reader, map)?` for `moSketchPattern_c`.
pub(super) fn sketch(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    let node = reader.scope("pattern", |reader| {
        linear_patterns::common_with_variation(reader, map, super::pattern_variation::read)
    })?;
    let flags = reader.checked(node.unknown_2c.ok_or_else(|| {
        super::invalid("pattern node flags None; expected stored modern u32 unknown_2c")
    }))?;
    reader.scope("payload", |reader| sketch_payload(reader, map, flags))
}

fn sketch_payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, flags: u32) -> Result<()> {
    declared(reader, map, "profile_1130", &["moProfileRef_w"])?;
    declared(
        reader,
        map,
        "reference_1138",
        model_references::WRAPPER_CLASSES,
    )?;
    double_vector(reader, "point_1140")?;
    reader.scope("transforms_1148", super::pattern_transform_words::read)?;
    sketch_pairs(reader, flags)
}

fn sketch_pairs(reader: &mut ModelDecoder<'_>, flags: u32) -> Result<()> {
    if flags & 0x800 == 0 {
        return Ok(());
    }
    reader.i32("raw_1150")?;
    for index in 0..configuration_io::positive_count(reader, "declared_count_1158", 8)? {
        reader.i32(&format!("pairs_1158[{index}][0]"))?;
        reader.i32(&format!("pairs_1158[{index}][1]"))?;
    }
    Ok(())
}

/// Preserve the range descriptor's raw byte and nullable complete display target.
/// Example: `pattern_extensions::all_range(reader, map)?`.
pub(super) fn all_range(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(
        reader,
        map,
        "dimension_08",
        super::display_dimension::CLASSES,
    )?;
    reader.u8("raw_10").map(|_| ())
}

/// Preserve a dimension-pattern instance with its inherited body and typed parent.
/// Example: `pattern_extensions::instance(reader, map)?`.
pub(super) fn instance(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("payload", |reader| instance_payload(reader, map))
}

fn instance_payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32("raw_800")?;
    reader.object(
        "pattern_808",
        map,
        ModelObjectRole::Declared(&["moDimPattern_c"]),
    )?;
    reader.i32s(&["raw_810", "raw_814", "raw_804"])
}
