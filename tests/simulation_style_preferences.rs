//! C-885 raw state and independently traced local preference decisions (#166).
use bessemer::{FailureClass, simulation::*};
#[allow(dead_code, reason = "Shared support also serves full history packets.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};

const POLICY: &[u8] = include_bytes!("fixtures/simulation-post-dialog/raw-preferences-policy.bin");
const VIEWS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/raw-preferences-projection.bin");
const MEMBERS: [&str; 12] = [
    "1f0", "204", "208", "230", "234", "238", "23c", "240", "28c", "290", "294", "298",
];
const TRANSFERS: [usize; 4] = [1, 2, 3, 7];
const PARENTS: SimulationPostDialogLimits = SimulationPostDialogLimits {
    maximum_bytes: 10000000,
    maximum_fields: 200000,
    maximum_entries: 10000000,
    maximum_text_bytes: 0x200000,
    embedded: bessemer::model_archives::ModelArchiveLimits {
        maximum_bytes: 1000000,
        maximum_fields: 100000,
        maximum_entries: 1000000,
        maximum_depth: 128,
        maximum_objects: 100000,
    },
};
const STYLES: SimulationPostStyleLimits = SimulationPostStyleLimits {
    maximum_bytes: 4096,
    maximum_fields: 116,
    maximum_entries: 512,
    maximum_text_bytes: 1024,
    maximum_characters: 512,
};

fn from_slots(slots: [Option<u32>; 12]) -> SimulationStyleRawWords {
    SimulationStyleRawWords {
        word_1f0: slots[0],
        word_204: slots[1],
        word_208: slots[2],
        word_230: slots[3],
        word_234: slots[4],
        word_238: slots[5],
        word_23c: slots[6],
        word_240: slots[7],
        word_28c: slots[8],
        word_290: slots[9],
        word_294: slots[10],
        word_298: slots[11],
    }
}

fn slots_of(words: SimulationStyleRawWords) -> [Option<u32>; 12] {
    [
        words.word_1f0,
        words.word_204,
        words.word_208,
        words.word_230,
        words.word_234,
        words.word_238,
        words.word_23c,
        words.word_240,
        words.word_28c,
        words.word_290,
        words.word_294,
        words.word_298,
    ]
}

fn read_slot(cursor: &mut &[u8]) -> Option<u32> {
    let present = read_word(cursor);
    let word = read_word(cursor);
    assert!(present <= 1);
    if present == 0 {
        assert_eq!(word, 0);
    }
    (present == 1).then_some(word)
}

fn encoded_slots(slots: [Option<u32>; 12]) -> Vec<u32> {
    slots
        .into_iter()
        .flat_map(|word| [u32::from(word.is_some()), word.unwrap_or(0)])
        .collect()
}

fn assert_export(words: SimulationStyleRawWords, expected: [Option<u32>; 12]) -> Vec<u32> {
    let actual = words.local_preference_export();
    if let Some(index) = TRANSFERS
        .iter()
        .position(|index| expected[*index].is_none())
    {
        return assert_missing_export(actual.unwrap_err(), index);
    }
    let actual = actual.unwrap();
    let result = [
        actual.word_204,
        actual.word_208,
        actual.word_230,
        actual.word_240,
    ];
    assert_eq!(result, TRANSFERS.map(|index| expected[index].unwrap()));
    [vec![0], result.to_vec()].concat()
}

fn assert_missing_export(error: bessemer::BessemerError, index: usize) -> Vec<u32> {
    assert_eq!(error.class, FailureClass::Missing);
    assert_eq!(
        error.entity,
        format!("style.word_{}", MEMBERS[TRANSFERS[index]])
    );
    assert!(error.cause.contains("word None"));
    vec![index as u32 + 1, 0, 0, 0, 0]
}

fn assert_set(
    words: SimulationStyleRawWords,
    arguments: [u32; 6],
    pointed: Option<i32>,
    enters: u32,
) -> (SimulationStyleRawWords, [Option<u32>; 12]) {
    let incoming = SimulationStylePreferenceWords {
        word_204: arguments[0],
        word_208: arguments[1],
        word_230: arguments[2],
        word_240: arguments[3],
    };
    assert!(enters <= 1);
    let mut expected = slots_of(words);
    if enters == 1 {
        for (index, value) in TRANSFERS.into_iter().zip(arguments) {
            expected[index] = Some(value);
        }
    }
    let actual = words.after_local_preference_set(incoming, pointed);
    assert_eq!(slots_of(actual), expected);
    (actual, expected)
}

fn assert_copy(
    words: SimulationStyleRawWords,
    arguments: [u32; 6],
    pointed: Option<i32>,
    clears: u32,
) -> (SimulationStyleRawWords, [Option<u32>; 12]) {
    assert!(clears <= 1);
    let mut expected = slots_of(words);
    expected[4] = Some(if clears == 1 { 0 } else { arguments[4] });
    expected[5] = Some(if clears == 1 { 0 } else { arguments[5] });
    let actual = words.after_local_preference_copy(arguments[4], arguments[5], pointed);
    assert_eq!(slots_of(actual), expected);
    (actual, expected)
}

fn replay_policy_case(cursor: &mut &[u8]) -> Vec<u32> {
    let present = read_word(cursor);
    assert!(present <= 1);
    let pointed = read_word(cursor) as i32;
    let (enters, clears) = (read_word(cursor), read_word(cursor));
    let before = std::array::from_fn(|_| read_slot(cursor));
    let words = from_slots(before);
    let arguments = std::array::from_fn(|_| read_word(cursor));
    let pointed = (present == 1).then_some(pointed);
    let (set, set_slots) = assert_set(words, arguments, pointed, enters);
    let (copy, copy_slots) = assert_copy(words, arguments, pointed, clears);
    assert_eq!(slots_of(words), before);
    [
        encoded_slots(set_slots),
        encoded_slots(copy_slots),
        assert_export(words, before),
        assert_export(set, set_slots),
        assert_export(copy, copy_slots),
    ]
    .concat()
}

fn replay_policies(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 11518);
    let output = (0..count)
        .flat_map(|_| replay_policy_case(&mut cursor))
        .collect();
    assert!(cursor.is_empty());
    output
}

fn stored_style_words(family: u32, version: u32, raw: &[u8]) -> SimulationStyleRawWords {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let words = parent.raw_style_words().unwrap();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.raw_style_words().unwrap(), words);
        return words;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let words = style.raw_style_words().unwrap();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.raw_style_words().unwrap(), words);
    words
}

fn replay_projections(mut cursor: &[u8], count: usize) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor) as usize, count);
    let mut output = Vec::new();
    for _ in 0..count {
        let family = read_word(&mut cursor);
        let version = read_word(&mut cursor);
        let length = read_word(&mut cursor) as usize;
        let raw = take_projection(&mut cursor, length);
        let expected = std::array::from_fn(|_| read_slot(&mut cursor));
        let actual = stored_style_words(family, version, raw);
        assert_eq!(slots_of(actual), expected);
        output.extend(encoded_slots(expected));
        output.extend(assert_export(actual, expected));
    }
    assert!(cursor.is_empty());
    output
}

#[test]
fn all_instruction_traced_local_transitions_preserve_input_and_required_missing_state() {
    replay_policies(POLICY);
}

#[test]
fn split_initializer_stores_remain_explicit_and_are_never_file_defaults() {
    assert_eq!(
        slots_of(SimulationStyleRawWords::initializer_words()),
        [0, 0, 0, 1, 0, 0, 0, 1, 200, 50, 0, 1].map(Some)
    );
    let absent = from_slots([None; 12]);
    assert_eq!(slots_of(absent), [None; 12]);
    assert!(absent.local_preference_export().is_err());
}

#[test]
fn all_owned_views_retain_raw_words_presence_and_archive_bytes() {
    replay_projections(VIEWS, 425);
}

#[test]
fn canonical_owned_style_views_match_across_eight_workers() {
    let serial = replay_projections(VIEWS, 425);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| assert_eq!(replay_projections(VIEWS, 425), serial));
        }
    });
}

#[test]
fn public_value_types_and_eight_workers_match_serial_results() {
    fn require_threads<T: Send + Sync>() {}
    require_threads::<SimulationStyleRawWords>();
    require_threads::<SimulationStylePreferenceWords>();
    let serial = replay_policies(POLICY);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| assert_eq!(replay_policies(POLICY), serial));
        }
    });
}

#[test]
#[ignore = "Requires fresh complete pinned native raw-word projections."]
fn all_native_views_retain_raw_words_presence_and_archive_bytes() {
    replay_projections(
        &std::fs::read("target/simulation-raw-preferences-projection-native.bin").unwrap(),
        276,
    );
}

#[test]
#[ignore = "Requires native projections; output supports independent process replay."]
fn export_all_raw_preference_results_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-raw-preferences-projection-native.bin").unwrap();
    let output = [
        replay_policies(POLICY),
        replay_projections(VIEWS, 425),
        replay_projections(&native, 276),
    ]
    .concat();
    if let Some(path) = std::env::var_os("BESSEMER_RAW_PREFERENCES_OUTPUT") {
        std::fs::write(
            path,
            output
                .into_iter()
                .flat_map(u32::to_le_bytes)
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }
}
