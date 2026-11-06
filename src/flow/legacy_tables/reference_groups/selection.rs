use super::*;
use FlowReferenceEntryField as Field;

pub(super) fn entry_field(entry: FlowReferenceEntry<'_>, field: Field) -> Result<FieldSelection> {
    match field {
        Field::Reference(index) => array(entry.span.references, index, ScalarKind::Signed32),
        Field::FirstWord(index) => array(entry.span.continuation.first, index, ScalarKind::Word32),
        Field::SecondWord(index) => {
            array(entry.span.continuation.second, index, ScalarKind::Word32)
        }
        Field::Descriptor(index) => descriptor(entry, index),
        Field::Member20 => required(entry.span.member20, ScalarKind::Signed32, field),
        _ => scalar(entry, field),
    }
}

pub(super) fn array(
    span: (usize, usize),
    index: usize,
    kind: ScalarKind,
) -> Result<FieldSelection> {
    let count = (span.1 - span.0) / 4;
    if index >= count {
        return Err(table_index_error(416, "array word", index, count));
    }
    Ok(FieldSelection {
        offset: span.0 + index * 4,
        kind,
    })
}

fn descriptor(entry: FlowReferenceEntry<'_>, index: usize) -> Result<FieldSelection> {
    if index >= 7 {
        return Err(table_index_error(416, "descriptor", index, 7));
    }
    let (width, kind) = if entry.owner.version() == 1 {
        (1, ScalarKind::NarrowWord16)
    } else {
        (2, ScalarKind::Word16)
    };
    Ok(FieldSelection {
        offset: entry.span.continuation.descriptors + index * width,
        kind,
    })
}

fn scalar(entry: FlowReferenceEntry<'_>, field: Field) -> Result<FieldSelection> {
    let (index, kind) = match field {
        Field::DiscardedPrefix => (0, ScalarKind::Byte),
        Field::State => (1, ScalarKind::Byte),
        Field::Flag2 => (2, ScalarKind::Byte),
        Field::Flag4 => (3, ScalarKind::Byte),
        Field::SecondCount => (4, secondary_kind(entry.owner.version())),
        Field::DiscardedSuffix => (5, ScalarKind::Word16),
        _ => {
            return Err(reference_invalid(format!(
                "field {field:?}; expected an existing scalar selector"
            )));
        }
    };
    required(entry.span.continuation.scalars[index], kind, field)
}

fn required(offset: Option<usize>, kind: ScalarKind, field: Field) -> Result<FieldSelection> {
    let offset = offset.ok_or_else(|| reference_missing(format!("entry field {field:?}")))?;
    Ok(FieldSelection { offset, kind })
}

fn secondary_kind(version: i32) -> ScalarKind {
    match version {
        1 => ScalarKind::Signed8,
        0 | 2..=9 => ScalarKind::UnsignedCount16,
        _ => ScalarKind::Signed32,
    }
}

pub(super) fn width(kind: ScalarKind) -> usize {
    match kind {
        ScalarKind::Byte | ScalarKind::NarrowWord16 | ScalarKind::Signed8 => 1,
        ScalarKind::Word16 | ScalarKind::UnsignedCount16 => 2,
        ScalarKind::Signed32 | ScalarKind::Word32 => 4,
    }
}

pub(super) fn header_index(field: FlowReferenceHeaderField) -> usize {
    match field {
        FlowReferenceHeaderField::Discarded => 0,
        FlowReferenceHeaderField::Stored => 1,
    }
}
