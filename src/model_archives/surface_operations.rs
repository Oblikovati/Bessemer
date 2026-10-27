use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

pub(super) fn cut(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    declared(reader, map, "atom_feature_808", body_features::CLASSES)?;
    reader.i32s(&["raw_818", "raw_81c"])?;
    declared(reader, map, "face_820", &["moFaceRef_c"])?;
    declared(reader, map, "surface_810", &["moConstSurfRef_w"])?;
    declared(reader, map, "plane_900", &["moPlaneRef_w"])?;
    model_lists::pointers(reader, map, "array_8c0", ModelObjectRole::Any)?;
    model_lists::pointers(reader, map, "array_8e0", ModelObjectRole::Any)?;
    reader.scope("chooser_838", |reader| {
        body_features::direct_chooser(reader, map)
    })
}

pub(super) fn trim(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("trim_data_800", |reader| trim_prefix(reader, map))?;
    reader.scope("chooser_808", |reader| {
        body_features::direct_chooser(reader, map)
    })?;
    reader.scope("trim_data_800", |reader| trim_suffix(reader, map))
}

fn trim_prefix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["array_20", "array_48"] {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    declared(reader, map, "profile_18", super::profile_features::CLASSES)?;
    declared(reader, map, "direction_40", &["moDirectionSpec_c"])?;
    reader.u32("raw_10")?;
    Ok(())
}

fn trim_suffix(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    declared(reader, map, "solid_68", &["moSolidRef_w"])?;
    for label in ["entity_keeper_78", "array_a0"] {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    reader.i32s(&["raw_08", "raw_0c"])
}
