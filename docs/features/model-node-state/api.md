# Stored node-state API

`PartConfigurationArchive::node_state` and
`PartResolvedFeaturesArchive::node_state` project a physically defined node from
a handle to its signed `unknown_0c` state field. The handle must belong to that
archive revision. An alias-only object occurrence has no new state field; select
the owning definition in its carrier. Inherited configuration fields belong to
the configuration archive rather than the separate feature stream.

The immutable `ModelNodeStoredState` owns its raw state, both flag words and
source field handles. `flags_2c` preserves absence separately from stored zero.
The unresolved-bit views retain every bit without a role in the declared class
scope. `parameter_class` identifies one of the seven verified parameter classes;
generic nodes receive no parameter-only meaning.

`stored_flag` reads the seven named update bits, ID-assignment request and the
four parameter roles through `ModelNodeStoredFlag`. A parameter request on a
generic node returns a typed unsupported error. A request for a missing second
word returns a typed missing error. These views do not evaluate native virtual
state, reference counts, equations or pointer allocation.

```rust
use bessemer::model_archives::{
    ModelFieldHandle, ModelNodeStoredFlag, PartResolvedFeaturesArchive,
};

fn mark_stored_update(
    source: &PartResolvedFeaturesArchive,
    state_field: &ModelFieldHandle,
) -> bessemer::Result<Vec<u8>> {
    let node = source.node_state(state_field)?;
    let edit = source.edit_node_flag(&node, ModelNodeStoredFlag::NeedsUpdate, true)?;
    edit.archive().to_bytes()
}
```

`edit_node_flag` changes only the selected bit and delegates to the existing
complete archive edit operation. It returns completion, exact stored-schema
coverage, diagnostics, achieved fidelity, ordered changed-field parents and the
unchanged-field count. Source buffers, other fields and unresolved bits remain
isolated. No-op edits retain identities; changed revisions reject old views.
Foreign views and invalid field selections fail before a result is published.
Both new public types satisfy `Send` and `Sync`; reading takes no lock.

Existing constructors and general field edits remain available. No field was
renamed and no byte layout or old archive version boundary changed. Native
setters can clear neighboring bits, and native needsUpdate or dimension sharing
can consult other state; this stored-bit operation does neither. Whole-word
semantic credit, native application acceptance and geometry checks remain
outside this API's stored-archive coverage.

The [source and fixture evidence](README.md) records independent literals,
exact spans and native bindings. Local focused checks pass; complete retained
corpus, fresh-process and immutable-head/platform acceptance are recorded
separately as they complete.

Humanizer is unavailable; maintainer prose review remains pending.
