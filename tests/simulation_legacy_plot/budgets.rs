use super::*;

fn exact_legacy_limits(plot: &SimulationLegacyPlot) -> SimulationLegacyPlotLimits {
    let characters = plot
        .fields()
        .iter()
        .map(|field| match &field.value {
            SimulationLegacyPlotValue::Text(text) => text.character_count(),
            _ => 0,
        })
        .max()
        .unwrap_or(0);
    SimulationLegacyPlotLimits {
        maximum_bytes: plot.to_bytes().len(),
        maximum_fields: plot.fields().len(),
        maximum_entries: plot.coverage().declared_entries,
        maximum_text_bytes: characters * 2,
        maximum_characters: characters,
    }
}

fn reduced_legacy_limits(exact: SimulationLegacyPlotLimits) -> Vec<SimulationLegacyPlotLimits> {
    (0..5)
        .filter_map(|axis| reduced_legacy_axis(exact, axis))
        .collect()
}

fn reduced_legacy_axis(
    mut limits: SimulationLegacyPlotLimits,
    axis: u8,
) -> Option<SimulationLegacyPlotLimits> {
    let bound = match axis {
        0 => &mut limits.maximum_bytes,
        1 => &mut limits.maximum_fields,
        2 => &mut limits.maximum_entries,
        3 => &mut limits.maximum_text_bytes,
        _ => &mut limits.maximum_characters,
    };
    *bound = bound.checked_sub(1)?;
    Some(limits)
}

#[test]
fn every_budget_axis_accepts_exact_and_rejects_one_below_without_counting_fixed_buffers_as_text() {
    for plot in owned_plots() {
        let exact = exact_legacy_limits(&plot);
        let raw = plot.to_bytes();
        let read = SimulationLegacyPlot::from_bytes(&raw, plot.version(), exact).unwrap();
        assert_eq!(read.to_bytes(), raw);
        assert_eq!(
            read.coverage().limits.maximum_entries,
            exact.maximum_entries
        );
        SimulationLegacyPlot::from_fields(plot.fields().to_vec(), plot.version(), exact).unwrap();
        for limits in reduced_legacy_limits(exact) {
            assert!(SimulationLegacyPlot::from_bytes(&raw, plot.version(), limits).is_err());
            assert!(
                SimulationLegacyPlot::from_fields(plot.fields().to_vec(), plot.version(), limits)
                    .is_err()
            );
        }
    }
}

#[test]
fn signed_counts_and_the_documented_512_character_profile_remain_strict_under_larger_caller_budgets()
 {
    let version = SimulationPersistVersion::new(700003).unwrap();
    let limits = SimulationLegacyPlotLimits {
        maximum_text_bytes: 4096,
        maximum_characters: 4096,
        ..LIMITS
    };
    for count in [i32::MIN, -1, 513, i32::MAX] {
        let mut raw = vec![0; 1344];
        raw.extend_from_slice(&count.to_le_bytes());
        raw.extend_from_slice(&[0; 1026]);
        let error = SimulationLegacyPlot::from_bytes(&raw, version, limits).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.entity.contains("text_550"));
        assert!(error.cause.contains(&count.to_string()));
        assert!(error.cause.contains("512"));
    }
    assert!(SimulationLegacyPlotText::from_payload(&[1], 2).is_err());
    assert!(SimulationLegacyPlotText::from_payload(&[0, 0xd8], 1).is_err());
}

#[test]
fn changing_fixed_buffer_width_or_exceeding_text_profile_never_publishes_an_edit() {
    for plot in owned_plots() {
        let before = plot.to_bytes();
        for (ordinal, field) in plot.fields().iter().enumerate() {
            let invalid = match &field.value {
                SimulationLegacyPlotValue::RawBytes(raw) => {
                    SimulationLegacyPlotValue::RawBytes(raw[..raw.len() - 1].into())
                }
                SimulationLegacyPlotValue::Text(_) => SimulationLegacyPlotValue::Text(
                    SimulationLegacyPlotText::from_payload(&[0; 1026], 1026).unwrap(),
                ),
                _ => continue,
            };
            assert!(
                plot.edit_field(&plot.field_id(ordinal).unwrap(), invalid)
                    .is_err()
            );
        }
        assert_eq!(plot.to_bytes(), before);
    }
}
