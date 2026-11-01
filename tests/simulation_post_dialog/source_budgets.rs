use super::*;

fn source_limits(metrics: &[usize; 9]) -> SimulationPostDialogLimits {
    SimulationPostDialogLimits {
        maximum_bytes: metrics[0],
        maximum_fields: metrics[1],
        maximum_entries: metrics[2],
        maximum_text_bytes: metrics[3],
        embedded: bessemer::model_archives::ModelArchiveLimits {
            maximum_bytes: metrics[4],
            maximum_fields: metrics[5],
            maximum_objects: metrics[6],
            ..LIMITS.embedded
        },
    }
}

fn lower_axis(
    mut limits: SimulationPostDialogLimits,
    axis: usize,
) -> Option<SimulationPostDialogLimits> {
    let selected = match axis {
        0 => &mut limits.maximum_bytes,
        1 => &mut limits.maximum_fields,
        2 => &mut limits.maximum_entries,
        3 => &mut limits.maximum_text_bytes,
        4 => &mut limits.embedded.maximum_bytes,
        5 => &mut limits.embedded.maximum_fields,
        _ => &mut limits.embedded.maximum_objects,
    };
    if *selected == 0 {
        return None;
    }
    *selected -= 1;
    Some(limits)
}

fn assert_source_case(body: &SimulationPostDialog, metrics: &[usize; 9]) {
    let exact = source_limits(metrics);
    let parsed = SimulationPostDialog::from_bytes(&body.to_bytes(), body.version(), exact).unwrap();
    let built =
        SimulationPostDialog::from_fields(body.fields().to_vec(), body.version(), exact).unwrap();
    assert_eq!(parsed.to_bytes(), body.to_bytes());
    assert_eq!(built.to_bytes(), body.to_bytes());
    assert_eq!(parsed.coverage().embedded_archives, metrics[7]);
    assert_eq!(parsed.coverage().embedded_fields, metrics[8]);
    assert_eq!(parsed.coverage().declared_entries, metrics[2]);
    for axis in 0..7 {
        if let Some(reduced) = lower_axis(exact, axis) {
            assert_reduced_source_budget(body, reduced, axis);
        }
    }
}

#[test]
fn all_owned_budget_axes_and_nested_coverage_match_independent_source_denominators() {
    let mut cursor = include_bytes!("../fixtures/simulation-post-dialog/resources.bin").as_slice();
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 225);
    for body in owned_dialogs() {
        assert_eq!(read_word(&mut cursor) as i32, body.version().value());
        let mut metrics = [0; 9];
        for value in &mut metrics {
            *value = read_word(&mut cursor) as usize;
        }
        assert_source_case(&body, &metrics);
    }
    assert!(cursor.is_empty());
}

fn assert_reduced_source_budget(
    body: &SimulationPostDialog,
    reduced: SimulationPostDialogLimits,
    axis: usize,
) {
    assert!(
        SimulationPostDialog::from_bytes(&body.to_bytes(), body.version(), reduced).is_err(),
        "{axis}"
    );
    assert!(
        SimulationPostDialog::from_fields(body.fields().to_vec(), body.version(), reduced).is_err(),
        "{axis}"
    );
}
