use super::display_records::declared;
use super::{
    ModelDecoder, ModelObjectRole, body_features, mate_plane_collections, model_references,
};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Retain complete ordinary concentric mates without applying runtime reference remaps.
/// Example: `concentric_mates::read(reader, map)?` for `moMateConcentric`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("model_feature", |reader| {
        body_features::model_feature(reader, map)
    })?;
    reader.scope("mate", |reader| base(reader, map))?;
    reader.scope("payload", |reader| payload(reader, map))
}

fn base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    numeric_base(reader)?;
    reader.u16("raw_3ac")?;
    reader.u16("raw_3b0")?;
    if reader.u8("has_vector_3b8")? != 0 {
        double_vector(reader, "vector_3b8")?;
    }
    references(reader, map)
}

fn numeric_base(reader: &mut ModelDecoder<'_>) -> Result<()> {
    reader.i32s(&["raw_388", "raw_38c"])?;
    if reader.version() > 0x2bb4 {
        reader.i32("raw_380")?;
    }
    if reader.version() > 0x3afc {
        for index in 0..reader.signed_count("words_368.count", 4)? {
            reader.i32(&format!("words_368[{index}]"))?;
        }
    }
    Ok(())
}

fn references(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for index in 0..reader.signed_count("references_3c0.count", 6)? {
        reader.i32(&format!("references_3c0[{index}].key"))?;
        declared(
            reader,
            map,
            &format!("references_3c0[{index}].reference"),
            model_references::WRAPPER_CLASSES,
        )?;
    }
    Ok(())
}

fn payload(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    owned_pointers(reader, map)?;
    reader.u16("raw_430")?;
    reader.i32("raw_478")?;
    reader.text("raw_480")?;
    reader.text("raw_488")?;
    reader.i32("raw_4b0")?;
    limits(reader)
}

fn owned_pointers(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "array_428", &["suObArray"])?;
    mate_plane_collections::pointers(
        reader,
        map,
        "entities_408",
        ModelObjectRole::Declared(model_references::WRAPPER_CLASSES),
    )?;
    mate_plane_collections::pointers(
        reader,
        map,
        "lists_440",
        ModelObjectRole::Declared(&["suObList"]),
    )?;
    optional_data(reader, map)
}

fn optional_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for (offset, class) in [
        ("468", "moMateBushingData_c"),
        ("460", "moMateFrictionData_c"),
        ("470", "moMateAnalysisData_c"),
    ] {
        declared(reader, map, &format!("data_{offset}"), &[class])?;
    }
    Ok(())
}

fn limits(reader: &mut ModelDecoder<'_>) -> Result<()> {
    if reader.i32("raw_4bc")? == -1 {
        return Ok(());
    }
    reader.i32("raw_4c0")?;
    reader.f64s(&["raw_4c8", "raw_4d0"])?;
    reader.i32s(&["raw_4d8_id", "raw_4e0_id"])
}
