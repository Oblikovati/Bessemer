use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists, profile_geometry};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) const CLASSES: &[&str] = &[
    "moProfileFeature_c",
    "mo3DProfileFeature_c",
    "moOriginProfileFeature_c",
];

pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("model_feature", |reader| {
        body_features::model_feature(reader, map)
    })?;
    reader.i32("absolute_reference_plane")?;
    reader.object("pattern_feature", map, ModelObjectRole::Any)?;
    if reader.i32("reused")? != 0 {
        return reused(reader, map);
    }
    declared(reader, map, "sketch", &["sgSketch"])?;
    reader.scope("chains", |reader| {
        super::profile_chains::collection(reader, map)
    })?;
    reader.scope("geometry", |reader| profile_geometry::geometry(reader, map))?;
    reader.scope("contours", |reader| contours(reader, map))?;
    reader.scope("placement", |reader| placement(reader, map))?;
    reader.scope("suffix", |reader| suffix(reader, map))
}

fn reused(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.required_object("source_profile", map, ModelObjectRole::Declared(CLASSES))?;
    declared(reader, map, "contour_chooser", &["moContourChooser_c"])?;
    reader.u16("raw_3b8")?;
    declared(reader, map, "selection_handle_3b0", &["sgLineHandle"])?;
    reader.scope("contours", |reader| contours(reader, map))
}

pub(super) fn contours(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.u32("word_3d8")?;
    for index in 0..reader.count_with_minimum("entries.count", 4)? {
        reader.scope(&format!("entries[{index}]"), |reader| {
            declared(reader, map, "contour", &["moOneContour_c"])?;
            let count = reader.u16("indices.count")?;
            let count = reader.counted_entries("contour indices", i64::from(count), 4)?;
            for index in 0..count {
                reader.u32(&format!("indices[{index}]"))?;
            }
            Ok(())
        })?;
    }
    Ok(())
}

fn placement(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "line_handle", &["sgLineHandle"])?;
    declared(reader, map, "reference_plane", &["moCompRefPlane_c"])?;
    reader.transform("transform")?;
    if reader.u16("has_transform_e8")? != 0 {
        reader.transform("transform_e8")?;
    }
    reader.u16("word_100")?;
    declared(reader, map, "profile_reference", &["moProfileRef_w"])
}

fn suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.i32s(&["word_11c", "word_118"])?;
    reader.u32("word_2a0")?;
    reader.u16("word_3b8")?;
    declared(reader, map, "contour_chooser", &["moContourChooser_c"])?;
    for label in ["array_128", "array_148"] {
        reader.scope(label, |reader| {
            model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
        })?;
    }
    if reader.version() >= 0x2b92 {
        reader.i32("word_424")?;
    }
    Ok(())
}
