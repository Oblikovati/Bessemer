use super::*;

fn complete_components() -> [ColorComponent<'static>; 4] {
    [
        ("style.color_1f4.component_0", (3, 7), Some(0)),
        ("style.color_1f4.component_1", (7, 11), Some(1)),
        ("style.color_1f4.component_2", (11, 15), Some(0x80000000)),
        ("style.color_1f4.component_3", (15, 19), Some(0x7fc00042)),
    ]
}

#[test]
fn complete_colors_allow_unaligned_spans_and_empty_results_stay_complete() {
    let colors = collect_colors(complete_components().into_iter()).unwrap();
    assert_eq!(colors.len(), 1);
    assert_eq!(colors[0].wire_range, (3, 19));
    assert_eq!(colors[0].color.bits(), [0, 1, 0x80000000, 0x7fc00042]);
    assert!(collect_colors(std::iter::empty()).unwrap().is_empty());
}

#[test]
fn incomplete_groups_fail_without_publishing_an_earlier_complete_color() {
    for length in 1..4 {
        let mut fields = complete_components().to_vec();
        fields.extend_from_slice(&complete_components()[..length]);
        assert!(collect_colors(fields.into_iter()).is_err());
    }
}

#[test]
fn every_component_requires_the_exact_path_span_and_scalar_kind() {
    for index in 0..4 {
        let mut components = complete_components();
        components[index].2 = None;
        assert!(collect_colors(components.into_iter()).is_err());
        let mut components = complete_components();
        components[index].1.1 += 1;
        assert!(collect_colors(components.into_iter()).is_err());
    }
    let mut components = complete_components();
    components[2].0 = "style.other.component_2";
    assert!(collect_colors(components.into_iter()).is_err());
}

#[test]
fn mismatched_field_and_span_counts_fail_before_zipping() {
    let field = SimulationThermalField {
        path: "style.color_1f4.component_0".into(),
        value: SimulationThermalValue::Unsigned(0),
    };
    let error = collect_wire_colors(&[field], &[]).unwrap_err();
    assert!(error.cause.contains("1 fields and 0 spans"));
    assert!(error.cause.contains("expected equal counts"));
}
