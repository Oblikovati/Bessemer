use super::*;
#[path = "edits/patches.rs"]
mod patches;
use patches::{SimpleBits, SimpleCell, SimplePatch, bytes, patches};
#[path = "edits/apply.rs"]
mod apply;
use apply::apply;

#[test]
fn every_owned_existing_field_has_exact_noop_full_patch_history_and_fresh_reverse() {
    for case in cases().into_iter().skip(4) {
        check_case_edits(&case);
    }
}

#[test]
#[ignore = "explicit complete native 407/437/411/413 existing-field census"]
fn every_native_existing_simple_table_field_preserves_complete_decoded_bytes() {
    for case in cases().into_iter().take(4) {
        check_case_edits(&case);
    }
}

fn check_case_edits(case: &SimpleCase) {
    let source = read_case(case);
    let requests = patches(case);
    if case.frame.name.starts_with("native") {
        println!(
            "native {}: {} field requests",
            case.frame.identifier,
            requests.len()
        );
    }
    for patch in requests {
        check_patch(&source, case, &patch);
    }
}

fn check_patch(source: &SimpleView, case: &SimpleCase, patch: &SimplePatch) {
    let noop = apply(source, patch.cell, patch.before).unwrap();
    assert_eq!(noop.result.source().as_bytes(), source.source().as_bytes());
    assert_eq!(noop.result.record(), source.record());
    check_history(&noop.carrier, false);
    let edit = apply(source, patch.cell, patch.after).unwrap();
    let mut expected = case.frame.decoded.clone();
    let after = bytes(patch.after);
    expected[patch.offset..patch.offset + after.len()].copy_from_slice(&after);
    assert_eq!(edit.result.decoded(), expected);
    assert_eq!(edit.source.record(), source.record());
    assert_eq!(edit.source.source().as_bytes(), source.source().as_bytes());
    check_history(&edit.carrier, expected != case.frame.decoded);
    check_reverse(source, &edit.result, case, patch);
}

fn check_reverse(
    source: &SimpleView,
    changed: &SimpleView,
    case: &SimpleCase,
    patch: &SimplePatch,
) {
    let reverse = apply(changed, patch.cell, patch.before).unwrap();
    assert_eq!(reverse.result.decoded(), case.frame.decoded);
    assert_eq!(source.decoded(), case.frame.decoded);
    if !case.frame.compressed {
        assert_eq!(
            reverse.result.source().as_bytes(),
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

pub(super) fn first_edit_bytes(source: &SimpleView) -> bessemer::Result<Vec<u8>> {
    let edit = apply(source, first_cell(source), first_bits(source))?;
    Ok(edit.result.source().as_bytes().to_vec())
}

fn first_cell(source: &SimpleView) -> SimpleCell {
    match source {
        SimpleView::Parallel(_) => SimpleCell::Parallel {
            group: 0,
            array: 0,
            index: 0,
        },
        SimpleView::LeafPairs(_) => SimpleCell::Leaf {
            row: 0,
            component: 0,
        },
        SimpleView::Five(_) => SimpleCell::Five {
            row: 0,
            component: 0,
        },
        SimpleView::Bytes(_) => SimpleCell::Byte(0),
    }
}

fn first_bits(source: &SimpleView) -> SimpleBits {
    match source {
        SimpleView::Parallel(_) | SimpleView::LeafPairs(_) => SimpleBits::Wide(0x8000000000000001),
        SimpleView::Five(_) => SimpleBits::Word(0x80000001),
        SimpleView::Bytes(_) => SimpleBits::Byte(0x80),
    }
}
