use super::case_support;
use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn dialog_end_cell_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> SimulationLegendEndCellInputs {
    let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
    let inputs = parent.legend_end_cell_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_end_cell_inputs().unwrap(), inputs);
    inputs
}

fn style_end_cell_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> SimulationLegendEndCellInputs {
    let parent = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let inputs = parent.legend_end_cell_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_end_cell_inputs().unwrap(), inputs);
    inputs
}

fn archive_end_cell_inputs(family: u32, version: u32, raw: &[u8]) -> SimulationLegendEndCellInputs {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        return dialog_end_cell_inputs(raw, version);
    }
    assert_eq!(family, 2);
    style_end_cell_inputs(raw, version)
}

fn assert_stored_end_cell(cursor: &mut &[u8], inputs: SimulationLegendEndCellInputs) -> [u32; 6] {
    let expected: [u32; 6] = std::array::from_fn(|_| read_word(cursor));
    let [mask, flag, red, green, blue, alpha] = expected;
    assert!(mask < 4);
    assert_eq!(inputs.enabled_raw, (mask & 1 != 0).then_some(flag as i32));
    let color = SimulationRgbaColor::from_bits([red, green, blue, alpha]);
    assert_eq!(inputs.color, (mask & 2 != 0).then_some(color));
    expected
}

fn assert_end_cell_parent_case(
    runtime: SimulationLegendEndCellRuntime,
    position: usize,
    inputs: SimulationLegendEndCellInputs,
) {
    if position == 4 {
        assert_eq!(runtime.enabled_at_gate_raw, inputs.enabled_raw);
    }
    if (4..=5).contains(&position) {
        assert_eq!(runtime.color_at_rgb_copy, inputs.color);
    }
}

fn replay_end_cell_parent(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let inputs = archive_end_cell_inputs(family, version, &raw);
    let mut words = assert_stored_end_cell(cursor, inputs).to_vec();
    assert_eq!(read_word(cursor), 8);
    for position in 0..8 {
        let mut probe = *cursor;
        assert_end_cell_parent_case(
            case_support::read_end_cell_runtime(&mut probe),
            position,
            inputs,
        );
        words.extend(case_support::assert_end_cell_case(cursor));
    }
    let before = inputs;
    raw.fill(0xff);
    assert_eq!(inputs, before);
    words
}

pub fn replay_end_cell_projection(mut cursor: &[u8], expected_count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), expected_count);
    let mut words = Vec::new();
    for _ in 0..expected_count {
        words.extend(replay_end_cell_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}

pub fn replay_end_cell_parallel(packet: &[u8], count: u32) -> Vec<u32> {
    let serial = replay_end_cell_projection(packet, count);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| replay_end_cell_projection(packet, count)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    serial
}
