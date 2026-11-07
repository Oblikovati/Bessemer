use super::*;

const MUTATIONS: &[u8] = include_bytes!("../fixtures/simulation-mesh-property/malformed.bin");
const HARDCAPS: &[u8] = include_bytes!("../fixtures/simulation-mesh-property/hardcaps.bin");

fn complete_mutant(raw: &[u8], version: SimulationPersistVersion) -> bool {
    let Ok(parsed) = SimulationMeshProperty::from_bytes(raw, version, LIMITS) else {
        return false;
    };
    assert_eq!(parsed.to_bytes(), raw);
    assert_constructed(&parsed);
    true
}

#[test]
fn all_owned_grammar_mutations_match_actual_source_outcomes_and_complete_construction() {
    let mut cursor = MUTATIONS;
    let (mut contexts, mut accepted, mut rejected) = (0, 0, 0);
    while !cursor.is_empty() {
        let body = read_property(&mut cursor);
        let raw = body.to_bytes();
        let counts = load_limits::check_mutations(&mut cursor, &raw, |image| {
            complete_mutant(image, body.version())
        });
        assert_eq!(body.to_bytes(), raw);
        contexts += 1;
        accepted += counts.0;
        rejected += counts.1;
    }
    assert_eq!((contexts, accepted, rejected), (588, 2052, 59739));
}

fn check_hardcap(cursor: &mut &[u8]) -> bool {
    assert_eq!(read_word(cursor), 0);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let accepted = read_word(cursor) != 0;
    let fields = read_word(cursor) as usize;
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let parsed = SimulationMeshProperty::from_bytes(raw, version, LIMITS);
    assert_eq!(parsed.is_ok(), accepted, "version={version:?}, size={size}");
    if let Ok(parsed) = parsed {
        assert_eq!(parsed.fields().len(), fields);
        assert_eq!(parsed.to_bytes(), raw);
        assert_constructed(&parsed);
    }
    accepted
}

#[test]
fn all_sixteen_fixed_caps_have_complete_sufficient_payload_controls_on_both_sides() {
    let mut cursor = HARDCAPS;
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        if check_hardcap(&mut cursor) {
            accepted += 1;
        } else {
            rejected += 1;
        }
    }
    assert_eq!((accepted, rejected), (16, 16));
}
