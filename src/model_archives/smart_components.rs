use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

const INITIAL_FOLDERS: &[(&str, &str)] = &[
    ("feature_folder_e8", "moSmartComponentFeatFolder_c"),
    ("component_folder_e0", "moSmartComponentCompFolder_c"),
];

/// Require the native data object before the optional saved external reference.
/// Example: `smart_components::read(reader, map)?`.
pub(super) fn read(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("model_feature", |reader| {
        body_features::model_feature(reader, map)
    })?;
    reader.required_object(
        "data_368",
        map,
        ModelObjectRole::Declared(&["moSmartComponentFeatureData_c"]),
    )?;
    declared(reader, map, "external_378", &["moExtObject_c"])
}

/// Preserve direct member containers in native order without pointer substitutes.
/// Example: `smart_components::feature_data(reader, map)?`.
pub(super) fn feature_data(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    array(reader, map, "array_48")?;
    array(reader, map, "array_68")?;
    declared(reader, map, "context_88", &["moContextChainComponent_c"])?;
    array(reader, map, "list_90")?;
    data_tail(reader, map)
}

fn data_tail(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for (label, class) in INITIAL_FOLDERS {
        declared(reader, map, label, &[class])?;
    }
    reader.i32s(&["raw_f8", "raw_fc"])?;
    reader.scope("sensor_map_100", |reader| sensor_map(reader, map))?;
    declared(
        reader,
        map,
        "reference_folder_f0",
        &["moSmartComponentRefFolder_c"],
    )?;
    reader.i32("raw_130").map(|_| ())
}

/// Keep the complete inherited folder before its direct object array.
/// Example: `smart_components::folder(reader, map)?`.
pub(super) fn folder(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("folder", |reader| features::folder(reader, map))?;
    array(reader, map, "array_3b8")
}

/// Retain ordered duplicate keys and null values using ordinary ReadCount.
/// Example: `smart_components::sensor_map(reader, map)?`.
pub(super) fn sensor_map(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    for index in 0..reader.count_with_minimum("entries.count", 3)? {
        reader.text(&format!("entries[{index}].key"))?;
        reader.object(
            &format!("entries[{index}].object"),
            map,
            ModelObjectRole::Any,
        )?;
    }
    Ok(())
}

/// Preserve a direct native container without allocating another archive identity.
/// Example: `smart_components::array(reader, map, "array_48")?`.
pub(super) fn array(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    reader.scope(label, |reader| {
        model_lists::pointers(reader, map, "items", ModelObjectRole::Any)
    })
}
