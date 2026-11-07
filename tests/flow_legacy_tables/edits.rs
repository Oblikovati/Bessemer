use super::*;
#[path = "patches.rs"]
mod patches;
use patches::{PointerPatch, pointer_patches};

#[test]
fn every_owned_existing_field_has_exact_noop_complete_patch_history_and_fresh_reverse() {
    for case in cases().into_iter().skip(3) {
        check_edits(&case);
    }
}

#[test]
#[ignore = "explicit complete native record-401/405 field census"]
fn every_native_existing_field_preserves_complete_decoded_bytes_and_reverses_logically() {
    for case in cases().into_iter().take(3) {
        check_edits(&case);
    }
}

fn check_edits(case: &TableCase) {
    match &case.expected {
        ExpectedTable::Pointers(_) => check_pointer_case(case),
        ExpectedTable::Words(words) => check_word_case(case, words),
    }
}

fn check_pointer_patch(source: &FlowPointerTables, case: &TableCase, patch: PointerPatch) {
    check_pointer_noop(source, &patch);
    let edit = source.replace_field(patch.field, patch.after).unwrap();
    let mut expected = case.frame.decoded.clone();
    expected[patch.offset..patch.offset + patch.bytes.len()].copy_from_slice(&patch.bytes);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().record(), source.record());
    assert_eq!(
        edit.source().source().as_bytes(),
        source.source().as_bytes()
    );
    check_history(edit.carrier_edit(), expected != case.frame.decoded);
    check_pointer_reverse(source, edit.result(), &patch, case);
}

fn check_word_patch(source: &FlowLegacyWords, case: &TableCase, index: usize, word: u32) {
    check_word_noop(source, index, word);
    let edit = source.replace_word(index, word ^ 0x80000001).unwrap();
    let mut expected = case.frame.decoded.clone();
    let offset = source.payload_interval().0 + index * 4;
    expected[offset..offset + 4].copy_from_slice(&(word ^ 0x80000001).to_le_bytes());
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().record(), source.record());
    assert_eq!(
        edit.source().source().as_bytes(),
        source.source().as_bytes()
    );
    check_history(edit.carrier_edit(), true);
    check_word_reverse(source, edit.result(), index, word, case);
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
        let expected = if changed && ordinal == 1 {
            FlowPayloadRelation::Derived
        } else {
            FlowPayloadRelation::Copied
        };
        assert_eq!(edit.history()[ordinal].relation, expected);
        if ordinal != 1 {
            assert_eq!(before.payload, after.payload);
            assert_eq!(before.identifier, after.identifier);
        }
    }
}

#[test]
fn caller_input_mutation_and_eight_workers_keep_table_edits_isolated() {
    let mut case = cases()
        .into_iter()
        .find(|case| {
            case.frame.name == "pairs-v3-full"
                && !case.frame.compressed
                && case.frame.pointer_count == Some(8)
        })
        .unwrap();
    let source = pointer_table(&case);
    case.frame.stored.fill(0);
    check_workers(&source);
    assert_eq!(source.decoded_bytes(), case.frame.decoded);
}

fn check_pointer_noop(source: &FlowPointerTables, patch: &PointerPatch) {
    let noop = source.replace_field(patch.field, patch.before).unwrap();
    assert_eq!(noop.result().record(), source.record());
    assert_eq!(
        noop.result().source().as_bytes(),
        source.source().as_bytes()
    );
    check_history(noop.carrier_edit(), false);
}

fn check_pointer_reverse(
    source: &FlowPointerTables,
    changed: &FlowPointerTables,
    patch: &PointerPatch,
    case: &TableCase,
) {
    let reverse = changed.replace_field(patch.field, patch.before).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), case.frame.decoded);
    if !case.frame.compressed {
        assert_eq!(
            reverse.result().source().as_bytes(),
            source.source().as_bytes()
        );
    }
}

fn check_word_noop(source: &FlowLegacyWords, index: usize, word: u32) {
    let noop = source.replace_word(index, word).unwrap();
    assert_eq!(noop.result().record(), source.record());
    assert_eq!(
        noop.result().source().as_bytes(),
        source.source().as_bytes()
    );
    check_history(noop.carrier_edit(), false);
}

fn check_word_reverse(
    source: &FlowLegacyWords,
    changed: &FlowLegacyWords,
    index: usize,
    word: u32,
    case: &TableCase,
) {
    let reverse = changed.replace_word(index, word).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), case.frame.decoded);
    if !case.frame.compressed {
        assert_eq!(
            reverse.result().source().as_bytes(),
            source.source().as_bytes()
        );
    }
}

fn check_pointer_case(case: &TableCase) {
    let source = pointer_table(case);
    let patches = pointer_patches(case);
    if case.frame.name.starts_with("native") {
        println!(
            "native {} context {:?}: {} field requests",
            case.frame.identifier,
            case.frame.pointer_count,
            patches.len()
        );
    }
    for patch in patches {
        check_pointer_patch(&source, case, patch);
    }
}
fn check_word_case(case: &TableCase, words: &ExpectedWords) {
    let source = word_table(case);
    if case.frame.name.starts_with("native") {
        println!("native 405: {} field requests", words.words.len());
    }
    for (index, word) in words.words.iter().enumerate() {
        check_word_patch(&source, case, index, *word);
    }
}
fn check_workers(source: &FlowPointerTables) {
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|worker| scope.spawn(move || check_worker(source, worker)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}
fn check_worker(table: &FlowPointerTables, worker: i32) {
    let field = FlowPointerTableField::Member {
        group: 0,
        component: 0,
    };
    let edit = table
        .replace_field(field, FlowPointerTableValue::Signed32(worker))
        .unwrap();
    assert_eq!(member_zero(edit.result()), worker);
    let reverse = edit
        .result()
        .replace_field(field, FlowPointerTableValue::Signed32(i32::MIN))
        .unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        table.source().as_bytes()
    );
}

fn member_zero(table: &FlowPointerTables) -> i32 {
    table.group(0).unwrap().body().unwrap().member_triplet()[0]
}
