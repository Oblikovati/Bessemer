use super::*;
use std::collections::BTreeSet;

fn first_reference_record(body: &SimulationPostDialog, ordinal: usize) -> Vec<u8> {
    let path = body.fields()[ordinal].path.strip_suffix(".count").unwrap();
    let prefix = format!("{path}.record[0].");
    let first = ordinal + 1;
    assert!(body.fields()[first].path.starts_with(&prefix));
    let last = (first..body.fields().len())
        .take_while(|index| body.fields()[*index].path.starts_with(&prefix))
        .last()
        .unwrap();
    body.to_bytes()[body.field_offsets()[first].0..body.field_offsets()[last].1].to_vec()
}

fn replaced_references(body: &SimulationPostDialog, ordinal: usize, count: i32) -> Vec<u8> {
    let record = first_reference_record(body, ordinal);
    let path = body.fields()[ordinal].path.strip_suffix(".count").unwrap();
    let prefix = format!("{path}.record[");
    let last = (ordinal + 1..body.fields().len())
        .take_while(|index| body.fields()[*index].path.starts_with(&prefix))
        .last()
        .unwrap();
    let original = body.to_bytes();
    let start = body.field_offsets()[ordinal].0;
    let end = body.field_offsets()[last].1;
    let mut replacement = count.to_le_bytes().to_vec();
    for _ in 0..count.max(0) {
        replacement.extend_from_slice(&record);
    }
    [&original[..start], replacement.as_slice(), &original[end..]].concat()
}

fn assert_reference_count_cap(body: &SimulationPostDialog, ordinal: usize) {
    for count in [-1, 10000, 10001] {
        let raw = replaced_references(body, ordinal, count);
        let limits = reference_control_limits(raw.len());
        let result = SimulationPostDialog::from_bytes(&raw, body.version(), limits);
        assert_eq!(
            result.is_ok(),
            count == 10000,
            "{} {count}: {:?}",
            body.fields()[ordinal].path,
            result.as_ref().err()
        );
        if let Ok(parsed) = result {
            assert_rebuilt_reference_control(&parsed, &raw, limits);
        }
    }
}

#[test]
fn each_populated_reference_role_requires_bounded_complete_records() {
    let mut checked = BTreeSet::new();
    for body in owned_dialogs() {
        for (ordinal, field) in body.fields().iter().enumerate() {
            if !field.path.contains("reference") || !field.path.ends_with(".count") {
                continue;
            }
            if field.value != SimulationPostDialogValue::Signed(0)
                && checked.insert(field.path.clone())
            {
                assert_reference_count_cap(&body, ordinal);
            }
        }
    }
    assert_eq!(checked.len(), 9);
}

fn assert_rebuilt_reference_control(
    parsed: &SimulationPostDialog,
    raw: &[u8],
    limits: SimulationPostDialogLimits,
) {
    assert_eq!(parsed.to_bytes(), raw);
    let built =
        SimulationPostDialog::from_fields(parsed.fields().to_vec(), parsed.version(), limits)
            .unwrap();
    assert_eq!(built.to_bytes(), raw);
}

fn reference_control_limits(bytes: usize) -> SimulationPostDialogLimits {
    // Isolate the imported count cap from the size of complete embedded records.
    SimulationPostDialogLimits {
        maximum_bytes: bytes,
        maximum_fields: 1000000,
        ..LIMITS
    }
}
