use super::*;

fn text_case(count: i32, payload: &[u8]) -> (Vec<u8>, SimulationPersistVersion, usize) {
    let body = owned_styles()
        .into_iter()
        .find(|body| body.version().value() == 700006)
        .unwrap();
    let ordinal = body
        .fields()
        .iter()
        .position(|field| matches!(field.value, SimulationPostStyleValue::Text(_)))
        .unwrap();
    let (start, end) = body.field_offsets()[ordinal];
    let original = body.to_bytes();
    let mut raw = original[..start].to_vec();
    raw.extend_from_slice(&count.to_le_bytes());
    raw.extend_from_slice(payload);
    raw.extend_from_slice(&original[end..]);
    (raw, body.version(), body.fields().len())
}

#[test]
fn style_text_retains_raw_units_and_uses_its_own_character_budget() {
    let payload = vec![0; 1024];
    let (raw, version, _) = text_case(512, &payload);
    let body = SimulationPostStyle::from_bytes(&raw, version, LIMITS).unwrap();
    assert_eq!(body.to_bytes(), raw);
    let (larger, version, _) = text_case(513, &[0; 1026]);
    assert!(SimulationPostStyle::from_bytes(&larger, version, LIMITS).is_err());
    let limits = SimulationPostStyleLimits {
        maximum_characters: 513,
        maximum_entries: 513,
        maximum_text_bytes: 1026,
        ..LIMITS
    };
    assert_eq!(
        SimulationPostStyle::from_bytes(&larger, version, limits)
            .unwrap()
            .to_bytes(),
        larger
    );
}

#[test]
fn exact_style_resource_axes_and_malformed_text_are_checked_independently() {
    let (raw, version, fields) = text_case(2, &[0, 0xd8, 0, 0]);
    let exact = SimulationPostStyleLimits {
        maximum_bytes: raw.len(),
        maximum_fields: fields,
        maximum_entries: 2,
        maximum_text_bytes: 4,
        maximum_characters: 2,
    };
    assert!(SimulationPostStyle::from_bytes(&raw, version, exact).is_ok());
    for axis in 0..5 {
        let limits = one_below_style_budget(exact, axis);
        assert!(
            SimulationPostStyle::from_bytes(&raw, version, limits).is_err(),
            "{axis}"
        );
    }
}

#[test]
fn style_character_text_owns_its_code_units() {
    let mut supplied = vec![0, 0xd8, 0, 0];
    let text = SimulationPostStyleText::from_payload(&supplied, supplied.len()).unwrap();
    supplied.fill(0xff);
    assert_eq!(text.payload(), &[0, 0xd8, 0, 0]);
    assert_eq!(text.character_count(), 2);
    assert!(SimulationPostStyleText::from_payload(&[1], 2).is_err());
}

fn one_below_style_budget(
    mut limits: SimulationPostStyleLimits,
    axis: usize,
) -> SimulationPostStyleLimits {
    match axis {
        0 => limits.maximum_bytes -= 1,
        1 => limits.maximum_fields -= 1,
        2 => limits.maximum_entries -= 1,
        3 => limits.maximum_text_bytes -= 1,
        _ => limits.maximum_characters -= 1,
    }
    limits
}

#[test]
fn malformed_style_character_counts_are_rejected() {
    for count in [-1, i32::MAX] {
        let (invalid, version, _) = text_case(count, &[]);
        assert!(SimulationPostStyle::from_bytes(&invalid, version, LIMITS).is_err());
    }
}
