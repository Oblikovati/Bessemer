use super::*;
#[path = "edits/patches.rs"]
mod patches;
use patches::{patches, LinkPatch};

#[test]
fn every_owned_existing_field_has_exact_noop_patch_history_and_fresh_reverse() {
    for case in cases().into_iter().skip(2) {
        check_edits(&case);
    }
}

#[test]
#[ignore = "explicit complete native record-402 field census"]
fn every_native_existing_link_field_preserves_complete_decoded_bytes() {
    for case in cases().into_iter().take(2) {
        check_edits(&case);
    }
}

fn check_edits(case: &LinkCase) {
    let source = links(case);
    let requests = patches(case);
    if case.name.starts_with("native") {
        println!(
            "native 402 context {:?}: {} field requests",
            case.pointer_count,
            requests.len()
        );
    }
    for patch in requests {
        check_patch(&source, case, patch);
    }
}

fn bytes(value: FlowLinkRecordValue) -> Vec<u8> {
    match value {
        FlowLinkRecordValue::Byte(v) => vec![v],
        FlowLinkRecordValue::Signed32(v) => v.to_le_bytes().to_vec(),
        FlowLinkRecordValue::WideBits(v) => v.to_le_bytes().to_vec(),
    }
}

fn check_patch(source: &FlowLinkRecords, case: &LinkCase, patch: LinkPatch) {
    let noop = source.replace_field(patch.field, patch.before).unwrap();
    assert_eq!(
        noop.result().source().as_bytes(),
        source.source().as_bytes()
    );
    assert_eq!(noop.result().record(), source.record());
    check_history(noop.carrier_edit(), false);
    let edit = source.replace_field(patch.field, patch.after).unwrap();
    let expected_bytes = bytes(patch.after);
    let mut expected = case.decoded.clone();
    expected[patch.offset..patch.offset + expected_bytes.len()].copy_from_slice(&expected_bytes);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().record(), source.record());
    assert_eq!(
        edit.source().source().as_bytes(),
        source.source().as_bytes()
    );
    check_history(edit.carrier_edit(), expected != case.decoded);
    check_reverse(source, edit.result(), &patch, case);
}

fn check_reverse(
    source: &FlowLinkRecords,
    changed: &FlowLinkRecords,
    patch: &LinkPatch,
    case: &LinkCase,
) {
    let reverse = changed.replace_field(patch.field, patch.before).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), case.decoded);
    assert_eq!(source.decoded_bytes(), case.decoded);
    assert_eq!(
        source.source().latest(402).unwrap().unwrap().payload,
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
        check_relation(edit, ordinal, changed);
        if ordinal != 1 {
            assert_eq!(before.payload, after.payload);
        }
    }
}

#[test]
fn eight_workers_keep_reads_and_edits_isolated_after_caller_input_mutation() {
    let mut case = cases()
        .into_iter()
        .find(|c| !c.compressed && c.name == "v9-full")
        .unwrap();
    let source = links(&case);
    case.stored.fill(0);
    let expected = case.decoded;
    std::thread::scope(|scope| {
        for code in 0..8 {
            let source = &source;
            let expected = &expected;
            scope.spawn(move || {
                check_worker(source, expected, code);
            });
        }
    });
}

fn check_relation(edit: &FlowCarrierEdit, ordinal: usize, changed: bool) {
    assert_eq!(
        edit.history()[ordinal].relation,
        if changed && ordinal == 1 {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        }
    );
}

fn check_worker(source: &FlowLinkRecords, expected: &[u8], code: u8) {
    let edit = source
        .replace_field(
            FlowLinkRecordField::EntryCode { entry: 0 },
            FlowLinkRecordValue::Byte(code),
        )
        .unwrap();
    assert_eq!(source.decoded_bytes(), expected);
    assert_eq!(edit.result().entry(0).unwrap().code_byte(), code);
}
