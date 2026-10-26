use super::{ModelDecoder, ModelObjectRole, model_lists};
use crate::archive_codec::{MaterialRead, ObjectReader};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn positive_count(
    reader: &mut ModelDecoder<'_>,
    label: &str,
    minimum: usize,
) -> Result<usize> {
    let raw = reader.i32(label)?;
    reader.signed_count_value(label, raw.max(0), minimum)
}

pub(super) fn empty(reader: &mut ModelDecoder<'_>, label: &str, count: usize) -> Result<()> {
    if count == 0 {
        return Ok(());
    }
    reader.checked(Err(BessemerError::new(
        "configuration context",
        label,
        FailureClass::Unsupported,
        format!("{label} count or presence {count}; expected an empty supported branch"),
    )))
}

pub(super) fn empty_signed(
    reader: &mut ModelDecoder<'_>,
    label: &str,
    minimum: usize,
) -> Result<()> {
    let count = positive_count(reader, label, minimum)?;
    empty(reader, label, count)
}

pub(super) fn absent(reader: &mut ModelDecoder<'_>, label: &str, presence: i32) -> Result<()> {
    if presence == 0 {
        return Ok(());
    }
    reader.checked(Err(BessemerError::new(
        "configuration context",
        label,
        FailureClass::Unsupported,
        format!("{label} presence {presence}; expected 0 in the supported branch"),
    )))
}

pub(super) fn pointers(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
    role: ModelObjectRole<'_>,
) -> Result<()> {
    for index in 0..positive_count(reader, &format!("{label}.count"), 2)? {
        reader.object(&format!("{label}[{index}]"), map, role)?;
    }
    Ok(())
}

pub(super) fn list(
    reader: &mut ModelDecoder<'_>,
    map: &mut ObjectReader,
    label: &str,
) -> Result<()> {
    model_lists::pointers(reader, map, label, ModelObjectRole::Any)
}

pub(super) fn strings(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    for index in 0..reader.count_with_minimum(&format!("{label}.count"), 1)? {
        reader.text(&format!("{label}[{index}]"))?;
    }
    Ok(())
}

pub(super) fn integer_pairs(
    reader: &mut ModelDecoder<'_>,
    label: &str,
    unsigned_value: bool,
) -> Result<()> {
    for index in 0..reader.unsigned_count(&format!("{label}.count"), 8)? {
        reader.i32(&format!("{label}[{index}].key"))?;
        if unsigned_value {
            reader.u32(&format!("{label}[{index}].value"))?;
        } else {
            reader.i32(&format!("{label}[{index}].value"))?;
        }
    }
    Ok(())
}

pub(super) fn empty_string_int_map(reader: &mut ModelDecoder<'_>, label: &str) -> Result<()> {
    let count = map_count64(reader, label)?;
    let count = i64::try_from(count).map_err(|_| {
        super::invalid(format!(
            "{label} count {count}; expected <= i64::MAX bounded entries"
        ))
    });
    let count = reader.checked(count)?;
    let count = reader.counted_entries(label, count, 5)?;
    empty(reader, label, count)
}

fn map_count64(reader: &mut ModelDecoder<'_>, label: &str) -> Result<u64> {
    let word = reader.u16(&format!("{label}.count_word"))?;
    if word != u16::MAX {
        return Ok(u64::from(word));
    }
    let dword = reader.u32(&format!("{label}.count_dword"))?;
    if dword != u32::MAX {
        return Ok(u64::from(dword));
    }
    reader.u64(&format!("{label}.count_qword"))
}
