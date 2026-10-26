# Private model field reader

Feature #44 now has a model-specific field builder and scalar encoder. They are
compiled only by tests while the complete model root operation is being built.
No model root API is exported. The implementation will stay private until the
owned part passes typed read, construction, independent edit, whole-document
replacement, reopening and authentic restoration with both sections complete.

The reader owns typed primitive values and CString framing after decoding.
Encoding reconstructs those values without retaining the input bytes. The
shared material serializers use the model reader through `MaterialRead`, with
model-specific error context, field paths and byte/field/entry/depth budgets.
Grouped primitive reads have one shared implementation for model and display
callers; their order and scalar operations are unchanged.

A failed read, scope or budget check poisons the operation-local builder. Later
reads and `finish` return its first typed failure, even when a failed field write
already consumed the last input byte. A complete cursor alone cannot publish a
partial field list. This state is local to the builder and introduces no shared
mutable state.

Nine focused Rust tests passed. They reconstruct all 845 native appearance bytes,
check the independently reviewed color and variable-length username bytes,
reopen and revert the edit, reject every incomplete prefix and a trailing byte,
check exact budgets and overflow, preserve NaN payloads and signed zero, and
compare four parallel encodings after mutating the input buffer. These tests
also require the completed field storage to be shareable across threads. Final
all-target clippy passed. The full PR gate also passed after the grouped-reader
change, including all 204 doctests. After cursor/version storage was made
private, the nine tests and all-target clippy passed again.

This is a building block for the complete configuration stream. It does not
decode the 24,028-byte root yet. Model object dispatch, ordered header imports,
scoped maps, complete lightweight and resolved bodies, derived section lengths,
whole-document acceptance and the remaining 906-case corpus gate remain part of
the feature. Native application and geometry acceptance are separate checks.

The humanizer skill was unavailable; prose remains subject to maintainer review.
