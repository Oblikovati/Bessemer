use super::load_mutations::check_mutations;
use super::*;

const MALFORMED: &[u8] = include_bytes!("../fixtures/simulation-solid-manager/malformed.bin");
const BUDGETS: &[u8] = include_bytes!("../fixtures/simulation-solid-manager/budgets.bin");

#[test]
fn every_targeted_input_matches_actual_source_acceptance_including_valid_controls() {
    let mut cursor = MALFORMED;
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        assert_eq!(read_word(&mut cursor), 0);
        let header = read_original(&mut cursor);
        let (yes, no) = check_mutations(&mut cursor, &header.to_bytes(), |image| {
            SimulationSolidManagerProperty::from_bytes(image, header.version(), LIMITS).is_ok()
        });
        accepted += yes;
        rejected += no;
    }
    assert_eq!((accepted, rejected), (15, 378));
}

type CapSelector = fn(&mut SimulationSolidManagerLimits) -> &mut usize;

fn cap_selectors() -> [CapSelector; 4] {
    [
        |limits| &mut limits.maximum_bytes,
        |limits| &mut limits.maximum_fields,
        |limits| &mut limits.maximum_entries,
        |limits| &mut limits.maximum_text_bytes,
    ]
}

fn assert_lowered(
    header: &SimulationSolidManagerProperty,
    exact: SimulationSolidManagerLimits,
    select: CapSelector,
) {
    let mut limits = exact;
    let value = select(&mut limits);
    if *value == 0 {
        return;
    }
    *value -= 1;
    assert!(
        SimulationSolidManagerProperty::from_bytes(&header.to_bytes(), header.version(), limits)
            .is_err(),
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
            SimulationSolidManagerProperty::from_bytes(&header.to_bytes(), header.version(), exact)
                .unwrap();
        assert_eq!(accepted.to_bytes(), header.to_bytes());
        assert_eq!(accepted.coverage().list_entries, exact.maximum_entries);
        for select in cap_selectors() {
            assert_lowered(&header, exact, select);
        }
        count += 1;
    }
    assert_eq!(count, 68);
}

fn assert_prefixes(header: &SimulationSolidManagerProperty) {
    let raw = header.to_bytes();
    for end in 0..raw.len() {
        assert!(
            SimulationSolidManagerProperty::from_bytes(&raw[..end], header.version(), LIMITS)
                .is_err(),
            "{:?} prefix {end}/{}",
            header.version(),
            raw.len()
        );
    }
    let mut appended = raw;
    appended.push(0);
    assert!(
        SimulationSolidManagerProperty::from_bytes(&appended, header.version(), LIMITS).is_err()
    );
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
    assert_eq!(count, 68);
}

fn read_exact_budget(
    cursor: &mut &[u8],
    header: &SimulationSolidManagerProperty,
) -> SimulationSolidManagerLimits {
    SimulationSolidManagerLimits {
        maximum_bytes: header.to_bytes().len(),
        maximum_entries: read_word(cursor) as usize,
        maximum_text_bytes: read_word(cursor) as usize,
        maximum_fields: read_word(cursor) as usize,
    }
}
