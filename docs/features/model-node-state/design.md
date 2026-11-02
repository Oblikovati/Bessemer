# Archive integration contract

The view starts from a revision-scoped handle to an existing node update-state
field in a complete `PartConfigurationArchive` or
`PartResolvedFeaturesArchive`. It checks the exact adjacent signed state,
unsigned first flags word and signed ID paths. The optional second word stays
optional. Only the node reader emits that complete field sequence; complete
archive construction already validates its schema-derived paths. This avoids
adding decoder storage or changing the existing field layout and byte stream.

Parameter roles additionally require the direct owning object definition and
its exact `.node` scope. The finite supported classes are
`moDensityParameter_c`, `moLengthParameter_c`, `moAngleParameter_c`,
`moIntegerParameter_c`, `moParameter_c`, `moScalerParameter_c` and
`moStringParameter_c`. A loaded-class object is still a definition with its
resolved class. An instance alias has no new node fields. Generic favorite and
history items deliberately use identical node framing and upper flag bits in
the owned corpus, so a path or offset match alone cannot grant parameter roles.

The published view owns its raw words and source field handles. Queries describe
stored bits and unresolved bits. A selected-bit edit must resolve those handles
in the source archive, preserve every other bit and call the existing complete
field-edit operation. That operation supplies completion, exact coverage,
diagnostics, fidelity, ordered changed-field parents and the unchanged-field
count. No-op edits retain source identities. Foreign or stale views and
parameter-only requests on generic nodes fail before publication.

This is an additive stored-schema projection. Existing field names, constructors,
readers, byte reconstruction and general field edits remain available. Native
setter cascades and effective runtime predicates are separate from changing a
stored bit. Early lightweight-node boundary tests do not establish complete
legacy part-archive support. Public integration must use complete supported
archive contexts and preserve their existing version rejection boundaries.

The owned feature streams use the verified blank part's97-slot inherited map.
Each has two definitions of one class, using a new-class tag followed by a
loaded-class tag, then an alias to the first definition and an empty required
MBSM map. The independent node oracle records exact raw words and byte offsets.
These inputs establish source acceptance; Rust views, edits, complete byte/field
preservation, history and concurrent/process acceptance remain to be verified.

Humanizer is unavailable; maintainer prose review remains pending.
