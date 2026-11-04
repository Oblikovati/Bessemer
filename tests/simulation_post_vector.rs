//! Static provider/local vector outputs preserve every raw word and selected-table provenance (#166).
use bessemer::simulation::*;
#[allow(
    dead_code,
    reason = "Shared projection readers also serve complete history packets."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const PROVIDERS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/component-vector.bin");
const DIRECTIONS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/vector-direction.bin");
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

fn read_provider(cursor: &mut &[u8]) -> SimulationComponentVectorInputs {
    SimulationComponentVectorInputs {
        analysis: read_word(cursor) as i32,
        plot_type: read_word(cursor) as i32,
        component: read_word(cursor) as i32,
        fourth_word: read_word(cursor) as i32,
    }
}

fn read_direction(cursor: &mut &[u8]) -> SimulationVectorDirectionInputs {
    let provider = read_provider(cursor);
    let word_570 = read_word(cursor) as i32;
    let present = read_word(cursor);
    assert!(present <= 1);
    SimulationVectorDirectionInputs {
        provider,
        word_570,
        loaded_text_nonempty: present == 1,
    }
}

fn read_view(cursor: &mut &[u8]) -> SimulationComponentVectorView {
    let source = match read_word(cursor) {
        0 => SimulationComponentVectorSource::LocalFixed,
        1 => SimulationComponentVectorSource::Table,
        2 => SimulationComponentVectorSource::ConstantOrDefault,
        unknown => panic!("{unknown}; expected source tag 0..=2"),
    };
    let present = read_word(cursor);
    assert!(present <= 1);
    let rva = read_word(cursor);
    SimulationComponentVectorView {
        source,
        table_rva: (present == 1).then_some(rva),
        output_bits: std::array::from_fn(|_| read_word(cursor)),
    }
}

fn projected_view(view: SimulationComponentVectorView) -> [u32; 7] {
    let source = match view.source {
        SimulationComponentVectorSource::LocalFixed => 0,
        SimulationComponentVectorSource::Table => 1,
        SimulationComponentVectorSource::ConstantOrDefault => 2,
    };
    let [a, b, c, d] = view.output_bits;
    [
        source,
        u32::from(view.table_rva.is_some()),
        view.table_rva.unwrap_or(0),
        a,
        b,
        c,
        d,
    ]
}

fn replay_providers(mut cursor: &[u8], count: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let mut output = Vec::new();
    for _ in 0..count {
        let inputs = read_provider(&mut cursor);
        let original = inputs;
        let expected = read_view(&mut cursor);
        let actual = inputs.component_vector();
        assert_eq!(actual, expected, "{inputs:?}");
        assert_eq!(inputs, original);
        output.extend(projected_view(actual));
    }
    assert!(cursor.is_empty());
    output
}

fn replay_directions(mut cursor: &[u8], count: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let mut output = Vec::new();
    for _ in 0..count {
        let inputs = read_direction(&mut cursor);
        let original = inputs;
        let expected = read_view(&mut cursor);
        let actual = inputs.vector_direction();
        assert_eq!(actual, expected, "{inputs:?}");
        assert_eq!(inputs, original);
        output.extend(projected_view(actual));
    }
    assert!(cursor.is_empty());
    output
}

#[test]
fn every_provider_family_alias_table_entry_and_boundary_matches_source() {
    replay_providers(PROVIDERS, 106560);
}

#[test]
fn every_explicit_selector_and_loaded_text_branch_matches_source() {
    replay_directions(DIRECTIONS, 3488);
}

fn assert_parent_projection(mut cursor: &[u8], count: usize) {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    for _ in 0..count {
        let version = SimulationPersistVersion::new(read_word(&mut cursor) as i32).unwrap();
        let length = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, length);
        let parent = SimulationPostDialog::from_bytes(raw, version, LIMITS).unwrap();
        for _ in 0..2 {
            let expected = read_direction(&mut cursor);
            let actual = parent
                .vector_direction_inputs(expected.loaded_text_nonempty)
                .unwrap();
            assert_eq!(actual, expected);
            assert_eq!(actual.vector_direction(), read_view(&mut cursor));
        }
        assert_eq!(parent.to_bytes(), raw);
    }
    assert!(cursor.is_empty());
}

#[test]
fn all_owned_parents_preserve_wire_bytes_under_both_explicit_loaded_states() {
    assert_parent_projection(
        include_bytes!("fixtures/simulation-post-dialog/vector-parent-projection.bin"),
        225,
    );
}

#[test]
#[ignore = "Requires freshly pinned complete native vector parent projections."]
fn all_native_parents_preserve_wire_bytes_under_both_explicit_loaded_states() {
    let packet = std::fs::read("target/simulation-vector-parent-projection-native.bin").unwrap();
    assert_parent_projection(&packet, 224);
}

#[test]
#[ignore = "Requires freshly pinned original C-878/C-880 native controls."]
fn all_original_native_inputs_match_provider_and_conditional_composition() {
    replay_providers(
        &std::fs::read("target/simulation-component-vector-native.bin").unwrap(),
        224,
    );
    replay_directions(
        &std::fs::read("target/simulation-vector-direction-native.bin").unwrap(),
        224,
    );
}

#[test]
fn eight_workers_and_all_public_traits_match_serial_output_and_provenance() {
    fn require_threads<T: Send + Sync>() {}
    require_threads::<SimulationComponentVectorInputs>();
    require_threads::<SimulationVectorDirectionInputs>();
    require_threads::<SimulationComponentVectorView>();
    require_threads::<SimulationComponentVectorSource>();
    let provider = replay_providers(PROVIDERS, 106560);
    let direction = replay_directions(DIRECTIONS, 3488);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                assert_eq!(replay_providers(PROVIDERS, 106560), provider);
                assert_eq!(replay_directions(DIRECTIONS, 3488), direction);
            });
        }
    });
}

#[test]
#[ignore = "Requires fresh native inputs; optional output supports process replay."]
fn export_vector_results_for_fresh_process_replay() {
    let native_provider = std::fs::read("target/simulation-component-vector-native.bin").unwrap();
    let native_direction = std::fs::read("target/simulation-vector-direction-native.bin").unwrap();
    let words = replay_providers(PROVIDERS, 106560)
        .into_iter()
        .chain(replay_providers(&native_provider, 224))
        .chain(replay_directions(DIRECTIONS, 3488))
        .chain(replay_directions(&native_direction, 224));
    let output: Vec<u8> = words.flat_map(u32::to_le_bytes).collect();
    if let Some(path) = std::env::var_os("BESSEMER_VECTOR_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
