use super::{
    Result, SimulationGuidField, SimulationGuidFieldId, SimulationGuidParts,
    SimulationGuidRevision, require,
};
use crate::records::{dword, region};
use crate::simulation::invalid;

const MAX_BYTES: usize = 0x100000;

pub(super) fn decode(
    bytes: &[u8],
    maximum: usize,
    revision: &SimulationGuidRevision,
) -> Result<([SimulationGuidField; 4], usize)> {
    validate_stream_extent(bytes.len(), maximum)?;
    let mut cursor = 0;
    let mut fields = Vec::new();
    for index in 0..4 {
        fields.push(read_field(bytes, &mut cursor, revision, index)?);
    }
    let fields = fields
        .try_into()
        .map_err(|fields: Vec<SimulationGuidField>| {
            invalid(
                "TimeStampGUIDs fields",
                format!("{} frames; expected exactly four", fields.len()),
            )
        })?;
    Ok((fields, cursor))
}

fn read_field(
    bytes: &[u8],
    cursor: &mut usize,
    revision: &SimulationGuidRevision,
    index: usize,
) -> Result<SimulationGuidField> {
    let stream_offset = *cursor;
    let byte_length = dword(bytes, *cursor)?;
    *cursor += 4;
    let units = if byte_length == 0 {
        None
    } else {
        Some(read_units(bytes, cursor, byte_length)?.into())
    };
    Ok(SimulationGuidField {
        id: SimulationGuidFieldId {
            revision: revision.clone(),
            index,
        },
        index,
        stream_offset,
        byte_length,
        units,
    })
}

fn read_units(bytes: &[u8], cursor: &mut usize, length: u32) -> Result<Vec<u16>> {
    validate_length(length as usize)?;
    let raw = region(bytes, *cursor, length as usize)?;
    require(
        raw.ends_with(&[0, 0]),
        "TimeStampGUIDs terminator",
        format!(
            "field ending {:?}; expected a complete two-byte zero terminator",
            &raw[raw.len() - 2..]
        ),
    )?;
    *cursor += length as usize;
    Ok(raw[..raw.len() - 2]
        .as_chunks::<2>()
        .0
        .iter()
        .map(|unit| u16::from_le_bytes(*unit))
        .collect())
}

pub(super) fn encode(parts: &SimulationGuidParts, maximum: usize) -> Result<Vec<u8>> {
    let lengths: Vec<_> = parts
        .fields
        .iter()
        .map(|field| field_length(field.as_deref()))
        .collect::<Result<_>>()?;
    validate_constructed_extent(&lengths, parts.tail.len(), maximum)?;
    let mut bytes = Vec::new();
    for (field, length) in parts.fields.iter().zip(lengths) {
        append_field(&mut bytes, field.as_deref(), length);
    }
    bytes.extend_from_slice(&parts.tail);
    Ok(bytes)
}

pub(super) fn field_length(units: Option<&[u16]>) -> Result<usize> {
    let Some(units) = units else {
        return Ok(0);
    };
    let length = units
        .len()
        .checked_mul(2)
        .and_then(|length| length.checked_add(2))
        .ok_or_else(|| {
            invalid(
                "TimeStampGUIDs UTF-16 length",
                format!(
                    "{} units; expected addressable byte count including terminator",
                    units.len()
                ),
            )
        })?;
    validate_length(length)?;
    Ok(length)
}

fn validate_length(length: usize) -> Result<()> {
    require(
        (2..=MAX_BYTES).contains(&length) && length.is_multiple_of(2),
        "TimeStampGUIDs byte count",
        format!(
            "{length}; expected null count zero or even UTF-16 count 2..={MAX_BYTES} including terminator"
        ),
    )
}

fn append_field(bytes: &mut Vec<u8>, units: Option<&[u16]>, length: usize) {
    bytes.extend_from_slice(&(length as u32).to_le_bytes());
    let Some(units) = units else {
        return;
    };
    for unit in units {
        bytes.extend_from_slice(&unit.to_le_bytes());
    }
    bytes.extend_from_slice(&[0, 0]);
}

fn validate_stream_extent(length: usize, maximum: usize) -> Result<()> {
    require(
        length >= 16 && length <= maximum,
        "TimeStampGUIDs extent",
        format!(
            "{} bytes; expected four bare count fields and at most {maximum} stream bytes",
            length
        ),
    )
}

fn validate_constructed_extent(
    lengths: &[usize],
    tail_length: usize,
    maximum: usize,
) -> Result<()> {
    let total = lengths
        .iter()
        .try_fold(16_usize, |total, length| total.checked_add(*length))
        .and_then(|total| total.checked_add(tail_length));
    require(
        total.is_some_and(|total| total <= maximum),
        "TimeStampGUIDs construction",
        format!(
            "field byte lengths {lengths:?}, tail {} bytes; expected at most {maximum} complete stream bytes",
            tail_length
        ),
    )
}
