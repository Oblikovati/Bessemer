use super::{ModelDecoder, ModelObjectRole, body_features, model_lists, stock_collections};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader, vectors::double_vector};

/// Compose the full stock body and append a mirror plane only for the concrete subtype.
/// Example: `stock_features::read(reader, map, false)?`.
pub(super) fn read(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    mirrored: bool,
) -> Result<()> {
    reader.scope("body", |reader| body_features::base(reader, map))?;
    reader.scope("derived", |reader| stock_collections::derived(reader, map))?;
    source_body(reader, map)?;
    direct_members(reader, map)?;
    reader.scope("late", |reader| stock_collections::late(reader, map))?;
    if mirrored {
        reader.object(
            "mirror_plane_f20",
            map,
            ModelObjectRole::Declared(&["moPlaneRef_w"]),
        )?;
    }
    Ok(())
}

fn source_body(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.object(
        "body_reference_a00",
        map,
        ModelObjectRole::Declared(&["moBodyRef_w"]),
    )?;
    let mode = reader.u32("raw_968")?;
    if matches!(mode, 1 | 2) {
        reader.scope("kernel_body", super::kernel_frames::read_body)?;
    }
    reader.scope("configuration_a70", super::stock_references::configuration)
}

fn direct_members(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    for label in ["array_a10", "array_a88"] {
        model_lists::pointers(reader, map, label, ModelObjectRole::Any)?;
    }
    reader.i32("raw_96c")?;
    model_lists::pointers(reader, map, "array_a30", ModelObjectRole::Any)?;
    double_vector(reader, "point_c10")?;
    reader.i32("raw_cc4")?;
    stock_collections::pairs(reader, "pairs_cc8")?;
    model_lists::pointers(reader, map, "list_cf8", ModelObjectRole::Any)?;
    super::fixed_transform_records::read(reader, "transform_c30.words", "stock")
}
