use super::case_support;
use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn parent_inverse_inputs(
    family: u32,
    version: SimulationPersistVersion,
    raw: &[u8],
) -> SimulationLegendPreferenceInputs {
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
        let inputs = parent.legend_preference_inputs().unwrap();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.legend_preference_inputs().unwrap(), inputs);
        return inputs;
    }
    assert_eq!(family, 2);
    let parent = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let inputs = parent.legend_preference_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_preference_inputs().unwrap(), inputs);
    inputs
}

fn replay_inverse_parent(cursor: &mut &[u8]) -> [u32; 15] {
    let family = read_word(cursor);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let inputs = parent_inverse_inputs(family, version, &raw);
    let mut probe = *cursor;
    let expected = case_support::read_inverse_source(&mut probe);
    assert_eq!(expected, SimulationLegendPreferenceSource::StoredA8(inputs));
    let words = case_support::assert_inverse_case(cursor);
    let before = inputs;
    raw.fill(0xff);
    assert_eq!(inputs, before);
    words
}

pub fn replay_inverse_projection(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let mut words = Vec::new();
    for _ in 0..count {
        words.extend(replay_inverse_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
