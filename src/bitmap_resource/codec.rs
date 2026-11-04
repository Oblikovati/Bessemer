use super::{BitmapResourceArray, invalid};
use crate::{Result, archive_codec::ArchiveCursor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CountWidth {
    Short,
    Extended,
}

impl CountWidth {
    pub(super) fn canonical(length: usize) -> Self {
        if length < usize::from(u16::MAX) {
            Self::Short
        } else {
            Self::Extended
        }
    }
}

pub(super) fn validate_payload(length: usize, maximum: usize) -> Result<u32> {
    if length > maximum {
        return Err(invalid(format!(
            "payload length {length}; expected at most {maximum} bytes"
        )));
    }
    u32::try_from(length).map_err(|_| {
        invalid(format!(
            "payload length {length}; expected a count representable as u32"
        ))
    })
}

pub(super) fn decode(bytes: &[u8], maximum: usize) -> Result<(Vec<u8>, CountWidth)> {
    let mut cursor = ArchiveCursor::new(bytes);
    let count = cursor.collection_count(1)?;
    validate_payload(count, maximum)?;
    let prefix = bytes.len() - cursor.remaining();
    let width = match prefix {
        2 => CountWidth::Short,
        6 => CountWidth::Extended,
        actual => {
            return Err(invalid(format!(
                "count prefix {actual} bytes; expected 2 or 6"
            )));
        }
    };
    let payload = cursor.take(count)?;
    cursor.finish()?;
    Ok((payload.to_vec(), width))
}

pub(super) fn encode(resource: &BitmapResourceArray) -> Result<Vec<u8>> {
    let count = validate_payload(resource.payload().len(), resource.maximum_payload)?;
    let extended = resource.preferred_width == CountWidth::Extended || count >= u32::from(u16::MAX);
    let mut output = Vec::with_capacity(resource.payload().len() + if extended { 6 } else { 2 });
    if extended {
        output.extend_from_slice(&u16::MAX.to_le_bytes());
        output.extend_from_slice(&count.to_le_bytes());
    } else {
        output.extend_from_slice(&(count as u16).to_le_bytes());
    }
    output.extend_from_slice(resource.payload());
    Ok(output)
}
