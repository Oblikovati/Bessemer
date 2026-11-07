use super::boundaries::owned_body;
use super::*;
use bessemer::simulation::{
    SimulationMaterialHistoryParent as Parent, SimulationMaterialRelation as Relation,
    SimulationMaterialValue as Value,
};

pub(super) fn ordinal(body: &SimulationStudyMaterial, path: &str) -> usize {
    body.fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap()
}

pub(super) fn replace(
    body: &SimulationStudyMaterial,
    path: &str,
    value: Value,
) -> SimulationMaterialEdit {
    body.edit_field(&body.field_id(ordinal(body, path)).unwrap(), value)
        .unwrap()
}

pub(super) fn assert_history(edit: &SimulationMaterialEdit, requested: usize, changed: bool) {
    assert_eq!(edit.history().len(), edit.source().fields().len());
    for (index, row) in edit.history().iter().enumerate() {
        let derived = changed && index == requested;
        let mut parents = vec![Parent::Source(edit.source().field_id(index).unwrap())];
        if derived {
            parents.push(Parent::Replacement);
        }
        assert_eq!(row.parents.as_ref(), parents);
        assert_eq!(row.result, edit.archive().field_id(index).unwrap());
        assert_eq!(
            row.relation,
            if derived {
                Relation::Derived
            } else {
                Relation::Copied
            }
        );
    }
}

pub(super) fn assert_preserved(edit: &SimulationMaterialEdit, requested: usize) {
    let before = edit.source().to_bytes();
    let after = edit.archive().to_bytes();
    let (start, end) = edit.source().field_offsets()[requested];
    let (next, last) = edit.archive().field_offsets()[requested];
    assert_eq!(start, next);
    assert_eq!(before[..start], after[..next]);
    assert_eq!(before[end..], after[last..]);
    for index in 0..edit.source().fields().len() {
        if index != requested {
            assert_eq!(
                edit.source().fields()[index],
                edit.archive().fields()[index]
            );
        }
    }
    assert_text_parents(edit.archive(), &after);
}

fn assert_replacement(source: &SimulationStudyMaterial, path: &str, value: Value) {
    let edit = replace(source, path, value.clone());
    let requested = ordinal(source, path);
    assert_eq!(edit.parent(), source.field_id(requested).unwrap());
    assert_eq!(edit.result(), edit.archive().field_id(requested).unwrap());
    assert_eq!(*edit.replacement(), value);
    assert_eq!(edit.archive().field(&edit.result()).unwrap().value, value);
    assert_eq!(edit.source().to_bytes(), source.to_bytes());
    assert_eq!(
        edit.coverage().completion,
        SimulationMaterialCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationMaterialCheckMode::ExactBodyAndPreservation
    );
    assert_eq!(edit.coverage().requested_pattern_error_bits, 0);
    assert_history(&edit, requested, true);
    assert_preserved(&edit, requested);
}

#[test]
fn all_material_value_kinds_have_isolated_edits_and_complete_retained_history() {
    let source = owned_body(891107, Some(1));
    let original = source.to_bytes();
    assert_replacement(&source, "word68", Value::Signed(-12345));
    assert_replacement(
        &source,
        "property0.scalar",
        Value::DoubleBits(0x7ff800000000abcd),
    );
    let text = SimulationPersistText::from_units(Some(&[65, 0xd800, 0, 66]), 64).unwrap();
    assert_replacement(&source, "property0.auxiliary_text", Value::Text(text));
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn every_exact_noop_keeps_all_original_occurrence_identities() {
    let source = owned_body(891107, Some(1));
    for (index, field) in source.fields().iter().enumerate() {
        let id = source.field_id(index).unwrap();
        let edit = source.edit_field(&id, field.value.clone()).unwrap();
        assert_eq!(edit.parent(), id);
        assert_eq!(edit.result(), id);
        assert_eq!(edit.archive().to_bytes(), source.to_bytes());
        assert_eq!(
            edit.coverage().completion,
            SimulationMaterialCompletion::Unchanged
        );
        assert_history(&edit, index, false);
    }
}

#[test]
fn absent_count_edits_update_declaration_coverage_without_inventing_points() {
    let source = owned_body(891107, Some(0));
    let edit = replace(&source, "property0.pair_count", Value::Signed(9999));
    assert_eq!(edit.coverage().declared_entries, 10018);
    assert_eq!(edit.coverage().point_pairs, 7);
    assert_history(&edit, ordinal(&source, "property0.pair_count"), true);
    assert_preserved(&edit, ordinal(&source, "property0.pair_count"));
}

#[test]
fn prefix_origin_edits_retain_the_scoped_source_without_capturing_carrier_bytes() {
    let source = owned_body(891107, Some(1));
    let mut carrier = source.to_bytes();
    carrier.extend_from_slice(b"following carrier");
    let (prefix, consumed) =
        SimulationStudyMaterial::from_prefix(&carrier, source.version(), LIMITS).unwrap();
    let edit = replace(&prefix, "word68", Value::Signed(-12345));
    assert_eq!(
        edit.source().coverage().boundary,
        SimulationMaterialBoundary::Prefix
    );
    assert_eq!(edit.source().to_bytes(), carrier[..consumed]);
    assert_text_parents(edit.source(), &carrier[..consumed]);
    assert_preserved(&edit, ordinal(&prefix, "word68"));
    assert_eq!(&carrier[consumed..], b"following carrier");
}

#[test]
fn invalid_types_structural_changes_foreign_and_stale_ids_preserve_the_source() {
    let source = owned_body(891107, Some(0));
    let foreign = owned_body(891107, Some(0));
    let original = source.to_bytes();
    assert!(
        source
            .edit_field(&foreign.field_id(0).unwrap(), Value::Signed(1))
            .is_err()
    );
    let id = source.field_id(ordinal(&source, "word68")).unwrap();
    assert!(source.edit_field(&id, Value::Unsigned(1)).is_err());
    let changed = source.edit_field(&id, Value::Signed(42)).unwrap();
    assert!(changed.archive().field(&id).is_err());
    assert_invalid_structure(&source);
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn material_edit_results_and_history_contracts_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<SimulationMaterialEdit>();
    shared::<SimulationMaterialHistory>();
    shared::<SimulationMaterialHistoryParent>();
    shared::<SimulationMaterialRelation>();
}

fn assert_invalid_structure(source: &SimulationStudyMaterial) {
    let count = source
        .field_id(ordinal(source, "property0.pair_count"))
        .unwrap();
    assert!(source.edit_field(&count, Value::Signed(-1)).is_err());
    let presence = source
        .field_id(ordinal(source, "property0.curve_present"))
        .unwrap();
    assert!(source.edit_field(&presence, Value::Signed(1)).is_err());
}
