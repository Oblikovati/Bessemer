use super::*;

const MEMBERS: [(SimulationLegacyPlotWideMember, usize, usize); 5] = [
    (SimulationLegacyPlotWideMember::Buffer30, 40, 128),
    (SimulationLegacyPlotWideMember::Buffer13C, 308, 128),
    (SimulationLegacyPlotWideMember::Buffer23C, 564, 128),
    (SimulationLegacyPlotWideMember::Buffer33C, 820, 128),
    (SimulationLegacyPlotWideMember::Buffer454, 1100, 64),
];

fn check_literal_wide_view(
    plot: &SimulationLegacyPlot,
    member: SimulationLegacyPlotWideMember,
    offset: usize,
    limit: usize,
    variant: usize,
) {
    let view = plot.fixed_wide_view(member).unwrap();
    if plot.version().value() < 700004 {
        assert!(view.is_none());
        return;
    }
    let view = view.unwrap();
    let expected_units = [limit, 0, limit - 1, limit / 2][variant];
    let raw = plot.to_bytes();
    assert_eq!(view.member(), member);
    assert_eq!(member.code_unit_limit(), limit);
    assert_eq!(view.source_offsets(), (offset, offset + limit * 2));
    assert_eq!(view.raw_bytes(), &raw[offset..offset + limit * 2]);
    assert_eq!(view.prefix_code_unit_count(), expected_units);
    assert_eq!(view.terminator_present(), variant != 0);
    check_raw_wide_partition(&view, expected_units);
}

fn check_raw_wide_partition(view: &SimulationLegacyPlotWideView, expected_units: usize) {
    assert_eq!(view.prefix_bytes(), &view.raw_bytes()[..expected_units * 2]);
    assert_eq!(
        view.remainder_bytes(),
        &view.raw_bytes()[expected_units * 2..]
    );
    assert_eq!(
        [view.prefix_bytes(), view.remainder_bytes()].concat(),
        view.raw_bytes()
    );
}

#[test]
fn every_fixed_buffer_keeps_raw_surrogates_unterminated_boundaries_and_nul_suffixes_with_the_historical_guard()
 {
    for (index, plot) in owned_plots().iter().enumerate() {
        for (member, offset, limit) in MEMBERS {
            check_literal_wide_view(plot, member, offset, limit, index % 4);
        }
    }
}

#[test]
fn wide_views_own_the_full_buffer_after_the_archive_and_caller_bytes_are_dropped() {
    let plot = owned_plots()
        .into_iter()
        .find(|plot| plot.version().value() == 700004)
        .unwrap();
    let view = plot
        .fixed_wide_view(SimulationLegacyPlotWideMember::Buffer30)
        .unwrap()
        .unwrap();
    let copied = view.clone();
    let expected = plot.to_bytes()[40..296].to_vec();
    drop(plot);
    assert_eq!(view, copied);
    assert_eq!(view.raw_bytes(), expected);
    assert_eq!(view.prefix_bytes(), expected);
    assert!(!view.terminator_present());
}
