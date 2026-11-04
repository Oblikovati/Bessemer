//! Complete bounded C889 plans against independent caller/pointer/alpha fixtures (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(
    dead_code,
    reason = "Shared support also serves field/history packets."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};
#[path = "support/triangle_color_plans.rs"]
mod plan_support;
const PLANS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/triangle-color-plans.bin");
const VIEWS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/triangle-color-projection.bin");
const PARENTS: SimulationPostDialogLimits = SimulationPostDialogLimits {
    maximum_bytes: 10000000,
    maximum_fields: 200000,
    maximum_entries: 10000000,
    maximum_text_bytes: 0x200000,
    embedded: bessemer::model_archives::ModelArchiveLimits {
        maximum_bytes: 1000000,
        maximum_fields: 100000,
        maximum_entries: 1000000,
        maximum_depth: 128,
        maximum_objects: 100000,
    },
};
const STYLES: SimulationPostStyleLimits = SimulationPostStyleLimits {
    maximum_bytes: 4096,
    maximum_fields: 116,
    maximum_entries: 512,
    maximum_text_bytes: 1024,
    maximum_characters: 512,
};

fn stored_triangle_controls(
    family: u32,
    version: u32,
    raw: &[u8],
) -> [bessemer::Result<SimulationTriangleColorControls>; 2] {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let views = [
            parent.hidden_parts_triangle_controls(),
            parent.model_tessellation_triangle_controls(),
        ];
        assert_eq!(parent.to_bytes(), raw);
        return views;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let views = [
        style.hidden_parts_triangle_controls(),
        style.model_tessellation_triangle_controls(),
    ];
    assert_eq!(style.to_bytes(), raw);
    views
}

fn triangle_control_words(controls: SimulationTriangleColorControls) -> Vec<u32> {
    match controls {
        SimulationTriangleColorControls::HiddenParts {
            selector_raw,
            color,
        } => [vec![selector_raw as u32], color.bits().to_vec()].concat(),
        SimulationTriangleColorControls::ModelTessellation {
            override_raw,
            selector_raw,
            color,
        } => [
            vec![override_raw as u32, selector_raw as u32],
            color.bits().to_vec(),
        ]
        .concat(),
    }
}

fn replay_triangle_view(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let length = read_word(cursor) as usize;
    let present = read_word(cursor);
    let raw = take_projection(cursor, length);
    let controls = stored_triangle_controls(family, version, raw);
    assert_eq!(controls, stored_triangle_controls(family, version, raw));
    assert_triangle_view(cursor, present, controls)
}

fn assert_triangle_view(
    cursor: &mut &[u8],
    present: u32,
    controls: [bessemer::Result<SimulationTriangleColorControls>; 2],
) -> Vec<u32> {
    if present == 0 {
        for result in controls {
            assert_eq!(result.unwrap_err().class, FailureClass::Missing);
        }
        return vec![0; 12];
    }
    assert_eq!(present, 1);
    let actual: Vec<u32> = controls
        .into_iter()
        .flat_map(|control| triangle_control_words(control.unwrap()))
        .collect();
    let expected: [u32; 11] = std::array::from_fn(|_| read_word(cursor));
    assert_eq!(actual, expected);
    [vec![1], actual].concat()
}

fn replay_triangle_views(mut cursor: &[u8], count: u32, present: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let rows: Vec<Vec<u32>> = (0..count)
        .map(|_| replay_triangle_view(&mut cursor))
        .collect();
    assert!(cursor.is_empty());
    assert_eq!(rows.iter().filter(|row| row[0] == 1).count(), present);
    rows.concat()
}

#[test]
fn complete_caller_group_and_alpha_matrix_matches_independent_pointer_oracle() {
    assert!(!plan_support::replay_triangle_color_cases(PLANS).is_empty());
}

#[test]
fn every_owned_parent_style_context_retains_presence_raw_controls_and_source_bytes() {
    assert_eq!(replay_triangle_views(VIEWS, 425, 405).len(), 425 * 12);
}

fn find_triangle_origin(
    origin: SimulationTriangleColorOrigin,
) -> plan_support::TriangleColorFixtureCase {
    let mut cursor = PLANS;
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    for _ in 0..count {
        let case = plan_support::read_triangle_color_case(&mut cursor);
        let header: [u32; 9] = std::array::from_fn(|_| read_word(&mut cursor));
        take_projection(&mut cursor, header[8] as usize * 44);
        let result =
            case.controls
                .plan_masked_daz_ftz_off(case.runtime, &case.groups, case.maximum);
        if result.is_ok_and(|plan| plan.calls().iter().any(|call| call.origin == origin)) {
            return case;
        }
    }
    panic!("fixture lacks color origin {origin:?}")
}

fn assert_triangle_isolation(origin: SimulationTriangleColorOrigin) {
    let mut case = find_triangle_origin(origin);
    let plan = case
        .controls
        .plan_masked_daz_ftz_off(case.runtime, &case.groups, case.maximum)
        .unwrap();
    let original = plan.clone();
    mutate_triangle_groups(&mut case.groups);
    case.runtime.group_count_raw = 0;
    case.controls = SimulationTriangleColorControls::HiddenParts {
        selector_raw: -1,
        color: SimulationRgbaColor::from_bits([0; 4]),
    };
    assert_eq!(plan, original);
    assert_ne!(plan.source_groups(), case.groups);
    assert_ne!(plan.source_controls(), case.controls);
    assert_ne!(plan.runtime_context(), case.runtime);
}

fn mutate_triangle_groups(groups: &mut [SimulationTriangleColorGroup]) {
    for group in groups {
        group.class_raw = 0;
        group.triangle_count = -1;
        group.payload_tokens = [0; 2];
        group.color_c = SimulationRgbaColor::from_bits([0; 4]);
        group.color_1c = group.color_c;
        group.lookup_matches_end = None;
    }
}

#[test]
fn all_three_color_origins_own_their_controls_groups_and_payload_provenance() {
    for origin in [
        SimulationTriangleColorOrigin::Supplied,
        SimulationTriangleColorOrigin::GroupC,
        SimulationTriangleColorOrigin::Group1C,
    ] {
        assert_triangle_isolation(origin);
    }
}

#[test]
fn mismatched_typed_callers_fail_before_publication() {
    for hidden in [true, false] {
        let mut case = find_triangle_origin(SimulationTriangleColorOrigin::GroupC);
        let color = SimulationRgbaColor::from_bits([0; 4]);
        case.controls = if hidden {
            SimulationTriangleColorControls::HiddenParts {
                selector_raw: 1,
                color,
            }
        } else {
            SimulationTriangleColorControls::ModelTessellation {
                override_raw: 1,
                selector_raw: 1,
                color,
            }
        };
        assert_triangle_mismatch(case);
    }
}

fn assert_triangle_mismatch(mut case: plan_support::TriangleColorFixtureCase) {
    case.runtime.caller = opposite_triangle_caller(case.controls);
    let error = case
        .controls
        .plan_masked_daz_ftz_off(case.runtime, &case.groups, case.maximum)
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert_eq!(error.entity, "caller kind");
    assert!(error.cause.contains("expected matching"));
}

fn opposite_triangle_caller(
    controls: SimulationTriangleColorControls,
) -> SimulationTriangleColorCaller {
    match controls {
        SimulationTriangleColorControls::HiddenParts { .. } => {
            SimulationTriangleColorCaller::ModelTessellation {
                renderer_type_code: 5,
                precheck_result: None,
            }
        }
        SimulationTriangleColorControls::ModelTessellation { .. } => {
            SimulationTriangleColorCaller::HiddenParts {
                option_result: Some(0),
                core_678_raw: None,
            }
        }
    }
}

#[test]
fn eight_workers_and_public_thread_traits_match_serial_results() {
    assert_triangle_thread_traits();
    let expected = plan_support::replay_triangle_color_cases(PLANS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| plan_support::replay_triangle_color_cases(PLANS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}

fn assert_triangle_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationTriangleColorControls>();
    require::<SimulationTriangleColorCaller>();
    require::<SimulationTriangleColorRuntime>();
    require::<SimulationTriangleColorGroup>();
    require::<SimulationTriangleColorCall>();
    require::<SimulationTriangleColorOrigin>();
    require::<SimulationTriangleColorStage>();
    require::<SimulationTriangleColorCompletion>();
    require::<SimulationTriangleColorCoverage>();
    require::<SimulationTriangleColorDiagnostic>();
    require::<SimulationTriangleColorPlan>();
}

#[test]
#[ignore = "Requires complete freshly pinned native projection bytes."]
fn every_native_parent_style_context_preserves_original_selector_and_rgba_bits() {
    replay_triangle_views(
        &std::fs::read("target/simulation-triangle-color-projection-native.bin").unwrap(),
        276,
        276,
    );
}

#[test]
#[ignore = "Writes deterministic output for independent process comparison."]
fn export_triangle_color_results_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-triangle-color-projection-native.bin").unwrap();
    let output = [
        plan_support::replay_triangle_color_cases(PLANS),
        replay_triangle_views(VIEWS, 425, 405),
        replay_triangle_views(&native, 276, 276),
    ]
    .concat();
    if let Some(path) = std::env::var_os("BESSEMER_TRIANGLE_COLOR_OUTPUT") {
        std::fs::write(
            path,
            output
                .into_iter()
                .flat_map(u32::to_le_bytes)
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }
}
