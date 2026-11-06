//! Independent complete shared-item grammar for raw126, compressed315 and versioned128.
#[path = "flow_conditional_items/api.rs"]
mod api;
#[path = "flow_conditional_items/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_field_values/stored_fields.rs"]
mod stored_fields;
#[path = "flow_conditional_items/types.rs"]
mod types;
use packet::*;
use std::collections::BTreeSet;
use types::*;

#[test]
fn every_conditional_item_rebuilds_all_source_fields_counts_intervals_and_transport_bytes() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 6659);
    for case in cases {
        check_complete(&case);
    }
}

fn check_complete(case: &ConditionalCase) {
    let mut rebuilt = Vec::new();
    for field in &case.fields {
        assert_eq!(
            rebuilt.len(),
            field.offset as usize,
            "{}:{}",
            case.name,
            field.path
        );
        assert_eq!(field.raw.len(), field.count as usize * field.width());
        rebuilt.extend_from_slice(&field.raw);
    }
    assert_eq!(rebuilt, case.decoded, "{}", case.name);
    check_scopes(case);
    check_counts(case);
    check_transport(case);
    assert!(case.complete && !case.runtime && !case.engineering);
}

fn check_scopes(case: &ConditionalCase) {
    assert_eq!(case.scopes[0].path, "[]");
    assert_eq!(case.scopes[0].start, 0);
    assert_eq!(case.scopes[0].end as usize, case.decoded.len());
    for scope in &case.scopes {
        assert!(scope.start <= scope.end && scope.end as usize <= case.decoded.len());
    }
    let paths: BTreeSet<_> = case.scopes.iter().map(|scope| &scope.path).collect();
    assert_eq!(paths.len(), case.scopes.len());
}

fn check_counts(case: &ConditionalCase) {
    let offsets: BTreeSet<_> = case.counts.iter().copied().collect();
    assert_eq!(offsets.len(), case.counts.len());
    for offset in offsets {
        let field = case
            .fields
            .iter()
            .find(|field| field.offset == offset && !field.raw.is_empty())
            .unwrap();
        assert_eq!(field.encoding, 1);
        assert_eq!(field.count, 1);
        assert!(i32::from_le_bytes(field.raw.as_slice().try_into().unwrap()) >= 0);
    }
}

fn check_transport(case: &ConditionalCase) {
    match case.identifier {
        126 => {
            assert_eq!(case.stored, case.decoded);
            assert_eq!(case.outer_version, None);
            assert_eq!(case.scale, 1);
        }
        315 => {
            assert_eq!(inflate(&case.stored), case.decoded);
            assert_eq!(case.version, 4);
            assert_eq!(case.outer_version, None);
        }
        128 => {
            let version = i32::from_le_bytes(case.stored[..4].try_into().unwrap());
            assert_eq!(case.outer_version, Some(version));
            assert_eq!(case.version, version);
            assert_eq!(case.scale, if version == 6 { 9 } else { 6 });
            assert_eq!(inflate(&case.stored[4..]), case.decoded);
        }
        other => panic!("Fixture identifier {other}; expected126/315/128"),
    }
}

fn inflate(stored: &[u8]) -> Vec<u8> {
    use std::io::Read;
    let mut decoder = flate2::read::ZlibDecoder::new(stored);
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();
    assert_eq!(decoder.total_in(), stored.len() as u64);
    decoded
}

#[test]
fn populated_native_conditional_records_preserve_their_complete_original_payloads() {
    let cases = fixture::cases();
    for (identifier, version, stored, decoded) in [
        (126, 2, 84301, 84301),
        (315, 4, 32005, 64956),
        (128, 6, 70322, 283045),
    ] {
        let case = cases
            .iter()
            .find(|case| case.name == format!("native-{identifier}"))
            .unwrap();
        check_native_extents(case, (identifier, version, stored, decoded));
        check_complete(case);
        assert_eq!(
            case.scopes
                .iter()
                .any(|scope| scope.path.contains("\"item\"")),
            identifier != 315
        );
    }
}

fn check_native_extents(case: &ConditionalCase, expected: (u32, i32, usize, usize)) {
    assert_eq!(
        (
            case.identifier,
            case.version,
            case.stored.len(),
            case.decoded.len()
        ),
        expected
    );
}

#[test]
fn all_activity_presence_and_mode_bytes_and_six_item_versions_have_owned_coverage() {
    let cases = fixture::cases();
    for role in ["active_byte", "present_byte", "mode_byte"] {
        let values: BTreeSet<_> = cases
            .iter()
            .flat_map(|case| &case.fields)
            .filter(|field| field.path.ends_with(&format!("\"{role}\"]")))
            .flat_map(|field| field.raw.iter().copied())
            .collect();
        assert_eq!(values, (0..=255u8).collect(), "{role}");
    }
    let versions: BTreeSet<_> = cases
        .iter()
        .flat_map(|case| &case.fields)
        .filter(|field| field.path.contains("\"item\"") && field.path.ends_with("\"version\"]"))
        .map(|field| i32::from_le_bytes(field.raw.as_slice().try_into().unwrap()))
        .collect();
    assert_eq!(versions, (0..=5).collect());
    for case in &cases {
        check_projections(case);
    }
}

fn check_projections(case: &ConditionalCase) {
    for (path, projection) in &case.projections {
        if path.ends_with("\"native_mode_boolean\"]") {
            let mode_path = path.replace("native_mode_boolean", "mode_byte");
            let field = case
                .fields
                .iter()
                .find(|field| field.path == mode_path)
                .unwrap();
            assert_eq!(*projection, field.raw[0] != 0);
        } else {
            assert!(path.ends_with("\"native_inverse_map_reproduced\"]"));
            assert!(!projection);
        }
    }
}

#[test]
fn conditional_raw_arrays_keep_all_word_and_wide_single_bits_complements_and_specials() {
    let cases = fixture::cases();
    let words = conditional_patterns(&cases, 2);
    let wides = conditional_patterns(&cases, 3);
    for bit in 0..32 {
        assert!(
            words.contains(&u64::from(1u32 << bit)) && words.contains(&u64::from(!(1u32 << bit)))
        );
    }
    for bit in 0..64 {
        assert!(wides.contains(&(1u64 << bit)) && wides.contains(&!(1u64 << bit)));
    }
    check_specials(&wides);
}

fn check_specials(wides: &BTreeSet<u64>) {
    for pattern in [
        0,
        0x8000000000000000,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000000042,
        0x7ff0000000000001,
    ] {
        assert!(wides.contains(&pattern));
    }
}

fn conditional_patterns(cases: &[ConditionalCase], encoding: u8) -> BTreeSet<u64> {
    stored_fields::patterns(cases.iter().flat_map(|case| &case.fields), encoding)
}
