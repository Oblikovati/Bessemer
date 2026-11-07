use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, model_lists, model_references, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Keep configuration keys at their stored version-dependent width and order.
/// Example: `library_references::configuration(reader, map)?`.
pub(super) fn configuration(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.u16("raw_20")?;
    reader.text("raw_18")?;
    reader.u32("raw_68")?;
    for index in 0..reader.signed_count("features_70.count", 4)? {
        reader.scope(&format!("features_70[{index}]"), |reader| {
            configuration_entry(reader, map)
        })?;
    }
    model_lists::pointers(reader, map, "array_28", ModelObjectRole::Any)?;
    model_lists::unsigned_words(reader, "words_48")
}

fn configuration_entry(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x2bc8 {
        reader.i32("key")?;
    } else {
        reader.u16("key")?;
    }
    declared(reader, map, "feature", &["moXFeatureData_c"])
}

/// Retain native four-byte time fields without interpreting their epoch.
/// Example: `library_references::feature(reader)?`.
pub(super) fn feature(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.text("raw_8")?;
    reader.u16("raw_10")?;
    reader.u32("time_18")?;
    reader.u16("raw_20")?;
    reader.u32("time_28")?;
    if reader.version() > 0x2bc8 {
        reader.i32("raw_40")?;
    }
    Ok(())
}

/// Retain the three uninterpreted plane words; e.g. `library_references::plane(reader)?`.
pub(super) fn plane(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32s(&["raw_8", "raw_c", "raw_10"])
}

/// Preserve cross-reference identities and raw point bits.
/// Example: `library_references::reference(reader, map)?`.
pub(super) fn reference(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.u16("raw_40")?;
    reader.text("raw_48")?;
    declared(
        reader,
        map,
        "reference_10",
        model_references::WRAPPER_CLASSES,
    )?;
    reader.object("object_8", map, ModelObjectRole::Any)?;
    reader.u8("raw_30")?;
    double_vector(reader, "point_18")
}

/// Preserve the node's typed identifier collection; e.g. `atom_node(reader, map)?`.
pub(super) fn atom_node(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_8")?;
    declared(reader, map, "ids_10", &["moAtomIdToFeatIds_c"])
}

/// Keep duplicate IDs and nested maps without applying runtime remaps.
/// Example: `library_references::atom_ids(reader, map)?`.
pub(super) fn atom_ids(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    signed_words(reader, "ids")?;
    for index in 0..reader.signed_count("map_18.count", 8)? {
        reader.scope(&format!("map_18[{index}]"), |reader| {
            reader.i32("key")?;
            signed_words(reader, "values")
        })?;
    }
    if reader.version() > 0x2b7e {
        for index in 0..reader.signed_count("bodies_28.count", 2)? {
            declared(
                reader,
                map,
                &format!("bodies_28[{index}]"),
                &["moCompSolidBody_c"],
            )?;
        }
    }
    Ok(())
}

fn signed_words(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    let count = reader.signed_count(&format!("{label}.count"), 4)?;
    sketch_arrays::integers(reader, label, count)
}
