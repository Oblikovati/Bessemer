use super::*;

pub(super) struct LinkPatch {
    pub field: FlowLinkRecordField,
    pub before: FlowLinkRecordValue,
    pub after: FlowLinkRecordValue,
    pub offset: usize,
}

pub(super) fn patches(case: &LinkCase) -> Vec<LinkPatch> {
    let mut result = Vec::new();
    for entry in &case.entries {
        entry_patches(&mut result, entry, case.pointer_count);
    }
    tail_patches(&mut result, &case.tail);
    if let Some(numeric) = &case.tail.numeric {
        numeric_patches(&mut result, numeric);
    }
    if let Some(tail) = &case.tail.final_tail {
        final_patches(&mut result, tail);
    }
    result
}

fn signed(field: FlowLinkRecordField, value: i32, offset: usize) -> LinkPatch {
    LinkPatch {
        field,
        before: FlowLinkRecordValue::Signed32(value),
        after: FlowLinkRecordValue::Signed32(value ^ i32::MIN),
        offset,
    }
}

fn wide_patch(field: FlowLinkRecordField, value: u64, offset: usize) -> LinkPatch {
    LinkPatch {
        field,
        before: FlowLinkRecordValue::WideBits(value),
        after: FlowLinkRecordValue::WideBits(value ^ 0x8000000000000001),
        offset,
    }
}

fn entry_patches(result: &mut Vec<LinkPatch>, entry: &LinkEntry, bound: Option<i32>) {
    entry_references(result, entry, bound);
    entry_wide_and_code(result, entry);
    entry_integers(result, entry);
}

fn tail_patches(result: &mut Vec<LinkPatch>, tail: &LinkTail) {
    tail_members(result, tail);
    for (index, &v) in tail.discarded.iter().enumerate() {
        result.push(signed(
            FlowLinkRecordField::Discarded { index },
            v,
            tail.start as usize + 44 + index * 4,
        ));
    }
    tail_control(result, tail);
    for (index, &v) in tail.conditional.iter().enumerate() {
        result.push(signed(
            FlowLinkRecordField::Conditional { index },
            v,
            tail.conditional_start as usize + index * 4,
        ));
    }
}

fn numeric_patches(result: &mut Vec<LinkPatch>, tail: &NumericTail) {
    for (index, &v) in tail.wide.iter().enumerate() {
        result.push(wide_patch(
            FlowLinkRecordField::NumericWide { index },
            v,
            tail.start as usize + 4 + index * 8,
        ));
    }
    if let Some(indices) = &tail.indices {
        for (index, &v) in indices.iter().enumerate() {
            result.push(signed(
                FlowLinkRecordField::NumericIndex { index },
                v,
                tail.start as usize + 4 + tail.wide.len() * 8 + index * 4,
            ));
        }
    }
}

fn final_patches(result: &mut Vec<LinkPatch>, tail: &FinalTail) {
    for (index, &v) in tail.values.iter().enumerate() {
        let field = FlowLinkRecordField::FinalValue { index };
        let offset = tail.start as usize + 4 + index * tail.width as usize;
        result.push(if tail.width == 8 {
            wide_patch(field, v, offset)
        } else {
            signed(field, v as i32, offset)
        });
    }
}

fn entry_references(result: &mut Vec<LinkPatch>, entry: &LinkEntry, bound: Option<i32>) {
    let ordinal = entry.ordinal as usize;
    for (component, &v) in entry.references.iter().enumerate() {
        let mut patch = signed(
            FlowLinkRecordField::EntryReference {
                entry: ordinal,
                component,
            },
            v,
            entry.start as usize + component * 4,
        );
        if let Some(bound) = bound {
            patch.after = FlowLinkRecordValue::Signed32(if v != -1 || bound == 0 { -1 } else { 0 });
        }
        result.push(patch);
    }
}

fn entry_wide_and_code(result: &mut Vec<LinkPatch>, entry: &LinkEntry) {
    let ordinal = entry.ordinal as usize;
    for (index, &v) in entry.wide.iter().enumerate() {
        result.push(wide_patch(
            FlowLinkRecordField::EntryWide {
                entry: ordinal,
                index,
            },
            v,
            entry.start as usize + 8 + index * 8,
        ));
    }
    result.push(LinkPatch {
        field: FlowLinkRecordField::EntryCode { entry: ordinal },
        before: FlowLinkRecordValue::Byte(entry.code),
        after: FlowLinkRecordValue::Byte(entry.code ^ 255),
        offset: entry.start as usize + 64,
    });
}

fn entry_integers(result: &mut Vec<LinkPatch>, entry: &LinkEntry) {
    let ordinal = entry.ordinal as usize;
    for (index, &v) in entry.variable.iter().enumerate() {
        result.push(signed(
            FlowLinkRecordField::EntryInteger {
                entry: ordinal,
                index,
            },
            v,
            entry.start as usize + 66 + index * 4,
        ));
    }
}

fn tail_members(result: &mut Vec<LinkPatch>, tail: &LinkTail) {
    for (index, &v) in tail.members.iter().enumerate() {
        let mut patch = signed(
            FlowLinkRecordField::TailMember { index },
            v,
            tail.start as usize + index * 4,
        );
        if index == 6 && tail.control.is_some_and(|c| c > 0) {
            patch.after = patch.before;
        }
        result.push(patch);
    }
}

fn tail_control(result: &mut Vec<LinkPatch>, tail: &LinkTail) {
    if let Some(v) = tail.control {
        let mut patch = signed(
            FlowLinkRecordField::Control,
            v,
            tail.conditional_start as usize - 4,
        );
        patch.after = FlowLinkRecordValue::Signed32(if v > 0 { v ^ 1 } else { -1 });
        result.push(patch);
    }
}
