use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists, profile_features};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Retain the complete inherited feature and its typed data pointer; e.g. `read_mold_feature(reader, map)?`.
pub(super) fn read_mold_feature(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("body", |reader| body_features::base(reader, map))?;
    declared(reader, map, "data_800", &["moMoldCoreCavitySolidsData_c"])
}

/// Preserve the owner, direct lists and dimension pointers; e.g. `read_mold_solids_data(reader, map)?`.
pub(super) fn read_mold_solids_data(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "owner_08", body_features::CLASSES)?;
    direct_lists(reader, map)?;
    declared(reader, map, "profile_50", profile_features::CLASSES)?;
    reader.i32s(&["raw_30", "raw_34"])?;
    for offset in ["58", "60"] {
        declared(
            reader,
            map,
            &format!("dimension_{offset}"),
            &["moDisplayDistanceDim_c"],
        )?;
    }
    declared(reader, map, "dimension_68", &["moDisplayAngularDim_c"])
}

fn direct_lists(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    // #215 / C-3006/C-3552: the collections are direct; current list APIs do not narrow item classes.
    for offset in ["70", "b8", "100"] {
        model_lists::pointers(reader, map, &format!("list_{offset}"), ModelObjectRole::Any)?;
    }
    Ok(())
}
