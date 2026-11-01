//! Conditional native color predicates stay separate from stored bitwise equality (#166).
use bessemer::simulation::SimulationRgbaColor;
#[allow(
    dead_code,
    reason = "Shared binary readers also serve full field/history projections."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/rgba-comparison.bin");

#[derive(Clone, Copy)]
struct ComparisonCase {
    left: SimulationRgbaColor,
    right: SimulationRgbaColor,
    equal: u32,
    different: u32,
}

fn read_cases(mut cursor: &[u8], count: usize) -> Vec<ComparisonCase> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let cases = (0..count)
        .map(|_| ComparisonCase {
            left: SimulationRgbaColor::from_bytes(take_projection(&mut cursor, 16)).unwrap(),
            right: SimulationRgbaColor::from_bytes(take_projection(&mut cursor, 16)).unwrap(),
            equal: read_word(&mut cursor),
            different: read_word(&mut cursor),
        })
        .collect();
    assert!(cursor.is_empty());
    cases
}

fn assert_case(case: ComparisonCase) -> [u32; 2] {
    let original = (case.left.to_bytes(), case.right.to_bytes());
    let equal = case.left.equal_masked_exceptions_daz_off(case.right);
    let different = case.left.different_masked_exceptions_daz_off(case.right);
    assert_eq!(
        [u32::from(equal), u32::from(different)],
        [case.equal, case.different]
    );
    assert_eq!(equal, case.right.equal_masked_exceptions_daz_off(case.left));
    assert_eq!(different, !equal);
    assert_eq!((case.left.to_bytes(), case.right.to_bytes()), original);
    [u32::from(equal), u32::from(different)]
}

#[test]
fn every_channel_bit_class_and_mixed_pair_matches_the_independent_source() {
    for case in read_cases(OWNED, 2048) {
        assert_case(case);
    }
}

#[test]
#[ignore = "Requires fresh pinned native raw-color pair expectations."]
fn every_ordered_native_color_pair_matches_the_declared_model() {
    let native = std::fs::read("target/simulation-rgba-comparison-native.bin").unwrap();
    for case in read_cases(&native, 676) {
        assert_case(case);
    }
}

#[test]
fn native_signed_zero_equivalence_does_not_redefine_stored_bitwise_equality() {
    let positive = SimulationRgbaColor::from_bits([0, 0, 0, 0x3f800000]);
    let negative = SimulationRgbaColor::from_bits([0x80000000, 0, 0, 0x3f800000]);
    assert_ne!(positive, negative);
    assert!(positive.equal_masked_exceptions_daz_off(negative));
}

#[test]
fn unordered_nan_model_is_nontransitive_and_alpha_still_participates() {
    let first = SimulationRgbaColor::from_bits([0, 0, 0, 0x3f800000]);
    let wildcard = SimulationRgbaColor::from_bits([0x7fa00042, 0, 0, 0x3f800000]);
    let second = SimulationRgbaColor::from_bits([0x3f800000, 0, 0, 0x3f800000]);
    assert!(first.equal_masked_exceptions_daz_off(wildcard));
    assert!(wildcard.equal_masked_exceptions_daz_off(second));
    assert!(!first.equal_masked_exceptions_daz_off(second));
    let other_alpha = SimulationRgbaColor::from_bits([0, 0, 0, 0x3f000000]);
    assert!(first.different_masked_exceptions_daz_off(other_alpha));
}

#[test]
fn eight_workers_match_all_serial_comparison_results() {
    let cases = read_cases(OWNED, 2048);
    let expected: Vec<_> = cases.iter().copied().map(assert_case).collect();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| replay_cases(&cases, &expected));
        }
    });
}

fn replay_cases(cases: &[ComparisonCase], expected: &[[u32; 2]]) {
    for (case, result) in cases.iter().zip(expected) {
        assert_eq!(&assert_case(*case), result);
    }
}

#[test]
#[ignore = "Requires fresh native cases; optional output supports process comparison."]
fn export_all_comparison_results_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-rgba-comparison-native.bin").unwrap();
    let mut cases = read_cases(OWNED, 2048);
    cases.extend(read_cases(&native, 676));
    let output: Vec<u8> = cases
        .into_iter()
        .flat_map(assert_case)
        .flat_map(u32::to_le_bytes)
        .collect();
    assert_eq!(output.len(), (2048 + 676) * 8);
    if let Some(path) = std::env::var_os("BESSEMER_RGBA_COMPARISON_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
