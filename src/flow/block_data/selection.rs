use super::*;
use super::{FieldSelection, ScalarKind};

pub(super) fn block_field(
    block: FlowLegacyBlock<'_>,
    field: FlowBlockHeaderField,
) -> FieldSelection {
    let (offset, kind) = match field {
        FlowBlockHeaderField::BlockByte => (0, ScalarKind::Byte),
        FlowBlockHeaderField::Word10 => (1, ScalarKind::Word16),
        FlowBlockHeaderField::Count12 => (3, ScalarKind::Word16),
    };
    FieldSelection {
        offset: block.span.start + offset,
        kind,
    }
}

pub(super) fn entry_field(
    entry: FlowLegacyBlockEntry<'_>,
    field: FlowBlockEntryField,
) -> Result<FieldSelection> {
    match field {
        FlowBlockEntryField::Descriptor { index } => descriptor(entry, index),
        FlowBlockEntryField::FirstWord { index } => array_word(entry.span.first, index),
        FlowBlockEntryField::SecondWord { index } => array_word(entry.span.second, index),
        _ => scalar(entry, field),
    }
}

fn descriptor(entry: FlowLegacyBlockEntry<'_>, index: usize) -> Result<FieldSelection> {
    if index >= 7 {
        return Err(invalid_block(format!(
            "descriptor index {index}; expected zero through six"
        )));
    }
    let (width, kind) = if entry.owner.prefix.version == 1 {
        (1, ScalarKind::NarrowWord16)
    } else {
        (2, ScalarKind::Word16)
    };
    Ok(FieldSelection {
        offset: entry.span.descriptors + index * width,
        kind,
    })
}

fn array_word(span: (usize, usize), index: usize) -> Result<FieldSelection> {
    let count = (span.1 - span.0) / 4;
    if index >= count {
        return Err(invalid_block(format!(
            "array word index {index}; expected below {count} in decoded span {span:?}"
        )));
    }
    Ok(FieldSelection {
        offset: span.0 + index * 4,
        kind: ScalarKind::Word32,
    })
}

fn scalar(entry: FlowLegacyBlockEntry<'_>, field: FlowBlockEntryField) -> Result<FieldSelection> {
    let (slot, kind) = scalar_spec(field, entry.owner.prefix.version).ok_or_else(|| {
        invalid_block(format!("field {field:?}; expected a scalar entry selector"))
    })?;
    let offset = entry.span.scalars[slot].ok_or_else(|| {
        block_failure(FailureClass::Missing, format!("entry {} field {field:?}, version {}; expected a physically stored field or the version-one member-20 alias", entry.ordinal, entry.owner.prefix.version))
    })?;
    Ok(FieldSelection { offset, kind })
}

fn scalar_spec(field: FlowBlockEntryField, version: i32) -> Option<(usize, ScalarKind)> {
    use FlowBlockEntryField::*;
    Some(match field {
        Identifier => (0, ScalarKind::Signed32),
        LegacyIndex => (1, ScalarKind::Signed32),
        Member20 => (2, ScalarKind::Signed32),
        Member28 => (3, ScalarKind::Signed32),
        DiscardedPrefix => (4, ScalarKind::Byte),
        State => (5, ScalarKind::Byte),
        Flag2 => (6, ScalarKind::Byte),
        Flag4 => (7, ScalarKind::Byte),
        SecondCount => (8, second_kind(version)),
        DiscardedSuffix => (9, ScalarKind::Word16),
        _ => return None,
    })
}

fn second_kind(version: i32) -> ScalarKind {
    match version {
        1 => ScalarKind::Signed8,
        2..=9 => ScalarKind::UnsignedCount16,
        _ => ScalarKind::Signed32,
    }
}
