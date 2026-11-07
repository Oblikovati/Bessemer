//! Explicit nearest-even packed import/assignment against independent copied-Source outputs (#166).
use bessemer::simulation::SimulationRgbaColor;
#[allow(
    dead_code,
    reason = "Shared binary readers also serve full field and history projections."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/colorref-import.bin");

#[derive(Clone, Copy)]
struct ImportCase<'a> {
    packed: u32,
    original: &'a [u8],
    constructed: &'a [u8],
    assigned: &'a [u8],
}

fn read_cases(mut cursor: &[u8], expected: usize) -> Vec<ImportCase<'_>> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, expected);
    let cases = (0..expected)
        .map(|_| ImportCase {
            packed: read_word(&mut cursor),
            original: take_projection(&mut cursor, 16),
            constructed: take_projection(&mut cursor, 16),
            assigned: take_projection(&mut cursor, 16),
        })
        .collect();
    assert!(cursor.is_empty());
    cases
}

fn assert_case(case: ImportCase<'_>) -> [u8; 32] {
    let original = SimulationRgbaColor::from_bytes(case.original).unwrap();
    let constructed = SimulationRgbaColor::from_colorref_nearest_even(case.packed);
    let assigned = original.assigned_colorref_nearest_even(case.packed);
    assert_eq!(constructed.to_bytes(), case.constructed);
    assert_eq!(assigned.to_bytes(), case.assigned);
    assert_eq!(original.to_bytes(), case.original);
    assert_eq!(constructed.bits()[3], 0x3f800000);
    assert_eq!(assigned.bits()[3], original.bits()[3]);
    let mut projected = [0; 32];
    projected[..16].copy_from_slice(&constructed.to_bytes());
    projected[16..].copy_from_slice(&assigned.to_bytes());
    projected
}

#[test]
fn exhaustive_channels_mixed_words_and_alpha_patterns_match_independent_source() {
    for case in read_cases(OWNED, 1025) {
        assert_case(case);
    }
}

#[test]
#[ignore = "Requires fresh pinned constructor/assignment native expectations."]
fn all_original_native_colors_match_their_constructor_and_assignment_models() {
    let packet = std::fs::read("target/simulation-colorref-import-native.bin").unwrap();
    for case in read_cases(&packet, 9672) {
        assert_case(case);
    }
}

#[test]
fn eight_workers_match_exact_serial_outputs_for_every_owned_import_case() {
    let cases = read_cases(OWNED, 1025);
    let serial: Vec<_> = cases.iter().copied().map(assert_case).collect();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| replay_serial(&cases, &serial));
        }
    });
}

fn replay_serial(cases: &[ImportCase<'_>], serial: &[[u8; 32]]) {
    for (case, expected) in cases.iter().zip(serial) {
        assert_eq!(&assert_case(*case), expected);
    }
}

#[test]
#[ignore = "Requires fresh native input; optional output supports fresh-process replay."]
fn export_owned_and_native_projections_for_process_replay() {
    let native = std::fs::read("target/simulation-colorref-import-native.bin").unwrap();
    let mut cases = read_cases(OWNED, 1025);
    cases.extend(read_cases(&native, 9672));
    let mut output = Vec::new();
    for case in cases {
        output.extend(assert_case(case));
    }
    assert_eq!(output.len(), (1025 + 9672) * 32);
    if let Some(path) = std::env::var_os("BESSEMER_COLORREF_IMPORT_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
