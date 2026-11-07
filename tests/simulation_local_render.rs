//! C-884 independent predicate results and complete stored-input projections (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(
    dead_code,
    reason = "Shared support also serves full field/history packets."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/local-render-predicate.bin");
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

fn read_local_case(cursor: &mut &[u8]) -> (SimulationLocalRenderInputs, u32) {
    let inputs = SimulationLocalRenderInputs {
        color_count: read_word(cursor) as i32,
        cell_count: read_word(cursor) as i32,
        horizontal_bits: read_word(cursor),
        vertical_bits: read_word(cursor),
    };
    let expected = read_word(cursor);
    assert_eq!(
        u32::from(inputs.passes_masked_exceptions_daz_off()),
        expected
    );
    (inputs, expected)
}

fn replay_local_cases(mut cursor: &[u8], count: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let results = (0..count).map(|_| read_local_case(&mut cursor).1).collect();
    assert!(cursor.is_empty());
    results
}

fn stored_local_inputs(
    family: u32,
    version: u32,
    raw: &[u8],
) -> bessemer::Result<SimulationLocalRenderInputs> {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let inputs = parent.local_render_inputs();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.local_render_inputs(), inputs);
        return inputs;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let inputs = style.local_render_inputs();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.local_render_inputs(), inputs);
    inputs
}

fn assert_projection(mut cursor: &[u8], count: usize) {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    for _ in 0..count {
        let family = read_word(&mut cursor);
        let version = read_word(&mut cursor);
        let length = read_word(&mut cursor) as usize;
        let available = read_word(&mut cursor);
        let raw = take_projection(&mut cursor, length);
        let actual = stored_local_inputs(family, version, raw);
        assert_projection_result(&mut cursor, actual, available);
    }
    assert!(cursor.is_empty());
}

fn assert_projection_result(
    cursor: &mut &[u8],
    actual: bessemer::Result<SimulationLocalRenderInputs>,
    available: u32,
) {
    if available == 0 {
        let error = actual.unwrap_err();
        assert_eq!(error.class, FailureClass::Missing);
        assert!(error.entity.starts_with("legend.word_"));
        return;
    }
    assert_eq!(available, 1);
    let (expected, result) = read_local_case(cursor);
    let inputs = actual.unwrap();
    assert_eq!(inputs, expected);
    assert_eq!(u32::from(inputs.passes_masked_exceptions_daz_off()), result);
}

#[test]
fn every_count_sign_exponent_and_position_branch_matches_independent_source() {
    replay_local_cases(OWNED, 18177);
}

#[test]
fn all_owned_complete_views_preserve_bytes_and_missing_legacy_state() {
    assert_projection(
        include_bytes!("fixtures/simulation-post-dialog/local-render-projection.bin"),
        425,
    );
}

#[test]
fn unordered_positions_and_unbounded_counts_follow_the_local_predicate() {
    let mut inputs = SimulationLocalRenderInputs {
        color_count: i32::MAX,
        cell_count: i32::MAX,
        horizontal_bits: 0xffa00042,
        vertical_bits: 0x7f800000,
    };
    assert!(inputs.passes_masked_exceptions_daz_off());
    inputs.horizontal_bits = 0x80000000;
    assert!(inputs.passes_masked_exceptions_daz_off());
    inputs.horizontal_bits = 0x80000001;
    assert!(!inputs.passes_masked_exceptions_daz_off());
    inputs.horizontal_bits = 0xff800000;
    assert!(!inputs.passes_masked_exceptions_daz_off());
}

#[test]
fn immutable_public_inputs_match_across_eight_workers() {
    fn require_threads<T: Send + Sync>() {}
    require_threads::<SimulationLocalRenderInputs>();
    let serial = replay_local_cases(OWNED, 18177);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| assert_eq!(replay_local_cases(OWNED, 18177), serial));
        }
    });
}

#[test]
#[ignore = "Requires fresh complete pinned native C-884 projections."]
fn all_native_complete_views_preserve_bytes() {
    assert_projection(
        &std::fs::read("target/simulation-local-render-projection-native.bin").unwrap(),
        276,
    );
}

#[test]
#[ignore = "Requires original native C-884 controls; output supports process replay."]
fn native_controls_and_fresh_process_output_match_source() {
    let native = std::fs::read("target/simulation-local-render-native.bin").unwrap();
    let results = replay_local_cases(OWNED, 18177)
        .into_iter()
        .chain(replay_local_cases(&native, 276));
    let output: Vec<u8> = results.flat_map(u32::to_le_bytes).collect();
    if let Some(path) = std::env::var_os("BESSEMER_LOCAL_RENDER_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
