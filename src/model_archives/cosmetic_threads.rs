use super::component_references::FEATURE_CLASSES;
use super::display_records::declared;
use super::geometric_references::SURFACE_CLASSES;
use super::{ModelDecoder, body_features, sketch_arrays};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn feature(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("model_feature", |reader| {
        body_features::model_feature(reader, map)
    })?;
    reader.scope("suffix", |reader| suffix(reader, map))
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "cylinder", &["moCylinderRef_w"])?;
    declared(reader, map, "edge_370", &["moCompEdge_c"])?;
    declared(reader, map, "data", &["moCThreadData_c"])?;
    reader.text("text_3b0")?;
    declared(reader, map, "dimension_3b8", &["moDisplayDistanceDim_c"])?;
    declared(reader, map, "dimension_3c0", &["moDisplayRadialDim_c"])?;
    reader.i32("cache_word_180")?;
    declared(reader, map, "surface_388", SURFACE_CLASSES)?;
    suffix_indices(reader, map)
}

fn suffix_indices(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&["word_360", "word_418", "word_41c"])?;
    let count = reader.signed_count("indices_5a0.count", 4)?;
    sketch_arrays::integers(reader, "indices_5a0", count)?;
    declared(reader, map, "reference_398", &["moRefWrapper_c"])?;
    reader.f64("value_3a0")?;
    declared(reader, map, "component_feature", FEATURE_CLASSES)?;
    versioned(reader, map)
}

fn versioned(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.version() > 0x27cf {
        declared(reader, map, "edge_380", &["moCompEdge_c"])?;
        declared(reader, map, "surface_390", SURFACE_CLASSES)?;
    }
    if reader.version() > 0x27de {
        reader.i32("word_378")?;
    }
    if reader.version() > 0x2f6a {
        reader.i32("word_424")?;
    }
    if reader.version() > 0x39d0 {
        reader.i32s(&["word_699", "word_69a"])?;
    }
    Ok(())
}
