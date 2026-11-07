use super::*;

fn all_owned(check: fn(&SimulationSolidChildProperty)) {
    for (mut cursor, kind) in [(EXPECTED, COMPONENT), (FOLDERS, FOLDER)] {
        while !cursor.is_empty() {
            let header = read_original(&mut cursor, kind);
            consume_expected_fields(&mut cursor);
            check(&header);
        }
    }
}

fn rejected(
    header: &SimulationSolidChildProperty,
    raw: &[u8],
    limits: SimulationSolidChildLimits,
) -> bool {
    SimulationSolidChildProperty::from_bytes(raw, header.version(), header.kind(), limits).is_err()
}

fn assert_prefixes(header: &SimulationSolidChildProperty) {
    let raw = header.to_bytes();
    for length in 0..raw.len() {
        assert!(rejected(header, &raw[..length], LIMITS));
    }
    let mut trailing = raw;
    trailing.push(0);
    assert!(rejected(header, &trailing, LIMITS));
}

#[test]
fn all_proper_prefixes_and_trailing_bytes_are_incomplete() {
    all_owned(assert_prefixes);
}

fn exact_limits(header: &SimulationSolidChildProperty) -> SimulationSolidChildLimits {
    let text = header
        .fields()
        .iter()
        .filter_map(|field| match &field.value {
            SimulationSolidChildValue::Text(text) => Some(text.payload().len()),
            _ => None,
        })
        .max()
        .unwrap_or(0);
    SimulationSolidChildLimits {
        maximum_bytes: header.to_bytes().len(),
        maximum_fields: header.fields().len(),
        maximum_entries: header.coverage().list_entries,
        maximum_text_bytes: text,
    }
}

fn reduced_limits(
    mut limits: SimulationSolidChildLimits,
    axis: usize,
) -> Option<SimulationSolidChildLimits> {
    let value = match axis {
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

fn assert_limits(header: &SimulationSolidChildProperty) {
    let limits = exact_limits(header);
    let raw = header.to_bytes();
    assert!(!rejected(header, &raw, limits));
    for axis in 0..4 {
        if let Some(reduced) = reduced_limits(limits, axis) {
            assert!(rejected(header, &raw, reduced), "axis {axis}");
        }
    }
}

#[test]
fn all_four_budgets_accept_exact_requirements_and_reject_one_below() {
    all_owned(assert_limits);
}

fn folder_with_count(count: i32, payload_count: usize) -> Vec<u8> {
    let mut raw = vec![0; 12 + payload_count * 4];
    raw[8..12].copy_from_slice(&count.to_le_bytes());
    raw
}

#[test]
fn static_folder_profile_rejects_negative_and_unavailable_counts() {
    let version = SimulationPersistVersion::new(0).unwrap();
    for count in [-1, i32::MIN, 1, i32::MAX] {
        let raw = folder_with_count(count, 0);
        assert!(
            SimulationSolidChildProperty::from_bytes(&raw, version, FOLDER, LIMITS).is_err(),
            "count {count}"
        );
    }
}

#[test]
fn folder_does_not_inherit_unrelated_manager_hundred_thousand_cap() {
    let version = SimulationPersistVersion::new(0).unwrap();
    let raw = folder_with_count(100001, 100001);
    let header = SimulationSolidChildProperty::from_bytes(&raw, version, FOLDER, LIMITS).unwrap();
    assert_eq!(header.coverage().list_entries, 100001);
    assert_eq!(header.fields().len(), 100004);
    assert_eq!(header.to_bytes(), raw);
}

#[test]
fn explicit_kinds_cannot_silently_autodetect_another_body() {
    let mut cursor = EXPECTED;
    let raw = read_original(&mut cursor, COMPONENT);
    assert!(
        SimulationSolidChildProperty::from_bytes(&raw.to_bytes(), raw.version(), FOLDER, LIMITS)
            .is_err()
    );
}

#[test]
fn targeted_component_mutations_match_every_actual_reader_outcome() {
    let mut cursor: &[u8] =
        include_bytes!("../fixtures/simulation-solid-children/component-mutations.bin");
    let (mut accepted, mut rejected) = (0, 0);
    while !cursor.is_empty() {
        assert_eq!(read_word(&mut cursor), 0);
        let header = read_original(&mut cursor, COMPONENT);
        let (yes, no) = load_mutations::check_mutations(&mut cursor, &header.to_bytes(), |raw| {
            !self::rejected(&header, raw, LIMITS)
        });
        accepted += yes;
        rejected += no;
    }
    assert_eq!((accepted, rejected), (48, 132));
}

fn assert_independent_budget(cursor: &mut &[u8]) {
    let kind = [COMPONENT, FOLDER][read_word(cursor) as usize];
    let header = read_original(cursor, kind);
    let entries = read_word(cursor) as usize;
    let text_bytes = read_word(cursor) as usize;
    let field_count = read_word(cursor) as usize;
    let limits = exact_limits(&header);
    assert_eq!(limits.maximum_entries, entries);
    assert_eq!(limits.maximum_text_bytes, text_bytes);
    assert_eq!(limits.maximum_fields, field_count);
    assert_limits(&header);
}

#[test]
fn independent_component_calls_and_static_folder_vectors_determine_all_budgets() {
    let mut cursor: &[u8] = include_bytes!("../fixtures/simulation-solid-children/budgets.bin");
    let mut count = 0;
    while !cursor.is_empty() {
        assert_independent_budget(&mut cursor);
        count += 1;
    }
    assert_eq!(count, 96);
}
