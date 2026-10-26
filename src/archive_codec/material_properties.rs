use super::MaterialRead;
use crate::Result;

pub(crate) fn user_properties(archive: &mut impl MaterialRead) -> Result<()> {
    let raw = archive.u32("count")?;
    let count = if raw >= 0x8000_0000 { 0 } else { raw as usize };
    archive.entries(count, "user properties")?;
    if count > archive.remaining() / 4 {
        return Err(archive.invalid(format!(
            "user property count {count}; expected entries fitting remaining bytes"
        )));
    }
    for index in 0..count {
        archive.scope(&format!("entries[{index}]"), property)?;
    }
    Ok(())
}

fn property(archive: &mut impl MaterialRead) -> Result<()> {
    match archive.u32("type")? {
        0 => archive.f32("value"),
        1 => {
            archive.i32("value")?;
            Ok(())
        }
        2 => archive.text("value"),
        _ => Ok(()),
    }
}
