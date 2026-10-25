use super::{archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    if archive.context.version.value() <= 0x2f80 {
        return Ok(());
    }
    let views = archive.count("view_count", 6)?;
    let configs = configuration_dimension(archive)?;
    require_rows(archive, views, configs)?;
    for index in 0..views {
        archive.scope(&format!("views[{index}]"), |archive| view(archive, configs))?;
    }
    Ok(())
}

fn configuration_dimension(archive: &mut DisplayDecoder<'_>) -> Result<usize> {
    let configs = archive.i32("configuration_count")?;
    if configs < 0 {
        return Err(invalid(format!(
            "ink configuration count {configs}; expected nonnegative dimension"
        )));
    }
    // #30: no configuration payload exists when the view dimension is zero.
    archive.entries(configs as usize, "ink configuration dimension")?;
    Ok(configs as usize)
}

fn require_rows(archive: &mut DisplayDecoder<'_>, views: usize, configs: usize) -> Result<()> {
    let rows = views.checked_mul(configs).ok_or_else(|| {
        invalid(format!(
            "ink dimensions {views} x {configs}; expected product fitting usize"
        ))
    })?;
    if rows > archive.cursor.remaining() / 13 {
        return Err(invalid(format!(
            "ink dimensions {views} x {configs}; expected rows fitting {} bytes at 13 bytes per configuration",
            archive.cursor.remaining()
        )));
    }
    archive.entries(rows, "ink configuration rows")
}

fn view(archive: &mut DisplayDecoder<'_>, configs: usize) -> Result<()> {
    archive.text("view_name")?;
    archive.i32("unknown_id")?;
    for index in 0..configs {
        archive.scope(&format!("configurations[{index}]"), configuration)?;
    }
    if archive.context.version.value() > 0x332b {
        archive.text("unknown_string")?;
    }
    Ok(())
}

fn configuration(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    archive.text("unknown_name")?;
    archive.i32s(&["unknown_id", "suppressed_state", "is_shown"])
}
