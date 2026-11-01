//! C858 semantic C0 views preserve stored absence and every raw signed pattern (#166).
use bessemer::simulation::*;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use legend_limits::{LEGEND_DIALOG_LIMITS, LEGEND_STYLE_LIMITS};
use load_projection::{read_word, take_projection};
const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/legend-resets-projection.bin");

fn grouping_parent(
    family: u32,
    version: SimulationPersistVersion,
    raw: &[u8],
) -> SimulationLegendGroupingInputs {
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, LEGEND_DIALOG_LIMITS).unwrap();
        let inputs = parent.legend_grouping_inputs().unwrap();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.legend_grouping_inputs().unwrap(), inputs);
        return inputs;
    }
    assert_eq!(family, 2);
    let parent = SimulationPostStyle::from_bytes(raw, version, LEGEND_STYLE_LIMITS).unwrap();
    let inputs = parent.legend_grouping_inputs().unwrap();
    assert_eq!(parent.to_bytes(), raw);
    assert_eq!(parent.legend_grouping_inputs().unwrap(), inputs);
    inputs
}

fn replay_grouping_parent(cursor: &mut &[u8]) -> [u32; 2] {
    let family = read_word(cursor);
    let version = SimulationPersistVersion::new(read_word(cursor) as i32).unwrap();
    let size = read_word(cursor) as usize;
    let mut raw = take_projection(cursor, size).to_vec();
    let inputs = grouping_parent(family, version, &raw);
    let present = read_word(cursor) & 1;
    let word = read_word(cursor);
    take_projection(cursor, 13 * 4);
    assert_eq!(
        inputs.locale_thousands_grouping_enabled_raw,
        (present == 1).then_some(word as i32)
    );
    assert_eq!(present == 1, version.value() >= 840008);
    let before = inputs;
    raw.fill(0xff);
    assert_eq!(inputs, before);
    [present, word]
}

fn replay_grouping(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let mut words = Vec::new();
    for _ in 0..count {
        words.extend(replay_grouping_parent(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}

#[test]
fn complete_owned_grouping_views_keep_absence_and_unrestricted_raw_words() {
    let words = replay_grouping(OWNED, 557);
    let stored: Vec<_> = words
        .as_chunks::<2>()
        .0
        .iter()
        .filter(|row| row[0] == 1)
        .map(|row| row[1] as i32)
        .collect();
    assert_eq!(stored.len(), 356);
    assert!(stored.contains(&0));
    assert!(stored.iter().any(|word| *word > 1));
    assert!(stored.iter().any(|word| *word < 0));
}

#[test]
fn public_grouping_inputs_are_send_sync_and_copied_values_are_independent() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendGroupingInputs>();
    let original = SimulationLegendGroupingInputs {
        locale_thousands_grouping_enabled_raw: Some(i32::MIN),
    };
    let mut copied = original;
    copied.locale_thousands_grouping_enabled_raw = None;
    assert_eq!(
        original.locale_thousands_grouping_enabled_raw,
        Some(i32::MIN)
    );
    assert_ne!(original, copied);
}

#[test]
fn eight_grouping_workers_match_serial_complete_parent_views() {
    let serial = replay_grouping(OWNED, 557);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| replay_grouping(OWNED, 557)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
#[ignore = "Requires freshly frozen native reference packets."]
fn native_grouping_views_and_process_export() {
    let native = std::fs::read("target/simulation-legend-resets-projection-native.bin").unwrap();
    let mut words = replay_grouping(OWNED, 557);
    let native_words = replay_grouping(&native, 276);
    assert_eq!(count_grouping_pattern(&native_words, None), 160);
    assert_eq!(count_grouping_pattern(&native_words, Some(1)), 84);
    assert_eq!(count_grouping_pattern(&native_words, Some(0)), 76);
    words.extend(native_words);
    assert_eq!(words.len(), 1666);
    export_grouping_words(words);
}

fn count_grouping_pattern(words: &[u32], selected: Option<u32>) -> usize {
    words
        .as_chunks::<2>()
        .0
        .iter()
        .filter(|row| row[0] == 1 && selected.is_none_or(|word| row[1] == word))
        .count()
}

fn export_grouping_words(words: Vec<u32>) {
    if let Some(path) = std::env::var_os("BESSEMER_LEGEND_GROUPING_OUTPUT") {
        std::fs::write(
            path,
            words
                .into_iter()
                .flat_map(u32::to_le_bytes)
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }
}
