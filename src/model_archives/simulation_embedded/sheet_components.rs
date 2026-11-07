use super::{component_boundaries, components, reference_wrappers, wrappers};
use crate::Result;
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::model_archives::{ModelDecoder, part_objects::absent_payload};

pub(super) fn read_object(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    name: &str,
) -> Option<Result<()>> {
    Some(match name {
        "moConstSurfRef_w" => constant_surface(reader, map),
        "moCompSurfaceBody_c" | "moCompPlaneSectionBody_c" => sheet_body(reader, map),
        "moCompRefSurface_c" => reference_surface(reader, map),
        _ => return None,
    })
}

fn constant_surface(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.archive_version().value() != 4281 {
        return reference_wrappers::solid(reader, map);
    }
    // C-737 binds this exact historical class; the general solid reader keeps its 4307 floor (#151).
    wrappers::common_wrapper(reader, map)?;
    let present = reader.i32("has_geometry")?;
    absent_payload(
        reader,
        "embedded 4281 constant-surface kernel body",
        present,
    )
}

fn sheet_body(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    if reader.archive_version().value() != 4281 {
        return component_boundaries::solid(reader, map);
    }
    // Both concrete sheet classes have the separately bound 4281 continuation (C-737, #151).
    components::reference_base(reader, map)?;
    component_boundaries::solid_tail(reader, map)
}

fn reference_surface(reader: &mut ModelDecoder<'_>, map: &mut ObjectReader) -> Result<()> {
    components::holder(reader, map)?;
    reader.i32("unknown_100")?;
    Ok(())
}
