use crate::{BessemerError, FailureClass, Result};

pub(crate) fn region(bytes: &[u8], offset: usize, size: usize) -> Result<&[u8]> {
    offset
        .checked_add(size)
        .and_then(|end| bytes.get(offset..end))
        .ok_or_else(|| {
            invalid(format!(
                "range {offset}+{size}; expected within {} bytes",
                bytes.len()
            ))
        })
}

pub(crate) fn word(bytes: &[u8], offset: usize) -> Result<u16> {
    let value = region(bytes, offset, 2)?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}

pub(crate) fn dword(bytes: &[u8], offset: usize) -> Result<u32> {
    let value = region(bytes, offset, 4)?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

pub(crate) fn archive_offset(bytes: &[u8], offset: usize) -> Result<usize> {
    let value = dword(bytes, offset)?;
    usize::try_from(value)
        .ok()
        .and_then(|n| n.checked_add(8))
        .ok_or_else(|| {
            invalid(format!(
                "archive offset {value}; expected representable file offset"
            ))
        })
}

pub(crate) fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new("parse", "container", FailureClass::Invalid, cause)
}

pub(crate) fn require(equal: bool, cause: impl Into<String>) -> Result<()> {
    if equal {
        return Ok(());
    }
    Err(invalid(cause))
}

pub(crate) fn equal<T: PartialEq + std::fmt::Debug>(
    actual: T,
    expected: T,
    field: &str,
) -> Result<()> {
    require(
        actual == expected,
        format!("{field} {actual:?}; expected {expected:?}"),
    )
}

pub(crate) fn name_bytes(raw: &[u8], version: u32) -> Vec<u8> {
    raw.iter()
        .map(|value| {
            if version > 2 {
                value.rotate_left(4)
            } else {
                *value
            }
        })
        .collect()
}
