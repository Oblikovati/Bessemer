use super::*;

const MALFORMED: &[u8] = include_bytes!("../fixtures/simulation-inertial-loads/malformed.bin");
const BUDGETS: &[u8] = include_bytes!("../fixtures/simulation-inertial-loads/budgets.bin");

#[test]
fn targeted_mutations_match_complete_actual_reference_acceptance() {
    let mut cursor = MALFORMED;
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        let (yes, no) = check_reference_mutations(&mut cursor);
        accepted += yes;
        rejected += no;
    }
    assert_eq!((accepted, rejected), (0, 15927));
}

fn splice_mutant(cursor: &mut &[u8], original: &[u8]) -> Vec<u8> {
    let start = word(cursor);
    let removed = word(cursor);
    let size = word(cursor);
    let (replacement, remaining) = cursor.split_at(size);
    *cursor = remaining;
    let mut image = original[..start].to_vec();
    image.extend_from_slice(replacement);
    image.extend_from_slice(&original[start + removed..]);
    image
}

fn read_budget_case(cursor: &mut &[u8]) -> SimulationInertialCollection {
    let kind = KINDS[word(cursor)];
    let version = SimulationPersistVersion::new(word(cursor) as i32).unwrap();
    let size = word(cursor);
    let (raw, remaining) = cursor.split_at(size);
    *cursor = remaining;
    SimulationInertialCollection::from_bytes(raw, version, kind, LIMITS).unwrap()
}

fn reference_limits(cursor: &mut &[u8]) -> SimulationInertialLimits {
    let mut limits = LIMITS;
    limits.maximum_entries = word(cursor);
    limits.maximum_text_bytes = word(cursor);
    limits.embedded.maximum_bytes = word(cursor);
    limits.embedded.maximum_objects = word(cursor);
    limits.maximum_fields = word(cursor);
    limits.embedded.maximum_fields = word(cursor);
    limits
}

fn assert_budget_failure(
    collection: &SimulationInertialCollection,
    limits: SimulationInertialLimits,
) {
    let original = collection.to_bytes();
    let result = SimulationInertialCollection::from_bytes(
        &original,
        collection.version(),
        collection.kind(),
        limits,
    );
    assert!(
        result.is_err(),
        "limits {limits:?}; expected a budget failure"
    );
    assert_eq!(collection.to_bytes(), original);
}

fn assert_reduced_budgets(
    collection: &SimulationInertialCollection,
    exact: SimulationInertialLimits,
) {
    for index in 0..7 {
        let mut limits = exact;
        let value = match index {
            0 => &mut limits.maximum_bytes,
            1 => &mut limits.maximum_fields,
            2 => &mut limits.maximum_entries,
            3 => &mut limits.maximum_text_bytes,
            4 => &mut limits.embedded.maximum_bytes,
            5 => &mut limits.embedded.maximum_objects,
            _ => &mut limits.embedded.maximum_fields,
        };
        if *value == 0 {
            continue;
        }
        *value -= 1;
        assert_budget_failure(collection, limits);
    }
}

#[test]
fn every_owned_context_obeys_independent_exact_and_one_below_budgets() {
    let mut cursor = BUDGETS;
    let mut count = 0;
    while !cursor.is_empty() {
        let collection = read_budget_case(&mut cursor);
        let mut limits = reference_limits(&mut cursor);
        limits.maximum_bytes = collection.to_bytes().len();
        assert_exact_budgets(&collection, limits);
        count += 1;
    }
    assert_eq!(count, 135);
}

#[test]
fn complete_owned_contexts_reject_every_proper_prefix_and_trailing_byte() {
    let mut cursor = OWNED;
    let mut count = 0;
    while !cursor.is_empty() {
        let (kind, version, raw) = candidate(&mut cursor);
        assert_prefix_rejections(kind, version, raw);
        for _ in 0..word(&mut cursor) {
            word(&mut cursor);
            word(&mut cursor);
        }
        count += 1;
    }
    assert_eq!(count, 135);
}

fn assert_exact_budgets(
    collection: &SimulationInertialCollection,
    limits: SimulationInertialLimits,
) {
    let exact = SimulationInertialCollection::from_bytes(
        &collection.to_bytes(),
        collection.version(),
        collection.kind(),
        limits,
    )
    .unwrap_or_else(|error| panic!("{error:?}; expected exact independent budgets {limits:?}"));
    assert_eq!(exact.to_bytes(), collection.to_bytes());
    assert_reduced_budgets(collection, limits);
}

fn assert_prefix_rejections(
    kind: SimulationInertialKind,
    version: SimulationPersistVersion,
    raw: &[u8],
) {
    for end in 0..raw.len() {
        assert!(
            SimulationInertialCollection::from_bytes(&raw[..end], version, kind, LIMITS).is_err(),
            "{kind:?} {version:?}, end {end}; expected incomplete collection"
        );
    }
    let mut trailing = raw.to_vec();
    trailing.push(0);
    assert!(SimulationInertialCollection::from_bytes(&trailing, version, kind, LIMITS).is_err());
}

fn check_reference_mutations(cursor: &mut &[u8]) -> (usize, usize) {
    let collection = read_budget_case(cursor);
    let (mut accepted, mut rejected) = (0, 0);
    for _ in 0..word(cursor) {
        let expected = word(cursor) != 0;
        let image = splice_mutant(cursor, &collection.to_bytes());
        let result = SimulationInertialCollection::from_bytes(
            &image,
            collection.version(),
            collection.kind(),
            LIMITS,
        );
        assert_eq!(
            result.is_ok(),
            expected,
            "expected actual reference acceptance"
        );
        accepted += usize::from(expected);
        rejected += usize::from(!expected);
    }
    (accepted, rejected)
}
