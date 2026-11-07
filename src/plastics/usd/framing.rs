use super::{Result, UsdCharacterWidth, UsdLimits, invalid};
use crate::archive_codec::ArchiveCursor;

#[derive(Debug, Clone, Copy)]
pub(super) struct UsdRecordSpan {
    pub offset: usize,
    pub payload_offset: usize,
    pub end: usize,
    pub width: UsdCharacterWidth,
    pub count: u64,
}

pub(super) fn decode_record(
    bytes: &[u8],
    offset: usize,
    maximum_payload: usize,
) -> Result<UsdRecordSpan> {
    let tail = bytes.get(offset..).ok_or_else(|| {
        invalid(
            "record offset",
            format!("{offset}; expected 0..={}", bytes.len()),
        )
    })?;
    let mut cursor = ArchiveCursor::new(tail);
    parse_span(bytes.len(), offset, &mut cursor, maximum_payload).map_err(|mut error| {
        error.operation = "decode Plastics USD";
        error.entity = format!("record at byte {offset}");
        error
    })
}

fn parse_span(
    length: usize,
    offset: usize,
    cursor: &mut ArchiveCursor<'_>,
    maximum: usize,
) -> Result<UsdRecordSpan> {
    let (width, count) = read_count(cursor)?;
    let payload_offset = length - cursor.remaining();
    cursor.take(payload_size(count, width, maximum)?)?;
    Ok(UsdRecordSpan {
        offset,
        payload_offset,
        end: length - cursor.remaining(),
        width,
        count,
    })
}

fn read_count(cursor: &mut ArchiveCursor<'_>) -> Result<(UsdCharacterWidth, u64)> {
    let first = cursor.u8()?;
    if first != 0xff {
        return Ok((UsdCharacterWidth::NarrowOpaque, u64::from(first)));
    }
    let word = cursor.u16()?;
    if word != 0xfffe {
        return Ok((
            UsdCharacterWidth::NarrowOpaque,
            extended_count(cursor, word)?,
        ));
    }
    Ok((UsdCharacterWidth::Utf16CodeUnits, wide_count(cursor)?))
}

fn wide_count(cursor: &mut ArchiveCursor<'_>) -> Result<u64> {
    let first = cursor.u8()?;
    if first != 0xff {
        return Ok(u64::from(first));
    }
    let word = cursor.u16()?;
    extended_count(cursor, word)
}

fn extended_count(cursor: &mut ArchiveCursor<'_>, word: u16) -> Result<u64> {
    if word != 0xffff {
        return Ok(u64::from(word));
    }
    let dword = cursor.u32()?;
    if dword != u32::MAX {
        return Ok(u64::from(dword));
    }
    cursor.u64()
}

fn payload_size(count: u64, width: UsdCharacterWidth, maximum: usize) -> Result<usize> {
    let bytes = count
        .checked_mul(width.byte_width() as u64)
        .and_then(|size| usize::try_from(size).ok());
    bytes.filter(|size| *size <= maximum).ok_or_else(|| invalid("record payload", format!("count {count}, width {}, maximum {maximum}; expected a representable byte length within the payload limit", width.byte_width())))
}

pub(super) fn decode_all(bytes: &[u8], limits: UsdLimits) -> Result<Vec<UsdRecordSpan>> {
    let mut records = Vec::new();
    let mut offset = 0;
    while offset < bytes.len() {
        if records.len() >= limits.maximum_records {
            return Err(invalid(
                "record count",
                format!(
                    "next record {} at byte {offset}; expected at most {} complete records",
                    records.len() + 1,
                    limits.maximum_records
                ),
            ));
        }
        let record = decode_record(bytes, offset, limits.maximum_payload_bytes)?;
        offset = record.end;
        records.push(record);
    }
    Ok(records)
}

pub(super) fn encode_count(count: u64, width: UsdCharacterWidth) -> Vec<u8> {
    let mut bytes = if width == UsdCharacterWidth::Utf16CodeUnits {
        vec![0xff, 0xfe, 0xff]
    } else {
        Vec::new()
    };
    if count < 255 {
        bytes.push(count as u8);
        return bytes;
    }
    bytes.push(0xff);
    append_extended_count(&mut bytes, count);
    bytes
}

fn append_extended_count(bytes: &mut Vec<u8>, count: u64) {
    if count < 0xfffe {
        bytes.extend_from_slice(&(count as u16).to_le_bytes());
        return;
    }
    bytes.extend_from_slice(&u16::MAX.to_le_bytes());
    if count < u64::from(u32::MAX) {
        bytes.extend_from_slice(&(count as u32).to_le_bytes());
        return;
    }
    bytes.extend_from_slice(&u32::MAX.to_le_bytes());
    bytes.extend_from_slice(&count.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::{UsdCharacterWidth, decode_record, encode_count};
    const COUNT_WIRES: &[(u64, &[u8])] = &[
        (0, &[0]),
        (254, &[254]),
        (255, &[255, 255, 0]),
        (65533, &[255, 253, 255]),
        (65534, &[255, 255, 255, 254, 255, 0, 0]),
        (0xfffffffe, &[255, 255, 255, 254, 255, 255, 255]),
        (
            0xffffffff,
            &[
                255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0,
            ],
        ),
        (u64::MAX, &[255; 15]),
    ];

    #[test]
    fn all_provider_count_boundaries_match_independent_literal_wires_without_payload_allocation() {
        for (count, expected) in COUNT_WIRES {
            assert_eq!(
                encode_count(*count, UsdCharacterWidth::NarrowOpaque),
                *expected
            );
            let wide = [b"\xff\xfe\xff".as_slice(), *expected].concat();
            assert_eq!(
                encode_count(*count, UsdCharacterWidth::Utf16CodeUnits),
                wide
            );
        }
    }

    #[test]
    fn invalid_record_offsets_are_typed_before_any_slice() {
        for offset in [2, usize::MAX] {
            let error = decode_record(&[0], offset, 0).unwrap_err();
            assert_eq!(error.class, crate::FailureClass::Invalid);
            assert!(error.cause.contains(&offset.to_string()));
            assert!(error.cause.contains("expected 0..=1"));
        }
    }
}
