use super::*;

const BUDGETS: &[u8] = include_bytes!("../fixtures/simulation-shell-property/budgets.bin");

fn assert_statistics(body: &SimulationShellProperty, cursor: &mut &[u8]) {
    let coverage = body.coverage();
    assert_eq!(coverage.base_end, read_word(cursor) as usize);
    assert_eq!(coverage.subtype, read_word(cursor) as i32);
    assert_eq!(coverage.layers, read_word(cursor) as usize);
    assert_eq!(coverage.materials, read_word(cursor) as usize);
    assert_eq!(coverage.descriptors, read_word(cursor) as usize);
    assert_eq!(coverage.embedded_archives, read_word(cursor) as usize);
    assert_eq!(coverage.embedded_fields, read_word(cursor) as usize);
}

fn check_boundary(
    body: &SimulationShellProperty,
    limits: SimulationShellPropertyLimits,
    valid: bool,
) {
    let parsed = SimulationShellProperty::from_bytes(
        &body.to_bytes(),
        body.version(),
        body.context().clone(),
        limits,
    );
    let constructed = SimulationShellProperty::from_fields(
        body.fields().to_vec(),
        body.version(),
        body.context().clone(),
        limits,
    );
    assert_eq!(parsed.is_ok(), valid, "{limits:?}");
    assert_eq!(constructed.is_ok(), valid, "{limits:?}");
    if let Ok(parsed) = parsed {
        assert_eq!(parsed.to_bytes(), body.to_bytes());
    }
    if let Ok(constructed) = constructed {
        assert_eq!(constructed.to_bytes(), body.to_bytes());
    }
}

fn check_packet(mut cursor: &[u8]) -> usize {
    let mut count = 0;
    while !cursor.is_empty() {
        let body = read_original(&mut cursor);
        let mut exact = load_limits::read_reference_limits(&mut cursor, LIMITS);
        exact.maximum_bytes = body.to_bytes().len();
        assert_eq!(body.coverage().declared_entries, exact.maximum_entries);
        assert_statistics(&body, &mut cursor);
        check_boundary(&body, exact, true);
        for reduced in load_limits::reduced_limits(exact) {
            check_boundary(&body, reduced, false);
        }
        count += 1;
    }
    count
}

#[test]
fn all_null_controls_accept_genuinely_zero_text_entries_and_embedded_budgets() {
    let packet = include_bytes!("../fixtures/simulation-shell-property/null-budgets.bin");
    assert_eq!(check_packet(packet), 10);
}

#[test]
fn all_owned_contexts_obey_independent_seven_axis_budgets_and_complete_statistics() {
    assert_eq!(check_packet(BUDGETS), 2284);
}

#[test]
#[ignore = "requires complete independently projected native shell budgets and statistics"]
fn all_native_accepted_contexts_obey_independent_budgets_and_statistics() {
    let path = std::env::var_os("BESSEMER_SHELL_PROPERTY_NATIVE_BUDGET_INPUT")
        .expect("set native shell Property budgets");
    assert_eq!(check_packet(&std::fs::read(path).unwrap()), 251);
}

fn check_trailing_byte(body: &SimulationShellProperty) {
    let original = body.to_bytes();
    let mut mutation = Vec::new();
    for word in [1, 0, original.len(), 0, 1] {
        project_word(&mut mutation, word);
    }
    mutation.push(0);
    let mut cursor = mutation.as_slice();
    let counts = load_limits::check_mutations(&mut cursor, &original, |raw| {
        SimulationShellProperty::from_bytes(raw, body.version(), body.context().clone(), LIMITS)
            .is_ok()
    });
    assert_eq!(counts, (0, 1));
    assert!(cursor.is_empty());
    assert_eq!(body.to_bytes(), original);
}

#[test]
fn every_owned_complete_context_rejects_a_trailing_byte_without_mutation() {
    let mut cursor = EXPECTED;
    let mut count = 0;
    while !cursor.is_empty() {
        let body = read_original(&mut cursor);
        consume_expected_fields(&mut cursor);
        check_trailing_byte(&body);
        count += 1;
    }
    assert_eq!(count, 2284);
}
