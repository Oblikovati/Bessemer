use super::*;

fn expected_entries(body: &SimulationPostDialog) -> usize {
    body.fields()
        .iter()
        .map(|field| match &field.value {
            SimulationPostDialogValue::Signed(count) if field.path.ends_with(".count") => {
                usize::try_from(*count).unwrap()
            }
            SimulationPostDialogValue::CharacterText(text) => text.character_count(),
            SimulationPostDialogValue::CountedBytes(frame) => frame.payload().len(),
            _ => 0,
        })
        .sum()
}

fn expected_text_budget(body: &SimulationPostDialog) -> usize {
    body.fields()
        .iter()
        .map(|field| match &field.value {
            SimulationPostDialogValue::CharacterText(text) => text.payload().len(),
            SimulationPostDialogValue::PersistText(text) => text.payload().len(),
            _ => 0,
        })
        .max()
        .unwrap_or(0)
}

fn exact_limits(body: &SimulationPostDialog) -> SimulationPostDialogLimits {
    SimulationPostDialogLimits {
        maximum_bytes: body.to_bytes().len(),
        maximum_fields: body.fields().len(),
        maximum_entries: expected_entries(body),
        maximum_text_bytes: expected_text_budget(body),
        ..LIMITS
    }
}

fn one_below(
    mut limits: SimulationPostDialogLimits,
    axis: usize,
) -> Option<SimulationPostDialogLimits> {
    let selected = match axis {
        0 => &mut limits.maximum_bytes,
        1 => &mut limits.maximum_fields,
        2 => &mut limits.maximum_entries,
        _ => &mut limits.maximum_text_bytes,
    };
    if *selected == 0 {
        return None;
    }
    *selected -= 1;
    Some(limits)
}

fn assert_failed_budget(body: &SimulationPostDialog, limits: SimulationPostDialogLimits) {
    let original = body.to_bytes();
    assert!(SimulationPostDialog::from_bytes(&original, body.version(), limits).is_err());
    assert!(
        SimulationPostDialog::from_fields(body.fields().to_vec(), body.version(), limits).is_err()
    );
    assert_eq!(body.to_bytes(), original);
}

#[test]
fn every_owned_parent_obeys_exact_and_one_below_outer_budgets() {
    for body in owned_dialogs() {
        let limits = exact_limits(&body);
        let parsed =
            SimulationPostDialog::from_bytes(&body.to_bytes(), body.version(), limits).unwrap();
        let built =
            SimulationPostDialog::from_fields(body.fields().to_vec(), body.version(), limits)
                .unwrap();
        assert_eq!(parsed.to_bytes(), body.to_bytes());
        assert_eq!(built.to_bytes(), body.to_bytes());
        assert_eq!(parsed.coverage().declared_entries, expected_entries(&body));
        for axis in 0..4 {
            if let Some(reduced) = one_below(limits, axis) {
                assert_failed_budget(&body, reduced);
            }
        }
    }
}
