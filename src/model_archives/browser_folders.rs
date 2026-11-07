use super::{ModelDecoder, ModelObjectRole, features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn item(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("node", |reader| {
        super::node::lightweight(reader, map).map(|_| ())
    })
}

pub(super) fn feature_item(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    item(reader, map)?;
    reader.object(
        "feature",
        map,
        ModelObjectRole::Declared(super::component_references::FEATURE_CLASSES),
    )
}

pub(super) fn live_section(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::folder(reader, map)?;
    reader.i32("raw_370")?;
    Ok(())
}

pub(super) fn favorite(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::folder(reader, map)?;
    model_lists::pointers(reader, map, "favorite_items", ModelObjectRole::Any)?;
    model_lists::pointers(reader, map, "subfolders", ModelObjectRole::Any)?;
    reader.i32("unknown_400")?;
    Ok(())
}

pub(super) fn history(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::folder(reader, map)?;
    model_lists::pointers(reader, map, "history_items", ModelObjectRole::Any)
}

pub(super) fn selection_sets(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::folder(reader, map)?;
    model_lists::pointers(reader, map, "selection_sets", ModelObjectRole::Any)?;
    reader.i32("unknown_3d8")?;
    Ok(())
}

/// Preserve selection-set words, comments and direct ordered item identities.
/// Example: `browser_folders::selection_set_data(reader, map)?`.
pub(super) fn selection_set_data(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    item(reader, map)?;
    reader.i32("raw_40")?;
    reader.text("raw_48")?;
    if reader.version() > 0x3720 {
        reader.object(
            "comment",
            map,
            ModelObjectRole::Declared(&["moFeatComment_c", "moNewFeatComment_c"]),
        )?;
    }
    model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
}

/// Retain the selection item's node stage and shared wrapper reference.
/// Example: `browser_folders::selection_item_data(reader, map)?`.
pub(super) fn selection_item_data(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    item(reader, map)?;
    reader.object(
        "reference_40",
        map,
        ModelObjectRole::Declared(super::model_references::WRAPPER_CLASSES),
    )
}

pub(super) fn body_folder(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::folder(reader, map)?;
    reader.i32s(&["raw_374", "raw_378"])?;
    reader.object(
        "material_folder",
        map,
        ModelObjectRole::Declared(&["moMaterialFolder_c"]),
    )?;
    reader.object(
        "cut_list_sort_options",
        map,
        ModelObjectRole::Declared(&["moCutListSortOptions_c"]),
    )
}

pub(super) fn material(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    features::folder(reader, map)?;
    reader.text("raw_370")?;
    reader.u32("raw_378_37c")?;
    Ok(())
}

pub(super) fn cut_list_sort(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.i32("raw_08")?;
    model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
}

pub(super) fn detail_cabinet(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("model_feature", |reader| {
        super::body_features::model_feature(reader, map)
    })?;
    model_lists::pointers(reader, map, "array_360", ModelObjectRole::Any)?;
    reader.f64s(&["raw_368", "raw_370"])?;
    model_lists::pointers(reader, map, "array_380", ModelObjectRole::Any)?;
    for index in 0..reader.signed_count("imported_annotations.count", 2)? {
        reader.object(
            &format!("imported_annotations[{index}]"),
            map,
            ModelObjectRole::Declared(&["moDisplayImportedAnnotation_c"]),
        )?;
    }
    if reader.archive_version().value() > 0x2b9a {
        reader.i32("raw_3e8")?;
    }
    Ok(())
}
