use super::*;

const MUTATIONS: &[u8] = include_bytes!("../fixtures/simulation-material/malformed.bin");
const BUDGETS: &[u8] = include_bytes!("../fixtures/simulation-material/budgets.bin");

fn read_original(cursor: &mut &[u8]) -> SimulationStudyMaterial {
    assert_eq!(
        word(cursor),
        0,
        "expected explicit study-manager Material profile"
    );
    let version = SimulationPersistVersion::new(word(cursor) as i32).unwrap();
    let size = word(cursor);
    SimulationStudyMaterial::from_bytes(take(cursor, size), version, LIMITS).unwrap()
}

#[test]
fn every_targeted_input_preserves_actual_source_acceptance_including_valid_alternatives() {
    let mut cursor = MUTATIONS;
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        let source = read_original(&mut cursor);
        let (yes, no) =
            super::load_mutations::check_mutations(&mut cursor, &source.to_bytes(), |image| {
                SimulationStudyMaterial::from_bytes(image, source.version(), LIMITS).is_ok()
            });
        accepted += yes;
        rejected += no;
    }
    assert_eq!((accepted, rejected), (13816, 26806));
}

fn exact_limits(cursor: &mut &[u8], bytes: usize) -> SimulationMaterialLimits {
    let maximum_entries = word(cursor);
    let maximum_text_bytes = word(cursor);
    let maximum_fields = word(cursor);
    SimulationMaterialLimits {
        maximum_bytes: bytes,
        maximum_fields,
        maximum_entries,
        maximum_text_bytes,
    }
}

fn one_below(exact: SimulationMaterialLimits, index: usize) -> Option<SimulationMaterialLimits> {
    let mut limits = exact;
    let value = match index {
        0 => &mut limits.maximum_bytes,
        1 => &mut limits.maximum_fields,
        2 => &mut limits.maximum_entries,
        _ => &mut limits.maximum_text_bytes,
    };
    if *value == 0 {
        return None;
    }
    *value -= 1;
    Some(limits)
}

fn assert_budget_boundary(source: &SimulationStudyMaterial, exact: SimulationMaterialLimits) {
    let raw = source.to_bytes();
    let standalone = SimulationStudyMaterial::from_bytes(&raw, source.version(), exact).unwrap();
    assert_eq!(standalone.to_bytes(), raw);
    let mut carrier = raw.clone();
    carrier.extend_from_slice(b"unrelated following carrier");
    let (prefix, consumed) =
        SimulationStudyMaterial::from_prefix(&carrier, source.version(), exact).unwrap();
    assert_eq!(consumed, raw.len());
    assert_eq!(prefix.to_bytes(), raw);
    assert_text_parents(&prefix, &raw);
    assert_lowered_limits(source, &raw, &carrier, exact);
    assert_eq!(source.to_bytes(), raw);
}

#[test]
fn every_owned_context_obeys_independent_exact_and_one_below_limits_in_both_boundary_modes() {
    let mut cursor = BUDGETS;
    let mut count = 0;
    while !cursor.is_empty() {
        let source = read_original(&mut cursor);
        let limits = exact_limits(&mut cursor, source.to_bytes().len());
        assert_budget_boundary(&source, limits);
        count += 1;
    }
    assert_eq!(count, 300);
}

fn assert_incomplete_prefixes(raw: &[u8], version: SimulationPersistVersion) {
    for end in 0..raw.len() {
        assert!(
            SimulationStudyMaterial::from_bytes(&raw[..end], version, LIMITS).is_err(),
            "{version:?}, {end}/{}; expected incomplete standalone body",
            raw.len()
        );
        assert!(
            SimulationStudyMaterial::from_prefix(&raw[..end], version, LIMITS).is_err(),
            "{version:?}, {end}/{}; expected incomplete prefix body",
            raw.len()
        );
    }
    let mut trailing = raw.to_vec();
    trailing.push(0);
    assert!(SimulationStudyMaterial::from_bytes(&trailing, version, LIMITS).is_err());
}

#[test]
fn all_owned_bodies_reject_every_proper_prefix_in_both_modes_and_standalone_trailing_bytes() {
    let mut cursor = OWNED;
    let mut count = 0;
    while !cursor.is_empty() {
        let version = SimulationPersistVersion::new(word(&mut cursor) as i32).unwrap();
        assert_eq!(word(&mut cursor), 1);
        let size = word(&mut cursor);
        assert_eq!(word(&mut cursor), size);
        assert_incomplete_prefixes(take(&mut cursor, size), version);
        let spans = word(&mut cursor);
        take(&mut cursor, spans * 8);
        count += 1;
    }
    assert_eq!(count, 300);
}

fn assert_lowered_limits(
    source: &SimulationStudyMaterial,
    raw: &[u8],
    carrier: &[u8],
    exact: SimulationMaterialLimits,
) {
    for index in 0..4 {
        if let Some(limits) = one_below(exact, index) {
            assert!(
                SimulationStudyMaterial::from_bytes(raw, source.version(), limits).is_err(),
                "{limits:?}; expected standalone one-below rejection"
            );
            assert!(
                SimulationStudyMaterial::from_prefix(carrier, source.version(), limits).is_err(),
                "{limits:?}; expected prefix one-below rejection"
            );
        }
    }
}
