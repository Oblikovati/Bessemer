use super::*;

pub(super) fn empty_string_records(
    count: usize,
    version: i32,
    profile: SimulationShellWireProfile,
) -> Vec<u8> {
    let stride = if version >= 890016 || profile == SimulationShellWireProfile::Writer {
        36
    } else {
        32
    };
    let mut raw = vec![0; 4 + count * stride];
    raw[..4].copy_from_slice(&i32::try_from(count).unwrap().to_le_bytes());
    raw
}

#[test]
fn fixed_record_cap_accepts_100000_complete_records_with_zero_text_payload_budget() {
    let raw = empty_string_records(100000, 890015, PROFILES[0]);
    let version = SimulationPersistVersion::new(890015).unwrap();
    let limits = SimulationShellGroupLimits {
        maximum_bytes: raw.len(),
        maximum_fields: 1_000_001,
        maximum_entries: 100000,
        maximum_text_bytes: 0,
    };
    let group = SimulationShellGroup::from_bytes(&raw, version, PROFILES[0], limits).unwrap();
    assert_eq!(group.coverage().record_count, 100000);
    assert_eq!(group.coverage().text_bytes, 0);
    assert_eq!(group.fields().len(), 1_000_001);
    assert_eq!(
        group.fields().last().unwrap().path.as_ref(),
        "record99999.word78"
    );
    assert_eq!(group.to_bytes(), raw);
}

#[test]
fn fixed_record_cap_rejects_100001_even_with_sufficient_payload_and_caller_budgets() {
    let raw = empty_string_records(100001, 890015, PROFILES[0]);
    let version = SimulationPersistVersion::new(890015).unwrap();
    let limits = SimulationShellGroupLimits {
        maximum_entries: 200000,
        ..LIMITS
    };
    let error = SimulationShellGroup::from_bytes(&raw, version, PROFILES[0], limits).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(error.cause.contains("100001") && error.cause.contains("0..=100000"));
}

#[test]
fn populated_empty_strings_keep_zero_payload_budget_under_both_late_field_profiles() {
    for profile in PROFILES {
        for version in [0, 890015, 890016] {
            assert_empty_string_record(version, profile);
        }
    }
}

#[test]
fn reader_gate_and_explicit_writer_profile_reject_the_opposite_nonempty_body_shape() {
    let before = empty_string_records(1, 890015, PROFILES[0]);
    let after = empty_string_records(1, 890016, PROFILES[0]);
    let old = SimulationPersistVersion::new(890015).unwrap();
    let new = SimulationPersistVersion::new(890016).unwrap();
    assert!(SimulationShellGroup::from_bytes(&before, new, PROFILES[0], LIMITS).is_err());
    assert!(SimulationShellGroup::from_bytes(&after, old, PROFILES[0], LIMITS).is_err());
    assert!(SimulationShellGroup::from_bytes(&before, old, PROFILES[1], LIMITS).is_err());
    assert!(SimulationShellGroup::from_bytes(&after, old, PROFILES[1], LIMITS).is_ok());
}

fn assert_empty_string_record(version: i32, profile: SimulationShellWireProfile) {
    let raw = empty_string_records(1, version, profile);
    let limits = SimulationShellGroupLimits {
        maximum_text_bytes: 0,
        ..LIMITS
    };
    let group = SimulationShellGroup::from_bytes(
        &raw,
        SimulationPersistVersion::new(version).unwrap(),
        profile,
        limits,
    )
    .unwrap();
    assert_eq!(group.coverage().record_count, 1);
    assert_eq!(group.coverage().text_bytes, 0);
    assert_eq!(
        group.coverage().completion,
        SimulationShellGroupCompletion::CompletedRecords
    );
}
