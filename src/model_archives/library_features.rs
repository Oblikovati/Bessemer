use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, library_references, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Preserve complete ordinary library features without resolving external documents.
/// Example: `library_features::read(reader, map)?` for `moLibraryFeature_c`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body_feature", |reader| body_features::base(reader, map))?;
    let mode = reader.u16("raw_ba0")?;
    direct_list(reader, map, "list_800")?;
    if mode == 0 {
        conditional(reader, map)?;
    }
    tail(reader, map)
}

fn direct_list(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader, label: &str) -> Result<()> {
    model_lists::pointers(reader, map, label, ModelObjectRole::Any)
}

fn conditional(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    direct_list(reader, map, "array_b98")?;
    reader.u32("time_cc0")?;
    reader.text("raw_ba8")?;
    for label in ["raw_bb0", "raw_c58", "raw_bb4", "raw_c5c"] {
        reader.u16(label)?;
    }
    reader.i32("raw_b88")?;
    double_vector(reader, "point_c88")?;
    direct_list(reader, map, "array_c60")?;
    declared(reader, map, "plane_da0", &["moXRefPlaneData_c"])
}

fn tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.text("raw_da8")?;
    versioned_words(reader, map)?;
    if reader.version() > 0x2b60 {
        component_map(reader)?;
    }
    atom_records(reader, map)?;
    if reader.version() > 0x42df {
        reader.u8("raw_e10")?;
    }
    Ok(())
}

fn versioned_words(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() <= 0x2b5c {
        return Ok(());
    }
    reader.i32s(&["raw_dc0", "raw_dd0"])?;
    declared(
        reader,
        map,
        "sketch_handle_dc8",
        &["moCompSketchEntHandle_c"],
    )?;
    if reader.version() > 0x2b5f {
        reader.i32s(&["raw_dd4", "raw_dd8"])?;
    }
    Ok(())
}

fn component_map(reader: &mut ModelDecoder<'_>) -> Result<()> {
    for index in 0..reader.signed_count("component_map_e18.count", 12)? {
        reader.scope(&format!("component_map_e18[{index}]"), |reader| {
            reader.i32s(&["first", "second", "third"])
        })?;
    }
    Ok(())
}

fn atom_records(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x2b7b && reader.version() < 0x2bac {
        reader.scope("legacy_atom_ids", |reader| {
            library_references::atom_ids(reader, map)
        })?;
    }
    if reader.version() > 0x2bab {
        for index in 0..reader.signed_count("atom_nodes.count", 2)? {
            declared(
                reader,
                map,
                &format!("atom_nodes[{index}]"),
                &["moAtomIdToFeatIdNode_c"],
            )?;
        }
    }
    Ok(())
}
