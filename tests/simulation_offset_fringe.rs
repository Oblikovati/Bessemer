//! Independent offset/fringe decisions preserve absent inputs and modeled local call order (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(
    dead_code,
    reason = "Shared projection support also serves full field/history packets."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const OFFSETS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/offset-request.bin");
const FRINGES: &[u8] = include_bytes!("fixtures/simulation-post-dialog/fringe-option.bin");
const LIMITS: SimulationPostDialogLimits = SimulationPostDialogLimits {
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

fn offset_outcome(inputs: SimulationOffsetRequestInputs) -> u32 {
    match inputs.needs_offset() {
        Ok(needed) => u32::from(needed),
        Err(error) => {
            assert_eq!(error.class, FailureClass::Missing);
            assert_eq!(error.entity, "moving_objects.word_48");
            assert!(error.cause.contains("word48 None"));
            2
        }
    }
}

fn read_offset(cursor: &mut &[u8]) -> (SimulationOffsetRequestInputs, u32) {
    let style_1b8 = read_word(cursor) as i32;
    let present = read_word(cursor);
    assert!(present <= 1);
    let word = read_word(cursor) as i32;
    let inputs = SimulationOffsetRequestInputs {
        style_1b8,
        word_48: (present == 1).then_some(word),
    };
    let expected = read_word(cursor);
    assert_eq!(offset_outcome(inputs), expected);
    (inputs, expected)
}

fn replay_offsets(mut cursor: &[u8], count: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let output = (0..count).map(|_| read_offset(&mut cursor).1).collect();
    assert!(cursor.is_empty());
    output
}

fn read_fringe(cursor: &mut &[u8]) -> (SimulationFringeOptionView, Vec<u32>) {
    let original = (read_word(cursor) as i32, read_word(cursor) as i32);
    let expected = SimulationFringeOptionView {
        stored_word_14: original.0,
        stored_word_48: original.1,
        runtime_word_48_after: read_word(cursor) as i32,
        fringe_option_result: read_word(cursor) as i32,
    };
    let actual = SimulationFringeOptionView::from_stored(original.0, original.1);
    assert_eq!(actual, expected);
    let mut output = vec![
        actual.runtime_word_48_after as u32,
        actual.fringe_option_result as u32,
    ];
    output.extend(assert_call_order(cursor, &actual));
    assert_eq!((actual.stored_word_14, actual.stored_word_48), original);
    (actual, output)
}

fn assert_call_order(cursor: &mut &[u8], view: &SimulationFringeOptionView) -> Vec<u32> {
    let mut output = Vec::new();
    for style_1b8 in [0, 1, 2] {
        for word in [view.stored_word_48, view.runtime_word_48_after] {
            let inputs = SimulationOffsetRequestInputs {
                style_1b8,
                word_48: Some(word),
            };
            let expected = read_word(cursor);
            assert_eq!(offset_outcome(inputs), expected);
            output.push(expected);
        }
    }
    output
}

fn replay_fringes(mut cursor: &[u8], count: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let output = (0..count)
        .flat_map(|_| read_fringe(&mut cursor).1)
        .collect();
    assert!(cursor.is_empty());
    output
}

#[test]
fn all_signed_offset_branches_and_genuine_missing_inputs_match_source() {
    replay_offsets(OFFSETS, 9075);
}

#[test]
fn all_signed_fringe_results_runtime_transitions_and_call_orders_match_source() {
    replay_fringes(FRINGES, 8639);
}

#[test]
fn modeled_call_order_changes_only_the_explicit_runtime_copy() {
    let before = SimulationOffsetRequestInputs {
        style_1b8: 0,
        word_48: Some(1),
    };
    let view = SimulationFringeOptionView::from_stored(7, 1);
    let after = SimulationOffsetRequestInputs {
        word_48: Some(view.runtime_word_48_after),
        ..before
    };
    assert!(before.needs_offset().unwrap());
    assert!(!after.needs_offset().unwrap());
    assert_eq!(before.word_48, Some(1));
    assert_eq!(view.stored_word_48, 1);
    assert_eq!(view.runtime_word_48_after, 6);
}

fn assert_parent_fringe(
    cursor: &mut &[u8],
    parent: &SimulationPostDialog,
    inputs: SimulationOffsetRequestInputs,
) {
    let flag = read_word(cursor);
    if flag == 0 {
        let error = parent.fringe_option_view().unwrap_err();
        assert_eq!(error.class, FailureClass::Missing);
        assert_eq!(error.entity, "moving_objects.word_48");
        return;
    }
    assert_eq!(flag, 1);
    let expected = read_fringe(cursor).0;
    let actual = parent.fringe_option_view().unwrap();
    assert_eq!(actual, expected);
    assert_eq!(offset_outcome(inputs), read_word(cursor));
    let after = SimulationOffsetRequestInputs {
        word_48: Some(actual.runtime_word_48_after),
        ..inputs
    };
    assert_eq!(offset_outcome(after), read_word(cursor));
}

fn assert_parent_projection(mut cursor: &[u8], count: usize) {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    for _ in 0..count {
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let length = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, length);
        let parent = SimulationPostDialog::from_bytes(raw, version, LIMITS).unwrap();
        let (expected, result) = read_offset(&mut cursor);
        let actual = parent.offset_request_inputs().unwrap();
        assert_eq!(actual, expected);
        assert_eq!(offset_outcome(actual), result);
        assert_parent_fringe(&mut cursor, &parent, actual);
        assert_eq!(parent.offset_request_inputs().unwrap(), expected);
        assert_eq!(parent.to_bytes(), raw);
    }
    assert!(cursor.is_empty());
}

#[test]
fn all_owned_parent_views_preserve_raw_bytes_and_missing_state() {
    assert_parent_projection(
        include_bytes!("fixtures/simulation-post-dialog/offset-fringe-projection.bin"),
        225,
    );
}

#[test]
#[ignore = "Requires fresh complete pinned native offset/fringe projections."]
fn all_native_parent_views_preserve_raw_bytes_and_missing_state() {
    let packet = std::fs::read("target/simulation-offset-fringe-projection-native.bin").unwrap();
    assert_parent_projection(&packet, 224);
}

#[test]
#[ignore = "Requires fresh original complete C-876/C-877 native controls."]
fn all_original_native_controls_match_both_independent_models() {
    replay_offsets(
        &std::fs::read("target/simulation-offset-request-native.bin").unwrap(),
        224,
    );
    replay_fringes(
        &std::fs::read("target/simulation-fringe-option-native.bin").unwrap(),
        208,
    );
}

#[test]
fn eight_workers_and_public_traits_match_all_serial_decisions() {
    fn require_threads<T: Send + Sync>() {}
    require_threads::<SimulationOffsetRequestInputs>();
    require_threads::<SimulationFringeOptionView>();
    let offsets = replay_offsets(OFFSETS, 9075);
    let fringes = replay_fringes(FRINGES, 8639);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                assert_eq!(replay_offsets(OFFSETS, 9075), offsets);
                assert_eq!(replay_fringes(FRINGES, 8639), fringes);
            });
        }
    });
}

#[test]
#[ignore = "Requires fresh native controls; optional output supports process replay."]
fn export_offset_fringe_results_for_fresh_process_replay() {
    let native_offsets = std::fs::read("target/simulation-offset-request-native.bin").unwrap();
    let native_fringes = std::fs::read("target/simulation-fringe-option-native.bin").unwrap();
    let words = replay_offsets(OFFSETS, 9075)
        .into_iter()
        .chain(replay_offsets(&native_offsets, 224))
        .chain(replay_fringes(FRINGES, 8639))
        .chain(replay_fringes(&native_fringes, 208));
    let output: Vec<u8> = words.flat_map(u32::to_le_bytes).collect();
    if let Some(path) = std::env::var_os("BESSEMER_OFFSET_FRINGE_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
