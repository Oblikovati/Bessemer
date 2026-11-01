use super::*;

fn legacy_raw(count: i32, payload: &[u8]) -> Vec<u8> {
    let mut raw = vec![0; 464];
    raw.extend_from_slice(&count.to_le_bytes());
    raw.extend_from_slice(payload);
    raw
}

#[test]
fn text_retains_code_units_and_uses_configurable_safety_bounds() {
    let payload = vec![0; 2050];
    let text = SimulationPostInfoText::from_payload(&payload, payload.len()).unwrap();
    assert_eq!(text.character_count(), 1025);
    assert_eq!(text.payload(), payload);
    let version = SimulationPersistVersion::new(700002).unwrap();
    let raw = legacy_raw(1025, &payload);
    let body = SimulationPostInfo::from_bytes(&raw, version, LIMITS).unwrap();
    assert_eq!(body.to_bytes(), raw);
    let mut limits = LIMITS;
    limits.maximum_characters = 1024;
    assert!(SimulationPostInfo::from_bytes(&raw, version, limits).is_err());
    assert!(SimulationPostInfoText::from_payload(&[1], 2).is_err());
    assert!(SimulationPostInfoText::from_payload(&[0, 0], 1).is_err());
}

#[test]
fn every_resource_axis_accepts_exact_and_rejects_one_below() {
    let version = SimulationPersistVersion::new(700002).unwrap();
    let raw = legacy_raw(2, &[0, 0xd8, 0, 0]);
    let exact = SimulationPostInfoLimits {
        maximum_bytes: raw.len(),
        maximum_fields: 117,
        maximum_entries: 2,
        maximum_text_bytes: 4,
        maximum_characters: 2,
    };
    assert!(SimulationPostInfo::from_bytes(&raw, version, exact).is_ok());
    for axis in 0..5 {
        let mut limits = exact;
        match axis {
            0 => limits.maximum_bytes -= 1,
            1 => limits.maximum_fields -= 1,
            2 => limits.maximum_entries -= 1,
            3 => limits.maximum_text_bytes -= 1,
            _ => limits.maximum_characters -= 1,
        }
        assert!(
            SimulationPostInfo::from_bytes(&raw, version, limits).is_err(),
            "{axis}"
        );
    }
    assert!(SimulationPostInfo::from_bytes(&legacy_raw(-1, &[]), version, LIMITS).is_err());
    assert!(SimulationPostInfo::from_bytes(&legacy_raw(i32::MAX, &[]), version, LIMITS).is_err());
}

#[test]
fn full_imported_text_cap_and_explicit_larger_budget_are_distinct() {
    let version = SimulationPersistVersion::new(700002).unwrap();
    let mut payload = vec![0; 0x200000];
    let raw = legacy_raw(0x100000, &payload);
    assert!(SimulationPostInfo::from_bytes(&raw, version, LIMITS).is_ok());
    payload.extend_from_slice(&[0, 0xd8]);
    let larger = legacy_raw(0x100001, &payload);
    assert!(SimulationPostInfo::from_bytes(&larger, version, LIMITS).is_err());
    let limits = SimulationPostInfoLimits {
        maximum_characters: 0x100001,
        maximum_entries: 0x100001,
        maximum_text_bytes: 0x200002,
        ..LIMITS
    };
    let body = SimulationPostInfo::from_bytes(&larger, version, limits).unwrap();
    assert_eq!(body.to_bytes(), larger);
}
