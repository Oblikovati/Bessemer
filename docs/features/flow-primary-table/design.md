# Primary-table implementation constraints

The complete supported profile is the raw primary-class version-0–3 grammar in
`flowrecord439.py`. Its complete check reaches exact EOF. A populated prefix
result from the separate reference helper is incomplete and must not be labeled
as a valid complete table. Empty complete payloads are successful empty results.

Reuse the existing `TableWire`, raw word views, typed record IDs, carrier patching
and physical history paths. Normalize the version once, and normalize an element
factory tag only when a typed list is populated. Unknown unused tags must remain
accepted. Presence uses nonzero truth, while the body's derived native flag uses
exact equality to one. The two predicates are deliberately different.

Keep optional body and typed-list parsing independent. Kinds two and three still
store a typed-list count when their group's presence byte is zero. Preserve both
groups in physical order and retain all duplicate or empty entries. Word/wide
rows have twelve-byte serialized strides; do not apply native struct alignment
or infer engineering interpretations from the raw patterns.

Bound bytes, items, nested elements/rows/blocks, field descriptors and values
independently before allocation or iteration. Counts and version framing are
structural. Existing-field edits need complete reparse, unchanged structural
layout and full carrier history. No-ops retain the original checkpoint and wire;
changed results must reopen and reverse byte for byte. IDs bind fields to the
current physical occurrence and revision, with typed errors for foreign/stale
IDs and invalid indices.

Publish immutable Send/Sync views with completion, exact coverage, diagnostics,
history and achieved serialized error. Keep primary-class assumptions, absent
populated native fixtures and native/runtime/engineering exclusions explicit.
Use the entire F439 field/interval corpus, malformed boundaries and independent
patches to verify the first complete read/edit operation before extending it.
