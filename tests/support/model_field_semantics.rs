use bessemer::model_archives::ModelField;

/// Check semantic values separately from retained framing; e.g. `assert_distinct_framing(short.fields(), wide.fields())`.
pub(crate) fn assert_distinct_framing(short: &[ModelField], extended: &[ModelField]) {
    assert_ne!(short, extended);
    assert_eq!(short.len(), extended.len());
    for (short, extended) in short.iter().zip(extended) {
        assert_eq!(short.path, extended.path);
        assert_eq!(short.value, extended.value);
    }
}
