//! Complete inertial/excitation records against actual copied-source native and owned packets (#136).
use bessemer::model_archives::ModelArchiveLimits;
use bessemer::simulation::{
    SimulationInertialCheckMode, SimulationInertialCollection, SimulationInertialCompletion,
    SimulationInertialCoverage, SimulationInertialDiagnostic, SimulationInertialField,
    SimulationInertialKind, SimulationInertialLimits, SimulationInertialValue,
    SimulationPersistVersion,
};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-inertial-loads/owned.bin");
const KINDS: [SimulationInertialKind; 3] = [
    SimulationInertialKind::Gravity,
    SimulationInertialKind::Centrifugal,
    SimulationInertialKind::BaseExcitation,
];
const LIMITS: SimulationInertialLimits = SimulationInertialLimits {
    maximum_bytes: 1000000,
    maximum_fields: 100000,
    maximum_entries: 2000000,
    maximum_text_bytes: 1000000,
    embedded: ModelArchiveLimits {
        maximum_bytes: 100000,
        maximum_fields: 50000,
        maximum_entries: 100000,
        maximum_depth: 256,
        maximum_objects: 20000,
    },
};

fn word(cursor: &mut &[u8]) -> usize {
    let (raw, remaining) = cursor.split_at(4);
    *cursor = remaining;
    u32::from_le_bytes(raw.try_into().unwrap()) as usize
}

fn candidate<'a>(
    cursor: &mut &'a [u8],
) -> (SimulationInertialKind, SimulationPersistVersion, &'a [u8]) {
    let kind = KINDS[word(cursor)];
    let version = SimulationPersistVersion::new(word(cursor) as i32).unwrap();
    assert_eq!(word(cursor), 1);
    let size = word(cursor);
    let (raw, remaining) = cursor.split_at(size);
    *cursor = remaining;
    (kind, version, raw)
}

fn assert_source_spans(collection: &SimulationInertialCollection, cursor: &mut &[u8]) {
    let mut fields = collection.field_offsets().iter().peekable();
    for _ in 0..word(cursor) {
        let (start, end) = (word(cursor), word(cursor));
        let mut consumed = start;
        while fields
            .peek()
            .is_some_and(|span| span.0 == consumed && (consumed < end || span.1 == end))
        {
            let (next, last) = *fields.next().unwrap();
            assert_eq!(next, consumed);
            consumed = last;
            if consumed == end {
                break;
            }
        }
        assert_eq!(consumed, end, "expected actual source span {start}..{end}");
    }
    assert!(fields.next().is_none());
}

fn assert_complete(collection: &SimulationInertialCollection, raw: &[u8]) {
    assert_eq!(collection.to_bytes(), raw);
    let constructed = SimulationInertialCollection::from_fields(
        collection.fields().to_vec(),
        collection.version(),
        collection.kind(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(constructed.to_bytes(), raw);
    assert_eq!(constructed.fields(), collection.fields());
    assert_eq!(constructed.field_offsets(), collection.field_offsets());
    assert_coverage(collection, raw);
}

fn replay(mut cursor: &[u8]) -> usize {
    let mut count = 0;
    while !cursor.is_empty() {
        let (kind, version, raw) = candidate(&mut cursor);
        let collection = SimulationInertialCollection::from_bytes(raw, version, kind, LIMITS)
            .unwrap_or_else(|error| {
                panic!(
                    "{kind:?} {version:?}: {error:?}; expected complete source-matched collection"
                )
            });
        assert_complete(&collection, raw);
        assert_source_spans(&collection, &mut cursor);
        count += 1;
    }
    count
}

#[test]
fn complete_owned_collections_match_all_actual_source_spans_and_reconstruct_exactly() {
    assert_eq!(replay(OWNED), 135);
}

#[test]
#[ignore = "Requires explicitly extracted complete inertial/excitation native candidates"]
fn complete_native_collections_match_every_original_retained_candidate() {
    let path = std::env::var("BESSEMER_INERTIAL_NATIVE_INPUT").expect(
        "BESSEMER_INERTIAL_NATIVE_INPUT=<unset>; expected explicit native candidate packet path",
    );
    let packet = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete candidate packet")
    });
    assert_eq!(replay(&packet), 18);
}

fn owned_collection(kind: SimulationInertialKind, version: i32) -> SimulationInertialCollection {
    let mut cursor = OWNED;
    while !cursor.is_empty() {
        let (selected, context, raw) = candidate(&mut cursor);
        let count = word(&mut cursor);
        cursor = &cursor[count * 8..];
        if selected == kind && context.value() == version {
            return SimulationInertialCollection::from_bytes(raw, context, kind, LIMITS).unwrap();
        }
    }
    panic!("{kind:?} {version}; expected an owned threshold case");
}

#[test]
fn excitation_retains_high_bit_unsigned_header_and_exact_three_four_text_gate() {
    for (version, expected_texts) in [(820011, 3), (820012, 4)] {
        let collection = owned_collection(KINDS[2], version);
        assert_excitation_gate(&collection, version, expected_texts);
    }
}

#[test]
fn empty_collections_are_completed_empty_with_exact_eof_for_each_family() {
    let version = SimulationPersistVersion::new(870022).unwrap();
    for kind in KINDS {
        let collection =
            SimulationInertialCollection::from_bytes(&[0; 4], version, kind, LIMITS).unwrap();
        assert_eq!(collection.record_count(), 0);
        assert_eq!(
            collection.coverage().completion,
            SimulationInertialCompletion::CompletedEmptyRecords
        );
        assert!(SimulationInertialCollection::from_bytes(&[0; 5], version, kind, LIMITS).is_err());
    }
}

#[test]
fn constructors_reject_wrong_paths_and_types_without_mutating_originals() {
    let source = owned_collection(KINDS[0], 840002);
    let original = source.to_bytes();
    let mut fields = source.fields().to_vec();
    fields[0].path = "fabricated.count".into();
    let error =
        SimulationInertialCollection::from_fields(fields, source.version(), source.kind(), LIMITS)
            .unwrap_err();
    assert!(error.cause.contains("fabricated.count") && error.cause.contains("collection.count"));
    let mut fields = source.fields().to_vec();
    fields[0].value = SimulationInertialValue::Unsigned(2);
    let error =
        SimulationInertialCollection::from_fields(fields, source.version(), source.kind(), LIMITS)
            .unwrap_err();
    assert!(error.cause.contains("Unsigned(2)") && error.cause.contains("Signed(2)"));
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn published_inertial_types_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<SimulationInertialCollection>();
    shared::<SimulationInertialKind>();
    shared::<SimulationInertialCoverage>();
    shared::<SimulationInertialLimits>();
    shared::<SimulationInertialField>();
    shared::<SimulationInertialValue>();
    shared::<SimulationInertialCheckMode>();
    shared::<SimulationInertialCompletion>();
    shared::<SimulationInertialDiagnostic>();
}

#[test]
fn collections_own_inputs_fields_and_returned_byte_buffers() {
    let original = owned_collection(KINDS[0], 840002);
    let expected = original.to_bytes();
    let mut input = expected.clone();
    let collection = SimulationInertialCollection::from_bytes(
        &input,
        original.version(),
        original.kind(),
        LIMITS,
    )
    .unwrap();
    input.fill(0);
    let mut returned = collection.to_bytes();
    returned.fill(0);
    let mut fields = collection.fields().to_vec();
    fields.clear();
    assert_eq!(collection.to_bytes(), expected);
    assert_eq!(collection.fields(), original.fields());
}

fn assert_coverage(collection: &SimulationInertialCollection, raw: &[u8]) {
    let coverage = collection.coverage();
    assert_eq!(coverage.checked_bytes, raw.len());
    assert_eq!(coverage.checked_fields, collection.fields().len());
    assert_eq!(coverage.record_count, collection.record_count());
    assert_eq!(coverage.kind, collection.kind());
    assert_eq!(coverage.version, collection.version());
    assert_eq!(
        coverage.check_mode,
        SimulationInertialCheckMode::ExactSelectedProfiles
    );
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.diagnostics.len(), 4);
}

fn assert_excitation_gate(
    collection: &SimulationInertialCollection,
    version: i32,
    expected_texts: usize,
) {
    let unsigned = collection
        .fields()
        .iter()
        .find(|field| field.path.as_ref() == "record0.excitation.unsigned_header")
        .unwrap();
    assert_eq!(unsigned.value, SimulationInertialValue::Unsigned(u32::MAX));
    let texts = collection
        .fields()
        .iter()
        .filter(|field| field.path.starts_with("record0.excitation.text["))
        .count();
    assert_eq!(texts, expected_texts);
    assert_eq!(
        collection
            .fields()
            .iter()
            .any(|field| field.path.as_ref() == "record0.excitation.mixed_double"),
        version >= 820012
    );
}

#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "simulation_inertial_collections/projection.rs"]
mod projection;

#[path = "simulation_inertial_collections/edits.rs"]
mod edits;

#[path = "simulation_inertial_collections/edit_oracles.rs"]
mod edit_oracles;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;

#[path = "support/binary_load_strings.rs"]
mod load_strings;

#[path = "simulation_inertial_collections/malformed.rs"]
mod malformed;

#[path = "support/binary_load_history.rs"]
mod load_history;

#[path = "simulation_inertial_collections/history_projection.rs"]
mod history_projection;
#[path = "simulation_inertial_collections/nested_history.rs"]
mod nested_history;

#[path = "support/binary_load_limits.rs"]
mod load_limits;

#[path = "support/binary_load_mutations.rs"]
mod load_mutations;
