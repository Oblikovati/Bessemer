use super::archive_decode::DisplayDecoder;
use crate::Result;

pub(super) const ASSEMBLY_CLASSES: &[&str] =
    &["uoTempAssemblySHDData_c", "uoTempSubAssemblySHDData_c"];

pub(super) fn shaded_component(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let component_id = archive.scope("viewer_comp_data", super::modern_component::read)?;
    part_tessellation(archive, component_id)?;
    archive.i32("unknown_320")?;
    archive.object_array("children", ASSEMBLY_CLASSES)?;
    archive.object("owner", ASSEMBLY_CLASSES)?;
    archive.i32("has_shrink_wrap_config")?;
    super::modern_decals::list(archive, "decals")?;
    if archive.context.version.value() >= 0x10f0 {
        archive.i32("unknown_370")?;
    }
    if archive.context.version.value() > 0x11af {
        super::modern_component::integers(archive, "hidden_body_atoms")?;
    }
    Ok(())
}

fn part_tessellation(
    archive: &mut DisplayDecoder<'_>,
    component_id: std::sync::Arc<super::ArchiveString>,
) -> Result<()> {
    let saved = archive.external_component.take();
    if archive.context.kind == super::DisplayDocumentKind::Assembly
        && !archive.restoring_configurations
        && archive
            .context
            .face_tessellation_components
            .contains(component_id.text())
        && !component_id.text().is_empty()
    {
        archive.external_component = Some((component_id, 0));
    }
    let result = archive.object("comp_part_tess_data", &["uoTempPartTessData_c"]);
    archive.external_component = saved;
    result
}
