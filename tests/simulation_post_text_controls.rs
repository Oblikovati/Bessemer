//! C855 assignment and C890 selection against independent raw-unit fixtures (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(
    dead_code,
    reason = "Shared support also serves complete field/history packets."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};
#[path = "support/post_text_operations.rs"]
mod operation_support;
use operation_support::{
    PostTextFixtureCase, append_post_frame, append_post_words, post_operation_bytes, read_post_text,
};
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/post-text-operations.bin");
const VIEWS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/post-text-projection.bin");
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

fn unknown_post_runtime() -> SimulationPostTextRuntime {
    SimulationPostTextRuntime {
        primary: None,
        lines: std::array::from_fn(|_| None),
    }
}

fn stored_post_text_controls(
    family: u32,
    version: u32,
    raw: &[u8],
) -> bessemer::Result<SimulationPostTextControls> {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let controls = parent.text_display_controls();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.text_display_controls(), controls);
        return controls;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let controls = style.text_display_controls();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.text_display_controls(), controls);
    controls
}

fn read_projected_post_controls(cursor: &mut &[u8]) -> SimulationPostTextControls {
    SimulationPostTextControls {
        primary_enabled_raw: read_word(cursor) as i32,
        seven_lines_enabled_raw: read_word(cursor) as i32,
        stored_text: read_post_text(cursor),
    }
}

fn assert_unknown_post_projection(
    cursor: &mut &[u8],
    controls: SimulationPostTextControls,
) -> Vec<u8> {
    let mut output = Vec::new();
    append_post_words(
        &mut output,
        &[
            1,
            controls.primary_enabled_raw as u32,
            controls.seven_lines_enabled_raw as u32,
        ],
    );
    append_post_frame(&mut output, &controls.stored_text);
    for (mode, maximum) in [(1, 0), (0, 512)] {
        output.extend(assert_post_projected_operation(
            cursor, &controls, mode, maximum,
        ));
    }
    output
}

fn assert_post_projected_operation(
    cursor: &mut &[u8],
    controls: &SimulationPostTextControls,
    mode: u32,
    maximum: usize,
) -> Vec<u8> {
    let length = read_word(cursor) as usize;
    let expected = take_projection(cursor, length);
    let case = PostTextFixtureCase {
        mode,
        controls: controls.clone(),
        runtime: unknown_post_runtime(),
        maximum,
    };
    let actual = post_operation_bytes(&case);
    assert_eq!(actual, expected);
    let mut output = (actual.len() as u32).to_le_bytes().to_vec();
    output.extend(actual);
    output
}

fn replay_post_projection_case(cursor: &mut &[u8]) -> Vec<u8> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let length = read_word(cursor) as usize;
    let present = read_word(cursor);
    let raw = take_projection(cursor, length);
    let actual = stored_post_text_controls(family, version, raw);
    if present == 0 {
        let error = actual.unwrap_err();
        assert_eq!(error.class, FailureClass::Missing);
        assert_eq!(error.entity, "style.word_23c");
        return 0_u32.to_le_bytes().to_vec();
    }
    assert_eq!(present, 1);
    let expected = read_projected_post_controls(cursor);
    let controls = actual.unwrap();
    assert_eq!(controls, expected);
    assert_unknown_post_projection(cursor, controls)
}

fn replay_post_projections(mut cursor: &[u8], count: u32, present: usize) -> Vec<u8> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let mut output = Vec::new();
    let mut found = 0;
    for _ in 0..count {
        let actual = replay_post_projection_case(&mut cursor);
        found += usize::from(actual[..4] == 1_u32.to_le_bytes());
        output.extend(actual);
    }
    assert_eq!(found, present);
    assert!(cursor.is_empty());
    output
}

#[test]
fn complete_assignment_current_selection_and_composition_match_independent_oracle() {
    assert!(!operation_support::replay_post_text_cases(OPERATIONS).is_empty());
}

#[test]
fn every_owned_archive_preserves_raw_controls_text_presence_and_unknown_runtime_state() {
    assert!(!replay_post_projections(VIEWS, 425, 405).is_empty());
}

fn find_known_post_case(mode: u32, kind: Option<u32>) -> PostTextFixtureCase {
    let mut cursor = OPERATIONS;
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    for _ in 0..count {
        let length = read_word(&mut cursor) as usize;
        let case = operation_support::read_post_text_case(take_projection(&mut cursor, length));
        let length = read_word(&mut cursor) as usize;
        let mut expected = take_projection(&mut cursor, length);
        if known_post_expected(&mut expected, mode, kind) {
            return case;
        }
    }
    panic!("fixture lacks mode {mode}, kind {kind:?}")
}

fn known_post_expected(expected: &mut &[u8], mode: u32, kind: Option<u32>) -> bool {
    if read_word(expected) != mode {
        return false;
    }
    if read_word(expected) == 0 {
        return false;
    }
    read_word(expected);
    read_word(expected);
    let assigned_kind = read_word(expected);
    kind.is_none_or(|wanted| assigned_kind == wanted)
}

fn mutate_post_inputs(case: &mut PostTextFixtureCase) {
    case.controls.stored_text = SimulationPostStyleText::from_payload(&[], 0).unwrap();
    case.controls.primary_enabled_raw = 0;
    case.controls.seven_lines_enabled_raw = 0;
    case.runtime.primary = None;
    case.runtime.lines = std::array::from_fn(|_| None);
}

fn assert_post_assignment_isolation(kind: u32) {
    let mut case = find_known_post_case(0, Some(kind));
    let assignment = case
        .controls
        .assign_primary(case.runtime.primary.as_ref(), case.maximum)
        .unwrap();
    let original = assignment.clone();
    mutate_post_inputs(&mut case);
    assert_eq!(assignment, original);
    assert!(assignment.primary_after().character_count() > 0);
}

fn assert_post_plan_isolation(mode: u32) {
    let mut case = find_known_post_case(mode, None);
    let plan = if mode == 1 {
        case.controls.plan_selection(&case.runtime, case.maximum)
    } else {
        case.controls
            .plan_after_assignment(&case.runtime, case.maximum)
    }
    .unwrap();
    let original = plan.clone();
    mutate_post_inputs(&mut case);
    assert_eq!(plan, original);
    assert!(plan.coverage().result_code_units > 0);
}

#[test]
fn assignment_and_both_selection_modes_remain_isolated_after_source_mutation() {
    assert_post_assignment_isolation(1);
    assert_post_assignment_isolation(2);
    assert_post_plan_isolation(1);
    assert_post_plan_isolation(2);
}

#[test]
fn counted_input_storage_copies_borrowed_bytes_before_publication() {
    let mut raw = 'A'
        .to_string()
        .encode_utf16()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    let text = SimulationPostStyleText::from_payload(&raw, 2).unwrap();
    let controls = SimulationPostTextControls {
        primary_enabled_raw: 1,
        seven_lines_enabled_raw: 0,
        stored_text: text,
    };
    let assignment = controls.assign_primary(None, 2).unwrap();
    raw.fill(0xff);
    assert_eq!(assignment.primary_after().payload(), b"A\0");
    assert_eq!(assignment.source_wire_text().payload(), b"A\0");
}

#[test]
fn eight_workers_and_all_public_thread_traits_match_serial_bytes() {
    assert_post_text_thread_traits();
    let expected = operation_support::replay_post_text_cases(OPERATIONS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| operation_support::replay_post_text_cases(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}

fn assert_post_text_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationPostTextControls>();
    require::<SimulationPostTextRuntime>();
    require::<SimulationPostTextOrigin>();
    require::<SimulationPostTextSelection>();
    require::<SimulationPostTextAssignmentKind>();
    require::<SimulationPostTextCompletion>();
    require::<SimulationPostTextCoverage>();
    require::<SimulationPostTextDiagnostic>();
    require::<SimulationPostTextAssignment>();
    require::<SimulationPostTextPlan>();
}

#[test]
#[ignore = "Requires complete freshly pinned native text/control fixtures."]
fn every_native_archive_retains_controls_and_requires_explicit_unknown_runtime_text() {
    replay_post_projections(
        &std::fs::read("target/simulation-post-text-projection-native.bin").unwrap(),
        276,
        276,
    );
}

#[test]
#[ignore = "Writes deterministic bytes for independent process comparison."]
fn export_post_text_results_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-post-text-projection-native.bin").unwrap();
    let output = [
        operation_support::replay_post_text_cases(OPERATIONS),
        replay_post_projections(VIEWS, 425, 405),
        replay_post_projections(&native, 276, 276),
    ]
    .concat();
    if let Some(path) = std::env::var_os("BESSEMER_POST_TEXT_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
