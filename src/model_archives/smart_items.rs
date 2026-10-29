use super::display_records::declared;
use super::{ModelDecoder, node, smart_components};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Use the node's byte-free resolved hook instead of a feature concept body.
/// Example: `smart_items::base(reader, map)?`.
pub(super) fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader
        .scope("node", |reader| node::lightweight(reader, map))
        .map(|_| ())
}

/// Retain both strings and the original nullable smart-feature owner identity.
/// Example: `smart_items::feature(reader, map)?`.
pub(super) fn feature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    declared(reader, map, "feature_40", &["moCompFeature_c"])?;
    reader.text("string_50")?;
    reader.i32("raw_58")?;
    declared(
        reader,
        map,
        "smart_feature_60",
        &["moSmartComponentFeature_c"],
    )?;
    reader.text("string_48")
}

/// Preserve separate transform pointers around the owner and mixed scalar fields.
/// Example: `smart_items::component(reader, map)?`.
pub(super) fn component(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    declared(reader, map, "transform_40", &["moCompXFormWrapper_c"])?;
    declared(
        reader,
        map,
        "smart_feature_60",
        &["moSmartComponentFeature_c"],
    )?;
    reader.text("string_50")?;
    reader.i32("raw_58")?;
    declared(reader, map, "transform_48", &["moCompXFormWrapper_c"])
}

/// Keep the reference item's direct array after all three stored strings.
/// Example: `smart_items::reference(reader, map)?`.
pub(super) fn reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    base(reader, map)?;
    reader.text("string_68")?;
    reader.text("string_70")?;
    declared(
        reader,
        map,
        "smart_feature_78",
        &["moSmartComponentFeature_c"],
    )?;
    reader.text("string_60")?;
    smart_components::array(reader, map, "array_40")
}
