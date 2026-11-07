use bessemer::Document;

/// Checks every unrelated stream and its order; e.g. `assert_unselected_document_streams(&before, &after, "Contents/DisplayLists");`.
pub(crate) fn assert_unselected_document_streams(
    before: &Document,
    after: &Document,
    selected: &str,
) {
    let before_names: Vec<_> = before.streams().map(|stream| stream.name).collect();
    let after_names: Vec<_> = after.streams().map(|stream| stream.name).collect();
    assert_eq!(before_names, after_names);
    for stream in before.streams().filter(|stream| stream.name != selected) {
        assert_eq!(
            before.read_stream(stream.name).unwrap(),
            after.read_stream(stream.name).unwrap()
        );
    }
}
