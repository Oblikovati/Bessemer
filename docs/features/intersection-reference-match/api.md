# Stored intersection-reference API

`PartResolvedFeaturesArchive::intersection_reference_states()` enumerates
physical ordinary sketch reference groups in wire order. Aliases remain aliases
and do not create duplicate physical records. An accepted complete archive with
no groups returns an empty vector. `intersection_reference_state()` selects a
record by its leading `raw_00` field handle.

`ModelIntersectionReferenceStoredWords` retains the complete u16 state word,
uninterpreted signed raw20, signed span-count state and signed backward-offset
state. No state enum, nonnegative constraint, count-sum invariant or native
matching operation is applied. All i32 values remain available, including -2
and INT_MIN. The existing complete decoder establishes the ordinary context and
version; the view additionally checks the actual preceding `sgSketch`
definition, marker zero, canonical indices, edge bounds and four adjacent
same-scope typed fields.

`ModelIntersectionReferenceStoredEdit` changes only the span-count state, only
the backward-offset state, or both together. The original state word, raw20,
references and every other field remain unchanged. Results use the existing
complete stored-archive validation, completion, diagnostics, fidelity and
ordered field history. A paired request reports only fields whose values
actually changed. No-ops retain revision identity. Foreign or stale views fail
before replacement; reacquire views from the resulting archive for later edits.

```rust
use bessemer::model_archives::{
    ModelIntersectionReferenceStoredEdit, PartResolvedFeaturesArchive,
};

fn replace_first_span(source: &PartResolvedFeaturesArchive) -> bessemer::Result<Vec<u8>> {
    let Some(reference) = source.intersection_reference_states()?.into_iter().next() else {
        return source.to_bytes();
    };
    let result = source.edit_intersection_reference_state(
        &reference,
        ModelIntersectionReferenceStoredEdit::SpanCountState(-2),
    )?;
    result.archive().to_bytes()
}
```

The first implementation covers complete ordinary part resolved-feature
archives. Other caller APIs need complete positive carriers before addition.
The owned matrix has accepted original Source roots and typed Rust edits.
Two fresh unchanged Source processes also accepted every actual exported edit
output. The owned document matrix passed complete edit/reopen/reversal in two
processes. Native archive edits passed all 68 physical groups in the eight
complete C-3307 streams, with literal byte patches and complete field/history
checks. Native complete-document edits also passed all 68 groups in two fresh
processes: reopen preserved every unrelated stream and reversal restored the
original files byte for byte. All 1,344 owned cases also passed mixed requests
across eight workers in two fresh processes. Complete operation traces and
wire exports match byte for byte, including resolved history and typed errors.
Wider version/caller boundaries, ThreadSanitizer and publication gates remain
pending. The newer index and classification roles
are tracked separately in #185.
File-operation acceptance does not establish geometry,
native application opening or rendering. The humanizer skill is unavailable;
prose review remains pending.
