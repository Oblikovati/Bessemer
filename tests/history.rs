//! Native and independent schema-oracle coverage for standalone history streams.

#[path = "support/history.rs"]
mod history_support;

use bessemer::history::{
    ArchiveString, DateCodeHistory, DateCodeId, HistoryContents, HistoryMemory, HistoryRecent,
    HistoryTail, RevisionStamp, StateDateCodes, TextEncoding, VersionHistory, VersionState,
};
use history_support::{CASES, MIXED, NATIVE, semantic_projection};

#[test]
fn all_stored_fields_and_shared_identities_match_the_reference() {
    for (name, bytes) in CASES {
        let history = VersionHistory::from_bytes(bytes).unwrap();
        let expected = include_str!("fixtures/history/expected.tsv")
            .lines()
            .find_map(|line| {
                line.split_once('\t')
                    .filter(|(filename, _)| filename == name)
                    .map(|(_, expected)| expected)
            })
            .unwrap();
        assert_eq!(semantic_projection(&history), expected, "{name}");
        assert_eq!(history.to_bytes().unwrap(), *bytes, "{name}");
        let rebuilt =
            VersionHistory::new(history.root_schema(), history.contents().clone()).unwrap();
        let reopened = VersionHistory::from_bytes(&rebuilt.to_bytes().unwrap()).unwrap();
        assert_eq!(semantic_projection(&reopened), expected, "rebuilt {name}");
    }
}

#[test]
fn native_history_encoder_reproduces_every_byte_without_original_bytes() {
    for (_, bytes) in &CASES[..3] {
        let source = VersionHistory::from_bytes(bytes).unwrap();
        let rebuilt = VersionHistory::new(source.root_schema(), source.contents().clone()).unwrap();
        assert_eq!(rebuilt.to_bytes().unwrap(), *bytes);
    }
}

#[test]
fn editing_stamps_preserves_unknown_fields_and_date_code_flags() {
    let source = VersionHistory::from_bytes(MIXED).unwrap();
    let baseline = semantic_projection(&source);
    let mut contents = source.contents().clone();
    contents.states[0].stamp.comment =
        ArchiveString::new("Reviewed λ😀", TextEncoding::Utf16).unwrap();
    contents.states[0].stamp.revision_status = 0xfedc;
    let edited = source.with_contents(contents.clone()).unwrap();
    let reopened = VersionHistory::from_bytes(&edited.to_bytes().unwrap()).unwrap();
    assert_eq!(semantic_projection(&reopened), semantic_projection(&edited));
    assert_eq!(reopened.contents().states[0].stamp.unknown_0c, 17);
    assert_eq!(reopened.contents().states[0].stamp.unknown_10, 29);
    assert_eq!(source.to_bytes().unwrap(), MIXED);
    assert_eq!(semantic_projection(&source), baseline);
    contents.states[0].stamp.time = 0;
    assert_eq!(edited.contents().states[0].stamp.time, 123);
}

#[test]
fn equivalent_edits_and_reverts_preserve_nonminimal_encodings() {
    let source = VersionHistory::from_bytes(MIXED).unwrap();
    assert_eq!(
        source
            .with_contents(source.contents().clone())
            .unwrap()
            .to_bytes()
            .unwrap(),
        MIXED
    );
    let mut contents = source.contents().clone();
    contents.states[0].stamp.time = 456;
    let edited = source.with_contents(contents).unwrap();
    let restored = edited.with_contents(source.contents().clone()).unwrap();
    assert_eq!(restored.to_bytes().unwrap(), MIXED);
}

#[test]
fn input_mutation_does_not_change_published_history() {
    let mut bytes = NATIVE.to_vec();
    let history = VersionHistory::from_bytes(&bytes).unwrap();
    bytes.fill(0);
    assert_eq!(history.to_bytes().unwrap(), NATIVE);
}

#[test]
fn all_public_history_types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<VersionHistory>();
    check::<HistoryContents>();
    check::<RevisionStamp>();
    check::<VersionState>();
    check::<StateDateCodes>();
    check::<DateCodeId>();
    check::<DateCodeHistory>();
    check::<HistoryTail>();
    check::<HistoryMemory>();
    check::<HistoryRecent>();
    check::<ArchiveString>();
    check::<TextEncoding>();
}

#[test]
fn concurrent_edits_are_deterministic_and_leave_source_unchanged() {
    let source = VersionHistory::from_bytes(MIXED).unwrap();
    let baseline = semantic_projection(&source);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    let mut contents = source.contents().clone();
                    contents.states[0].stamp.time = 987;
                    source.with_contents(contents).unwrap().to_bytes().unwrap()
                })
            })
            .collect();
        let outputs: Vec<_> = workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect();
        assert!(outputs.windows(2).all(|pair| pair[0] == pair[1]));
    });
    assert_eq!(source.to_bytes().unwrap(), MIXED);
    assert_eq!(semantic_projection(&source), baseline);
}
