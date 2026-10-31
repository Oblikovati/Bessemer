use super::*;
use bessemer::simulation::{
    SimulationThermalCheckMode as CheckMode, SimulationThermalCompletion as Completion,
    SimulationThermalHistoryParent as Parent, SimulationThermalNestedParent as NestedParent,
    SimulationThermalRelation as Relation, SimulationThermalValue as Value,
};

fn populated_collection() -> SimulationThermalCollection {
    let (kind, version, raw) = first_modern_owned();
    SimulationThermalCollection::from_bytes(raw, version, kind, LIMITS).unwrap()
}

pub(super) fn role_index(collection: &SimulationThermalCollection, path: &str) -> usize {
    collection
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap()
}

#[test]
fn thermal_read_coverage_distinguishes_empty_and_populated_exact_profiles() {
    let source = populated_collection();
    let coverage = source.coverage();
    assert_eq!(coverage.completion, Completion::CompletedRecords);
    assert_eq!(coverage.check_mode, CheckMode::ExactSelectedProfiles);
    assert_eq!(coverage.checked_fields, source.fields().len());
    assert_eq!(coverage.checked_bytes, source.to_bytes().len());
    assert_eq!(coverage.embedded_archives, 4);
    assert!(coverage.embedded_fields > 0);
    assert_eq!(coverage.diagnostics.len(), 4);
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    let empty =
        SimulationThermalCollection::from_bytes(&[0; 4], source.version(), source.kind(), LIMITS)
            .unwrap();
    assert_eq!(
        empty.coverage().completion,
        Completion::CompletedEmptyRecords
    );
}

#[test]
fn thermal_noop_edits_preserve_all_outer_and_inner_identities() {
    let source = populated_collection();
    let id = source.field_id(0).unwrap();
    assert_eq!(id.ordinal(), 0);
    let edit = source
        .edit_field(&id, source.field(&id).unwrap().value.clone())
        .unwrap();
    assert_eq!(edit.coverage().completion, Completion::Unchanged);
    assert_eq!(
        edit.coverage().check_mode,
        CheckMode::ExactSelectedProfilesAndPreservation
    );
    assert_eq!(edit.archive().field_id(0).unwrap(), id);
    assert_eq!(edit.parent(), edit.result());
    assert_outer_history(&edit, 0, false);
    assert_copied_nested(&edit);
    assert_eq!(
        edit.nested_history().len(),
        source.coverage().embedded_fields
    );
}

#[test]
fn changed_scalar_edits_retain_source_replacement_and_every_copy_relation() {
    let source = populated_collection();
    let ordinal = role_index(&source, "record0.base.integer");
    let id = source.field_id(ordinal).unwrap();
    let edit = source.edit_field(&id, Value::Signed(12345)).unwrap();
    assert_eq!(edit.coverage().completion, Completion::Changed);
    assert_eq!(edit.archive().fields()[ordinal].value, Value::Signed(12345));
    assert_outer_history(&edit, ordinal, true);
    assert!(edit.archive().field(&id).is_err());
    assert_eq!(edit.source().to_bytes(), source.to_bytes());
    assert_copied_nested(&edit);
}

#[test]
fn text_replacement_changes_length_without_changing_other_field_patterns() {
    let source = populated_collection();
    let ordinal = role_index(&source, "record0.base.text[0]");
    let text = bessemer::simulation::SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 32)
        .unwrap();
    let replacement = Value::Text(text);
    let edit = source
        .edit_field(&source.field_id(ordinal).unwrap(), replacement.clone())
        .unwrap();
    assert_eq!(edit.archive().fields()[ordinal].value, replacement);
    assert!(edit.archive().to_bytes().len() > source.to_bytes().len());
    assert_other_fields(&edit, ordinal);
    assert_eq!(edit.coverage().requested_pattern_error_bits, 0);
}

#[test]
fn thermal_edits_reject_foreign_ids_wrong_types_and_structural_count_changes() {
    let source = populated_collection();
    let foreign = populated_collection();
    assert!(
        source
            .edit_field(&foreign.field_id(0).unwrap(), Value::Signed(2))
            .is_err()
    );
    assert!(source.field_id(source.fields().len()).is_err());
    assert!(
        source
            .edit_field(&source.field_id(0).unwrap(), Value::Unsigned(2))
            .is_err()
    );
    assert!(
        source
            .edit_field(&source.field_id(0).unwrap(), Value::Signed(1))
            .is_err()
    );
    assert_eq!(source.record_count(), 2);
}

#[test]
fn same_width_reference_blob_edits_preserve_the_complete_container() {
    let source = populated_collection();
    let ordinal = role_index(&source, "record0.reference0.blob");
    let replacement = Value::Bytes(std::sync::Arc::from([1_u8, 2, 3]));
    let edit = source
        .edit_field(&source.field_id(ordinal).unwrap(), replacement.clone())
        .unwrap();
    assert_eq!(edit.archive().fields()[ordinal].value, replacement);
    let longer = Value::Bytes(std::sync::Arc::from([1_u8, 2, 3, 4]));
    assert!(
        source
            .edit_field(&source.field_id(ordinal).unwrap(), longer)
            .is_err()
    );
    assert_eq!(edit.source().to_bytes(), source.to_bytes());
}

#[test]
fn embedded_replacement_history_composes_deleted_and_derived_parents() {
    let source = populated_collection();
    let ordinal = role_index(&source, "record0.reference0.archive0");
    let replacement = empty_embedded();
    let edit = source
        .edit_field(
            &source.field_id(ordinal).unwrap(),
            Value::Embedded(Box::new(replacement.clone())),
        )
        .unwrap();
    assert_deleted_nested(&edit, ordinal);
    let derived = edit
        .nested_history()
        .iter()
        .find(|row| row.path.as_ref() == "root.archive_version")
        .unwrap();
    assert_eq!(derived.relation, Relation::Derived);
    assert!(derived.parents.iter().any(|parent| matches!(parent, NestedParent::Replacement(id) if *id == replacement.field_handle(0).unwrap())));
    assert_eq!(edit.archive().coverage().embedded_archives, 4);
}

fn empty_embedded() -> bessemer::model_archives::SimulationEmbeddedArchive {
    bessemer::model_archives::SimulationEmbeddedArchive::from_bytes(
        &[0x10, 0x27, 0, 0, 0, 0],
        bessemer::model_archives::SimulationEmbeddedProfile::Modern,
        LIMITS.embedded,
    )
    .unwrap()
}

pub(super) fn assert_outer_history(
    edit: &bessemer::simulation::SimulationThermalEdit,
    requested: usize,
    changed: bool,
) {
    assert_eq!(edit.history().len(), edit.source().fields().len());
    for (index, row) in edit.history().iter().enumerate() {
        let mut expected = vec![Parent::Source(edit.source().field_id(index).unwrap())];
        let relation = if changed && requested == index {
            expected.push(Parent::Replacement);
            Relation::Derived
        } else {
            Relation::Copied
        };
        assert_eq!(row.relation, relation);
        assert_eq!(row.parents.as_ref(), expected);
        assert_eq!(row.result, edit.archive().field_id(index).unwrap());
    }
}

pub(super) fn assert_copied_nested(edit: &bessemer::simulation::SimulationThermalEdit) {
    if edit.coverage().completion == Completion::Unchanged {
        assert_nested_identity(edit);
    }
    assert!(
        edit.nested_history()
            .iter()
            .all(|row| row.relation == Relation::Copied && row.parents.len() == 1)
    );
    assert_eq!(
        edit.nested_history().len(),
        edit.source().coverage().embedded_fields
    );
}

fn assert_nested_identity(edit: &bessemer::simulation::SimulationThermalEdit) {
    for row in edit.nested_history().iter() {
        let [NestedParent::Source(parent)] = row.parents.as_ref() else {
            panic!("expected copied source parent")
        };
        assert_eq!(row.result.as_ref(), Some(parent));
    }
}

pub(super) fn assert_other_fields(
    edit: &bessemer::simulation::SimulationThermalEdit,
    requested: usize,
) {
    for (index, (before, after)) in edit
        .source()
        .fields()
        .iter()
        .zip(edit.archive().fields())
        .enumerate()
    {
        if index != requested {
            assert_eq!(before, after);
        }
    }
}

fn assert_deleted_nested(edit: &bessemer::simulation::SimulationThermalEdit, requested: usize) {
    assert!(
        edit.nested_history()
            .iter()
            .any(|row| row.outer_parent.ordinal() == requested
                && row.relation == Relation::Deleted
                && row.result.is_none())
    );
}

#[test]
fn restored_embedded_shapes_retain_every_replacement_creation_parent() {
    let source = populated_collection();
    let ordinal = role_index(&source, "record0.reference0.archive0");
    let original = source.fields()[ordinal].value.clone();
    let removed = replace_value(
        &source,
        ordinal,
        Value::Embedded(Box::new(empty_embedded())),
    );
    let restored = replace_value(removed.archive(), ordinal, original);
    assert_eq!(restored.archive().to_bytes(), source.to_bytes());
    assert_created_nested(&restored);
}

fn assert_created_nested(restored: &bessemer::simulation::SimulationThermalEdit) {
    let created: Vec<_> = restored
        .nested_history()
        .iter()
        .filter(|row| row.relation == Relation::Created)
        .collect();
    assert!(!created.is_empty());
    for row in created {
        assert!(row.result.is_some());
        assert!(matches!(
            row.parents.as_ref(),
            [NestedParent::Replacement(_)]
        ));
    }
}

fn replace_value(
    source: &SimulationThermalCollection,
    ordinal: usize,
    value: Value,
) -> bessemer::simulation::SimulationThermalEdit {
    source
        .edit_field(&source.field_id(ordinal).unwrap(), value)
        .unwrap()
}

#[test]
fn all_six_typed_value_kinds_have_complete_isolated_edit_results() {
    let source = populated_collection();
    for (path, replacement) in replacement_values() {
        let ordinal = role_index(&source, path);
        let edit = replace_value(&source, ordinal, replacement.clone());
        assert_eq!(edit.replacement(), &replacement);
        assert_eq!(
            edit.archive().field(&edit.result()).unwrap().value,
            replacement
        );
        assert_outer_history(&edit, ordinal, true);
        assert_other_fields(&edit, ordinal);
    }
}

fn replacement_values() -> [(&'static str, Value); 6] {
    let text =
        bessemer::simulation::SimulationPersistText::from_payload(Some(&[0, 0]), 16).unwrap();
    [
        ("record0.base.integer", Value::Signed(1234)),
        ("record0.base.scalar_block.e8", Value::Unsigned(0x12345678)),
        (
            "record0.base.scalar_block.f8",
            Value::DoubleBits(0x7ff800000000abcd),
        ),
        ("record0.base.text[0]", Value::Text(text)),
        (
            "record0.reference0.blob",
            Value::Bytes(std::sync::Arc::from([1_u8, 2, 3])),
        ),
        (
            "record0.reference0.archive0",
            Value::Embedded(Box::new(empty_embedded())),
        ),
    ]
}

#[test]
fn thermal_edit_contracts_and_history_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<bessemer::simulation::SimulationThermalFieldId>();
    shared::<bessemer::simulation::SimulationThermalCompletion>();
    shared::<bessemer::simulation::SimulationThermalCheckMode>();
    shared::<bessemer::simulation::SimulationThermalDiagnostic>();
    shared::<bessemer::simulation::SimulationThermalCoverage>();
    shared::<bessemer::simulation::SimulationThermalRelation>();
    shared::<bessemer::simulation::SimulationThermalHistoryParent>();
    shared::<bessemer::simulation::SimulationThermalHistory>();
    shared::<bessemer::simulation::SimulationThermalNestedParent>();
    shared::<bessemer::simulation::SimulationThermalNestedHistory>();
    shared::<bessemer::simulation::SimulationThermalEdit>();
}

#[test]
fn concurrent_thermal_edits_publish_identical_bytes_and_isolated_histories() {
    let source = populated_collection();
    let ordinal = role_index(&source, "record0.base.integer");
    let expected = source
        .edit_field(&source.field_id(ordinal).unwrap(), Value::Signed(123))
        .unwrap()
        .archive()
        .to_bytes();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (source, expected) = (&source, &expected);
            scope.spawn(move || {
                exercise_thread(source, expected, ordinal);
            });
        }
    });
}

fn exercise_thread(source: &SimulationThermalCollection, expected: &[u8], ordinal: usize) {
    for _ in 0..32 {
        let edit = replace_value(source, ordinal, Value::Signed(123));
        assert_eq!(edit.archive().to_bytes(), expected);
        assert_outer_history(&edit, ordinal, true);
        assert_copied_nested(&edit);
    }
}

#[test]
fn every_owned_thermal_context_supports_complete_scalar_edit_history() {
    let mut cursor = OWNED;
    let mut count = 0;
    while !cursor.is_empty() {
        let (kind, version, _, raw) = read_candidate(&mut cursor);
        let spans = packet_word(&mut cursor) as usize;
        cursor = &cursor[spans * 8..];
        let source = SimulationThermalCollection::from_bytes(raw, version, kind, LIMITS).unwrap();
        let ordinal = role_index(&source, "record0.base.integer");
        let edit = replace_value(&source, ordinal, Value::Signed(12345));
        assert_outer_history(&edit, ordinal, true);
        assert_other_fields(&edit, ordinal);
        assert_copied_nested(&edit);
        assert_eq!(edit.source().to_bytes(), raw);
        count += 1;
    }
    assert_eq!(count, 295);
}

#[test]
fn embedded_text_prefix_width_changes_are_not_semantic_noops() {
    let source = populated_collection();
    let ordinal = role_index(&source, "record0.reference0.archive0");
    let short = super::load_strings::framed_string_archive(false, LIMITS.embedded);
    let extended = super::load_strings::framed_string_archive(true, LIMITS.embedded);
    assert_eq!(short.fields(), extended.fields());
    let prepared = replace_value(&source, ordinal, Value::Embedded(Box::new(short)));
    let edit = replace_value(
        prepared.archive(),
        ordinal,
        Value::Embedded(Box::new(extended.clone())),
    );
    super::load_strings::assert_wire_value(&edit.archive().fields()[ordinal].value, &extended);
    assert_eq!(edit.coverage().completion, Completion::Changed);
    assert_outer_history(&edit, ordinal, true);
    assert_other_fields(&edit, ordinal);
    let repeated = replace_value(edit.archive(), ordinal, Value::Embedded(Box::new(extended)));
    assert_eq!(repeated.coverage().completion, Completion::Unchanged);
    assert_eq!(repeated.result(), edit.result());
}
