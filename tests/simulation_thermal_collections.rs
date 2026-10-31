//! Complete binary thermal collections with independent retained outer candidate contexts (#134).
#[path = "simulation_thermal_collections/edit_oracles.rs"]
mod edit_oracles;
#[path = "simulation_thermal_collections/edits.rs"]
mod edits;
#[path = "simulation_thermal_collections/embedded_maps.rs"]
mod embedded_maps;
#[path = "simulation_thermal_collections/history_projection.rs"]
mod history_projection;
#[path = "simulation_thermal_collections/malformed.rs"]
mod malformed;
#[path = "simulation_thermal_collections/nested_history.rs"]
mod nested_history;
#[path = "simulation_thermal_collections/projection.rs"]
mod projection;
use bessemer::model_archives::ModelArchiveLimits;
use bessemer::simulation::{
    SimulationPersistVersion, SimulationThermalCollection, SimulationThermalKind,
    SimulationThermalLimits,
};

const LIMITS: SimulationThermalLimits = SimulationThermalLimits {
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
const KINDS: [SimulationThermalKind; 5] = [
    SimulationThermalKind::Temperature,
    SimulationThermalKind::Convection,
    SimulationThermalKind::HeatFlux,
    SimulationThermalKind::HeatPower,
    SimulationThermalKind::Radiation,
];
const OWNED: &[u8] = include_bytes!("fixtures/simulation-binary-thermal/thermal-owned.bin");

fn packet_word(cursor: &mut &[u8]) -> u32 {
    let (word, remaining) = cursor.split_at(4);
    *cursor = remaining;
    u32::from_le_bytes(word.try_into().unwrap())
}

fn read_candidate<'a>(
    cursor: &mut &'a [u8],
) -> (
    SimulationThermalKind,
    SimulationPersistVersion,
    bool,
    &'a [u8],
) {
    let kind = KINDS[packet_word(cursor) as usize];
    let version = SimulationPersistVersion::new(packet_word(cursor) as i32).unwrap();
    let accepted = packet_word(cursor) != 0;
    let size = packet_word(cursor) as usize;
    let (raw, remaining) = cursor.split_at(size);
    *cursor = remaining;
    (kind, version, accepted, raw)
}

fn check_candidate(cursor: &mut &[u8]) -> bool {
    let (kind, version, accepted, raw) = read_candidate(cursor);
    let result = SimulationThermalCollection::from_bytes(raw, version, kind, LIMITS);
    if !accepted {
        assert!(result.is_err(), "expected rejected {kind:?} {version:?}");
        return false;
    }
    let collection = result.unwrap_or_else(|error| panic!("{kind:?} {version:?}: {error:?}"));
    assert_complete_collection(&collection, raw);
    true
}

fn assert_complete_collection(collection: &SimulationThermalCollection, raw: &[u8]) {
    let (version, kind) = (collection.version(), collection.kind());
    assert_eq!(collection.to_bytes(), raw);
    let constructed = SimulationThermalCollection::from_fields(
        collection.fields().to_vec(),
        version,
        kind,
        LIMITS,
    )
    .unwrap();
    assert_eq!(constructed.to_bytes(), raw);
    assert_eq!(constructed.fields(), collection.fields());
    assert_eq!(constructed.field_offsets(), collection.field_offsets());
    assert_eq!(constructed.version(), version);
    assert_eq!(constructed.kind(), kind);
}

fn assert_source_spans(collection: &SimulationThermalCollection, cursor: &mut &[u8]) {
    let mut fields = collection.field_offsets().iter();
    for _ in 0..packet_word(cursor) {
        let start = packet_word(cursor) as usize;
        let end = packet_word(cursor) as usize;
        let (first, mut consumed) = *fields.next().unwrap();
        assert_eq!(first, start);
        while consumed < end {
            let (next, last) = *fields.next().unwrap();
            assert_eq!(next, consumed);
            consumed = last;
        }
        assert_eq!(consumed, end);
    }
    assert!(fields.next().is_none());
}

#[test]
fn populated_owned_thermal_collections_match_every_actual_reference_span() {
    let mut cursor = OWNED;
    let mut count = 0;
    while !cursor.is_empty() {
        let (kind, version, accepted, raw) = read_candidate(&mut cursor);
        assert!(accepted);
        let collection =
            SimulationThermalCollection::from_bytes(raw, version, kind, LIMITS).unwrap();
        assert_complete_collection(&collection, raw);
        assert_source_spans(&collection, &mut cursor);
        assert_eq!(collection.record_count(), 2);
        count += 1;
    }
    assert_eq!(count, 295);
}

#[test]
#[ignore = "Requires explicitly extracted complete thermal native candidates"]
fn all_native_thermal_candidates_preserve_complete_acceptance_and_bytes() {
    let path = std::env::var("BESSEMER_THERMAL_NATIVE_INPUT")
        .expect("BESSEMER_THERMAL_NATIVE_INPUT=<unset>; expected an explicit retained candidate packet path");
    let packet = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable candidate packet")
    });
    let mut cursor = packet.as_slice();
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        if check_candidate(&mut cursor) {
            accepted += 1;
        } else {
            rejected += 1;
        }
    }
    assert_eq!((accepted, rejected), (68, 7));
}

#[test]
fn empty_thermal_collections_are_completed_empty_results_in_each_family() {
    for kind in KINDS {
        let version = SimulationPersistVersion::new(890501).unwrap();
        let collection =
            SimulationThermalCollection::from_bytes(&[0; 4], version, kind, LIMITS).unwrap();
        assert_eq!(collection.record_count(), 0);
        assert_eq!(collection.fields().len(), 1);
        assert_eq!(collection.field_offsets(), &[(0, 4)]);
        assert_eq!(collection.to_bytes(), [0; 4]);
    }
}

#[test]
fn thermal_published_types_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<SimulationThermalCollection>();
    shared::<SimulationThermalKind>();
    shared::<SimulationThermalLimits>();
    shared::<bessemer::simulation::SimulationThermalField>();
    shared::<bessemer::simulation::SimulationThermalValue>();
}

#[test]
fn thermal_constructors_reject_fabricated_paths_and_value_shapes() {
    let version = SimulationPersistVersion::new(890501).unwrap();
    let collection =
        SimulationThermalCollection::from_bytes(&[0; 4], version, KINDS[0], LIMITS).unwrap();
    let mut fields = collection.fields().to_vec();
    fields[0].path = "fabricated.count".into();
    let error =
        SimulationThermalCollection::from_fields(fields, version, KINDS[0], LIMITS).unwrap_err();
    assert!(error.cause.contains("fabricated.count") && error.cause.contains("collection.count"));
    let mut fields = collection.fields().to_vec();
    fields[0].value = bessemer::simulation::SimulationThermalValue::Unsigned(0);
    let error =
        SimulationThermalCollection::from_fields(fields, version, KINDS[0], LIMITS).unwrap_err();
    assert!(error.cause.contains("Unsigned(0)") && error.cause.contains("Signed(0)"));
    assert_eq!(collection.to_bytes(), [0; 4]);
}

fn first_modern_owned() -> (
    SimulationThermalKind,
    SimulationPersistVersion,
    &'static [u8],
) {
    let mut cursor = OWNED;
    while !cursor.is_empty() {
        let (kind, version, _, raw) = read_candidate(&mut cursor);
        let spans = packet_word(&mut cursor) as usize;
        cursor = &cursor[spans * 8..];
        if version.value() >= 840002 {
            return (kind, version, raw);
        }
    }
    panic!("owned thermal corpus; expected modern populated reference archives")
}

#[test]
fn embedded_prefixes_obey_inner_byte_budgets_without_consuming_outer_suffixes() {
    let (kind, version, raw) = first_modern_owned();
    let source = SimulationThermalCollection::from_bytes(raw, version, kind, LIMITS).unwrap();
    let maximum = maximum_embedded_bytes(&source);
    assert!(maximum < raw.len());
    let mut limits = LIMITS;
    limits.embedded.maximum_bytes = maximum;
    let exact = SimulationThermalCollection::from_bytes(raw, version, kind, limits).unwrap();
    assert_eq!(exact.to_bytes(), raw);
    limits.embedded.maximum_bytes = maximum - 1;
    assert!(SimulationThermalCollection::from_bytes(raw, version, kind, limits).is_err());
    assert_eq!(source.to_bytes(), raw);
}

fn maximum_embedded_bytes(source: &SimulationThermalCollection) -> usize {
    source
        .fields()
        .iter()
        .filter_map(|field| match &field.value {
            bessemer::simulation::SimulationThermalValue::Embedded(archive) => {
                Some(archive.coverage().wire_bytes)
            }
            _ => None,
        })
        .max()
        .unwrap()
}

#[test]
fn thermal_collections_own_input_texts_and_returned_byte_buffers() {
    let (kind, version, raw) = first_modern_owned();
    let mut input = raw.to_vec();
    let source = SimulationThermalCollection::from_bytes(&input, version, kind, LIMITS).unwrap();
    input.fill(0xa5);
    let mut returned = source.to_bytes();
    returned.fill(0x5a);
    assert_eq!(source.to_bytes(), raw);
    assert_complete_collection(&source, raw);
}

#[path = "support/binary_load_projection.rs"]
mod load_projection;

#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;

#[path = "support/binary_load_strings.rs"]
mod load_strings;

#[path = "support/binary_load_history.rs"]
mod load_history;
