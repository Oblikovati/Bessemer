use super::load_limits::{check_mutations, read_reference_limits, reduced_limits};
use super::*;

const MALFORMED: &[u8] =
    include_bytes!("../fixtures/simulation-component-persist-id/malformed.bin");
const BUDGETS: &[u8] = include_bytes!("../fixtures/simulation-component-persist-id/budgets.bin");

fn read_original(cursor: &mut &[u8]) -> SimulationComponentPersistId {
    assert_eq!(
        read_word(cursor),
        0,
        "expected singleton solid-body profile"
    );
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    SimulationComponentPersistId::from_bytes(raw, version, LIMITS).unwrap()
}

#[test]
fn all_targeted_mutations_match_complete_actual_reference_outcomes() {
    let mut cursor = MALFORMED;
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        let original = read_original(&mut cursor);
        let (yes, no) = check_mutations(&mut cursor, &original.to_bytes(), |image| {
            SimulationComponentPersistId::from_bytes(image, original.version(), LIMITS).is_ok()
        });
        accepted += yes;
        rejected += no;
    }
    assert_eq!((accepted, rejected), (20, 231));
}

fn assert_budget_boundary(
    property: &SimulationComponentPersistId,
    exact: SimulationComponentPersistIdLimits,
) {
    let original = property.to_bytes();
    let accepted = SimulationComponentPersistId::from_bytes(&original, property.version(), exact)
        .unwrap_or_else(|error| panic!("{error:?}; expected exact independent limits {exact:?}"));
    assert_eq!(accepted.to_bytes(), original);
    for limits in reduced_limits(exact) {
        assert!(
            SimulationComponentPersistId::from_bytes(&original, property.version(), limits)
                .is_err(),
            "limits {limits:?}; expected one-below budget rejection"
        );
        assert_eq!(property.to_bytes(), original);
    }
}

#[test]
fn all_owned_contexts_obey_independent_exact_and_one_below_outer_and_embedded_limits() {
    let mut cursor = BUDGETS;
    let mut count = 0;
    while !cursor.is_empty() {
        let original = read_original(&mut cursor);
        let mut limits = read_reference_limits(&mut cursor, LIMITS);
        limits.maximum_bytes = original.to_bytes().len();
        assert_budget_boundary(&original, limits);
        count += 1;
    }
    assert_eq!(count, 20);
}

fn assert_prefixes(raw: &[u8], version: SimulationPersistVersion) {
    for end in 0..raw.len() {
        assert!(
            SimulationComponentPersistId::from_bytes(&raw[..end], version, LIMITS).is_err(),
            "version {version:?}, prefix {end}/{}; expected incomplete body",
            raw.len()
        );
    }
    let mut appended = raw.to_vec();
    appended.push(0);
    assert!(SimulationComponentPersistId::from_bytes(&appended, version, LIMITS).is_err());
}

#[test]
fn every_owned_body_rejects_every_proper_prefix_and_appended_byte() {
    let mut cursor = EXPECTED;
    let mut count = 0;
    while !cursor.is_empty() {
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let size = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, size);
        assert_prefixes(raw, version);
        consume_expected_fields(&mut cursor);
        count += 1;
    }
    assert_eq!(count, 20);
}

#[test]
fn persist_blob_fixed_cap_is_checked_with_sufficient_payload_and_caller_budgets() {
    let body = owned_property(0, 0).to_bytes();
    let version = SimulationPersistVersion::new(0).unwrap();
    for count in [1000000_i32, 1000001] {
        let mut raw = body[..8].to_vec();
        raw.extend_from_slice(&count.to_le_bytes());
        raw.resize(raw.len() + count as usize, 0xaa);
        raw.extend_from_slice(&body[12..]);
        let result = SimulationComponentPersistId::from_bytes(&raw, version, LIMITS);
        assert_eq!(result.is_ok(), count == 1000000);
        if let Ok(decoded) = result {
            assert_eq!(decoded.to_bytes(), raw);
        }
    }
}
