//! Complete C-888 local caller/helper behavior with explicit geometry/native exclusions (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(
    dead_code,
    reason = "Shared support also serves complete field/history packets."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};
#[path = "support/aux_triangle_plans.rs"]
mod plan_support;

const REQUESTS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/aux-triangle-request.bin");
const VIEWS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/aux-triangle-projection.bin");
const PLANS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/aux-triangle-plans.bin");
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

fn read_controls(cursor: &mut &[u8]) -> SimulationAuxiliaryTriangleControls {
    SimulationAuxiliaryTriangleControls {
        enable_raw: read_word(cursor) as i32,
        group_selector_raw: read_word(cursor) as i32,
        color: SimulationRgbaColor::from_bits(std::array::from_fn(|_| read_word(cursor))),
    }
}

fn request_flags(controls: SimulationAuxiliaryTriangleControls) -> [u32; 2] {
    [
        u32::from(controls.helper_requested_masked_exceptions()),
        u32::from(controls.per_group_rgb_requested()),
    ]
}

fn replay_requests(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 86632);
    let mut output = Vec::new();
    for _ in 0..86632 {
        let enable_raw = read_word(&mut cursor) as i32;
        let group_selector_raw = read_word(&mut cursor) as i32;
        let alpha = read_word(&mut cursor);
        let controls = SimulationAuxiliaryTriangleControls {
            enable_raw,
            group_selector_raw,
            color: SimulationRgbaColor::from_bits([0x7fa00042, 0x80000000, u32::MAX, alpha]),
        };
        output.extend(assert_request(&mut cursor, controls));
    }
    assert!(cursor.is_empty());
    output
}

fn assert_request(cursor: &mut &[u8], controls: SimulationAuxiliaryTriangleControls) -> [u32; 3] {
    let expected: [u32; 3] = std::array::from_fn(|_| read_word(cursor));
    let flags = request_flags(controls);
    let capped = controls.renderer_color_masked_exceptions();
    assert_eq!(flags, expected[..2]);
    assert_eq!(capped.is_some(), expected[0] == 1);
    if let Some(color) = capped {
        assert_eq!(color.bits()[..3], controls.color.bits()[..3]);
    }
    let alpha = capped.map_or(0, |color| color.bits()[3]);
    assert_eq!(alpha, expected[2]);
    [flags[0], flags[1], alpha]
}

fn stored_controls(
    family: u32,
    version: u32,
    raw: &[u8],
) -> bessemer::Result<SimulationAuxiliaryTriangleControls> {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let controls = parent.auxiliary_triangle_controls();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.auxiliary_triangle_controls(), controls);
        return controls;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let controls = style.auxiliary_triangle_controls();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.auxiliary_triangle_controls(), controls);
    controls
}

fn assert_view(
    cursor: &mut &[u8],
    actual: bessemer::Result<SimulationAuxiliaryTriangleControls>,
    present: u32,
) -> Vec<u32> {
    if present == 0 {
        let error = actual.unwrap_err();
        assert_eq!(error.class, FailureClass::Missing);
        assert_eq!(error.entity, "style.word_204");
        return vec![0; 9];
    }
    assert_eq!(present, 1);
    let expected = read_controls(cursor);
    let flags = [read_word(cursor), read_word(cursor)];
    let actual = actual.unwrap();
    assert_eq!(actual, expected);
    assert_eq!(request_flags(actual), flags);
    project_view(actual, flags)
}

fn project_view(actual: SimulationAuxiliaryTriangleControls, flags: [u32; 2]) -> Vec<u32> {
    [
        vec![
            1,
            actual.enable_raw as u32,
            actual.group_selector_raw as u32,
        ],
        actual.color.bits().to_vec(),
        flags.to_vec(),
    ]
    .concat()
}

fn replay_views(mut cursor: &[u8], count: usize, expected_present: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let mut output = Vec::new();
    let mut found = 0;
    for _ in 0..count {
        let family = read_word(&mut cursor);
        let version = read_word(&mut cursor);
        let length = read_word(&mut cursor) as usize;
        let present = read_word(&mut cursor);
        let raw = take_projection(&mut cursor, length);
        output.extend(assert_view(
            &mut cursor,
            stored_controls(family, version, raw),
            present,
        ));
        found += present as usize;
    }
    assert!(cursor.is_empty());
    assert_eq!(found, expected_present);
    output
}

#[test]
fn every_binary32_class_and_threshold_neighbor_matches_actual_source_gate() {
    replay_requests(REQUESTS);
}
#[test]
fn all_owned_views_preserve_original_words_colors_and_missing_groups() {
    replay_views(VIEWS, 425, 224);
}
#[test]
fn complete_helper_plans_match_native_stride_oracle_and_fail_incomplete_inputs() {
    plan_support::replay_plans(PLANS);
}
#[test]
fn published_plans_own_values_after_source_mutation() {
    plan_support::assert_source_isolation(PLANS);
}

#[test]
fn public_types_and_eight_workers_match_serial_requests_views_and_plans() {
    assert_public_thread_traits();
    let serial = replay_owned_results();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| assert_eq!(replay_owned_results(), serial));
        }
    });
}

fn assert_public_thread_traits() {
    fn require_threads<T: Send + Sync>() {}
    require_threads::<SimulationAuxiliaryTriangleControls>();
    require_threads::<SimulationAuxTriangleRuntime>();
    require_threads::<SimulationAuxTriangleGroup>();
    require_threads::<SimulationAuxTriangleCall>();
    require_threads::<SimulationAuxTrianglePlan>();
    require_threads::<SimulationAuxTriangleCoverage>();
    require_threads::<SimulationAuxTriangleColorSource>();
    require_threads::<SimulationAuxTriangleRejection>();
    require_threads::<SimulationAuxTriangleStage>();
    require_threads::<SimulationAuxTriangleCompletion>();
    require_threads::<SimulationAuxTriangleDiagnostic>();
}

fn replay_owned_results() -> (Vec<u32>, Vec<u32>, Vec<u32>) {
    (
        replay_requests(REQUESTS),
        replay_views(VIEWS, 425, 224),
        plan_support::replay_plans(PLANS),
    )
}

#[test]
#[ignore = "Requires fresh complete pinned native auxiliary projections."]
fn all_native_views_preserve_original_words_colors_and_missing_groups() {
    replay_views(
        &std::fs::read("target/simulation-aux-triangle-projection-native.bin").unwrap(),
        276,
        160,
    );
}

#[test]
#[ignore = "Requires native projections; output supports independent process comparison."]
fn export_aux_triangle_results_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-aux-triangle-projection-native.bin").unwrap();
    let output = [
        replay_requests(REQUESTS),
        replay_views(VIEWS, 425, 224),
        replay_views(&native, 276, 160),
        plan_support::replay_plans(PLANS),
    ]
    .concat();
    if let Some(path) = std::env::var_os("BESSEMER_AUX_TRIANGLE_OUTPUT") {
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
