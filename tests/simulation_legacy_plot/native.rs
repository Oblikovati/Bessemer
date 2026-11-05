use super::*;

fn native_plots() -> Vec<SimulationLegacyPlot> {
    projected_plots(
        &std::fs::read("target/simulation-legacy-plot-native-projection.bin").unwrap(),
        96,
    )
}

fn validate_native_attempt(cursor: &mut &[u8], output: &mut Vec<u8>) -> bool {
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let accepted = read_word(cursor) != 0;
    let dispatch = read_word(cursor) != 0;
    let source_end = read_word(cursor);
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    assert_eq!(dispatch, version.value() < 700335);
    check_native_read(raw, version, accepted, source_end);
    project_word(output, usize::from(accepted));
    project_word(output, usize::from(dispatch));
    accepted
}

fn check_native_read(
    raw: &[u8],
    version: SimulationPersistVersion,
    accepted: bool,
    source_end: u32,
) {
    let result = SimulationLegacyPlot::from_bytes(raw, version, LIMITS);
    assert_eq!(
        result.is_ok(),
        accepted,
        "{}:{}:{source_end}",
        version.value(),
        raw.len()
    );
    match result {
        Ok(plot) => {
            assert_eq!(plot.to_bytes(), raw);
            assert_eq!(source_end as usize, raw.len());
        }
        Err(error) => {
            assert_eq!(error.class, bessemer::FailureClass::Invalid);
            assert_eq!(error.entity, format!("archive at byte {source_end}"));
            assert!(!error.operation.is_empty() && !error.cause.is_empty());
        }
    }
}

fn project_native_attempts() -> Vec<u8> {
    let packet = std::fs::read("target/simulation-legacy-plot-native.bin").unwrap();
    let mut cursor = packet.as_slice();
    let mut output = Vec::new();
    let (mut count, mut accepted) = (0, 0);
    while !cursor.is_empty() {
        accepted += usize::from(validate_native_attempt(&mut cursor, &mut output));
        count += 1;
    }
    assert_eq!((count, accepted), (120, 96));
    output
}

#[test]
#[ignore = "Requires all original native packets from freeze_legacy_plot.py."]
fn all_original_helper_outcomes_and_dispatch_qualifications_match_without_lowering_versions() {
    project_native_attempts();
    native_wide::project_original_wide_views(&mut Vec::new());
    for plot in native_plots() {
        check_read_coverage(&plot);
        let built =
            SimulationLegacyPlot::from_fields(plot.fields().to_vec(), plot.version(), LIMITS)
                .unwrap();
        assert_eq!(built.to_bytes(), plot.to_bytes());
        project_complete_operation(&plot);
    }
}

#[test]
#[ignore = "Requires native packets and explicit BESSEMER_LEGACY_PLOT_PROCESS_OUTPUT for process replay."]
fn export_complete_owned_and_native_operations_for_fresh_process_replay() {
    let mut output = project_native_attempts();
    native_wide::project_original_wide_views(&mut output);
    for plot in owned_plots().into_iter().chain(native_plots()) {
        output.extend(project_complete_operation(&plot));
    }
    let destination = std::env::var("BESSEMER_LEGACY_PLOT_PROCESS_OUTPUT").unwrap();
    std::fs::write(destination, output).unwrap();
}
