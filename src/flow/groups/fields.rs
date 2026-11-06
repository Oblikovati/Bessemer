use super::*;

#[derive(Clone, Copy)]
pub(super) enum MemberField {
    Word,
    WideWord,
    Flag,
}

pub(super) fn has_flags(version: FlowGroupVersion) -> bool {
    matches!(version, FlowGroupVersion::V5 | FlowGroupVersion::V6)
}

pub(super) fn member_offset(
    version: FlowGroupVersion,
    span: &GroupSpan,
    member: usize,
    field: MemberField,
) -> usize {
    if version == FlowGroupVersion::V6 {
        let offset = match field {
            MemberField::Word => 0,
            MemberField::WideWord => 4,
            MemberField::Flag => 12,
        };
        return span.values + 13 * member + offset;
    }
    match field {
        MemberField::Word => span.values + 4 * member,
        MemberField::WideWord => span.values + 4 * span.members + 8 * member,
        MemberField::Flag => span.values + 12 * span.members + member,
    }
}

pub(super) fn word(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

pub(super) fn wide_word(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
        bytes[offset + 4],
        bytes[offset + 5],
        bytes[offset + 6],
        bytes[offset + 7],
    ])
}

pub(super) fn value_bytes(value: FlowGroupValue) -> Vec<u8> {
    match value {
        FlowGroupValue::Word(word) => word.to_le_bytes().to_vec(),
        FlowGroupValue::WideWord(word) => word.to_le_bytes().to_vec(),
        FlowGroupValue::Flag(flag) => vec![flag],
        FlowGroupValue::Auxiliary(word) => word.to_le_bytes().to_vec(),
    }
}

pub(super) fn read_value(bytes: &[u8], offset: usize, kind: FlowGroupValue) -> FlowGroupValue {
    match kind {
        FlowGroupValue::Word(_) => FlowGroupValue::Word(word(bytes, offset)),
        FlowGroupValue::WideWord(_) => FlowGroupValue::WideWord(wide_word(bytes, offset)),
        FlowGroupValue::Flag(_) => FlowGroupValue::Flag(bytes[offset]),
        FlowGroupValue::Auxiliary(_) => FlowGroupValue::Auxiliary(word(bytes, offset) as i32),
    }
}
