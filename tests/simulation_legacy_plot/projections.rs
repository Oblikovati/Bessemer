use super::*;

pub(super) fn encoded_legacy_value(value: &SimulationLegacyPlotValue) -> (usize, Vec<u8>) {
    match value {
        SimulationLegacyPlotValue::RawWordBits(bits) => (7, bits.to_le_bytes().to_vec()),
        SimulationLegacyPlotValue::RawQwordBits(bits) => (9, bits.to_le_bytes().to_vec()),
        SimulationLegacyPlotValue::RawBytes(bytes) => (4, bytes.to_vec()),
        SimulationLegacyPlotValue::Text(text) => (6, text.to_bytes()),
    }
}

pub(super) fn project_plot(plot: &SimulationLegacyPlot) -> Vec<u8> {
    let mut output = Vec::new();
    project_word(&mut output, plot.fields().len());
    for (field, &(start, end)) in plot.fields().iter().zip(plot.field_offsets()) {
        let (kind, raw) = encoded_legacy_value(&field.value);
        project_word(&mut output, field.path.len());
        output.extend_from_slice(field.path.as_bytes());
        project_word(&mut output, kind);
        project_word(&mut output, start);
        project_word(&mut output, end);
        assert_eq!(raw.len(), end - start);
        output.extend(raw);
    }
    output
}

fn read_projected_plot(cursor: &mut &[u8]) -> SimulationLegacyPlot {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    let expected = *cursor;
    consume_expected_fields(cursor);
    let plot = SimulationLegacyPlot::from_bytes(raw, version, LIMITS).unwrap();
    assert_eq!(plot.to_bytes(), raw);
    assert_eq!(
        project_plot(&plot),
        expected[..expected.len() - cursor.len()]
    );
    plot
}

pub(super) fn projected_plots(mut cursor: &[u8], expected: usize) -> Vec<SimulationLegacyPlot> {
    let mut plots = Vec::new();
    while !cursor.is_empty() {
        plots.push(read_projected_plot(&mut cursor));
    }
    assert_eq!(plots.len(), expected);
    plots
}

pub(super) fn check_read_coverage(plot: &SimulationLegacyPlot) {
    let coverage = plot.coverage();
    check_read_status(coverage);
    check_read_context(plot, coverage);
    check_read_diagnostics(coverage);
}

fn check_read_status(coverage: SimulationLegacyPlotCoverage) {
    assert_eq!(
        coverage.completion,
        SimulationLegacyPlotCompletion::CompletedBody
    );
    assert_eq!(
        coverage.check_mode,
        SimulationLegacyPlotCheckMode::ExactSelectedProfiles
    );
    assert_eq!(coverage.profile_character_cap, 512);
    assert_eq!(coverage.requested_pattern_error_bits, 0);
}

fn check_read_context(plot: &SimulationLegacyPlot, coverage: SimulationLegacyPlotCoverage) {
    assert_eq!(coverage.checked_bytes, plot.to_bytes().len());
    assert_eq!(coverage.checked_fields, plot.fields().len());
    assert_eq!(coverage.version, plot.version());
    assert_eq!(
        coverage.caller_version_gate_enabled,
        plot.version().value() < 700335
    );
    assert_eq!(
        plot.named_stream_dispatch_enabled(),
        coverage.caller_version_gate_enabled
    );
    let characters: usize = plot
        .fields()
        .iter()
        .map(|field| match &field.value {
            SimulationLegacyPlotValue::Text(text) => text.character_count(),
            _ => 0,
        })
        .sum();
    assert_eq!(coverage.declared_entries, characters);
}

fn check_read_diagnostics(coverage: SimulationLegacyPlotCoverage) {
    assert_eq!(
        coverage.diagnostics,
        [
            SimulationLegacyPlotDiagnostic::NativeApplicationNotExecuted,
            SimulationLegacyPlotDiagnostic::RuntimeVersionNotInferred,
            SimulationLegacyPlotDiagnostic::PhysicalSemanticsUnassigned,
            SimulationLegacyPlotDiagnostic::GeometryNotChecked
        ]
    );
}
