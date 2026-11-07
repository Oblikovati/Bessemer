use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn assert_position_sources(
    view: &SimulationLegendPositionFactors,
    paths: &[&str],
    spans: &[(usize, usize)],
    identity: impl Fn(usize) -> SimulationThermalFieldId,
) {
    for factor in [view.horizontal(), view.vertical()].into_iter().flatten() {
        let ordinal = factor.source_id().ordinal();
        assert_eq!(factor.source_id(), &identity(ordinal));
        assert_eq!(paths[ordinal], factor.axis().field_path());
        assert_eq!(spans[ordinal], factor.wire_range());
        let history = factor.history();
        assert_eq!(history.result, identity(ordinal));
        assert_eq!(history.relation, SimulationThermalRelation::Copied);
        assert!(
            matches!(&history.parents[..], [SimulationThermalHistoryParent::Source(id)] if id == factor.source_id())
        );
    }
}

fn parent_position_view(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> SimulationLegendPositionFactors {
    let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
    let view = parent.legend_position_factors().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_position_factors().unwrap(), view);
    let paths: Vec<_> = parent
        .fields()
        .iter()
        .map(|field| field.path.as_ref())
        .collect();
    assert_position_sources(&view, &paths, parent.field_offsets(), |index| {
        parent.field_id(index).unwrap()
    });
    view
}

fn style_position_view(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> SimulationLegendPositionFactors {
    let style = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let view = style.legend_position_factors().unwrap();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.legend_position_factors().unwrap(), view);
    let paths: Vec<_> = style
        .fields()
        .iter()
        .map(|field| field.path.as_ref())
        .collect();
    assert_position_sources(&view, &paths, style.field_offsets(), |index| {
        style.field_id(index).unwrap()
    });
    view
}

fn archive_position_view(family: u32, version: u32, raw: &[u8]) -> SimulationLegendPositionFactors {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        return parent_position_view(raw, version);
    }
    assert_eq!(family, 2);
    style_position_view(raw, version)
}

fn position_factor_packet(factor: Option<&SimulationLegendPositionFactor>) -> Vec<u8> {
    let Some(factor) = factor else {
        return vec![0; 28];
    };
    let finite = factor.finite_value();
    let (start, end) = factor.wire_range();
    assert_eq!(end - start, 4);
    let words = [
        1,
        factor.raw_bits(),
        start as u32,
        end as u32,
        u32::from(finite.is_some()),
        finite.map_or(0, f32::to_bits),
        factor.source_id().ordinal() as u32,
    ];
    words.into_iter().flat_map(u32::to_le_bytes).collect()
}

fn assert_position_coverage(view: &SimulationLegendPositionFactors) {
    let count = usize::from(view.horizontal().is_some()) + usize::from(view.vertical().is_some());
    let coverage = view.coverage();
    assert_eq!(
        coverage.check_mode,
        SimulationThermalCheckMode::ExactSelectedProfiles
    );
    assert_eq!(
        (coverage.present_factors, coverage.checked_bytes),
        (count, count * 4)
    );
    assert!(!coverage.normalized && !coverage.wire_edited);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    let completion = if count == 0 {
        SimulationLegendPositionCompletion::CompletedAbsent
    } else {
        SimulationLegendPositionCompletion::CompletedFactors
    };
    assert_eq!(view.completion(), completion);
    assert_position_diagnostics(view);
}

fn assert_position_diagnostics(view: &SimulationLegendPositionFactors) {
    assert_eq!(
        view.diagnostics(),
        [
            SimulationLegendPositionDiagnostic::NativeNotExecuted,
            SimulationLegendPositionDiagnostic::PixelConsumersNotModeled,
            SimulationLegendPositionDiagnostic::RuntimeDimensionsNotInferred,
            SimulationLegendPositionDiagnostic::ScreenOriginNotInferred
        ]
    );
}

fn replay_position_parent(cursor: &mut &[u8]) -> Vec<u8> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let view = archive_position_view(family, version, &raw);
    let original = view.clone();
    assert_position_coverage(&view);
    let mut actual = position_factor_packet(view.horizontal());
    actual.extend(position_factor_packet(view.vertical()));
    assert_eq!(
        actual,
        take_projection(cursor, 56),
        "family {family} version {version}"
    );
    raw.fill(0xff);
    assert_eq!(view, original);
    assert_eq!(position_factor_packet(view.horizontal()), &actual[..28]);
    assert_eq!(position_factor_packet(view.vertical()), &actual[28..]);
    actual
}

pub fn replay_position_projection(mut cursor: &[u8], expected_count: u32) -> Vec<u8> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), expected_count);
    let mut output = Vec::new();
    for _ in 0..expected_count {
        output.extend(replay_position_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    output
}

pub fn replay_position_parallel(packet: &[u8], count: u32) -> Vec<u8> {
    let serial = replay_position_projection(packet, count);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| replay_position_projection(packet, count)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    serial
}
