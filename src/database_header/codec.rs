use super::DatabaseHeaderFields;
use crate::{BessemerError, FailureClass, Result};

const BUFFER_LENGTH: usize = 512;
const PREFIX_LENGTH: usize = 25;
const SIGNATURE: &[u8] = b"unqlite\xdb\x7c\x27\x12";

pub(super) fn decode(bytes: &[u8]) -> Result<(DatabaseHeaderFields, Vec<u8>)> {
    check_buffer(bytes)?;
    let count = usize::from(u16::from_be_bytes([bytes[23], bytes[24]]));
    if count > BUFFER_LENGTH - PREFIX_LENGTH {
        return Err(invalid(format!(
            "engine-name count {count}; expected <= 487 complete bytes"
        )));
    }
    let end = PREFIX_LENGTH + count;
    let fields = DatabaseHeaderFields {
        packed_time: read_word(bytes, 11)?,
        sector_size: read_word(bytes, 15)?,
        page_size: read_word(bytes, 19)?,
        engine_name: bytes[PREFIX_LENGTH..end].to_vec(),
    };
    validate(&fields, BUFFER_LENGTH - end)?;
    Ok((fields, bytes[end..].to_vec()))
}

fn check_buffer(bytes: &[u8]) -> Result<()> {
    if bytes.len() != BUFFER_LENGTH {
        return Err(invalid(format!(
            "buffer length {}; expected exactly 512",
            bytes.len()
        )));
    }
    if !bytes.starts_with(SIGNATURE) {
        return Err(invalid(format!(
            "signature {:02x?}; expected {:02x?}",
            &bytes[..11],
            SIGNATURE
        )));
    }
    Ok(())
}

fn read_word(bytes: &[u8], offset: usize) -> Result<u32> {
    let end = offset.saturating_add(4);
    let encoded = bytes
        .get(offset..end)
        .and_then(|slice| slice.try_into().ok());
    encoded.map(u32::from_be_bytes).ok_or_else(|| {
        invalid(format!(
            "word at {offset} in {} bytes; expected four bytes",
            bytes.len()
        ))
    })
}

pub(super) fn validate(fields: &DatabaseHeaderFields, tail: usize) -> Result<()> {
    let sector = fields.sector_size;
    if !(32..=65536).contains(&sector) || !sector.is_power_of_two() {
        return Err(invalid(format!(
            "stored S {sector}; expected power of two in 32..=65536"
        )));
    }
    let page = fields.page_size;
    if !(512..=65536).contains(&page) {
        return Err(invalid(format!(
            "stored P {page}; expected integer in 512..=65536"
        )));
    }
    validate_extent(fields.engine_name.len(), tail)
}

fn validate_extent(name: usize, tail: usize) -> Result<()> {
    if name > BUFFER_LENGTH - PREFIX_LENGTH || tail != BUFFER_LENGTH - PREFIX_LENGTH - name {
        return Err(invalid(format!(
            "name length {name}, tail length {tail}; expected name <= 487 and name + tail == 487"
        )));
    }
    Ok(())
}

pub(super) fn encode(fields: &DatabaseHeaderFields, tail: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(BUFFER_LENGTH);
    bytes.extend_from_slice(SIGNATURE);
    for word in [fields.packed_time, fields.sector_size, fields.page_size] {
        bytes.extend(word.to_be_bytes());
    }
    bytes.extend((fields.engine_name.len() as u16).to_be_bytes());
    bytes.extend_from_slice(&fields.engine_name);
    bytes.extend_from_slice(tail);
    bytes
}

fn invalid(cause: String) -> BessemerError {
    BessemerError::new(
        "database header",
        "selected 512-byte buffer",
        FailureClass::Invalid,
        cause,
    )
}

#[cfg(test)]
#[path = "../../tests/database_header/codec.rs"]
mod tests;
