//! C857 explicit reset events remain separate from absent wire words (#166).
use bessemer::simulation::*;
#[path = "support/legend_resets_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_resets_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-resets-operations.bin");
const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/legend-resets-projection.bin");

#[test]
fn native_member_order_distinguishes_three_two_and_no_zero_stores() {
    for (version, offsets) in [
        (700006, vec![0xc0, 0xc4, 0xc8]),
        (840007, vec![0xc0, 0xc4, 0xc8]),
        (840008, vec![0xc4, 0xc8]),
        (840009, vec![0xc4, 0xc8]),
        (840010, vec![]),
        (i32::MAX, vec![]),
    ] {
        assert_native_member_offsets(version, &offsets);
    }
}

fn assert_native_member_offsets(version: i32, offsets: &[u16]) {
    let plan = SimulationPersistVersion::new(version)
        .unwrap()
        .legend_read_zero_assignments()
        .unwrap();
    let actual: Vec<_> = plan
        .assignments()
        .iter()
        .map(|value| value.member.member_offset())
        .collect();
    assert_eq!(actual, offsets);
    assert!(plan.assignments().iter().all(|value| value.value_raw == 0));
}

#[test]
fn raw_class_is_unsupported_instead_of_completed_empty() {
    for version in [0, 700005] {
        assert_eq!(
            case_support::reset_result(SimulationPersistVersion::new(version).unwrap())[0],
            10
        );
    }
    assert_eq!(
        case_support::reset_result(SimulationPersistVersion::new(840010).unwrap())[0],
        1
    );
}

#[test]
fn copied_assignment_mutation_does_not_change_published_plan() {
    let version = SimulationPersistVersion::new(700006).unwrap();
    let plan = version.legend_read_zero_assignments().unwrap();
    let mut copied = plan.assignments().to_vec();
    copied[0].value_raw = -1;
    assert_eq!(plan.assignments()[0].value_raw, 0);
    assert_eq!(plan, version.legend_read_zero_assignments().unwrap());
}

#[test]
fn every_public_reset_type_is_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendResetMember>();
    require::<SimulationLegendResetAssignment>();
    require::<SimulationLegendResetCompletion>();
    require::<SimulationLegendResetHistory>();
    require::<SimulationLegendResetCoverage>();
    require::<SimulationLegendResetDiagnostic>();
    require::<SimulationLegendResetPlan>();
}

#[test]
fn frozen_source_cases_and_complete_owned_wire_contexts_match() {
    assert_eq!(case_support::replay_reset_cases(OPERATIONS).len(), 41370);
    assert_eq!(
        projection_support::replay_reset_projection(OWNED, 557).len(),
        7798
    );
}

#[test]
fn eight_workers_reproduce_serial_reset_results() {
    let serial = projection_support::replay_reset_projection(OWNED, 557);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    assert_eq!(case_support::replay_reset_cases(OPERATIONS).len(), 41370);
                    projection_support::replay_reset_projection(OWNED, 557)
                })
            })
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
#[ignore = "Requires freshly frozen native reference packets."]
fn native_reset_contexts_and_process_export() {
    let native = std::fs::read("target/simulation-legend-resets-projection-native.bin").unwrap();
    let mut words = case_support::replay_reset_cases(OPERATIONS);
    words.extend(projection_support::replay_reset_projection(OWNED, 557));
    words.extend(projection_support::replay_reset_projection(&native, 276));
    assert_eq!(words.len(), 53032);
    if let Some(path) = std::env::var_os("BESSEMER_LEGEND_RESETS_OUTPUT") {
        let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
        std::fs::write(path, bytes).unwrap();
    }
}
