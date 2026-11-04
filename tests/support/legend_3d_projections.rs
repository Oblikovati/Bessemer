use super::case_support::{self, Fixture3DCase};
use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn dialog_3d_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> (SimulationLegendHeightInputs, Option<i32>) {
    let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
    let inputs = (
        parent.legend_height_inputs().unwrap(),
        parent.legend_format_inputs().unwrap().precision_raw,
    );
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_height_inputs().unwrap(), inputs.0);
    assert_eq!(
        parent.legend_format_inputs().unwrap().precision_raw,
        inputs.1
    );
    inputs
}

fn style_3d_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> (SimulationLegendHeightInputs, Option<i32>) {
    let parent = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let inputs = (
        parent.legend_height_inputs().unwrap(),
        parent.legend_format_inputs().unwrap().precision_raw,
    );
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_height_inputs().unwrap(), inputs.0);
    assert_eq!(
        parent.legend_format_inputs().unwrap().precision_raw,
        inputs.1
    );
    inputs
}

fn archive_3d_inputs(
    family: u32,
    version: u32,
    raw: &[u8],
) -> (SimulationLegendHeightInputs, Option<i32>) {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        return dialog_3d_inputs(raw, version);
    }
    assert_eq!(family, 2);
    style_3d_inputs(raw, version)
}

fn assert_parent_3d_case(
    case: Fixture3DCase,
    position: usize,
    inputs: (SimulationLegendHeightInputs, Option<i32>),
) {
    match case {
        Fixture3DCase::Body(actual, _) => {
            assert!(position < 8);
            assert_eq!(actual, inputs.0);
        }
        Fixture3DCase::Background(actual) => {
            assert!(position >= 8);
            if position <= 12 {
                assert_eq!(actual.precision_at_width_raw, inputs.1);
            }
        }
    }
}

fn replay_3d_parent(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let inputs = archive_3d_inputs(family, version, &raw);
    assert_eq!(read_word(cursor), 16);
    let mut words = Vec::new();
    for position in 0..16 {
        let mut probe = *cursor;
        assert_parent_3d_case(case_support::read_3d_case(&mut probe), position, inputs);
        words.extend(case_support::assert_3d_case(cursor));
    }
    let before = inputs;
    raw.fill(0xff);
    assert_eq!(inputs, before);
    words
}

pub fn replay_3d_projection(mut cursor: &[u8], expected_count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), expected_count);
    let mut words = Vec::new();
    for _ in 0..expected_count {
        words.extend(replay_3d_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}

pub fn replay_3d_parallel(packet: &[u8], count: u32) -> Vec<u32> {
    let serial = replay_3d_projection(packet, count);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| replay_3d_projection(packet, count)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    serial
}
