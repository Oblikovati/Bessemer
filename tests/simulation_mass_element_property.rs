//! Complete single-body MassElement Property operations against independent source projections (#138).
use bessemer::model_archives::ModelArchiveLimits;
use bessemer::simulation::*;

#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{
    consume_expected_fields, project_field, project_word, read_word, take_projection,
};

const EXPECTED: &[u8] = include_bytes!("fixtures/simulation-mass-element-property/projection.bin");
const LIMITS: SimulationMassElementLimits = SimulationMassElementLimits {
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

fn assert_constructed(property: &SimulationMassElementProperty, raw: &[u8]) {
    let constructed = SimulationMassElementProperty::from_fields(
        property.fields().to_vec(),
        property.version(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(constructed.to_bytes(), raw);
    assert_eq!(constructed.fields(), property.fields());
    assert_eq!(constructed.field_offsets(), property.field_offsets());
    assert_eq!(constructed.version(), property.version());
    assert_ne!(
        constructed.field_id(0).unwrap(),
        property.field_id(0).unwrap()
    );
}

fn project_case(cursor: &mut &[u8], output: &mut Vec<u8>) {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let property = SimulationMassElementProperty::from_bytes(raw, version, LIMITS).unwrap();
    assert_constructed(&property, raw);
    assert_coverage(&property, raw);
    project_word(output, version.value() as usize);
    project_word(output, size);
    output.extend_from_slice(&property.to_bytes());
    project_word(output, property.fields().len());
    for (field, span) in property.fields().iter().zip(property.field_offsets()) {
        project_field(output, field, *span);
    }
    consume_expected_fields(cursor);
}

fn assert_coverage(property: &SimulationMassElementProperty, raw: &[u8]) {
    let coverage = property.coverage();
    assert_eq!(coverage.body_count, 1);
    assert_eq!(
        coverage.completion,
        SimulationMassElementCompletion::CompletedBody
    );
    assert_eq!(
        coverage.check_mode,
        SimulationMassElementCheckMode::ExactSelectedProfiles
    );
    assert_eq!(coverage.checked_bytes, raw.len());
    assert_eq!(coverage.checked_fields, property.fields().len());
    assert_eq!(coverage.version, property.version());
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.diagnostics.len(), 4);
    assert_single_body_shape(property);
}

#[test]
fn every_owned_body_matches_complete_independent_fields_and_constructs_exactly() {
    assert_eq!(
        load_projection::project_packet(EXPECTED, 196, project_case),
        EXPECTED
    );
}

#[test]
#[ignore = "Requires explicitly extracted complete native mass-element Property projections"]
fn every_native_body_matches_complete_independent_fields_with_optional_export() {
    let path = std::env::var("BESSEMER_MASS_ELEMENT_NATIVE_PROJECTION_INPUT")
        .expect("BESSEMER_MASS_ELEMENT_NATIVE_PROJECTION_INPUT=<unset>; expected explicit native field packet path");
    let packet = std::fs::read(&path).unwrap_or_else(|error| {
        panic!("input {path:?}: {error}; expected readable complete native field projections")
    });
    let mut output = load_projection::project_packet(EXPECTED, 196, project_case);
    output.extend(load_projection::project_packet(&packet, 2, project_case));
    if let Some(path) = std::env::var_os("BESSEMER_MASS_ELEMENT_PROJECTION_OUTPUT") {
        std::fs::write(&path, output).unwrap_or_else(|error| {
            panic!("output {path:?}: {error}; expected writable explicit full field export")
        });
    }
}

fn selector_value(property: &SimulationMassElementProperty) -> Option<i32> {
    property.fields().iter().find_map(|field| {
        if field.path.as_ref() != "record0.mass.selector" {
            return None;
        }
        let SimulationMassElementValue::Signed(value) = field.value else {
            panic!("{:?}; expected signed mass selector", field.value);
        };
        Some(value)
    })
}

fn owned_property(version: i32, selector: Option<i32>) -> SimulationMassElementProperty {
    let mut cursor = EXPECTED;
    while !cursor.is_empty() {
        let context = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let size = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, size);
        consume_expected_fields(&mut cursor);
        if context.value() != version {
            continue;
        }
        let property = SimulationMassElementProperty::from_bytes(raw, context, LIMITS).unwrap();
        if selector_value(&property) == selector {
            return property;
        }
    }
    panic!("version {version}, selector {selector:?}; expected an owned complete mass Property");
}

#[test]
fn mandatory_body_rejects_empty_and_collection_prefixed_inputs() {
    let version = SimulationPersistVersion::new(890204).unwrap();
    assert!(SimulationMassElementProperty::from_bytes(&[], version, LIMITS).is_err());
    let property = owned_property(890204, Some(1));
    let mut counted = 1_i32.to_le_bytes().to_vec();
    counted.extend(property.to_bytes());
    assert!(SimulationMassElementProperty::from_bytes(&counted, version, LIMITS).is_err());
}

#[test]
fn selector_equality_retains_duplicate_signed_pairs_only_from_the_exact_gate() {
    assert_eq!(selector_value(&owned_property(890203, None)), None);
    for selector in [-1, 0, 1, 2] {
        assert_selector_pairs(selector);
    }
}

#[test]
fn runtime_only_threshold_adds_no_persistence_fields() {
    let before = owned_property(891106, Some(1));
    for version in [891107, 891108] {
        let after = owned_property(version, Some(1));
        assert_eq!(before.to_bytes(), after.to_bytes());
        assert_eq!(before.fields(), after.fields());
        assert_eq!(before.field_offsets(), after.field_offsets());
    }
}

#[path = "simulation_mass_element_property/concurrency.rs"]
mod concurrency;
#[path = "simulation_mass_element_property/edits.rs"]
mod edits;
#[path = "simulation_mass_element_property/history.rs"]
mod history;
#[path = "support/binary_load_edit_case.rs"]
mod load_edit_case;
#[path = "support/binary_load_history.rs"]
mod load_history;
#[path = "support/binary_load_limits.rs"]
mod load_limits;
#[path = "support/binary_load_strings.rs"]
mod load_strings;
#[path = "simulation_mass_element_property/malformed.rs"]
mod malformed;

fn assert_single_body_shape(property: &SimulationMassElementProperty) {
    assert_eq!(
        property.fields()[0].path.as_ref(),
        "record0.references.count"
    );
    assert!(
        !property
            .fields()
            .iter()
            .any(|field| field.path.as_ref() == "collection.count")
    );
}

fn assert_selector_pairs(selector: i32) {
    let property = owned_property(890204, Some(selector));
    let pairs: Vec<_> = property
        .fields()
        .iter()
        .filter(|field| field.path.starts_with("record0.mass.pairs["))
        .map(|field| field.value.clone())
        .collect();
    let expected = if selector == 1 {
        vec![-1, 2, -1, 2, i32::MIN, i32::MAX]
    } else {
        vec![]
    };
    assert_eq!(
        pairs,
        expected
            .into_iter()
            .map(SimulationMassElementValue::Signed)
            .collect::<Vec<_>>()
    );
}

#[path = "support/binary_load_mutations.rs"]
mod load_mutations;
