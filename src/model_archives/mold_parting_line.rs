use super::display_records::declared;
use super::{ModelDecoder, ModelObjectRole, body_features, model_lists};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};

/// Retain the complete feature and counted kernel frames; e.g. `read_parting_line_feature(reader, map)?`.
pub(super) fn read_parting_line_feature(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    reader.scope("body", |reader| body_features::base(reader, map))?;
    declared(reader, map, "data_800", &["moMoldPartingLineData_c"])?;
    reader.i32("raw_af0")?;
    let count = super::configuration_io::positive_count(reader, "bodies_af4.count", 1)?;
    for index in 0..count {
        reader.scope(
            &format!("bodies_af4[{index}]"),
            super::kernel_frames::read_body,
        )?;
    }
    reader.i32s(&["raw_af8", "raw_afc"])
}

/// Preserve saved settings without native list reconstruction; e.g. `read_parting_line_settings(reader, map)?`.
pub(super) fn read_parting_line_settings(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "owner_08", body_features::CLASSES)?;
    read_line_lists(reader, map, &["78"])?;
    declared(reader, map, "direction_10", &["moDirectionSpec_c"])?;
    reader.f64("raw_18")?;
    reader.i32("raw_1d0")?;
    declared(reader, map, "body_28", &["moCompAtomBody_c"])?;
    read_line_lists(reader, map, &["c0"])?;
    reader.i32s(&["raw_1d8", "raw_1e4"])?;
    read_line_lists(reader, map, &["30", "108", "188"])?;
    reader.i32s(&["raw_1e0", "raw_1dc", "raw_1e8"])
}

fn read_line_lists(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    offsets: &[&str],
) -> Result<()> {
    for offset in offsets {
        model_lists::pointers(reader, map, &format!("list_{offset}"), ModelObjectRole::Any)?;
    }
    Ok(())
}

/// Keep inherited generic object references and the edge word; e.g. `read_mpl_edge(reader, map)?`.
pub(super) fn read_mpl_edge(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    reader.require_ordinary_part()?;
    read_mpl_element_base(reader, map)?;
    reader.i32("raw_70").map(|_| ())
}

fn read_mpl_element_base(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    // #216 / C-2969: these four native codecs are CObject pointers, not component-only pointers.
    for offset in ["08", "10", "18", "28"] {
        reader.object(&format!("reference_{offset}"), map, ModelObjectRole::Any)?;
    }
    declared(reader, map, "split_info_30", &["moMPLFaceSplitInfo_c"])?;
    reader.i32("raw_20").map(|_| ())
}

/// Preserve the typed face pointer and raw split word; e.g. `read_mpl_split_info(reader, map)?`.
pub(super) fn read_mpl_split_info(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
) -> Result<()> {
    reader.require_ordinary_part()?;
    declared(reader, map, "face_08", &["moCompFace_c"])?;
    reader.i32("raw_70").map(|_| ())
}
