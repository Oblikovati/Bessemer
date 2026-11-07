use super::case_support;
use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn dialog_hidden_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> SimulationHiddenPartsMigrationInputs {
    let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
    let inputs = parent.hidden_parts_migration_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.hidden_parts_migration_inputs().unwrap(), inputs);
    inputs
}

fn style_hidden_inputs(
    raw: &[u8],
    version: SimulationPersistVersion,
) -> SimulationHiddenPartsMigrationInputs {
    let parent = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let inputs = parent.hidden_parts_migration_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.hidden_parts_migration_inputs().unwrap(), inputs);
    inputs
}

fn archive_hidden_inputs(
    family: u32,
    version: u32,
    raw: &[u8],
) -> SimulationHiddenPartsMigrationInputs {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        return dialog_hidden_inputs(raw, version);
    }
    assert_eq!(family, 2);
    style_hidden_inputs(raw, version)
}

fn stored_hidden_words(
    cursor: &mut &[u8],
    inputs: SimulationHiddenPartsMigrationInputs,
) -> [u32; 2] {
    let present = read_word(cursor);
    let word = read_word(cursor);
    assert!(present <= 1);
    assert_eq!(
        inputs.stored_word_1f0_raw,
        (present == 1).then_some(word as i32)
    );
    [present, word]
}

fn replay_hidden_contexts(
    cursor: &mut &[u8],
    inputs: SimulationHiddenPartsMigrationInputs,
) -> Vec<u32> {
    assert_eq!(read_word(cursor), 8);
    let mut words = Vec::new();
    for position in 0..8 {
        let mut probe = *cursor;
        let supplied = case_support::read_hidden_inputs(&mut probe);
        assert_eq!(supplied.version, inputs.version);
        if position == 0 {
            assert_eq!(supplied, inputs);
        }
        words.extend(case_support::assert_hidden_case(cursor));
    }
    words
}

fn replay_hidden_parent(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let inputs = archive_hidden_inputs(family, version, &raw);
    assert_eq!(inputs.version.value(), version as i32);
    let mut words = stored_hidden_words(cursor, inputs).to_vec();
    words.extend(replay_hidden_contexts(cursor, inputs));
    let before = inputs;
    raw.fill(0xff);
    assert_eq!(inputs, before);
    words
}

pub fn replay_hidden_projection(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let mut words = Vec::new();
    for _ in 0..count {
        words.extend(replay_hidden_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}

pub fn replay_hidden_parallel(packet: &[u8], count: u32) -> Vec<u32> {
    let serial = replay_hidden_projection(packet, count);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| replay_hidden_projection(packet, count)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
    serial
}
