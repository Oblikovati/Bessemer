//! Complete beam child Property reads, construction and isolated edits (#142).
use bessemer::model_archives::ModelArchiveLimits;
use bessemer::simulation::*;

#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word as word, take_projection as take};
#[path = "simulation_beam_property/concurrency.rs"]
mod concurrency;
#[path = "simulation_beam_property/edits.rs"]
mod edits;
#[path = "support/binary_load_limits.rs"]
mod load_limits;
#[path = "support/binary_load_mutations.rs"]
mod load_mutations;
#[path = "support/binary_load_strings.rs"]
mod load_strings;
#[path = "simulation_beam_property/malformed.rs"]
mod malformed;
#[path = "simulation_beam_property/nested.rs"]
mod nested;
#[path = "simulation_beam_property/projection.rs"]
mod projection;

const LIMITS: SimulationBeamLimits = SimulationBeamLimits {
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
const OWNED: &[u8] = include_bytes!("fixtures/simulation-beam-property/owned.bin");

struct BeamCase<'a> {
    version: i32,
    raw: &'a [u8],
    spans: Vec<(usize, usize)>,
}

fn case<'a>(cursor: &mut &'a [u8], native: bool) -> BeamCase<'a> {
    let first = word(cursor);
    let second = word(cursor);
    let version = if native {
        assert_eq!(second, 1);
        first
    } else {
        second
    };
    let count = word(cursor) as usize;
    let raw = take(cursor, count);
    let spans = (0..word(cursor))
        .map(|_| (word(cursor) as usize, word(cursor) as usize))
        .collect();
    BeamCase {
        version: version as i32,
        raw,
        spans,
    }
}

fn read(case: &BeamCase<'_>) -> SimulationBeamProperty {
    SimulationBeamProperty::from_bytes(
        case.raw,
        SimulationPersistVersion::new(case.version).unwrap(),
        LIMITS,
    )
    .unwrap()
}

fn check_case(case: &BeamCase<'_>) {
    let property = read(case);
    assert_eq!(property.to_bytes(), case.raw);
    let constructed =
        SimulationBeamProperty::from_fields(property.fields().to_vec(), property.version(), LIMITS)
            .unwrap();
    assert_eq!(constructed.to_bytes(), case.raw);
    assert_eq!(constructed.fields(), property.fields());
    assert_eq!(constructed.field_offsets(), property.field_offsets());
    assert_ne!(
        constructed.field_id(0).unwrap(),
        property.field_id(0).unwrap()
    );
    check_spans(case, &property);
}

fn check_spans(case: &BeamCase<'_>, property: &SimulationBeamProperty) {
    let mut end = 0;
    for &(start, next) in &case.spans {
        assert_eq!(start, end);
        assert!(property.field_offsets().iter().any(|span| span.0 == start));
        assert!(property.field_offsets().iter().any(|span| span.1 == next));
        end = next;
    }
    assert_eq!(end, case.raw.len());
    assert_eq!(property.coverage().checked_bytes, case.raw.len());
    assert_eq!(
        property.coverage().completion,
        SimulationBeamCompletion::CompletedBody
    );
}

fn check_packet(mut packet: &[u8], native: bool) -> usize {
    let mut count = 0;
    while !packet.is_empty() {
        check_case(&case(&mut packet, native));
        count += 1;
    }
    count
}

#[test]
fn every_owned_profile_has_complete_source_spans_and_forced_construction() {
    assert_eq!(check_packet(OWNED, false), 104);
}

#[test]
#[ignore = "requires explicit complete freshly extracted native beam packet"]
fn every_native_property_matches_all_original_complete_outcomes() {
    let path = std::env::var_os("BESSEMER_BEAM_NATIVE_INPUT")
        .expect("set BESSEMER_BEAM_NATIVE_INPUT to the fresh full native packet");
    let packet = std::fs::read(path).unwrap();
    assert_eq!(check_packet(&packet, true), 163);
}

fn ordinal(property: &SimulationBeamProperty, path: &str) -> usize {
    property
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap()
}

#[test]
fn short_edits_preserve_width_extremes_history_and_input_isolation() {
    let source = read(&case(&mut &OWNED[..], false));
    let index = ordinal(&source, "record0.beam.shorte2");
    let id = source.field_id(index).unwrap();
    assert_eq!(
        source.field(&id).unwrap().value,
        SimulationBeamValue::SignedShort(-32768)
    );
    let edit = source
        .edit_field(&id, SimulationBeamValue::SignedShort(32767))
        .unwrap();
    check_short_edit(&source, &edit, index, id);
}

fn check_short_edit(
    source: &SimulationBeamProperty,
    edit: &SimulationBeamEdit,
    index: usize,
    id: SimulationBeamFieldId,
) {
    let (start, end) = source.field_offsets()[index];
    assert_eq!(end - start, 2);
    assert_eq!(
        &edit.archive().to_bytes()[start..end],
        &32767i16.to_le_bytes()
    );
    assert_eq!(edit.source().to_bytes(), source.to_bytes());
    assert_eq!(
        &edit.archive().to_bytes()[..start],
        &source.to_bytes()[..start]
    );
    assert_eq!(&edit.archive().to_bytes()[end..], &source.to_bytes()[end..]);
    assert_eq!(edit.history()[index].parents.len(), 2);
    assert_eq!(
        edit.coverage().completion,
        SimulationBeamCompletion::Changed
    );
    assert_eq!(edit.replacement(), &SimulationBeamValue::SignedShort(32767));
    assert_eq!(edit.result(), edit.archive().field_id(index).unwrap());
    assert_eq!(edit.parent(), id);
}

#[test]
fn every_exact_noop_preserves_ids_and_all_source_parents() {
    let mut packet = OWNED;
    while !packet.is_empty() {
        let property = read(&case(&mut packet, false));
        for (index, field) in property.fields().iter().enumerate() {
            let id = property.field_id(index).unwrap();
            let edit = property.edit_field(&id, field.value.clone()).unwrap();
            assert_eq!(edit.result(), id);
            assert_eq!(edit.archive().to_bytes(), property.to_bytes());
            assert_eq!(
                edit.coverage().completion,
                SimulationBeamCompletion::Unchanged
            );
            assert_eq!(edit.history().len(), property.fields().len());
            assert!(edit.history().iter().all(|row| row.parents.len() == 1));
        }
    }
}

#[test]
fn typed_short_boundary_rejects_shared_bytes_words_and_foreign_identities() {
    let property = read(&case(&mut &OWNED[..], false));
    let id = property
        .field_id(ordinal(&property, "record0.beam.shorte2"))
        .unwrap();
    for value in [
        SimulationThermalValue::Bytes(vec![0, 0].into()),
        SimulationThermalValue::Signed(0),
    ] {
        assert!(
            property
                .edit_field(&id, SimulationBeamValue::Shared(value))
                .is_err()
        );
    }
    let foreign = read(&case(&mut &OWNED[..], false)).field_id(0).unwrap();
    assert!(property.field(&foreign).is_err());
    assert!(property.field_id(property.fields().len()).is_err());
    check_wrong_short_construction(&property, &id);
}

fn check_wrong_short_construction(property: &SimulationBeamProperty, id: &SimulationBeamFieldId) {
    let mut fields = property.fields().to_vec();
    fields[id.ordinal()].value =
        SimulationBeamValue::Shared(SimulationThermalValue::Bytes(vec![0, 0].into()));
    assert!(SimulationBeamProperty::from_fields(fields, property.version(), LIMITS).is_err());
}

#[test]
fn every_owned_proper_prefix_and_trailing_byte_rejects() {
    let mut packet = OWNED;
    while !packet.is_empty() {
        let case = case(&mut packet, false);
        let version = SimulationPersistVersion::new(case.version).unwrap();
        for end in 0..case.raw.len() {
            assert!(
                SimulationBeamProperty::from_bytes(&case.raw[..end], version, LIMITS).is_err(),
                "{} prefix {end}",
                case.version
            );
        }
        let mut trailing = case.raw.to_vec();
        trailing.push(0);
        assert!(SimulationBeamProperty::from_bytes(&trailing, version, LIMITS).is_err());
    }
}

#[test]
fn published_beam_types_are_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationBeamProperty>();
    check::<SimulationBeamField>();
    check::<SimulationBeamValue>();
    check::<SimulationBeamLimits>();
    check::<SimulationBeamEdit>();
    check::<SimulationBeamCoverage>();
    check::<SimulationBeamCompletion>();
    check::<SimulationBeamCheckMode>();
    check::<SimulationBeamDiagnostic>();
    check::<SimulationBeamFieldId>();
    check::<SimulationBeamHistory>();
    check::<SimulationBeamHistoryParent>();
    check::<SimulationBeamNestedHistory>();
    check::<SimulationBeamNestedParent>();
    check::<SimulationBeamRelation>();
}
