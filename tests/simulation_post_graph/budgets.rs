use super::*;

const BUDGETS: &[u8] = include_bytes!("../fixtures/simulation-post-graph/budgets.bin");
const CAPS: &[u8] = include_bytes!("../fixtures/simulation-post-graph/character-caps.bin");

fn exact_requirements(cursor: &mut &[u8], size: usize) -> SimulationPostGraphLimits {
    SimulationPostGraphLimits {
        maximum_bytes: size,
        maximum_fields: read_word(cursor) as usize,
        maximum_entries: read_word(cursor) as usize,
        maximum_text_bytes: read_word(cursor) as usize,
        embedded: LIMITS.embedded,
    }
}

fn require_budget(
    body: &SimulationPostGraphParameters,
    limits: SimulationPostGraphLimits,
    accepted: bool,
) {
    let read = SimulationPostGraphParameters::from_bytes(&body.to_bytes(), body.version(), limits);
    let built =
        SimulationPostGraphParameters::from_fields(body.fields().to_vec(), body.version(), limits);
    assert_eq!(read.is_ok(), accepted, "{limits:?}");
    assert_eq!(built.is_ok(), accepted, "{limits:?}");
    if let Ok(read) = read {
        assert_eq!(read.fields(), body.fields());
    }
    if let Ok(built) = built {
        assert_eq!(built.to_bytes(), body.to_bytes());
    }
}

fn one_below_axes(body: &SimulationPostGraphParameters, limits: SimulationPostGraphLimits) {
    let axes = [
        limits.maximum_bytes,
        limits.maximum_fields,
        limits.maximum_entries,
        limits.maximum_text_bytes,
    ];
    for (axis, required) in axes.into_iter().enumerate().filter(|(_, count)| *count > 0) {
        let mut reduced = limits;
        match axis {
            0 => reduced.maximum_bytes = required - 1,
            1 => reduced.maximum_fields = required - 1,
            2 => reduced.maximum_entries = required - 1,
            3 => reduced.maximum_text_bytes = required - 1,
            _ => panic!("axis {axis}; expected four outer resource axes"),
        }
        require_budget(body, reduced, false);
    }
}

fn verify_budget_packet(mut cursor: &[u8], expected: usize) {
    let mut count = 0;
    while !cursor.is_empty() {
        let body = read_body(&mut cursor);
        let limits = exact_requirements(&mut cursor, body.to_bytes().len());
        assert_eq!(body.coverage().checked_fields, limits.maximum_fields);
        assert_eq!(body.coverage().declared_entries, limits.maximum_entries);
        require_budget(&body, limits, true);
        one_below_axes(&body, limits);
        count += 1;
    }
    assert_eq!(count, expected);
}

#[test]
fn every_owned_context_obeys_independent_exact_and_one_below_budgets() {
    verify_budget_packet(BUDGETS, 270);
}

#[test]
#[ignore = "requires fresh complete independent native Results resource packet"]
fn every_native_context_obeys_independent_exact_and_one_below_budgets() {
    let path = std::env::var_os("BESSEMER_POST_GRAPH_NATIVE_BUDGET_INPUT")
        .expect("set full native Results budget packet");
    verify_budget_packet(&std::fs::read(path).unwrap(), 224);
}

fn check_cap_case(cursor: &mut &[u8]) {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let accepted = read_word(cursor) != 0;
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let parsed = SimulationPostGraphParameters::from_bytes(raw, version, LIMITS);
    assert_eq!(parsed.is_ok(), accepted, "{version:?}/{size}");
    if let Ok(body) = parsed {
        assert_constructed(&body);
        assert_eq!(body.coverage().declared_entries, 1024);
        assert_eq!(body.fields().len(), 79);
    }
}

#[test]
fn sufficient_payloads_enforce_character_count_caps_without_relying_on_eof() {
    let mut cursor = CAPS;
    for _ in 0..12 {
        check_cap_case(&mut cursor);
    }
    assert!(cursor.is_empty());
}
