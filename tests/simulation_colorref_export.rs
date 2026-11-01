//! Conditional integer GetCOLORREF projection against independent copied-Source outputs (#166).
use bessemer::simulation::*;
#[allow(
    dead_code,
    reason = "Shared binary readers also serve complete field/history projections."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/colorref-export.bin");
const RGB: [SimulationColorChannel; 3] = [
    SimulationColorChannel::Red,
    SimulationColorChannel::Green,
    SimulationColorChannel::Blue,
];

fn assert_packet(mut cursor: &[u8], expected_count: usize) -> Vec<[u32; 2]> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, expected_count);
    let mut output = Vec::new();
    for _ in 0..expected_count {
        let raw = take_projection(&mut cursor, 16);
        let projected = read_word(&mut cursor);
        let packed = read_word(&mut cursor);
        output.push(assert_case(raw, projected, packed));
    }
    assert!(cursor.is_empty());
    output
}

fn assert_case(raw: &[u8], projected: u32, packed: u32) -> [u32; 2] {
    let color = SimulationRgbaColor::from_bytes(raw).unwrap();
    let result = color.colorref_nearest_even();
    let actual = match result {
        SimulationColorrefProjection::Projected { original, packed } => {
            assert_eq!(original, color);
            [1, packed]
        }
        SimulationColorrefProjection::OutsideModelDomain { original, .. } => {
            assert_eq!(original, color);
            [0, 0]
        }
    };
    assert_eq!(actual, [projected, packed], "{:08x?}", color.bits());
    assert_eq!(color.to_bytes(), raw);
    actual
}

#[test]
fn all_exponent_significand_and_signed_conversion_boundaries_match_source() {
    assert_packet(OWNED, 10503);
}

#[test]
#[ignore = "Requires freshly verified original native conditional projection expectations."]
fn all_original_native_color_projections_match_the_retained_source_model() {
    let packet = std::fs::read("target/simulation-colorref-export-native.bin").unwrap();
    assert_packet(&packet, 4836);
}

#[test]
fn documented_truncation_wrap_and_alpha_independence_are_exact() {
    let color = SimulationRgbaColor::from_bits([0x3f000000, 0xbf000000, 0x40000000, 0x7fa00042]);
    assert_eq!(
        color.colorref_nearest_even(),
        SimulationColorrefProjection::Projected {
            original: color,
            packed: 0x00fe817f
        }
    );
}

fn assert_excluded_channel(word: u32, reason: SimulationColorrefDomainFailure) {
    for (index, channel) in RGB.into_iter().enumerate() {
        let mut bits = [0, 0, 0, 0x7fa00042];
        bits[index] = word;
        let color = SimulationRgbaColor::from_bits(bits);
        assert_eq!(
            color.colorref_nearest_even(),
            SimulationColorrefProjection::OutsideModelDomain {
                original: color,
                channel,
                reason
            }
        );
    }
}

#[test]
fn excluded_domains_keep_the_original_and_identify_the_failing_channel() {
    use SimulationColorrefDomainFailure::*;
    for (word, reason) in [
        (0x7f800000, NonfiniteInput),
        (0x7fc00042, NonfiniteInput),
        (1, SubnormalInput),
        (0x7f7fffff, NonfiniteProduct),
        (0x4b008081, Signed32ProductRange),
        (0xcb008082, Signed32ProductRange),
    ] {
        assert_excluded_channel(word, reason);
    }
}

#[test]
fn projection_outcomes_and_domain_diagnostics_are_send_sync() {
    fn assert_threads<T: Send + Sync>() {}
    assert_threads::<SimulationColorrefProjection>();
    assert_threads::<SimulationColorrefDomainFailure>();
}

#[test]
fn eight_workers_match_all_serial_projection_and_exclusion_results() {
    let expected = assert_packet(OWNED, 10503);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let expected = &expected;
            scope.spawn(move || assert_eq!(&assert_packet(OWNED, 10503), expected));
        }
    });
}

#[test]
#[ignore = "Requires fresh native expectations; optional output supports process replay."]
fn export_owned_and_native_model_results_for_process_comparison() {
    let native = std::fs::read("target/simulation-colorref-export-native.bin").unwrap();
    let mut results = assert_packet(OWNED, 10503);
    results.extend(assert_packet(&native, 4836));
    let output: Vec<u8> = results
        .into_iter()
        .flatten()
        .flat_map(u32::to_le_bytes)
        .collect();
    assert_eq!(output.len(), (10503 + 4836) * 8);
    if let Some(path) = std::env::var_os("BESSEMER_COLORREF_EXPORT_OUTPUT") {
        std::fs::write(path, output).unwrap();
    }
}
