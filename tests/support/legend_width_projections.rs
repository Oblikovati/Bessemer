use super::case_support;
use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn dialog_width_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> SimulationLegendWidthInputs {
    let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
    let inputs = parent.legend_width_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_width_inputs().unwrap(), inputs);
    inputs
}

fn style_width_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> SimulationLegendWidthInputs {
    let parent = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let inputs = parent.legend_width_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_width_inputs().unwrap(), inputs);
    inputs
}

fn archive_width_inputs(family: u32, version: u32, raw: &[u8]) -> SimulationLegendWidthInputs {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        return dialog_width_inputs(raw, version);
    }
    assert_eq!(family, 2);
    style_width_inputs(raw, version)
}

fn stored_width_words(cursor: &mut &[u8], inputs: SimulationLegendWidthInputs) -> [u32; 2] {
    let present = read_word(cursor);
    let selector = read_word(cursor);
    assert!(present <= 1);
    assert_eq!(
        inputs.selector_raw,
        (present == 1).then_some(selector as i32)
    );
    [present, selector]
}

fn replay_width_contexts(cursor: &mut &[u8], inputs: SimulationLegendWidthInputs) -> Vec<u32> {
    assert_eq!(read_word(cursor), 8);
    let mut words = Vec::new();
    for position in 0..8 {
        let mut probe = *cursor;
        let runtime = case_support::read_width_runtime(&mut probe);
        if (3..=5).contains(&position) {
            assert_eq!(runtime.selector_at_entry_raw, inputs.selector_raw);
        }
        words.extend(case_support::assert_width_case(cursor));
    }
    words
}

fn replay_width_parent(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let inputs = archive_width_inputs(family, version, &raw);
    let mut words = stored_width_words(cursor, inputs).to_vec();
    words.extend(replay_width_contexts(cursor, inputs));
    let before = inputs;
    raw.fill(0xff);
    assert_eq!(inputs, before);
    words
}

pub fn replay_width_projection(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let mut words = Vec::new();
    for _ in 0..count {
        words.extend(replay_width_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}

pub fn replay_width_parallel(packet: &[u8], count: u32) -> Vec<u32> {
    let serial = replay_width_projection(packet, count);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| replay_width_projection(packet, count)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    serial
}
