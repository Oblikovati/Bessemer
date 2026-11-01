//! Numeric unit codes and unresolved runtime context remain distinct results (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(
    dead_code,
    reason = "Shared projection readers also serve complete history packets."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/unit-type.bin");
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

fn read_inputs(cursor: &mut &[u8]) -> SimulationUnitTypeInputs {
    let analysis = read_word(cursor) as i32;
    let plot_type = read_word(cursor) as i32;
    let component = read_word(cursor) as i32;
    let mask = read_word(cursor);
    assert!(mask < 8);
    let words: [i32; 3] = std::array::from_fn(|_| read_word(cursor) as i32);
    SimulationUnitTypeInputs {
        analysis,
        plot_type,
        component,
        word_be8: (mask & 1 != 0).then_some(words[0]),
        word_bec: (mask & 2 != 0).then_some(words[1]),
        actual_freq_disp_flag: (mask & 4 != 0).then_some(words[2]),
    }
}

fn unit_outcome(inputs: SimulationUnitTypeInputs) -> [u32; 2] {
    match inputs.unit_type_code() {
        Ok(code) => [0, code as u32],
        Err(error) => {
            assert_eq!(error.class, FailureClass::Missing);
            assert_eq!(error.operation, "component unit type");
            match error.entity.as_str() {
                "ActualFreqDispFlag" => [1, 0],
                "loaded BE8/BEC" => [2, 0],
                unknown => panic!("{unknown}; expected explicit flag or modifier context failure"),
            }
        }
    }
}

fn assert_case(cursor: &mut &[u8]) -> (SimulationUnitTypeInputs, [u32; 2]) {
    let inputs = read_inputs(cursor);
    let original = inputs;
    let expected = [read_word(cursor), read_word(cursor)];
    assert_eq!(unit_outcome(inputs), expected, "{inputs:?}");
    assert_eq!(inputs, original);
    (inputs, expected)
}

fn replay_units(mut cursor: &[u8], count: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let output = (0..count)
        .flat_map(|_| assert_case(&mut cursor).1)
        .collect();
    assert!(cursor.is_empty());
    output
}

#[test]
fn all_family_boundaries_precedence_and_optional_contexts_match_source() {
    replay_units(OWNED, 202840);
}

#[test]
#[ignore = "Requires freshly pinned original C-881/C-882 native contexts and alternatives."]
fn all_native_known_codes_missing_contexts_and_explicit_alternatives_match_source() {
    replay_units(
        &std::fs::read("target/simulation-unit-type-native.bin").unwrap(),
        224,
    );
    replay_units(
        &std::fs::read("target/simulation-unit-flag-alternatives.bin").unwrap(),
        168,
    );
}

fn assert_parent_choices(cursor: &mut &[u8], parent: &SimulationPostDialog) {
    assert_eq!(read_word(cursor), 1);
    assert_eq!(read_word(cursor), 16);
    for _ in 0..16 {
        let (expected, outcome) = assert_case(cursor);
        let actual = parent
            .unit_type_inputs(expected.word_be8, expected.actual_freq_disp_flag)
            .unwrap();
        assert_eq!(actual, expected);
        assert_eq!(unit_outcome(actual), outcome);
    }
}

fn assert_parent_projection(mut cursor: &[u8], count: usize) {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    for _ in 0..count {
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let length = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, length);
        let parent = SimulationPostDialog::from_bytes(raw, version, LIMITS).unwrap();
        assert_parent_choices(&mut cursor, &parent);
        assert_eq!(parent.to_bytes(), raw);
    }
    assert!(cursor.is_empty());
}

#[test]
fn all_owned_parent_context_choices_preserve_stored_bec_and_wire_bytes() {
    assert_parent_projection(
        include_bytes!("fixtures/simulation-post-dialog/unit-parent-projection.bin"),
        225,
    );
}

#[test]
#[ignore = "Requires freshly pinned complete native unit parent projections."]
fn all_native_parent_context_choices_preserve_stored_bec_and_wire_bytes() {
    let packet = std::fs::read("target/simulation-unit-parent-projection-native.bin").unwrap();
    assert_parent_projection(&packet, 224);
}

#[test]
fn numeric_fallback_is_complete_while_required_unknown_state_is_missing() {
    let inputs = SimulationUnitTypeInputs {
        analysis: 1,
        plot_type: 1,
        component: 0,
        word_be8: None,
        word_bec: None,
        actual_freq_disp_flag: None,
    };
    assert_eq!(unit_outcome(inputs), [1, 0]);
    assert_eq!(
        unit_outcome(SimulationUnitTypeInputs {
            actual_freq_disp_flag: Some(2),
            ..inputs
        }),
        [0, 9999]
    );
    assert_eq!(
        unit_outcome(SimulationUnitTypeInputs {
            actual_freq_disp_flag: Some(1),
            ..inputs
        }),
        [0, 0]
    );
}

#[test]
fn eight_workers_and_public_traits_match_every_serial_numeric_or_missing_result() {
    fn require_threads<T: Send + Sync>() {}
    require_threads::<SimulationUnitTypeInputs>();
    let expected = replay_units(OWNED, 202840);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| assert_eq!(replay_units(OWNED, 202840), expected));
        }
    });
}

#[test]
#[ignore = "Requires fresh native contexts; optional output supports process replay."]
fn export_unit_results_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-unit-type-native.bin").unwrap();
    let alternatives = std::fs::read("target/simulation-unit-flag-alternatives.bin").unwrap();
    let words = replay_units(OWNED, 202840)
        .into_iter()
        .chain(replay_units(&native, 224))
        .chain(replay_units(&alternatives, 168));
    let output: Vec<u8> = words.flat_map(u32::to_le_bytes).collect();
    if let Some(path) = std::env::var_os("BESSEMER_UNIT_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
