//! Complete historical plot operations against literal and frozen source expectations (#174).
use bessemer::simulation::*;
#[allow(
    dead_code,
    reason = "Shared projection and history support also serves embedded archive families."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::*;
#[path = "simulation_legacy_plot/projections.rs"]
mod projections;
use projections::*;
#[path = "simulation_legacy_plot/operations.rs"]
mod operations;
use operations::*;
#[path = "simulation_legacy_plot/budgets.rs"]
mod budgets;
#[path = "simulation_legacy_plot/concurrency.rs"]
mod concurrency;
#[path = "simulation_legacy_plot/contracts.rs"]
mod contracts;
#[path = "simulation_legacy_plot/native.rs"]
mod native;
#[path = "simulation_legacy_plot/native_wide.rs"]
mod native_wide;
#[path = "simulation_legacy_plot/wide_views.rs"]
mod wide_views;

const LIMITS: SimulationLegacyPlotLimits = SimulationLegacyPlotLimits {
    maximum_bytes: 8192,
    maximum_fields: 51,
    maximum_entries: 2048,
    maximum_text_bytes: 1024,
    maximum_characters: 512,
};
const OWNED: &[u8] = include_bytes!("fixtures/simulation-legacy-plot/projection.bin");

fn owned_plots() -> Vec<SimulationLegacyPlot> {
    projected_plots(OWNED, 52)
}

#[test]
fn every_owned_field_matches_independent_kind_span_and_raw_bytes() {
    for plot in owned_plots() {
        let built =
            SimulationLegacyPlot::from_fields(plot.fields().to_vec(), plot.version(), LIMITS)
                .unwrap();
        assert_eq!(built.fields(), plot.fields());
        assert_eq!(built.field_offsets(), plot.field_offsets());
        assert_eq!(built.to_bytes(), plot.to_bytes());
        assert_ne!(built.field_id(0).unwrap(), plot.field_id(0).unwrap());
        check_read_coverage(&plot);
    }
}

#[test]
fn every_proper_owned_prefix_and_appended_byte_fails_without_publishing_partial_values() {
    for plot in owned_plots() {
        let raw = plot.to_bytes();
        for end in 0..raw.len() {
            assert!(
                SimulationLegacyPlot::from_bytes(&raw[..end], plot.version(), LIMITS).is_err(),
                "{}:{end}",
                plot.version().value()
            );
        }
        let mut trailing = raw.clone();
        trailing.push(0);
        assert!(SimulationLegacyPlot::from_bytes(&trailing, plot.version(), LIMITS).is_err());
        assert_eq!(plot.to_bytes(), raw);
    }
}

fn assert_threads<T: Send + Sync>() {}

#[test]
fn every_public_historical_plot_type_is_send_sync() {
    assert_threads::<SimulationLegacyPlot>();
    assert_threads::<SimulationLegacyPlotField>();
    assert_threads::<SimulationLegacyPlotValue>();
    assert_threads::<SimulationLegacyPlotText>();
    assert_threads::<SimulationLegacyPlotLimits>();
    assert_threads::<SimulationLegacyPlotCoverage>();
    assert_threads::<SimulationLegacyPlotEdit>();
    assert_threads::<SimulationLegacyPlotFieldId>();
    assert_threads::<SimulationLegacyPlotCheckMode>();
    assert_threads::<SimulationLegacyPlotCompletion>();
    assert_threads::<SimulationLegacyPlotDiagnostic>();
    assert_threads::<SimulationLegacyPlotHistory>();
    assert_threads::<SimulationLegacyPlotHistoryParent>();
    assert_threads::<SimulationLegacyPlotRelation>();
    assert_threads::<SimulationLegacyPlotWideMember>();
    assert_threads::<SimulationLegacyPlotWideView>();
}
