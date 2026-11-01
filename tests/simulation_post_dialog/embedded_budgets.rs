use super::*;

fn exact_embedded_limits(body: &SimulationPostDialog) -> SimulationPostDialogLimits {
    let mut limits = LIMITS;
    limits.embedded.maximum_bytes = 0;
    limits.embedded.maximum_fields = 0;
    limits.embedded.maximum_objects = 0;
    for field in body.fields() {
        let SimulationPostDialogValue::Embedded(archive) = &field.value else {
            continue;
        };
        limits.embedded.maximum_bytes = limits
            .embedded
            .maximum_bytes
            .max(archive.to_bytes().unwrap().len());
        limits.embedded.maximum_fields = limits.embedded.maximum_fields.max(archive.fields().len());
        limits.embedded.maximum_objects = limits
            .embedded
            .maximum_objects
            .max(archive.coverage().new_objects);
    }
    limits
}

fn reduced_embedded(
    mut limits: SimulationPostDialogLimits,
    axis: usize,
) -> Option<SimulationPostDialogLimits> {
    let selected = match axis {
        0 => &mut limits.embedded.maximum_bytes,
        1 => &mut limits.embedded.maximum_fields,
        _ => &mut limits.embedded.maximum_objects,
    };
    if *selected == 0 {
        return None;
    }
    *selected -= 1;
    Some(limits)
}

fn assert_embedded_budget(
    body: &SimulationPostDialog,
    limits: SimulationPostDialogLimits,
    accepted: bool,
) {
    let bytes = body.to_bytes();
    let parsed = SimulationPostDialog::from_bytes(&bytes, body.version(), limits);
    assert_eq!(parsed.is_ok(), accepted, "{:?}", body.version());
    let built = SimulationPostDialog::from_fields(body.fields().to_vec(), body.version(), limits);
    assert_eq!(built.is_ok(), accepted, "{:?}", body.version());
    if let Ok(parsed) = parsed {
        assert_eq!(parsed.to_bytes(), bytes);
    }
    assert_eq!(body.to_bytes(), bytes);
}

#[test]
fn complete_parent_embedded_roots_obey_independent_byte_field_and_object_budgets() {
    for body in owned_dialogs() {
        let exact = exact_embedded_limits(&body);
        assert_embedded_budget(&body, exact, true);
        for axis in 0..3 {
            if let Some(reduced) = reduced_embedded(exact, axis) {
                assert_embedded_budget(&body, reduced, false);
            }
        }
    }
}
