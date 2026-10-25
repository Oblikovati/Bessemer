use super::{DisplayDocumentKind, archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>, views: usize) -> Result<()> {
    let count = archive.i32("config_count")?;
    if count < 1 {
        return Ok(());
    }
    let count = count as usize;
    header(archive, count)?;
    let saved = archive.restoring_configurations;
    // #28: uiEnvironment_c +0xB14 disables directory-backed facets throughout assembly restores.
    archive.restoring_configurations |= archive.context.kind == DisplayDocumentKind::Assembly;
    let result = body(archive, count, views);
    archive.restoring_configurations = saved;
    result
}

fn header(archive: &mut DisplayDecoder<'_>, count: usize) -> Result<()> {
    if count > archive.cursor.remaining() {
        return Err(invalid(format!(
            "configuration count {count}; expected names fitting {} remaining bytes",
            archive.cursor.remaining()
        )));
    }
    archive.entries(count, "configuration names")?;
    archive.text("active_config_name")?;
    archive.i32("active_config_id")?;
    for index in 0..count {
        archive.text(&format!("config_names[{index}]"))?;
    }
    Ok(())
}

fn body(archive: &mut DisplayDecoder<'_>, count: usize, views: usize) -> Result<()> {
    archive.entries(count, "configuration records")?;
    for index in 0..count {
        archive.scope(&format!("configs[{index}]"), record)?;
    }
    super::configuration_views::read(archive, count, views)?;
    if archive.context.version.value() > 0x2fa5 {
        archive.scope("unknown_tail", super::configuration_tail::read)?;
    }
    Ok(())
}

fn record(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.i32("config_id")?;
    if archive.context.kind == DisplayDocumentKind::Part {
        return archive.cobject("tessellation");
    }
    archive.object_array("shd_data", super::modern_assembly::ASSEMBLY_CLASSES)?;
    if archive.context.version.value() > 0x240e {
        archive.cobject("viz_prop")?;
    }
    Ok(())
}
