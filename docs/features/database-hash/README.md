# Inline hash-page buffers

`database_hash` implements the complete single-buffer inline operation documented
in clause 9.2, C-3595/C-3601/C-3602/C-3603. Its behavior version is
`C3595-inline-hash-buffer-v1`. The caller selects
`HashBufferProfile::InstalledInline`, supplies the current engine extent, and
sets byte/aggregate-record budgets. The file header's P is not used to infer that
extent or the current runtime engine/provider identity.

The decoder reads the 12-byte page header, the ordered four-byte free-record
chain and the ordered 26-byte cell-header chain. Inline keys and values are
owned raw bytes. Signed-boundary hash words, unsigned64 outgoing IDs, raw free
sizes, embedded NUL and non-UTF-8 bytes remain unchanged. Values are not assigned
a universal MsgPack interpretation.

Every required header/payload must fit. Conflicting metadata/cell spans, cycles
and exhausted budgets fail before publication. The decoder reserves each cell
header before interpreting its overflow metadata; Bug #225 records the focused
regression. Empty payload spans reserve no bytes and may share the next cell's
boundary. A nonzero overflow identifier returns an explicit unsupported-profile
error, without publishing a partially decoded page.

Free sizes below four are retained because the selected reader does not impose
the update writer's minimum. `free_size_sum` reproduces the reader's modulo16
sum. This is not a validated allocator-fit or distinct-free-byte count.
Coverage separates interpreted metadata/inline payloads from retained opaque
bytes. Outgoing page identifiers are preserved without following the page graph.

`edit_value` replaces one existing complete value with the same byte length.
It rebuilds and reparses an isolated buffer, checks every other decoded field,
and returns completion, source/result snapshots, exact local coverage, zero
achieved reparse error, explicit diagnostics and ordered byte-span history.
No-ops and empty values remain distinguishable from failures. All untouched
bytes, including free-region contents, are retained.

Sixty independent literal buffers cover full-width fields and raw free sizes.
Both actual page-2 buckets in the existing blank-part database streams match
independent metadata and payload-span oracles: seven cells and three free
records. Every native cell has an exact value-edit/reopen/reversal check. Two
selected value replacements also reopen complete containing documents while
preserving every unrelated logical stream. Eight workers share immutable
snapshots, and compile-time checks cover every public type's Send/Sync bounds.

Run `cargo test --test database_hash` and
`python -B docs/features/database-hash/fixture_bytes.py`.
The source snapshot, native/capture hashes, native span oracle and execution
records are kept here for issues #224 and #225. The humanizer skill is
unavailable; prose review remains with the maintainer.

Required next work toward goal #2 includes supplied-context overflow key/value
pages and resume cursors, index/file-graph admission, allocation/relinking and
length-changing edits. Native callbacks, key-function correctness, opaque value
meaning, whole-database consistency, commit and durability remain unproved.
This inline-buffer milestone does not replace those requirements.
