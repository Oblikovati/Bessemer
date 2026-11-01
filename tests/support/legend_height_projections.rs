use super::case_support;
use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn parent_height_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> SimulationLegendHeightInputs {
    let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
    let inputs = parent.legend_height_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_height_inputs().unwrap(), inputs);
    inputs
}

fn style_height_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> SimulationLegendHeightInputs {
    let style = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let inputs = style.legend_height_inputs().unwrap();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.legend_height_inputs().unwrap(), inputs);
    inputs
}

fn archive_height_inputs(family: u32, version: u32, raw: &[u8]) -> SimulationLegendHeightInputs {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        return parent_height_inputs(raw, version);
    }
    assert_eq!(family, 2);
    style_height_inputs(raw, version)
}

fn replay_height_parent(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let inputs = archive_height_inputs(family, version, &raw);
    assert_eq!(read_word(cursor), 5);
    let mut output = Vec::new();
    for _ in 0..5 {
        let mut probe = *cursor;
        assert_eq!(case_support::read_height_case(&mut probe).inputs, inputs);
        output.extend(case_support::assert_height_case(cursor));
    }
    let before = inputs;
    raw.fill(0xff);
    assert_eq!(inputs, before);
    output
}

pub fn replay_height_projection(mut cursor: &[u8], expected_count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), expected_count);
    let mut output = Vec::new();
    for _ in 0..expected_count {
        output.extend(replay_height_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    output
}

pub fn replay_height_parallel(packet: &[u8], count: u32) -> Vec<u32> {
    let serial = replay_height_projection(packet, count);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| replay_height_projection(packet, count)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    serial
}
