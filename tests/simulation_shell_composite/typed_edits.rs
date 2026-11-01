use super::*;
use std::sync::Arc;

fn typed_replacement(
    value: &SimulationShellCompositeValue,
) -> Option<(SimulationShellCompositeValue, Vec<u8>)> {
    match value {
        SimulationShellCompositeValue::DoubleBits(bits) => {
            let changed = bits ^ 0x8000000000000001;
            Some((
                SimulationShellCompositeValue::DoubleBits(changed),
                changed.to_le_bytes().to_vec(),
            ))
        }
        SimulationShellCompositeValue::Bytes(bytes) if !bytes.is_empty() => {
            let mut changed = bytes.to_vec();
            changed[0] ^= 0xff;
            Some((
                SimulationShellCompositeValue::Bytes(Arc::from(changed.clone())),
                changed,
            ))
        }
        _ => None,
    }
}

fn check_typed_edit(
    body: &SimulationShellComposite,
    index: usize,
    replacement: SimulationShellCompositeValue,
    raw: &[u8],
) {
    let original = body.to_bytes();
    let (start, end) = body.field_offsets()[index];
    let expected = [&original[..start], raw, &original[end..]].concat();
    let parent = body.field_id(index).unwrap();
    let edit = body.edit_field(&parent, replacement).unwrap();
    assert_eq!(edit.archive().to_bytes(), expected);
    assert_eq!(edit.source().to_bytes(), original);
    assert_eq!(body.to_bytes(), original);
    assert_ne!(edit.result(), parent);
    assert_typed_history(body, &edit, index);
    assert_supplied_nested_parents(&edit, index);
}

fn assert_supplied_nested_parents(edit: &SimulationShellCompositeEdit, changed: usize) {
    let SimulationShellCompositeValue::Embedded(donor) = edit.replacement() else {
        return;
    };
    for row in edit
        .nested_history()
        .iter()
        .filter(|row| row.outer_result.ordinal() == changed)
    {
        if let Some(result) = &row.result {
            let supplied = donor.field_handle(result.index()).unwrap();
            assert!(
                row.parents
                    .contains(&SimulationShellCompositeNestedParent::Replacement(supplied))
            );
        }
    }
}

fn assert_typed_history(
    body: &SimulationShellComposite,
    edit: &SimulationShellCompositeEdit,
    changed: usize,
) {
    for (index, row) in edit.history().iter().enumerate() {
        let relation = if index == changed {
            SimulationShellCompositeRelation::Derived
        } else {
            SimulationShellCompositeRelation::Copied
        };
        assert_eq!(row.relation, relation);
        assert_eq!(row.result, edit.archive().field_id(index).unwrap());
        assert!(
            row.parents
                .contains(&SimulationShellCompositeHistoryParent::Source(
                    body.field_id(index).unwrap()
                ))
        );
        if index != changed {
            assert_eq!(body.fields()[index], edit.archive().fields()[index]);
        }
    }
}

#[test]
fn raw_double_bits_and_populated_blobs_edit_only_the_requested_complete_span() {
    let mut changed = 0;
    for body in contracts::owned_composites() {
        for (index, field) in body.fields().iter().enumerate() {
            if let Some((replacement, raw)) = typed_replacement(&field.value) {
                check_typed_edit(&body, index, replacement, &raw);
                changed += 1;
            }
        }
    }
    assert!(changed > 1000);
}

#[test]
fn complete_embedded_archive_replacement_preserves_sibling_bytes_and_tracks_supplied_parent() {
    let bodies = contracts::owned_composites();
    let donors = embedded_donors(&bodies);
    let mut changed = 0;
    for body in &bodies {
        for (index, field) in body.fields().iter().enumerate() {
            if let SimulationShellCompositeValue::Embedded(original) = &field.value {
                let donor = distinct_donor(&donors, original);
                let SimulationShellCompositeValue::Embedded(archive) = donor else {
                    panic!("expected archive donor")
                };
                check_typed_edit(body, index, donor.clone(), &archive.to_bytes().unwrap());
                changed += 1;
            }
        }
    }
    assert_eq!(changed, 120);
}

fn embedded_donors(bodies: &[SimulationShellComposite]) -> Vec<SimulationShellCompositeValue> {
    bodies
        .iter()
        .flat_map(|body| body.fields())
        .filter(|field| matches!(field.value, SimulationShellCompositeValue::Embedded(_)))
        .map(|field| field.value.clone())
        .collect()
}

fn distinct_donor<'a>(
    donors: &'a [SimulationShellCompositeValue],
    original: &bessemer::model_archives::SimulationEmbeddedArchive,
) -> &'a SimulationShellCompositeValue {
    donors.iter().find(|value| matches!(value, SimulationShellCompositeValue::Embedded(archive) if archive.to_bytes() != original.to_bytes())).unwrap()
}
