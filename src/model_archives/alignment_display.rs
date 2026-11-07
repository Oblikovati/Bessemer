use super::display_records::declared;
use super::{ModelDecoder, component_references};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Retain the native order-dimension suffix after its distance-style shared body.
/// Example: `alignment_display::suffix(reader, map)?` for `moDisplayOrdDim_c`.
pub(super) fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "shared_order_ff0", &["moDisplayOrdDim_c"])?;
    declared(reader, map, "list_ff8", &["suObList"])?;
    numeric_prefix(reader)?;
    declared(
        reader,
        map,
        "component_1000",
        component_references::FEATURE_CLASSES,
    )?;
    reader.i32s(&["raw_1040", "raw_1044"])?;
    references(reader, map)?;
    reader.i32("raw_fe8")?;
    reader.f64("raw_1048")
}

fn numeric_prefix(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.u16("raw_1008")?;
    reader.f64s(&["raw_1028", "raw_1030"])?;
    reader.u16("raw_1038")?;
    reader.u16("raw_103c")?;
    reader.u8("raw_1010")?;
    Ok(())
}

fn references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "reference_fb8", &["moDispObjRef_c"])?;
    for index in 0..reader.unsigned_count("references_fc8.count", 2)? {
        declared(
            reader,
            map,
            &format!("references_fc8[{index}]"),
            &["moDispObjRef_c"],
        )?;
    }
    Ok(())
}
