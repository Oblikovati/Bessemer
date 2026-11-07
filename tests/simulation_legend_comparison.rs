//! Independent conditional legend branches and stored-state projection (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(
    dead_code,
    reason = "Shared projection support also serves full field/history readers."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/legend-comparison.bin");
const WORDS: [u32; 16] = [
    8, 12, 16, 20, 0xa8, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4, 0xc8, 0xd4, 0xe8, 0xfc, 0xac, 0xb0,
];
const COLORS: [u32; 12] = [
    0x18, 0x28, 0x38, 0x48, 0x58, 0x68, 0x78, 0x88, 0x98, 0xd8, 0xec, 0x100,
];
const LIMITS: SimulationPostStyleLimits = SimulationPostStyleLimits {
    maximum_bytes: 4096,
    maximum_fields: 116,
    maximum_entries: 512,
    maximum_text_bytes: 1024,
    maximum_characters: 512,
};

fn input_field(path: String, bits: u32) -> SimulationPostStyleField {
    SimulationPostStyleField {
        path: path.into(),
        value: SimulationPostStyleValue::RawWordBits(bits),
    }
}

fn state_fields(cursor: &mut &[u8]) -> Vec<SimulationPostStyleField> {
    let mask = u64::from(read_word(cursor)) | (u64::from(read_word(cursor)) << 32);
    let mut fields = Vec::new();
    for index in 0..64 {
        let bits = read_word(cursor);
        if mask & (1 << index) == 0 {
            continue;
        }
        fields.push(input_field(slot_path(index), bits));
    }
    fields
}

fn slot_path(index: usize) -> String {
    if index < 16 {
        return format!("legend.word_{:x}", WORDS[index]);
    }
    format!(
        "legend.color_{:x}.component_{}",
        COLORS[(index - 16) / 4],
        (index - 16) % 4
    )
}

fn state(cursor: &mut &[u8]) -> SimulationLegendComparisonState {
    SimulationLegendComparisonState::from_fields(&state_fields(cursor)).unwrap()
}

fn comparison_outcome(
    left: &SimulationLegendComparisonState,
    right: &SimulationLegendComparisonState,
) -> u32 {
    match left.different_masked_exceptions_daz_off(right) {
        Ok(different) => u32::from(different),
        Err(error) if error.class == FailureClass::Missing => 2,
        Err(error) if error.class == FailureClass::Invalid => 3,
        Err(error) => panic!("unexpected comparison failure: {error}"),
    }
}

struct LegendPair {
    left: SimulationLegendComparisonState,
    right: SimulationLegendComparisonState,
    expected: u32,
}

fn read_pairs(mut cursor: &[u8], expected_count: usize) -> Vec<LegendPair> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, expected_count);
    let mut pairs = Vec::new();
    for _ in 0..expected_count {
        let left = state(&mut cursor);
        let right = state(&mut cursor);
        let expected = read_word(&mut cursor);
        pairs.push(LegendPair {
            left,
            right,
            expected,
        });
    }
    assert!(cursor.is_empty());
    pairs
}

fn assert_pair(pair: &LegendPair) -> u32 {
    let original = (pair.left.clone(), pair.right.clone());
    assert_eq!(comparison_outcome(&pair.left, &pair.right), pair.expected);
    assert_eq!((&pair.left, &pair.right), (&original.0, &original.1));
    pair.expected
}

fn replay_pairs(cursor: &[u8], expected_count: usize) -> Vec<u32> {
    read_pairs(cursor, expected_count)
        .iter()
        .map(assert_pair)
        .collect()
}

#[test]
fn every_owned_conditional_branch_and_missing_input_matches_source() {
    replay_pairs(OWNED, 8253);
}

#[test]
#[ignore = "Requires freshly pinned original C-871 native-state mutations."]
fn all_original_native_states_and_isolated_mutations_match_source() {
    let packet = std::fs::read("target/simulation-legend-comparison-native.bin").unwrap();
    replay_pairs(&packet, 7085);
}

fn assert_style_projection(mut cursor: &[u8], count: usize) {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    for _ in 0..count {
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let size = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, size);
        let style = SimulationPostStyle::from_bytes(raw, version, LIMITS).unwrap();
        assert_eq!(style.legend_comparison_state().unwrap(), state(&mut cursor));
        assert_eq!(style.to_bytes(), raw);
    }
    assert!(cursor.is_empty());
}

#[test]
fn every_complete_owned_style_projects_all_raw_inputs_and_absences() {
    assert_style_projection(
        include_bytes!("fixtures/simulation-post-dialog/legend-style-projection.bin"),
        200,
    );
}

#[test]
#[ignore = "Requires freshly pinned complete native PostStyle projections."]
fn every_complete_native_style_projects_all_raw_inputs_and_absences() {
    let packet = std::fs::read("target/simulation-legend-style-projection-native.bin").unwrap();
    assert_style_projection(&packet, 52);
}

#[test]
fn duplicate_and_incomplete_color_inputs_fail_with_typed_causes() {
    let field = input_field("legend.word_8".into(), 0);
    let error = SimulationLegendComparisonState::from_fields(&[field.clone(), field]).unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("duplicate"));
    let partial = input_field("legend.color_18.component_0".into(), 0);
    let error = SimulationLegendComparisonState::from_fields(&[partial]).unwrap_err();
    assert!(error.cause.contains("1 components"));
}

#[test]
fn text_is_not_an_explicit_loaded_raw_word() {
    let text = SimulationPostStyleText::from_payload(&[], 0).unwrap();
    let nonword = SimulationPostStyleField {
        path: "legend.word_8".into(),
        value: SimulationPostStyleValue::Text(text),
    };
    let error = SimulationLegendComparisonState::from_fields(&[nonword]).unwrap_err();
    assert!(error.cause.contains("expected explicit raw32"));
}

#[test]
fn explicit_fields_are_copied_and_missing_state_is_not_a_defaulted_success() {
    let mut supplied = vec![input_field("legend.word_8".into(), 0)];
    let copied = SimulationLegendComparisonState::from_fields(&supplied).unwrap();
    supplied[0].value = SimulationPostStyleValue::Signed(1);
    let changed = SimulationLegendComparisonState::from_fields(&supplied).unwrap();
    assert_eq!(comparison_outcome(&copied, &changed), 1);
    let error = copied
        .different_masked_exceptions_daz_off(&copied)
        .unwrap_err();
    assert_eq!(error.class, FailureClass::Missing);
    assert_eq!(error.entity, "legend.word_c");
}

#[test]
fn raw_state_equality_does_not_adopt_unordered_float_equality() {
    let fields = state_fields(&mut &OWNED[8..]);
    let left = SimulationLegendComparisonState::from_fields(&fields).unwrap();
    let mut changed = fields;
    let position = changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "legend.word_ac")
        .unwrap();
    position.value = SimulationPostStyleValue::Float32Bits(0x7fa00042);
    let right = SimulationLegendComparisonState::from_fields(&changed).unwrap();
    assert_ne!(left, right);
    assert_eq!(comparison_outcome(&left, &right), 0);
}

#[test]
fn eight_workers_match_the_full_serial_branch_matrix() {
    fn require_threads<T: Send + Sync>() {}
    require_threads::<SimulationLegendComparisonState>();
    let pairs = read_pairs(OWNED, 8253);
    let expected: Vec<_> = pairs.iter().map(assert_pair).collect();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| assert_eq!(pairs.iter().map(assert_pair).collect::<Vec<_>>(), expected));
        }
    });
}

#[test]
#[ignore = "Requires fresh native expectations; optional output supports process replay."]
fn export_comparison_outcomes_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-legend-comparison-native.bin").unwrap();
    let outcomes = replay_pairs(OWNED, 8253)
        .into_iter()
        .chain(replay_pairs(&native, 7085));
    let output: Vec<u8> = outcomes.flat_map(u32::to_le_bytes).collect();
    if let Some(path) = std::env::var_os("BESSEMER_LEGEND_COMPARISON_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
