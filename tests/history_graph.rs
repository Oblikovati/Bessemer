//! Shared references, structural edits, and extended archive-map indices.

use std::collections::BTreeMap;

use bessemer::history::{
    ArchiveString, DateCodeHistory, DateCodeId, HistoryContents, HistoryTail, RevisionStamp,
    StateDateCodes, TextEncoding, VersionHistory, VersionState,
};

const NATIVE: &[u8] = include_bytes!("fixtures/history/blank-part.bin");
const MIXED: &[u8] = include_bytes!("fixtures/history/mixed.bin");

#[test]
fn changing_a_shared_date_history_updates_all_referring_states() {
    let source = VersionHistory::from_bytes(MIXED).unwrap();
    let mut contents = source.contents().clone();
    let StateDateCodes::History(identity) = contents.states[3].date_codes else {
        panic!("modern fixture state");
    };
    contents
        .date_histories
        .get_mut(&identity)
        .unwrap()
        .codes
        .push(0xf123_4567);
    let encoded = source.with_contents(contents).unwrap().to_bytes().unwrap();
    let reopened = VersionHistory::from_bytes(&encoded).unwrap();
    for index in [3, 5, 9] {
        assert_eq!(
            reopened.contents().states[index].date_codes,
            StateDateCodes::History(identity)
        );
    }
    assert_eq!(
        reopened.contents().date_histories[&identity].codes,
        [0x0100_0001, u32::MAX, 0xf123_4567]
    );
    assert_eq!(source.to_bytes().unwrap(), MIXED);
    assert_eq!(
        source.contents().date_histories[&identity].codes,
        [0x0100_0001, u32::MAX]
    );
}

#[test]
fn adding_removing_and_reordering_states_uses_the_last_version_for_the_tail() {
    let source = VersionHistory::from_bytes(MIXED).unwrap();
    let mut contents = source.contents().clone();
    contents.states.reverse();
    contents.tail = HistoryTail::default();
    let reversed = source.with_contents(contents).unwrap();
    let reopened = VersionHistory::from_bytes(&reversed.to_bytes().unwrap()).unwrap();
    assert_eq!(reopened.contents().states.last().unwrap().file_version, 38);
    let mut ancient = reopened.contents().clone();
    ancient.states.retain(|state| state.file_version < 0x175);
    ancient.date_histories.clear();
    ancient.states.push(ancient.states[0].clone());
    let edited = reopened.with_contents(ancient).unwrap();
    let parsed = VersionHistory::from_bytes(&edited.to_bytes().unwrap()).unwrap();
    assert_eq!(parsed.contents().states.len(), 2);
    assert!(parsed.contents().date_histories.is_empty());
    assert_eq!(source.to_bytes().unwrap(), MIXED);
}

#[test]
fn a_new_ancient_history_needs_no_modern_objects_or_tail() {
    let stamp = RevisionStamp {
        revision_status: 2,
        unknown_0c: 17,
        unknown_10: 29,
        time: 123,
        comment: ArchiveString::new("text", TextEncoding::Latin1).unwrap(),
    };
    let state = VersionState {
        stamp,
        file_version: 38,
        date_codes: StateDateCodes::Absent,
    };
    let contents = HistoryContents {
        states: vec![state],
        date_histories: BTreeMap::new(),
        tail: HistoryTail::default(),
    };
    let history = VersionHistory::new(99, contents.clone()).unwrap();
    let parsed = VersionHistory::from_bytes(&history.to_bytes().unwrap()).unwrap();
    assert_eq!(parsed.contents(), &contents);
    assert_eq!(parsed.root_schema(), 99);
}

#[test]
fn distinct_objects_with_equal_values_are_not_merged() {
    let source = VersionHistory::from_bytes(NATIVE).unwrap();
    let mut contents = source.contents().clone();
    let record = contents.date_histories.values().next().unwrap().clone();
    let identity = DateCodeId::new(100).unwrap();
    contents.date_histories.insert(identity, record);
    let mut state = contents.states[0].clone();
    state.date_codes = StateDateCodes::History(identity);
    contents.states.push(state);
    let edited = source.with_contents(contents).unwrap();
    let parsed = VersionHistory::from_bytes(&edited.to_bytes().unwrap()).unwrap();
    assert_eq!(parsed.contents().date_histories.len(), 2);
    assert_ne!(
        parsed.contents().states[0].date_codes,
        parsed.contents().states[1].date_codes
    );
}

#[test]
fn encoder_and_decoder_cross_the_32_bit_class_and_object_tag_boundary() {
    let source = VersionHistory::from_bytes(NATIVE).unwrap();
    let mut contents = source.contents().clone();
    let template = contents.states[0].clone();
    contents.states.clear();
    contents.date_histories.clear();
    for ordinal in 0..16_384_u32 {
        let identity = DateCodeId::new(ordinal + 100).unwrap();
        contents.date_histories.insert(
            identity,
            DateCodeHistory {
                schema: ordinal as u16,
                codes: vec![ordinal],
            },
        );
        let mut state = template.clone();
        state.date_codes = StateDateCodes::History(identity);
        contents.states.push(state);
    }
    let last = DateCodeId::new(20_000).unwrap();
    contents.date_histories.insert(
        last,
        DateCodeHistory {
            schema: 16_383,
            codes: vec![9],
        },
    );
    let mut state = template;
    state.date_codes = StateDateCodes::History(last);
    contents.states.extend([state.clone(), state]);
    let encoded = source.with_contents(contents).unwrap().to_bytes().unwrap();
    assert!(
        encoded
            .windows(6)
            .any(|span| span == [0xff, 0x7f, 1, 0x80, 0, 0x80])
    );
    assert!(
        encoded
            .windows(6)
            .any(|span| span == [0xff, 0x7f, 3, 0x80, 0, 0])
    );
    let parsed = VersionHistory::from_bytes(&encoded).unwrap();
    assert_eq!(parsed.contents().date_histories.len(), 16_385);
    assert_eq!(
        parsed.contents().states[16_384].date_codes,
        parsed.contents().states[16_385].date_codes
    );
}
