use super::*;

const MALFORMED: &[u8] = include_bytes!("../fixtures/simulation-shell-property/malformed.bin");
const HARDCAPS: &[u8] = include_bytes!("../fixtures/simulation-shell-property/hardcaps.bin");

fn accepts_complete_mutant(raw: &[u8], source: &SimulationShellProperty) -> bool {
    let parsed = SimulationShellProperty::from_bytes(
        raw,
        source.version(),
        source.context().clone(),
        LIMITS,
    );
    let Ok(body) = parsed else { return false };
    assert_eq!(body.to_bytes(), raw);
    let constructed = SimulationShellProperty::from_fields(
        body.fields().to_vec(),
        body.version(),
        body.context().clone(),
        LIMITS,
    )
    .unwrap();
    assert_eq!(constructed.to_bytes(), raw);
    assert_eq!(constructed.fields(), body.fields());
    assert_eq!(constructed.context(), body.context());
    true
}

#[test]
fn all_targeted_mutations_match_actual_reference_acceptance_and_complete_construction() {
    let mut cursor = MALFORMED;
    let mut totals = (0, 0, 0);
    while !cursor.is_empty() {
        let body = read_original(&mut cursor);
        let original = body.to_bytes();
        let (accepted, rejected) = load_limits::check_mutations(&mut cursor, &original, |raw| {
            accepts_complete_mutant(raw, &body)
        });
        assert_eq!(body.to_bytes(), original);
        totals.0 += 1;
        totals.1 += accepted;
        totals.2 += rejected;
    }
    assert_eq!(totals, (2284, 45548, 231802));
}

fn read_cap_case(cursor: &mut &[u8]) -> bool {
    let kind = KINDS[read_word(cursor) as usize];
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let expected = read_word(cursor) != 0;
    let fields = read_word(cursor) as usize;
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let context = shell_context(kind, SimulationShellLayerPresence::Unknown);
    let parsed = SimulationShellProperty::from_bytes(raw, version, context, LIMITS);
    assert_eq!(parsed.is_ok(), expected, "{kind:?} {version:?} {size}");
    if let Ok(body) = parsed {
        assert_eq!(body.to_bytes(), raw);
        assert_eq!(body.fields().len(), fields);
        assert_eq!(body.coverage().checked_fields, fields);
    }
    expected
}

#[test]
fn all_six_fixed_caps_use_sufficient_payload_and_permissive_caller_budgets_for_both_kinds() {
    let mut cursor = HARDCAPS;
    let mut counts = (0, 0);
    while !cursor.is_empty() {
        let accepted = read_cap_case(&mut cursor);
        counts.0 += usize::from(accepted);
        counts.1 += usize::from(!accepted);
    }
    assert_eq!(counts, (12, 12));
}

fn assert_prefixes(body: &SimulationShellProperty) -> usize {
    let raw = body.to_bytes();
    for end in 0..raw.len() {
        let parsed = SimulationShellProperty::from_bytes(
            &raw[..end],
            body.version(),
            body.context().clone(),
            LIMITS,
        );
        assert!(
            parsed.is_err(),
            "{:?} {:?} prefix {end}/{}; expected incomplete body",
            body.context(),
            body.version(),
            raw.len()
        );
    }
    assert_eq!(body.to_bytes(), raw);
    raw.len()
}

#[test]
fn every_owned_context_rejects_every_proper_prefix_without_mutating_the_complete_source() {
    let mut cursor = EXPECTED;
    let mut totals = (0, 0);
    while !cursor.is_empty() {
        let body = read_original(&mut cursor);
        consume_expected_fields(&mut cursor);
        totals.0 += 1;
        totals.1 += assert_prefixes(&body);
    }
    assert_eq!(totals, (2284, 2954046));
}
