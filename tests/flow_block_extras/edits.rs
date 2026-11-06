use super::*;
#[path = "edits/values.rs"]
mod values;
use values::{changed_value, samples, value_bytes};

#[test]
fn every_small_owned_existing_element_has_exact_noop_patch_history_and_reverse() {
    let cases = cases();
    let frames = oracle::frames();
    for (case, frame) in cases.iter().zip(&frames).skip(1) {
        if large(case) {
            continue;
        }
        let source = extras(case);
        for (ordinal, field) in frame.fields.iter().enumerate() {
            if !field.editable {
                continue;
            }
            for index in 0..field.layout.element_count {
                check_patch(&source, case, frame, ordinal, index);
            }
        }
    }
}

#[test]
fn large_owned_first_last_field_family_samples_preserve_complete_decoded_bytes() {
    let cases = cases();
    let frames = oracle::frames();
    for (case, frame) in cases
        .iter()
        .zip(&frames)
        .skip(1)
        .filter(|(case, _)| large(case))
    {
        let source = extras(case);
        for (ordinal, index) in samples(frame) {
            check_patch(&source, case, frame, ordinal, index);
        }
    }
}

#[test]
#[ignore = "explicit native field-family sample validation; not an exhaustive native edit census"]
fn native_first_last_field_family_samples_preserve_complete_decoded_bytes() {
    let cases = cases();
    let frames = oracle::frames();
    let case = &cases[0];
    let frame = &frames[0];
    let source = extras(case);
    let requests = samples(frame);
    println!(
        "native 408: {} deterministic first/last field-family requests",
        requests.len()
    );
    for (ordinal, index) in requests {
        check_patch(&source, case, frame, ordinal, index);
    }
}

fn large(case: &ExtraCase) -> bool {
    ["list-max", "bytes-max", "nested-max"]
        .iter()
        .any(|profile| case.name.ends_with(profile))
}

fn check_patch(
    source: &FlowBlockExtras,
    case: &ExtraCase,
    frame: &ExpectedExtraFrame,
    ordinal: usize,
    index: usize,
) {
    let field = &frame.fields[ordinal];
    let width = field.layout.element_bytes;
    let before = expected_value(
        field.encoding,
        &field.packed[index * width..(index + 1) * width],
    );
    let id = source.field_at(ordinal).unwrap().id();
    check_noop(source, &id, index, before);
    let after = changed_value(case, field, before);
    let edit = source.replace_field(&id, index, after).unwrap();
    let expected = patched_bytes(case, field, index, after);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().record(), source.record());
    assert_eq!(
        edit.result().field_at(ordinal).unwrap().value(index),
        Some(after)
    );
    check_history(edit.carrier_edit(), expected != case.decoded);
    check_reverse(source, edit.result(), ordinal, index, before, case);
}

fn check_noop(
    source: &FlowBlockExtras,
    id: &FlowBlockExtraFieldId,
    index: usize,
    before: FlowBlockExtraValue,
) {
    let edit = source.replace_field(id, index, before).unwrap();
    assert_eq!(
        edit.result().source().as_bytes(),
        source.source().as_bytes()
    );
    assert_eq!(edit.result().record(), source.record());
    assert_eq!(edit.result().field(id).unwrap().id(), *id);
    check_history(edit.carrier_edit(), false);
}

fn check_reverse(
    source: &FlowBlockExtras,
    changed: &FlowBlockExtras,
    ordinal: usize,
    index: usize,
    before: FlowBlockExtraValue,
    case: &ExtraCase,
) {
    let id = changed.field_at(ordinal).unwrap().id();
    let reverse = changed.replace_field(&id, index, before).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), case.decoded);
    assert_eq!(source.decoded_bytes(), case.decoded);
    assert_eq!(
        source.source().latest(408).unwrap().unwrap().payload,
        case.stored
    );
    if !case.compressed {
        assert_eq!(
            reverse.result().source().as_bytes(),
            source.source().as_bytes()
        );
    }
}

fn check_history(edit: &FlowCarrierEdit, changed: bool) {
    assert_eq!(edit.history().len(), 3);
    for (ordinal, (before, after)) in edit
        .source()
        .records()
        .zip(edit.result().records())
        .enumerate()
    {
        assert_eq!(edit.history()[ordinal].source, before.id);
        assert_eq!(edit.history()[ordinal].result, after.id);
        let relation = if changed && ordinal == 1 {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        };
        assert_eq!(edit.history()[ordinal].relation, relation);
        if ordinal != 1 {
            assert_eq!(before.payload, after.payload);
        }
    }
}

fn patched_bytes(
    case: &ExtraCase,
    field: &ExpectedExtraField,
    index: usize,
    after: FlowBlockExtraValue,
) -> Vec<u8> {
    let mut expected = case.decoded.clone();
    let offset = field.layout.first_byte + index * field.layout.stride_bytes;
    expected[offset..offset + field.layout.element_bytes].copy_from_slice(&value_bytes(after));
    expected
}
