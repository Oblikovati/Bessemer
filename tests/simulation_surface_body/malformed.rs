use super::load_mutations::check_mutations;
use super::*;

const MALFORMED: &[u8] = include_bytes!("../fixtures/simulation-surface-body/malformed.bin");
const BUDGETS: &[u8] = include_bytes!("../fixtures/simulation-surface-body/budgets.bin");

#[test]
fn every_targeted_input_matches_actual_source_acceptance_including_valid_controls() {
    let mut cursor = MALFORMED;
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        assert_eq!(read_word(&mut cursor), 0);
        let header = read_original(&mut cursor);
        let (yes, no) = check_mutations(&mut cursor, &header.to_bytes(), |image| {
            SimulationSurfaceBody::from_bytes(image, header.version(), LIMITS).is_ok()
        });
        accepted += yes;
        rejected += no;
    }
    assert_eq!((accepted, rejected), (96, 570));
}

type CapSelector = fn(&mut SimulationSurfaceBodyLimits) -> &mut usize;

fn cap_selectors() -> [CapSelector; 4] {
    [
        |limits| &mut limits.maximum_bytes,
        |limits| &mut limits.maximum_fields,
        |limits| &mut limits.maximum_entries,
        |limits| &mut limits.maximum_text_bytes,
    ]
}

fn assert_lowered(
    header: &SimulationSurfaceBody,
    exact: SimulationSurfaceBodyLimits,
    select: CapSelector,
) {
    let mut limits = exact;
    let value = select(&mut limits);
    if *value == 0 {
        return;
    }
    *value -= 1;
    assert!(
        SimulationSurfaceBody::from_bytes(&header.to_bytes(), header.version(), limits).is_err(),
        "expected one-below rejection: {limits:?}"
    );
}

#[test]
fn every_owned_context_obeys_independent_exact_and_one_below_all_four_caller_budgets() {
    let mut cursor = BUDGETS;
    let mut count = 0;
    while !cursor.is_empty() {
        let header = read_original(&mut cursor);
        let exact = read_exact_budget(&mut cursor, &header);
        let accepted =
            SimulationSurfaceBody::from_bytes(&header.to_bytes(), header.version(), exact).unwrap();
        assert_eq!(accepted.to_bytes(), header.to_bytes());
        assert_eq!(accepted.coverage().list_entries, exact.maximum_entries);
        for select in cap_selectors() {
            assert_lowered(&header, exact, select);
        }
        count += 1;
    }
    assert_eq!(count, 96);
}

fn assert_prefixes(header: &SimulationSurfaceBody) {
    let raw = header.to_bytes();
    for end in 0..raw.len() {
        assert!(
            SimulationSurfaceBody::from_bytes(&raw[..end], header.version(), LIMITS).is_err(),
            "{:?} prefix {end}/{}",
            header.version(),
            raw.len()
        );
    }
    let mut appended = raw;
    appended.push(0);
    assert!(SimulationSurfaceBody::from_bytes(&appended, header.version(), LIMITS).is_err());
}

#[test]
fn every_owned_body_rejects_every_proper_prefix_and_appended_byte() {
    let mut cursor = EXPECTED;
    let mut count = 0;
    while !cursor.is_empty() {
        let header = read_original(&mut cursor);
        consume_expected_fields(&mut cursor);
        assert_prefixes(&header);
        count += 1;
    }
    assert_eq!(count, 96);
}

fn read_exact_budget(
    cursor: &mut &[u8],
    header: &SimulationSurfaceBody,
) -> SimulationSurfaceBodyLimits {
    SimulationSurfaceBodyLimits {
        maximum_bytes: header.to_bytes().len(),
        maximum_entries: read_word(cursor) as usize,
        maximum_text_bytes: read_word(cursor) as usize,
        maximum_fields: read_word(cursor) as usize,
    }
}

#[test]
fn list_cap_uses_complete_payloads_and_sufficient_caller_budgets() {
    let source = owned_property(870008, 0);
    let raw = source.to_bytes();
    assert!(raw.ends_with(&[0; 4]));
    for count in [100000_i32, 100001] {
        let mut image = raw[..raw.len() - 4].to_vec();
        image.extend_from_slice(&count.to_le_bytes());
        image.resize(image.len() + count as usize * 4, 0);
        let result = SimulationSurfaceBody::from_bytes(&image, source.version(), LIMITS);
        assert_eq!(result.is_ok(), count == 100000);
        if let Ok(decoded) = result {
            assert_eq!(decoded.to_bytes(), image);
        }
    }
}
