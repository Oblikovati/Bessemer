use super::*;

pub(super) struct PointerPatch {
    pub(super) field: FlowPointerTableField,
    pub(super) before: FlowPointerTableValue,
    pub(super) after: FlowPointerTableValue,
    pub(super) offset: usize,
    pub(super) bytes: Vec<u8>,
}

pub(super) fn pointer_patches(case: &TableCase) -> Vec<PointerPatch> {
    let ExpectedTable::Pointers(expected) = &case.expected else {
        panic!("expected pointer fixture")
    };
    let mut result = Vec::new();
    for group in &expected.groups {
        group_patches(&mut result, case, group);
    }
    result
}

fn body_patches(
    result: &mut Vec<PointerPatch>,
    case: &TableCase,
    group: &ExpectedGroup,
    body: &ExpectedGroupBody,
    mut offset: usize,
) {
    let ordinal = group.ordinal as usize;
    offset = member_patches(result, ordinal, body, offset);
    offset += 12;
    offset = discarded_patches(result, ordinal, body, offset);
    offset = wide_patches(result, ordinal, body, offset);
    offset = pre_pair_patch(result, ordinal, body.pre_pairs, offset);
    offset = pair_patches(result, case.frame.pointer_count, ordinal, body, offset);
    assert_eq!(offset, group.end as usize);
}

fn discarded_patches(
    result: &mut Vec<PointerPatch>,
    group: usize,
    body: &ExpectedGroupBody,
    mut offset: usize,
) -> usize {
    for (triplet, values) in body.discarded.iter().enumerate() {
        for (component, word) in values.iter().enumerate() {
            signed_patch(
                result,
                FlowPointerTableField::DiscardedTriplet {
                    group,
                    triplet,
                    component,
                },
                *word,
                offset,
                None,
            );
            offset += 4;
        }
    }
    offset
}

fn wide_patches(
    result: &mut Vec<PointerPatch>,
    group: usize,
    body: &ExpectedGroupBody,
    mut offset: usize,
) -> usize {
    for (array, expected) in body.arrays.iter().enumerate() {
        assert_eq!(offset, expected.start as usize);
        for (index, bits) in expected.bits.iter().enumerate() {
            let after = bits ^ 0x8000000000000001;
            result.push(PointerPatch {
                field: FlowPointerTableField::WideWord {
                    group,
                    array,
                    index,
                },
                before: FlowPointerTableValue::WideBits(*bits),
                after: FlowPointerTableValue::WideBits(after),
                offset,
                bytes: after.to_le_bytes().to_vec(),
            });
            offset += 8;
        }
        assert_eq!(offset, expected.end as usize);
    }
    offset
}

fn signed_patch(
    result: &mut Vec<PointerPatch>,
    field: FlowPointerTableField,
    before: i32,
    offset: usize,
    after: Option<i32>,
) {
    let next = after.unwrap_or(before ^ 0x80000001_u32 as i32);
    result.push(PointerPatch {
        field,
        before: FlowPointerTableValue::Signed32(before),
        after: FlowPointerTableValue::Signed32(next),
        offset,
        bytes: next.to_le_bytes().to_vec(),
    });
}

fn group_patches(result: &mut Vec<PointerPatch>, case: &TableCase, group: &ExpectedGroup) {
    let mut offset = group.start as usize + 4;
    if let Some(word) = group.discarded_count {
        signed_patch(
            result,
            FlowPointerTableField::DiscardedCount {
                group: group.ordinal as usize,
            },
            word,
            offset,
            None,
        );
        offset += 4;
    }
    if let Some(body) = &group.body {
        body_patches(result, case, group, body, offset);
    }
}

fn member_patches(
    result: &mut Vec<PointerPatch>,
    ordinal: usize,
    body: &ExpectedGroupBody,
    mut offset: usize,
) -> usize {
    for (component, word) in body.members.iter().enumerate() {
        signed_patch(
            result,
            FlowPointerTableField::Member {
                group: ordinal,
                component,
            },
            *word,
            offset,
            None,
        );
        offset += 4;
    }
    offset
}

fn pre_pair_patch(
    result: &mut Vec<PointerPatch>,
    ordinal: usize,
    word: Option<i32>,
    mut offset: usize,
) -> usize {
    if let Some(word) = word {
        signed_patch(
            result,
            FlowPointerTableField::DiscardedPrePairs { group: ordinal },
            word,
            offset,
            None,
        );
        offset += 4;
    }
    offset
}

fn pair_patches(
    result: &mut Vec<PointerPatch>,
    context: Option<u32>,
    group: usize,
    body: &ExpectedGroupBody,
    mut offset: usize,
) -> usize {
    for (pair, values) in body.pairs.iter().enumerate() {
        for (component, value) in values.iter().enumerate() {
            push_pair_component(result, context, group, pair, component, *value, offset);
            offset += 4;
        }
    }
    offset
}
fn push_pair_component(
    result: &mut Vec<PointerPatch>,
    context: Option<u32>,
    group: usize,
    pair: usize,
    component: usize,
    value: i32,
    offset: usize,
) {
    let after = context.map(|bound| ((value as u32 + 1) % bound) as i32);
    let field = FlowPointerTableField::PairIndex {
        group,
        pair,
        component,
    };
    signed_patch(result, field, value, offset, after);
}
