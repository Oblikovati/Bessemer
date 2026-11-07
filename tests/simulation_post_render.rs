//! Named rendering decisions retain raw words, explicit runtime presence and missing older state (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(
    dead_code,
    reason = "Shared projection readers also serve complete history packets."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const GATES: &[u8] = include_bytes!("fixtures/simulation-post-dialog/legend-layer0-gate.bin");
const TESS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/model-tessellation.bin");
const PARENT_LIMITS: SimulationPostDialogLimits = SimulationPostDialogLimits {
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
const STYLE_LIMITS: SimulationPostStyleLimits = SimulationPostStyleLimits {
    maximum_bytes: 4096,
    maximum_fields: 116,
    maximum_entries: 512,
    maximum_text_bytes: 1024,
    maximum_characters: 512,
};

fn read_gate(cursor: &mut &[u8]) -> (SimulationLegendLayer0Gate, u32) {
    let suppression_raw = read_word(cursor) as i32;
    let present = read_word(cursor);
    assert!(present <= 1);
    (
        SimulationLegendLayer0Gate {
            suppression_raw,
            plot_object_present: present == 1,
        },
        read_word(cursor),
    )
}

fn replay_gates(mut cursor: &[u8], count: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let mut output = Vec::new();
    for _ in 0..count {
        let (gate, expected) = read_gate(&mut cursor);
        let original = gate;
        assert_eq!(u32::from(gate.enters_body()), expected);
        assert_eq!(gate, original);
        output.push(expected);
    }
    assert!(cursor.is_empty());
    output
}

fn read_tess_inputs(cursor: &mut &[u8]) -> SimulationModelTessellationInputs {
    SimulationModelTessellationInputs {
        style_21c: read_word(cursor) as i32,
        hidden_option: read_word(cursor) as i32,
        dialog_518: read_word(cursor) as i32,
        style_1f0: read_word(cursor) as i32,
        plot_type: read_word(cursor) as i32,
        word_18: read_word(cursor) as i32,
    }
}

fn replay_tessellation(mut cursor: &[u8], count: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let mut output = Vec::new();
    for _ in 0..count {
        let inputs = read_tess_inputs(&mut cursor);
        let original = inputs;
        let expected = read_word(&mut cursor);
        assert_eq!(u32::from(inputs.needs_model_tessellation()), expected);
        assert_eq!(inputs, original);
        output.push(expected);
    }
    assert!(cursor.is_empty());
    output
}

#[test]
fn every_signed_gate_neighbor_matches_independent_source() {
    replay_gates(GATES, 872);
}

#[test]
fn every_tessellation_branch_combination_matches_independent_source() {
    replay_tessellation(TESS, 21060);
}

enum RenderSnapshot {
    Parent(SimulationPostDialog),
    Style(SimulationPostStyle),
}

impl RenderSnapshot {
    fn gate(&self, present: bool) -> SimulationLegendLayer0Gate {
        match self {
            Self::Parent(parent) => parent.legend_layer0_gate(present).unwrap(),
            Self::Style(style) => style.legend_layer0_gate(present).unwrap(),
        }
    }

    fn bytes(&self) -> Vec<u8> {
        match self {
            Self::Parent(parent) => parent.to_bytes(),
            Self::Style(style) => style.to_bytes(),
        }
    }
}

fn read_snapshot(cursor: &mut &[u8]) -> RenderSnapshot {
    let family = read_word(cursor);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let raw = take_projection(cursor, size);
    match family {
        0 => RenderSnapshot::Parent(
            SimulationPostDialog::from_bytes(raw, version, PARENT_LIMITS).unwrap(),
        ),
        2 => RenderSnapshot::Style(
            SimulationPostStyle::from_bytes(raw, version, STYLE_LIMITS).unwrap(),
        ),
        unexpected => panic!("{unexpected}; expected complete parent/style family 0/2"),
    }
}

fn assert_tess_projection(cursor: &mut &[u8], snapshot: &RenderSnapshot) {
    let flag = read_word(cursor);
    let RenderSnapshot::Parent(parent) = snapshot else {
        assert_eq!(flag, 2);
        return;
    };
    if flag == 0 {
        let error = parent.model_tessellation_inputs().unwrap_err();
        assert_eq!(error.class, FailureClass::Missing);
        assert_eq!(error.entity, "style.word_1ec");
        return;
    }
    assert_eq!(flag, 1);
    let expected = read_tess_inputs(cursor);
    let actual = parent.model_tessellation_inputs().unwrap();
    assert_eq!(actual, expected);
    assert_eq!(
        u32::from(actual.needs_model_tessellation()),
        read_word(cursor)
    );
}

fn assert_projection(mut cursor: &[u8], count: usize) {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    for _ in 0..count {
        let snapshot = read_snapshot(&mut cursor);
        let original = snapshot.bytes();
        for _ in 0..2 {
            let (expected, result) = read_gate(&mut cursor);
            let actual = snapshot.gate(expected.plot_object_present);
            assert_eq!(actual, expected);
            assert_eq!(u32::from(actual.enters_body()), result);
        }
        assert_tess_projection(&mut cursor, &snapshot);
        assert_eq!(snapshot.bytes(), original);
    }
    assert!(cursor.is_empty());
}

#[test]
fn all_owned_profiles_project_raw_controls_and_missing_older_runtime_state() {
    assert_projection(
        include_bytes!("fixtures/simulation-post-dialog/render-policy-projection.bin"),
        425,
    );
}

#[test]
#[ignore = "Requires freshly pinned complete native rendering-decision projections."]
fn all_native_profiles_project_raw_controls_and_missing_older_runtime_state() {
    let packet = std::fs::read("target/simulation-render-policy-projection-native.bin").unwrap();
    assert_projection(&packet, 276);
}

#[test]
#[ignore = "Requires freshly pinned original C-872/C-873 native inputs."]
fn every_original_native_control_matches_both_independent_models() {
    replay_gates(
        &std::fs::read("target/simulation-layer0-gate-native.bin").unwrap(),
        552,
    );
    replay_tessellation(
        &std::fs::read("target/simulation-model-tessellation-native.bin").unwrap(),
        160,
    );
}

#[test]
fn eight_workers_and_public_traits_preserve_all_serial_model_results() {
    fn require_threads<T: Send + Sync>() {}
    require_threads::<SimulationLegendLayer0Gate>();
    require_threads::<SimulationModelTessellationInputs>();
    let gates = replay_gates(GATES, 872);
    let tess = replay_tessellation(TESS, 21060);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                assert_eq!(replay_gates(GATES, 872), gates);
                assert_eq!(replay_tessellation(TESS, 21060), tess);
            });
        }
    });
}

#[test]
#[ignore = "Requires fresh native packets; optional output supports process replay."]
fn export_render_decisions_for_fresh_process_replay() {
    let native_gate = std::fs::read("target/simulation-layer0-gate-native.bin").unwrap();
    let native_tess = std::fs::read("target/simulation-model-tessellation-native.bin").unwrap();
    let words = replay_gates(GATES, 872)
        .into_iter()
        .chain(replay_gates(&native_gate, 552))
        .chain(replay_tessellation(TESS, 21060))
        .chain(replay_tessellation(&native_tess, 160));
    let output: Vec<u8> = words.flat_map(u32::to_le_bytes).collect();
    if let Some(path) = std::env::var_os("BESSEMER_RENDER_POLICY_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
