use super::{archive_decode::DisplayDecoder, archive_error::invalid};
use crate::Result;

pub(super) fn read(archive: &mut DisplayDecoder<'_>) -> Result<()> {
    let count = layer_count(archive, "layers", 5)?;
    for index in 0..count {
        archive.i32(&format!("layers[{index}].key"))?;
        archive.text(&format!("layers[{index}].value"))?;
    }
    let label = if archive.context.version.value() >= 0x1be5 {
        "attributes"
    } else {
        "old_attributes"
    };
    let count = layer_count(archive, label, 8)?;
    for index in 0..count {
        archive.i32(&format!("{label}[{index}].key"))?;
        archive.i32(&format!("{label}[{index}].value"))?;
    }
    Ok(())
}

fn layer_count(archive: &mut DisplayDecoder<'_>, label: &str, minimum: usize) -> Result<usize> {
    if archive.context.version.value() >= 0x1be5 {
        return archive.count(&format!("{label}.count"), minimum);
    }
    let short = archive.u16(&format!("{label}.count_word"))?;
    let count = if short == u16::MAX {
        extended_count(archive, label)?
    } else {
        u64::from(short)
    };
    checked_entries(archive, label, count, minimum)
}

fn checked_entries(
    archive: &mut DisplayDecoder<'_>,
    label: &str,
    count: u64,
    minimum: usize,
) -> Result<usize> {
    let count = bounded_usize(count, label)?;
    if count > archive.cursor.remaining() / minimum {
        return Err(invalid(format!(
            "{label} count {count}; expected entries of at least {minimum} bytes fitting {} bytes",
            archive.cursor.remaining()
        )));
    }
    archive.entries(count, label)?;
    Ok(count)
}

fn bounded_usize(count: u64, label: &str) -> Result<usize> {
    usize::try_from(count).map_err(|_| {
        invalid(format!(
            "{label} count {count}; expected a count fitting usize"
        ))
    })
}

fn extended_count(archive: &mut DisplayDecoder<'_>, label: &str) -> Result<u64> {
    let count = archive.u32(&format!("{label}.count_dword"))?;
    if count != u32::MAX {
        return Ok(u64::from(count));
    }
    let low = archive.u32(&format!("{label}.count_low"))?;
    let high = archive.u32(&format!("{label}.count_high"))?;
    Ok(u64::from(low) | (u64::from(high) << 32))
}
