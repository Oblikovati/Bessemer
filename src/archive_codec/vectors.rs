use super::MaterialRead;
use crate::Result;

pub(crate) fn double_vector(archive: &mut impl MaterialRead, label: &str) -> Result<()> {
    for index in 0..3 {
        archive.f64(&format!("{label}[{index}]"))?;
    }
    Ok(())
}
