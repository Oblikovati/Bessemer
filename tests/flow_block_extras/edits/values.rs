use super::*;

pub(super) fn changed_value(
    case: &ExtraCase,
    field: &ExpectedExtraField,
    before: FlowBlockExtraValue,
) -> FlowBlockExtraValue {
    match before {
        FlowBlockExtraValue::Byte(value) => {
            FlowBlockExtraValue::Byte(changed_byte(case, field, value))
        }
        FlowBlockExtraValue::Signed32(value) => {
            FlowBlockExtraValue::Signed32(changed_signed(case, field, value))
        }
        FlowBlockExtraValue::WideBits(value) => {
            FlowBlockExtraValue::WideBits(value ^ 0x8000000000000001)
        }
    }
}

fn changed_byte(case: &ExtraCase, field: &ExpectedExtraField, value: u8) -> u8 {
    use FlowBlockExtraFieldKind::*;
    match field.kind {
        BodyFlag => body_flag(case, field.entity, value),
        OuterFlag | NestedFlag | FinalFlag => {
            if value == 1 {
                1
            } else {
                alternate_nonzero(value)
            }
        }
        AdditionalFlag | ExtraFlag => {
            if value == 0 {
                0
            } else {
                alternate_nonzero(value)
            }
        }
        _ => value ^ 0x80,
    }
}

fn alternate_nonzero(value: u8) -> u8 {
    if value == 2 { 3 } else { 2 }
}

fn body_flag(case: &ExtraCase, entity: FlowBlockExtraEntity, value: u8) -> u8 {
    let FlowBlockExtraEntity::Block(block) = entity else {
        panic!("body flag scope {entity:?}; expected Block")
    };
    if case.blocks[block].token == -1 {
        return value ^ 255;
    }
    if value == 0 {
        return 0;
    }
    alternate_nonzero(value)
}

fn changed_signed(case: &ExtraCase, field: &ExpectedExtraField, value: i32) -> i32 {
    if field.kind != FlowBlockExtraFieldKind::Token {
        return value ^ i32::MIN ^ 1;
    }
    let FlowBlockExtraEntity::Block(block) = field.entity else {
        panic!("token scope {:?}; expected Block", field.entity)
    };
    if value == -1 && !case.blocks[block].flag.is_some_and(|v| v != 0) {
        return value;
    }
    if value == 42 { 43 } else { 42 }
}

pub(super) fn value_bytes(value: FlowBlockExtraValue) -> Vec<u8> {
    match value {
        FlowBlockExtraValue::Byte(value) => vec![value],
        FlowBlockExtraValue::Signed32(value) => value.to_le_bytes().to_vec(),
        FlowBlockExtraValue::WideBits(value) => value.to_le_bytes().to_vec(),
    }
}

pub(super) fn samples(frame: &ExpectedExtraFrame) -> Vec<(usize, usize)> {
    let mut groups = Vec::new();
    let mut result = Vec::new();
    for (ordinal, field) in frame.fields.iter().enumerate() {
        if !field.editable || field.layout.element_count == 0 {
            continue;
        }
        let key = (field.kind, field.encoding, entity_tag(field.entity));
        if let Some((_, last)) = groups.iter_mut().find(|(stored, _)| *stored == key) {
            *last = ordinal;
        } else {
            groups.push((key, ordinal));
            add_edges(&mut result, ordinal, field.layout.element_count);
        }
    }
    add_last_edges(&mut result, frame, &groups);
    result
}

fn add_edges(result: &mut Vec<(usize, usize)>, ordinal: usize, count: usize) {
    for index in [0, count - 1] {
        if !result.contains(&(ordinal, index)) {
            result.push((ordinal, index));
        }
    }
}

fn entity_tag(entity: FlowBlockExtraEntity) -> u8 {
    use FlowBlockExtraEntity::*;
    match entity {
        Record => 0,
        Block(_) => 1,
        Body(_) => 2,
        First { .. } => 3,
        FirstMixed { .. } => 4,
        Second { .. } => 5,
        Third { .. } => 6,
        Fourth(_) => 7,
        NestedRow { .. } => 8,
        NestedEntry { .. } => 9,
        Final(_) => 10,
        Extra(_) => 11,
        ExtraMixed(_) => 12,
    }
}

type SampleFamily = (FlowBlockExtraFieldKind, FlowBlockExtraEncoding, u8);

fn add_last_edges(
    result: &mut Vec<(usize, usize)>,
    frame: &ExpectedExtraFrame,
    groups: &[(SampleFamily, usize)],
) {
    for &(_, ordinal) in groups {
        add_edges(result, ordinal, frame.fields[ordinal].layout.element_count);
    }
}
