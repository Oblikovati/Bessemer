use super::load_limits::{check_mutations, read_reference_limits, reduced_limits};
use super::*;

const MALFORMED: &[u8] = include_bytes!("../fixtures/simulation-beam-property/malformed.bin");
const BUDGETS: &[u8] = include_bytes!("../fixtures/simulation-beam-property/budgets.bin");

fn read_original(cursor: &mut &[u8]) -> SimulationBeamProperty {
    assert_eq!(word(cursor), 0);
    let version = word(cursor) as i32;
    let size = word(cursor) as usize;
    read(&BeamCase {
        version,
        raw: take(cursor, size),
        spans: Vec::new(),
    })
}

#[test]
fn every_targeted_mutation_matches_complete_actual_source_acceptance() {
    let mut cursor = MALFORMED;
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        let original = read_original(&mut cursor);
        let (yes, no) = check_mutations(&mut cursor, &original.to_bytes(), |image| {
            SimulationBeamProperty::from_bytes(image, original.version(), LIMITS).is_ok()
        });
        accepted += yes;
        rejected += no;
    }
    assert_eq!((accepted, rejected), (0, 1548));
}

fn assert_budget(property: &SimulationBeamProperty, exact: SimulationBeamLimits) {
    let bytes = property.to_bytes();
    let accepted = SimulationBeamProperty::from_bytes(&bytes, property.version(), exact)
        .unwrap_or_else(|error| panic!("{error:?}; expected independent exact limits {exact:?}"));
    assert_eq!(accepted.to_bytes(), bytes);
    for limits in reduced_limits(exact) {
        assert!(
            SimulationBeamProperty::from_bytes(&bytes, property.version(), limits).is_err(),
            "expected one-below rejection: {limits:?}"
        );
        assert_eq!(property.to_bytes(), bytes);
    }
}

#[test]
fn every_owned_context_obeys_independent_exact_and_one_below_outer_and_embedded_limits() {
    let mut cursor = BUDGETS;
    let mut count = 0;
    while !cursor.is_empty() {
        let property = read_original(&mut cursor);
        let mut limits = read_reference_limits(&mut cursor, LIMITS);
        limits.maximum_bytes = property.to_bytes().len();
        assert_budget(&property, limits);
        count += 1;
    }
    assert_eq!(count, 104);
}
