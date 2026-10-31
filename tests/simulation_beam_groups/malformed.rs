use super::load_limits::{check_mutations, read_reference_limits, reduced_limits};
use super::*;

const MALFORMED: &[u8] = include_bytes!("../fixtures/simulation-beam-groups/malformed.bin");
const BUDGETS: &[u8] = include_bytes!("../fixtures/simulation-beam-groups/budgets.bin");

fn mutation_original(cursor: &mut &[u8]) -> SimulationBeamGroupBody {
    let expected = read_word(cursor) as i32;
    let index = read_word(cursor);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let kind = kind(index, expected);
    let size = read_word(cursor) as usize;
    SimulationBeamGroupBody::from_bytes(take_projection(cursor, size), version, kind, LIMITS)
        .unwrap()
}

#[test]
fn every_targeted_mutation_matches_complete_actual_reference_outcomes() {
    let mut cursor = MALFORMED;
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        let body = mutation_original(&mut cursor);
        let (yes, no) = check_mutations(&mut cursor, &body.to_bytes(), |image| {
            SimulationBeamGroupBody::from_bytes(image, body.version(), body.kind(), LIMITS).is_ok()
        });
        accepted += yes;
        rejected += no;
    }
    assert_eq!((accepted, rejected), (0, 3104));
}

fn assert_budget(body: &SimulationBeamGroupBody, exact: SimulationBeamGroupLimits) {
    let raw = body.to_bytes();
    let accepted = SimulationBeamGroupBody::from_bytes(&raw, body.version(), body.kind(), exact)
        .unwrap_or_else(|error| panic!("{error:?}; expected exact independent limits {exact:?}"));
    assert_eq!(accepted.to_bytes(), raw);
    for limits in reduced_limits(exact) {
        assert!(
            SimulationBeamGroupBody::from_bytes(&raw, body.version(), body.kind(), limits).is_err(),
            "expected one-below rejection: {limits:?}"
        );
        assert_eq!(body.to_bytes(), raw);
    }
}

#[test]
fn every_owned_context_obeys_independent_exact_and_one_below_outer_and_embedded_limits() {
    let mut cursor = BUDGETS;
    let mut count = 0;
    while !cursor.is_empty() {
        let body = read_body(&mut cursor);
        let mut limits = read_reference_limits(&mut cursor, LIMITS);
        limits.maximum_bytes = body.to_bytes().len();
        assert_budget(&body, limits);
        count += 1;
    }
    assert_eq!(count, 560);
}

fn assert_prefixes(body: &SimulationBeamGroupBody) {
    let raw = body.to_bytes();
    for end in 0..raw.len() {
        assert!(
            SimulationBeamGroupBody::from_bytes(&raw[..end], body.version(), body.kind(), LIMITS)
                .is_err(),
            "{:?} {:?} prefix {end}/{}",
            body.kind(),
            body.version(),
            raw.len()
        );
    }
    let mut appended = raw;
    appended.push(0);
    assert!(
        SimulationBeamGroupBody::from_bytes(&appended, body.version(), body.kind(), LIMITS)
            .is_err()
    );
}

#[test]
fn every_owned_body_rejects_every_proper_prefix_and_trailing_byte() {
    let mut cursor = BUDGETS;
    let mut count = 0;
    while !cursor.is_empty() {
        let body = read_body(&mut cursor);
        read_reference_limits(&mut cursor, LIMITS);
        assert_prefixes(&body);
        count += 1;
    }
    assert_eq!(count, 560);
}
