use super::ModelDecoder;
use crate::Result;
use crate::archive_codec::{MaterialRead, vectors::double_vector};

pub(super) fn doubles(reader: &mut ModelDecoder<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        reader.f64(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn integers(reader: &mut ModelDecoder<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        reader.i32(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn words(reader: &mut ModelDecoder<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        reader.u16(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn vectors(reader: &mut ModelDecoder<'_>, label: &str, count: usize) -> Result<()> {
    for index in 0..count {
        double_vector(reader, &format!("{label}[{index}]"))?;
    }
    Ok(())
}
