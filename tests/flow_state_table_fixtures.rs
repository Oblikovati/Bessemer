//! Owned complete state-table baselines before adding typed readers or editors.
#[path = "flow_state_tables/context_api.rs"]
mod context_api;
#[path = "flow_state_tables/fixture.rs"]
mod fixture;
#[path = "flow_field_values/packet.rs"]
mod packet;
#[path = "flow_state_tables/paired_api.rs"]
mod paired_api;
#[path = "flow_state_tables/arrays.rs"]
mod state_arrays;
#[path = "flow_state_tables/carrier.rs"]
mod state_carrier;
#[path = "flow_state_tables/process.rs"]
mod state_process;
#[path = "flow_state_tables/stored_api.rs"]
mod stored_api;
#[path = "flow_field_values/stored_fields.rs"]
mod stored_fields;
use fixture::StateCase;
use packet::*;
use std::collections::BTreeSet;

#[test]
fn complete_state_tables_rebuild_every_field_scope_count_and_version() {
    let cases = fixture::cases();
    assert_eq!(cases.len(), 2404);
    for case in cases {
        check_fields(&case);
        check_scopes(&case);
        check_counts(&case);
        assert_eq!(
            i32::from_le_bytes(case.raw[..4].try_into().unwrap()),
            case.version
        );
        assert!(case.complete && !case.runtime && !case.engineering);
    }
}
fn check_fields(case: &StateCase) {
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
    assert_eq!(rebuilt, case.raw, "{}", case.name);
}
fn check_scopes(case: &StateCase) {
    assert_eq!(case.scopes[0], ("[]".to_owned(), 0, case.raw.len() as u32));
    let paths: BTreeSet<_> = case.scopes.iter().map(|(path, _, _)| path).collect();
    assert_eq!(paths.len(), case.scopes.len());
    for (_, start, end) in &case.scopes {
        assert!(start <= end && *end as usize <= case.raw.len());
    }
}
fn check_counts(case: &StateCase) {
    let unique: BTreeSet<_> = case.counts.iter().collect();
    assert_eq!(unique.len(), case.counts.len());
    for offset in &case.counts {
        let field = case
            .fields
            .iter()
            .find(|field| field.offset == *offset && field.count > 0)
            .unwrap();
        assert_eq!((field.encoding, field.count), (1, 1));
        assert!(i32::from_le_bytes(field.raw.as_slice().try_into().unwrap()) >= 0);
    }
}

#[test]
fn native_samples_preserve_complete_payloads_without_owner_context_or_runtime_credit() {
    let cases = fixture::cases();
    for (identifier, bytes, version, context) in [
        (129, 507048, 9, None),
        (138, 13488, 3, Some(3369)),
        (139, 2988, 4, None),
        (417, 12, 1, None),
        (432, 20, 1, None),
    ] {
        let case = cases
            .iter()
            .find(|case| case.identifier == identifier && case.name.starts_with("native-"))
            .unwrap();
        assert_eq!(
            (case.raw.len(), case.version, case.context),
            (bytes, version, context)
        );
        assert!(!case.runtime && !case.engineering);
        check_fields(case);
    }
}

#[test]
fn tags_context_fixed_widths_and_scalar_version_projections_keep_stored_shapes() {
    let cases = fixture::cases();
    let tags: BTreeSet<_> = cases
        .iter()
        .filter(|case| case.identifier == 129)
        .flat_map(|case| &case.fields)
        .filter(|field| field.path == "[\"tags_raw\"]")
        .flat_map(|field| field.raw.iter().copied())
        .collect();
    assert_eq!(tags, (0..=255).collect());
    for case in &cases {
        check_record_shape(case);
    }
}
fn check_record_shape(case: &StateCase) {
    if case.identifier == 432 {
        check_scalar_shape(case);
    }
    if case.identifier == 139 {
        for field in &case.fields {
            if field.path.ends_with("\"settings_wide_patterns\"]") {
                assert_eq!(field.count, 320);
            }
            if field.path.ends_with("\"settings_i32_patterns\"]") {
                assert_eq!(field.count, 100);
            }
        }
    }
    if case.identifier == 417 {
        check_quotients(case);
    }
}
fn check_scalar_shape(case: &StateCase) {
    assert_eq!(
        case.projections,
        vec![(
            "[\"native_recomputes_members\"]".to_owned(),
            case.version < 2
        )]
    );
    let width = if [1, 2].contains(&case.version) {
        20
    } else {
        16
    };
    assert_eq!(case.raw.len(), width);
}
fn check_quotients(case: &StateCase) {
    assert_eq!(case.quotients.len(), 2);
    for (path, quotient) in &case.quotients {
        let prefix = path.strip_suffix(']').unwrap();
        let count = scalar_field(case, &format!("{prefix},\"count\"]"));
        let expected = if count == 0 {
            None
        } else {
            Some(scalar_field(case, &format!("{prefix},\"wide_count\"]")) / count)
        };
        assert_eq!(*quotient, expected);
    }
}
fn scalar_field(case: &StateCase, path: &str) -> i32 {
    let field = case.fields.iter().find(|field| field.path == path).unwrap();
    i32::from_le_bytes(field.raw.as_slice().try_into().unwrap())
}

#[test]
fn raw_state_arrays_retain_single_bits_complements_and_nonfinite_patterns() {
    let cases = fixture::cases();
    let words = stored_fields::patterns(cases.iter().flat_map(|case| &case.fields), 2);
    let wides = stored_fields::patterns(cases.iter().flat_map(|case| &case.fields), 3);
    for bit in 0..32 {
        assert!(
            words.contains(&u64::from(1u32 << bit)) && words.contains(&u64::from(!(1u32 << bit)))
        );
    }
    for bit in 0..64 {
        assert!(wides.contains(&(1u64 << bit)) && wides.contains(&!(1u64 << bit)));
    }
    check_special_patterns(&wides);
}
fn check_special_patterns(wides: &BTreeSet<u64>) {
    for value in [
        0,
        0x8000000000000000,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000000042,
        0x7ff0000000000001,
    ] {
        assert!(wides.contains(&value));
    }
}
