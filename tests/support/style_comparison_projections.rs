use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use super::{result_support::assert_style_pair, state_support::read_style_state};
use bessemer::simulation::*;

fn style_parent_state(
    family: u32,
    version: SimulationPersistVersion,
    raw: &[u8],
) -> SimulationStyleComparisonState {
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
        let state = parent.style_comparison_state().unwrap();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.style_comparison_state().unwrap(), state);
        return state;
    }
    assert_eq!(family, 2);
    let parent = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let state = parent.style_comparison_state().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.style_comparison_state().unwrap(), state);
    state
}

fn replay_style_parent(cursor: &mut &[u8]) -> [u32; 25] {
    let family = read_word(cursor);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let state = style_parent_state(family, version, &raw);
    let expected = read_style_state(cursor);
    assert_eq!(state, expected);
    let words = assert_style_pair(&state, &state, cursor);
    raw.fill(0xff);
    assert_eq!(state, expected);
    words
}

pub fn replay_style_projections(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let mut words = Vec::new();
    for _ in 0..count {
        words.extend(replay_style_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
