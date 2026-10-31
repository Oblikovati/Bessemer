use super::*;

fn counted_null_header(version: i32, first: usize, second: usize) -> Vec<u8> {
    let mut raw: Vec<u8> = [
        -2147483648_i32,
        2147483647,
        -1,
        2,
        i32::try_from(first).unwrap(),
    ]
    .into_iter()
    .flat_map(i32::to_le_bytes)
    .collect();
    raw.resize(raw.len() + first * 4, 0);
    if version >= 870008 {
        raw.extend_from_slice(&i32::try_from(second).unwrap().to_le_bytes());
        raw.resize(raw.len() + second * 4, 0);
    }
    raw
}

fn assert_case(version: i32, first: usize, second: usize, accepted: bool) {
    let raw = counted_null_header(version, first, second);
    let context = SimulationPersistVersion::new(version).unwrap();
    let result = SimulationSolidManagerProperty::from_bytes(&raw, context, LIMITS);
    assert_eq!(
        result.is_ok(),
        accepted,
        "version {version} lists {first}/{second}"
    );
    match result {
        Ok(header) => {
            assert_accepted(&header, &raw, version, first, second);
        }
        Err(error) => {
            assert!(error.cause.contains("100001"));
            assert!(error.cause.contains("0..=100000"));
        }
    }
}

#[test]
fn exact_one_hundred_thousand_entry_caps_and_one_above_use_sufficient_complete_payloads() {
    for (version, first, second, accepted) in [
        (870004, 100000, 0, true),
        (870008, 100000, 100000, true),
        (870004, 100001, 0, false),
        (870008, 0, 100001, false),
    ] {
        assert_case(version, first, second, accepted);
    }
}

fn assert_accepted(
    header: &SimulationSolidManagerProperty,
    raw: &[u8],
    version: i32,
    first: usize,
    second: usize,
) {
    assert_eq!(header.to_bytes(), raw);
    assert_eq!(header.coverage().list_entries, first + second);
    assert_eq!(
        header.fields().len(),
        5 + first + if version >= 870008 { 1 + second } else { 0 }
    );
}
