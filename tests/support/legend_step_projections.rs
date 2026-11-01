use super::case_support;
use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn dialog_step_inputs(raw: &[u8], version: SimulationPersistVersion) -> SimulationLegendStepInputs {
    let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
    let inputs = parent.legend_step_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_step_inputs().unwrap(), inputs);
    inputs
}

fn style_step_inputs(raw: &[u8], version: SimulationPersistVersion) -> SimulationLegendStepInputs {
    let parent = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let inputs = parent.legend_step_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_step_inputs().unwrap(), inputs);
    inputs
}

fn archive_step_inputs(family: u32, version: u32, raw: &[u8]) -> SimulationLegendStepInputs {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        return dialog_step_inputs(raw, version);
    }
    assert_eq!(family, 2);
    style_step_inputs(raw, version)
}

fn assert_stored_step(cursor: &mut &[u8], inputs: SimulationLegendStepInputs) -> [u32; 2] {
    let present = read_word(cursor);
    let raw = read_word(cursor);
    assert!(present <= 1);
    let expected = (present == 1).then_some(raw as i32);
    assert_eq!(inputs.layout_switch_raw, expected);
    assert_eq!(inputs.step_layout_nonzero(), expected.map(|word| word != 0));
    [present, raw]
}

fn replay_step_parent(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let inputs = archive_step_inputs(family, version, &raw);
    let mut words = assert_stored_step(cursor, inputs).to_vec();
    assert_eq!(read_word(cursor), 8);
    for _ in 0..8 {
        words.extend(case_support::assert_step_case(cursor));
    }
    let before = inputs;
    raw.fill(0xff);
    assert_eq!(inputs, before);
    words
}

pub fn replay_step_projection(mut cursor: &[u8], expected_count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), expected_count);
    let mut words = Vec::new();
    for _ in 0..expected_count {
        words.extend(replay_step_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}

pub fn replay_step_parallel(packet: &[u8], count: u32) -> Vec<u32> {
    let serial = replay_step_projection(packet, count);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| replay_step_projection(packet, count)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    serial
}
