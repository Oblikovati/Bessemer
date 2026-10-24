use super::{archive_decode::DisplayDecoder, archive_error::unsupported};
use crate::Result;

pub(super) fn empty_list(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<()> {
    let count = archive.u32(&format!("{label}.count"))?;
    archive.entries(count as usize, label)?;
    if count == 0 {
        return Ok(());
    }
    Err(unsupported(format!(
        "{label} count {count}; expected implemented decal body"
    )))
}
