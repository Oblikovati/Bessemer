use super::*;

#[test]
fn all_carrier_limits_are_positive_and_each_aggregate_bound_is_enforced() {
    let case = cases().remove(5);
    let bounds = exact_carrier_limits(&case);
    for index in 0..5 {
        let mut invalid = bounds;
        set_limit(&mut invalid, index, 0);
        assert!(ElectricalArchive::from_bytes(&case.raw, invalid).is_err());
    }
    ElectricalArchive::from_bytes(&case.raw, bounds).unwrap();
    for index in 0..4 {
        let mut invalid = bounds;
        decrease_limit(&mut invalid, index);
        assert!(ElectricalArchive::from_bytes(&case.raw, invalid).is_err());
    }
}

fn exact_carrier_limits(case: &Case) -> ElectricalLimits {
    ElectricalLimits {
        maximum_bytes: case.raw.len(),
        maximum_members: case.members.len(),
        maximum_member_bytes: case
            .members
            .iter()
            .map(|member| member.payload.len())
            .max()
            .unwrap(),
        maximum_expanded_bytes: case.members.iter().map(|member| member.payload.len()).sum(),
        maximum_ini_records: 8192,
    }
}

fn set_limit(limits: &mut ElectricalLimits, index: usize, value: usize) {
    match index {
        0 => limits.maximum_bytes = value,
        1 => limits.maximum_members = value,
        2 => limits.maximum_member_bytes = value,
        3 => limits.maximum_expanded_bytes = value,
        4 => limits.maximum_ini_records = value,
        _ => panic!("limit {index}; expected 0..5"),
    }
}

fn decrease_limit(limits: &mut ElectricalLimits, index: usize) {
    let value = match index {
        0 => limits.maximum_bytes,
        1 => limits.maximum_members,
        2 => limits.maximum_member_bytes,
        3 => limits.maximum_expanded_bytes,
        _ => panic!("limit {index}; expected 0..4"),
    };
    set_limit(limits, index, value - 1);
}

#[test]
fn plain_zip_budget_excludes_adapter_bytes_and_stored_growth_is_rejected_at_exact_limit() {
    let case = cases().remove(5);
    let mut bounds = limits();
    bounds.maximum_bytes = case.raw.len();
    let source = ElectricalArchive::from_bytes(&case.raw, bounds).unwrap();
    assert_eq!(source.coverage().wire_bytes, case.raw.len());
    let id = source.members().next().unwrap().id;
    let mut replacement = case.members[0].payload.clone();
    replacement.push(b'x');
    assert!(source.edit_member(&id, &replacement).is_err());
    assert_eq!(source.to_bytes(), case.raw);
    bounds.maximum_bytes = usize::MAX;
    ElectricalArchive::from_bytes(&case.raw, bounds).unwrap();
}

#[test]
fn ini_limits_cover_bytes_records_positive_configuration_and_result_growth() {
    let raw = b"[S]\nK=v\n";
    let exact = ElectricalIniLimits {
        maximum_bytes: raw.len(),
        maximum_records: 2,
    };
    let source = ElectricalIniDocument::from_bytes(raw, exact)
        .unwrap()
        .unwrap();
    assert!(source.edit_value(1, b"longer").is_err());
    for bounds in invalid_ini_limits(exact) {
        assert!(ElectricalIniDocument::from_bytes(raw, bounds).is_err());
    }
    assert_eq!(source.bytes(), raw);
}

fn invalid_ini_limits(exact: ElectricalIniLimits) -> [ElectricalIniLimits; 4] {
    [
        ElectricalIniLimits {
            maximum_bytes: exact.maximum_bytes - 1,
            ..exact
        },
        ElectricalIniLimits {
            maximum_records: 1,
            ..exact
        },
        ElectricalIniLimits {
            maximum_bytes: 0,
            ..exact
        },
        ElectricalIniLimits {
            maximum_records: 0,
            ..exact
        },
    ]
}

#[test]
fn archive_ini_limits_wrong_names_non_ascii_and_foreign_revisions_are_explicit() {
    let case = cases().remove(5);
    let mut bounds = limits();
    bounds.maximum_ini_records = 1;
    let source = ElectricalArchive::from_bytes(&case.raw, bounds).unwrap();
    let id = source.members().next().unwrap().id;
    assert!(source.archive_ini(&id).is_err());
    let opaque = parse(&cases()[7].raw);
    let id = opaque.member_named("Archive.ini").unwrap();
    assert!(opaque.archive_ini(&id).unwrap().is_none());
    assert!(opaque.edit_ini_value(&id, 0, b"value").is_err());
    assert!(source.archive_ini(&id).is_err());
    let unknown = source.members().nth(2).unwrap().id;
    assert!(source.archive_ini(&unknown).is_err());
}

#[test]
fn corrupt_payloads_partial_carriers_and_wrong_container_prefixes_fail_with_typed_errors() {
    let case = cases().remove(5);
    let offsets = directory_offsets(&case.raw, case.comment.len(), case.members.len());
    let mut corrupt = case.raw.clone();
    let first = offsets[0];
    let data = first
        + 30
        + usize::from(u16_at(&corrupt, first + 26))
        + usize::from(u16_at(&corrupt, first + 28));
    corrupt[data] ^= 1;
    let error = ElectricalArchive::from_bytes(&corrupt, limits()).unwrap_err();
    assert_eq!(error.operation, "electrical archive");
    for length in [0, 1, 21, case.raw.len() - 1] {
        assert!(ElectricalArchive::from_bytes(&case.raw[..length], limits()).is_err());
    }
    let mut prefixed = b"SMG".to_vec();
    prefixed.extend_from_slice(&case.raw);
    assert!(ElectricalArchive::from_bytes(&prefixed, limits()).is_err());
}
