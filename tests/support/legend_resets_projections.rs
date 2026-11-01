use super::case_support;
use super::legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use super::load_projection::{read_word, take_projection};
use bessemer::simulation::*;

fn parent_reset_words(family: u32, version: SimulationPersistVersion, raw: &[u8]) -> [u32; 4] {
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(
            parent.legend_read_zero_assignments(),
            version.legend_read_zero_assignments()
        );
        let words = std::array::from_fn(|index| dialog_word(&parent, RESET_LABELS[index]));
        return optional_words(words);
    }
    assert_eq!(family, 2);
    let parent = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(
        parent.legend_read_zero_assignments(),
        version.legend_read_zero_assignments()
    );
    let words = std::array::from_fn(|index| style_word(&parent, RESET_LABELS[index]));
    optional_words(words)
}

const RESET_LABELS: [&str; 3] = ["legend.word_c0", "legend.word_c4", "legend.word_c8"];

fn dialog_word(parent: &SimulationPostDialog, label: &str) -> Option<i32> {
    parent
        .fields()
        .iter()
        .find(|field| field.path.as_ref() == label)
        .map(|field| {
            let SimulationPostDialogValue::Signed(word) = field.value else {
                panic!("{label}; expected signed word")
            };
            word
        })
}

fn style_word(parent: &SimulationPostStyle, label: &str) -> Option<i32> {
    parent
        .fields()
        .iter()
        .find(|field| field.path.as_ref() == label)
        .map(|field| {
            let SimulationPostStyleValue::Signed(word) = field.value else {
                panic!("{label}; expected signed word")
            };
            word
        })
}

fn optional_words(words: [Option<i32>; 3]) -> [u32; 4] {
    let mask = words.iter().enumerate().fold(0, |mask, (index, word)| {
        mask | (u32::from(word.is_some()) << index)
    });
    [
        mask,
        words[0].unwrap_or(0) as u32,
        words[1].unwrap_or(0) as u32,
        words[2].unwrap_or(0) as u32,
    ]
}

fn replay_reset_parent(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let stored = parent_reset_words(family, version, &raw);
    assert_eq!(stored, std::array::from_fn(|_| read_word(cursor)));
    let mut probe = *cursor;
    assert_eq!(read_word(&mut probe), version.value() as u32);
    let mut words = stored.to_vec();
    words.extend(case_support::assert_reset_case(cursor));
    let plan = version.legend_read_zero_assignments();
    raw.fill(0xff);
    assert_eq!(version.legend_read_zero_assignments(), plan);
    words
}

pub fn replay_reset_projection(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let mut words = Vec::new();
    for _ in 0..count {
        words.extend(replay_reset_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
