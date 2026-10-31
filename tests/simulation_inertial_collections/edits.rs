use super::*;
use bessemer::model_archives::{SimulationEmbeddedArchive, SimulationEmbeddedProfile};
use bessemer::simulation::{
    SimulationInertialEdit, SimulationInertialFieldId, SimulationInertialHistory,
    SimulationInertialHistoryParent as Parent, SimulationInertialNestedHistory,
    SimulationInertialNestedParent as NestedParent, SimulationInertialRelation as Relation,
};

pub(super) fn role(collection: &SimulationInertialCollection, path: &str) -> usize {
    collection
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap()
}

fn replace(
    collection: &SimulationInertialCollection,
    path: &str,
    value: SimulationInertialValue,
) -> SimulationInertialEdit {
    collection
        .edit_field(&collection.field_id(role(collection, path)).unwrap(), value)
        .unwrap()
}

fn null_archive() -> SimulationEmbeddedArchive {
    let mut bytes = 14000_u32.to_le_bytes().to_vec();
    bytes.extend_from_slice(&[0, 0]);
    SimulationEmbeddedArchive::from_bytes(
        &bytes,
        SimulationEmbeddedProfile::Modern,
        LIMITS.embedded,
    )
    .unwrap()
}

fn replacements() -> [(&'static str, SimulationInertialValue); 6] {
    let text = bessemer::simulation::SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 32)
        .unwrap();
    [
        (
            "record0.base.integer",
            SimulationInertialValue::Signed(12345),
        ),
        (
            "record0.base.scalar_block.e8",
            SimulationInertialValue::Unsigned(0x12345678),
        ),
        (
            "record0.base.scalar_block.f8",
            SimulationInertialValue::DoubleBits(0x7ff800000000abcd),
        ),
        ("record0.base.text[0]", SimulationInertialValue::Text(text)),
        (
            "record0.reference0.blob",
            SimulationInertialValue::Bytes(std::sync::Arc::from([1_u8, 2, 3])),
        ),
        (
            "record0.reference0.archive0",
            SimulationInertialValue::Embedded(Box::new(null_archive())),
        ),
    ]
}

pub(super) fn assert_outer(edit: &SimulationInertialEdit, requested: usize, changed: bool) {
    assert_eq!(edit.history().len(), edit.source().fields().len());
    for (ordinal, row) in edit.history().iter().enumerate() {
        let derived = changed && requested == ordinal;
        assert_eq!(row.result, edit.archive().field_id(ordinal).unwrap());
        assert_eq!(
            row.relation,
            if derived {
                Relation::Derived
            } else {
                Relation::Copied
            }
        );
        let mut expected = vec![Parent::Source(edit.source().field_id(ordinal).unwrap())];
        if derived {
            expected.push(Parent::Replacement);
        }
        assert_eq!(row.parents.as_ref(), expected);
    }
}

pub(super) fn assert_isolated(edit: &SimulationInertialEdit, requested: usize) {
    let before = edit.source().to_bytes();
    let after = edit.archive().to_bytes();
    let (start, end) = edit.source().field_offsets()[requested];
    let (next_start, next_end) = edit.archive().field_offsets()[requested];
    assert_eq!(start, next_start);
    assert_eq!(&before[..start], &after[..next_start]);
    assert_eq!(&before[end..], &after[next_end..]);
    for (ordinal, (original, result)) in edit
        .source()
        .fields()
        .iter()
        .zip(edit.archive().fields())
        .enumerate()
    {
        if ordinal != requested {
            assert_eq!(original, result);
        }
    }
}

#[test]
fn each_family_supports_all_six_typed_replacements_with_complete_isolated_results() {
    for kind in KINDS {
        let source = owned_collection(kind, 840002);
        let original = source.to_bytes();
        for (path, value) in replacements() {
            assert_replacement(&source, path, value);
        }
        assert_eq!(source.to_bytes(), original);
    }
}

#[test]
fn noops_preserve_outer_and_every_embedded_occurrence_identity() {
    for kind in KINDS {
        assert_noop(&owned_collection(kind, 840002));
    }
}

#[test]
fn foreign_stale_out_of_range_wrong_type_and_structural_edits_preserve_inputs() {
    let source = owned_collection(KINDS[0], 840002);
    let foreign = owned_collection(KINDS[0], 840002);
    let original = source.to_bytes();
    assert_invalid_identities(&source, &foreign);
    assert_invalid_replacements(&source);
    let id = source.field_id(0).unwrap();
    let edit = replace(
        &source,
        "record0.base.integer",
        SimulationInertialValue::Signed(12345),
    );
    assert!(edit.archive().field(&id).is_err());
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn embedded_replacements_compose_deletions_and_replacement_only_creations() {
    let source = owned_collection(KINDS[0], 840002);
    let ordinal = role(&source, "record0.reference0.archive0");
    let original = source.fields()[ordinal].value.clone();
    let removed = replace(
        &source,
        "record0.reference0.archive0",
        SimulationInertialValue::Embedded(Box::new(null_archive())),
    );
    assert_deleted(&removed);
    let restored = replace(removed.archive(), "record0.reference0.archive0", original);
    assert_eq!(restored.archive().to_bytes(), source.to_bytes());
    assert_created(&restored);
}

#[test]
fn immutable_edit_contracts_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<SimulationInertialEdit>();
    shared::<SimulationInertialFieldId>();
    shared::<SimulationInertialHistory>();
    shared::<Parent>();
    shared::<SimulationInertialNestedHistory>();
    shared::<NestedParent>();
    shared::<Relation>();
}

fn threaded_edits(
    source: &SimulationInertialCollection,
    expected: &[Vec<u8>],
    ordinal: usize,
    thread: usize,
) {
    for step in 0..32 {
        let selected = (thread + step) % expected.len();
        let edit = replace(
            source,
            "record0.base.integer",
            SimulationInertialValue::Signed(12345 + selected as i32),
        );
        assert_eq!(edit.archive().to_bytes(), expected[selected]);
        assert_outer(&edit, ordinal, true);
        assert_isolated(&edit, ordinal);
        assert_eq!(
            edit.nested_history().len(),
            source.coverage().embedded_fields
        );
        assert!(
            edit.nested_history()
                .iter()
                .all(|row| row.relation == Relation::Copied && row.parents.len() == 1)
        );
    }
}

fn concurrent_family(kind: SimulationInertialKind) {
    let source = owned_collection(kind, 840002);
    let original = source.to_bytes();
    let ordinal = role(&source, "record0.base.integer");
    let expected = serial_images(&source);
    run_threads(&source, &expected, ordinal);
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn concurrent_edits_match_serial_bytes_and_every_parent_across_all_families() {
    for kind in KINDS {
        concurrent_family(kind);
    }
}

fn assert_replacement(
    source: &SimulationInertialCollection,
    path: &str,
    value: SimulationInertialValue,
) {
    let ordinal = role(source, path);
    let edit = replace(source, path, value.clone());
    assert_eq!(edit.parent(), source.field_id(ordinal).unwrap());
    assert_eq!(edit.replacement(), &value);
    assert_eq!(edit.archive().field(&edit.result()).unwrap().value, value);
    assert_eq!(
        edit.coverage().completion,
        SimulationInertialCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationInertialCheckMode::ExactSelectedProfilesAndPreservation
    );
    assert_outer(&edit, ordinal, true);
    assert_isolated(&edit, ordinal);
}

fn assert_noop(source: &SimulationInertialCollection) {
    let id = source.field_id(0).unwrap();
    let edit = source
        .edit_field(&id, source.field(&id).unwrap().value.clone())
        .unwrap();
    assert_eq!(
        edit.coverage().completion,
        SimulationInertialCompletion::Unchanged
    );
    assert_eq!(edit.result(), id);
    assert_outer(&edit, 0, false);
    assert_nested_noop(&edit);
}

fn assert_invalid_identities(
    source: &SimulationInertialCollection,
    foreign: &SimulationInertialCollection,
) {
    assert!(source.field(&foreign.field_id(0).unwrap()).is_err());
    assert!(source.field_id(source.fields().len()).is_err());
    assert!(
        source
            .edit_field(
                &foreign.field_id(0).unwrap(),
                SimulationInertialValue::Signed(2)
            )
            .is_err()
    );
}

fn assert_invalid_replacements(source: &SimulationInertialCollection) {
    let id = source.field_id(0).unwrap();
    assert!(
        source
            .edit_field(&id, SimulationInertialValue::Unsigned(2))
            .is_err()
    );
    assert!(
        source
            .edit_field(&id, SimulationInertialValue::Signed(1))
            .is_err()
    );
}

fn assert_deleted(edit: &SimulationInertialEdit) {
    let deleted: Vec<_> = edit
        .nested_history()
        .iter()
        .filter(|row| row.relation == Relation::Deleted)
        .collect();
    assert!(!deleted.is_empty());
    for row in deleted {
        assert!(row.result.is_none());
        assert!(matches!(row.parents.as_ref(), [NestedParent::Source(_)]));
    }
}

fn assert_created(edit: &SimulationInertialEdit) {
    let created: Vec<_> = edit
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

fn assert_nested_noop(edit: &SimulationInertialEdit) {
    assert_eq!(
        edit.nested_history().len(),
        edit.source().coverage().embedded_fields
    );
    for row in edit.nested_history() {
        let [NestedParent::Source(parent)] = row.parents.as_ref() else {
            panic!("expected one source parent");
        };
        assert_eq!(row.result.as_ref(), Some(parent));
        assert_eq!(row.relation, Relation::Copied);
    }
}

fn serial_images(source: &SimulationInertialCollection) -> Vec<Vec<u8>> {
    (0..4)
        .map(|index| {
            replace(
                source,
                "record0.base.integer",
                SimulationInertialValue::Signed(12345 + index),
            )
            .archive()
            .to_bytes()
        })
        .collect()
}

fn run_threads(source: &SimulationInertialCollection, expected: &[Vec<u8>], ordinal: usize) {
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|thread| scope.spawn(move || threaded_edits(source, expected, ordinal, thread)))
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
    });
}

#[test]
fn semantically_equal_embedded_text_with_different_wire_framing_is_a_real_edit() {
    let source = owned_collection(KINDS[0], 840002);
    let (edit, extended) = string_framing_edit(&source);
    let ordinal = role(edit.archive(), "record0.reference0.archive0");
    super::load_strings::assert_wire_value(&edit.archive().fields()[ordinal].value, &extended);
    assert_eq!(
        edit.coverage().completion,
        SimulationInertialCompletion::Changed
    );
    assert_outer(&edit, ordinal, true);
    assert_isolated(&edit, ordinal);
}

fn string_framing_edit(
    source: &SimulationInertialCollection,
) -> (SimulationInertialEdit, SimulationEmbeddedArchive) {
    let short = super::load_strings::framed_string_archive(false, LIMITS.embedded);
    let extended = super::load_strings::framed_string_archive(true, LIMITS.embedded);
    assert_eq!(short.fields(), extended.fields());
    assert_ne!(short.to_bytes().unwrap(), extended.to_bytes().unwrap());
    let prepared = replace(
        source,
        "record0.reference0.archive0",
        SimulationInertialValue::Embedded(Box::new(short)),
    );
    let edit = replace(
        prepared.archive(),
        "record0.reference0.archive0",
        SimulationInertialValue::Embedded(Box::new(extended.clone())),
    );
    (edit, extended)
}
