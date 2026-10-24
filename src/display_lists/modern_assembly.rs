use super::{archive_decode::DisplayDecoder, archive_error::unsupported};
use crate::Result;

const ASSEMBLY_CLASSES: &[&str] = &["uoTempAssemblySHDData_c", "uoTempSubAssemblySHDData_c"];

pub(super) fn shaded_component(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.scope("viewer_comp_data", super::modern_component::read)?;
    archive.object("comp_part_tess_data", &["uoTempPartTessData_c"])?;
    archive.i32("unknown_320")?;
    archive.object_array("children", ASSEMBLY_CLASSES)?;
    archive.object("owner", ASSEMBLY_CLASSES)?;
    archive.i32("has_shrink_wrap_config")?;
    let decals = archive.u32("decals.count")?;
    archive.entries(decals as usize, "component decals")?;
    if decals != 0 {
        return Err(unsupported(format!(
            "component decal count {decals}; expected implemented decal body"
        )));
    }
    archive.i32("unknown_370")?;
    super::modern_component::integers(archive, "hidden_body_atoms")
}
