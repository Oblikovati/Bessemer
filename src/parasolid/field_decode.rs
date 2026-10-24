use super::wire::TransmitCursor;
use super::{
    FieldDefinition, FieldExtent, FieldKind, FieldValues, StoredDouble, TransmitPointer, invalid,
};
use crate::Result;

pub(super) fn read(
    cursor: &mut TransmitCursor<'_>,
    field: &FieldDefinition,
    length: Option<u32>,
    delta: bool,
    maximum: usize,
) -> Result<FieldValues> {
    let count = checked_count(cursor, field, length, delta, maximum)?;
    match field.kind {
        FieldKind::Integer | FieldKind::Short | FieldKind::WideUnit => {
            read_integers(cursor, field.kind, count)
        }
        FieldKind::Pointer => read_pointers(cursor, count, delta),
        FieldKind::UnsignedByte | FieldKind::Character | FieldKind::Logical => {
            Ok(FieldValues::Bytes(cursor.take(count)?.to_vec()))
        }
        other => read_floats(cursor, other, count, delta),
    }
}

fn checked_count(
    cursor: &TransmitCursor<'_>,
    field: &FieldDefinition,
    length: Option<u32>,
    delta: bool,
    maximum: usize,
) -> Result<usize> {
    let count = element_count(field, length)?;
    let minimum = field.kind.minimum_bytes(delta)?;
    if count > maximum {
        return Err(invalid(format!(
            "field {:?} count {count}; expected at most {maximum} by element budget",
            field.name
        )));
    }
    if count > cursor.remaining() / minimum {
        return Err(invalid(format!(
            "field {:?} count {count}; expected at most {} elements of at least {minimum} bytes",
            field.name,
            cursor.remaining() / minimum
        )));
    }
    Ok(count)
}

fn read_integers(
    cursor: &mut TransmitCursor<'_>,
    kind: FieldKind,
    count: usize,
) -> Result<FieldValues> {
    match kind {
        FieldKind::Integer => Ok(FieldValues::Integers(
            (0..count)
                .map(|_| cursor.u32().map(|word| word as i32))
                .collect::<Result<_>>()?,
        )),
        FieldKind::Short => Ok(FieldValues::Shorts(
            (0..count)
                .map(|_| cursor.u16().map(|word| word as i16))
                .collect::<Result<_>>()?,
        )),
        FieldKind::WideUnit => Ok(FieldValues::WideUnits(
            (0..count).map(|_| cursor.u16()).collect::<Result<_>>()?,
        )),
        other => Err(invalid(format!(
            "integer field kind {other:?}; expected d,n,w"
        ))),
    }
}

pub(super) fn element_count(field: &FieldDefinition, length: Option<u32>) -> Result<usize> {
    match field.extent {
        FieldExtent::Scalar => Ok(1),
        FieldExtent::Fixed(count) => Ok(count as usize),
        FieldExtent::Variable => length.map(|count| count as usize).ok_or_else(|| {
            invalid(format!(
                "field {:?} length absent; expected stored variable length",
                field.name
            ))
        }),
    }
}

fn read_pointers(
    cursor: &mut TransmitCursor<'_>,
    count: usize,
    delta: bool,
) -> Result<FieldValues> {
    let mut pointers = Vec::with_capacity(count);
    for _ in 0..count {
        let reference = cursor.index()?;
        let delta_flag = if delta { Some(cursor.u8()?) } else { None };
        pointers.push(TransmitPointer {
            reference,
            delta_flag,
        });
    }
    Ok(FieldValues::Pointers(pointers))
}

fn read_floats(
    cursor: &mut TransmitCursor<'_>,
    kind: FieldKind,
    count: usize,
    delta: bool,
) -> Result<FieldValues> {
    match kind {
        FieldKind::Real => Ok(FieldValues::Reals(
            (0..count)
                .map(|_| cursor.u64().map(StoredDouble::from_bits))
                .collect::<Result<_>>()?,
        )),
        FieldKind::Interval => Ok(FieldValues::Intervals(read_groups(cursor, count)?)),
        FieldKind::Vector => Ok(FieldValues::Vectors(read_groups(cursor, count)?)),
        FieldKind::HVector if delta => Ok(FieldValues::DeltaHVectors(read_groups(cursor, count)?)),
        FieldKind::HVector => Ok(FieldValues::PartitionHVectors(read_groups(cursor, count)?)),
        FieldKind::Box => Ok(FieldValues::Boxes(read_groups(cursor, count)?)),
        other => Err(invalid(format!(
            "floating field kind {other:?}; expected f,i,v,h,b"
        ))),
    }
}

fn read_groups<const N: usize>(
    cursor: &mut TransmitCursor<'_>,
    count: usize,
) -> Result<Vec<[StoredDouble; N]>> {
    let mut groups = Vec::with_capacity(count);
    for _ in 0..count {
        let mut group = [StoredDouble::from_bits(0); N];
        for value in &mut group {
            *value = StoredDouble::from_bits(cursor.u64()?);
        }
        groups.push(group);
    }
    Ok(groups)
}
