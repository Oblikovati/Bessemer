# Flow selected values and byte-state dependency

Issue: #241. References: `../inventor-interface/solidworks/flowrecord112.py`
and `flowauxrecords.py`. This is remaining implementation work, not a shipped API.

Record 112 consumes exactly one raw 64-bit pattern for each selected entry in
an explicitly derived dense map. It has no version or count prefix. Complete
parity requires context derivation as well as payload access; accepting only an
arbitrary external count would omit the documented selection behavior.

The context combines complete legacy record-402 links, versions zero through
nine, with complete record-316 state bytes under an explicit fresh-map
assumption. Record 316 is a whole-zlib payload whose decoded body contains an
arbitrary signed header, a nonnegative signed count and exactly that many raw
bytes. Its header is retained without native dispatch. It is distinct from
record 12's raw count-plus-bytes layout and record 413's versioned layout.

Use the existing `FlowLinkEntry::native_code_byte` conversion once; do not
apply the version-zero conversion again. Exhaustively freeze the source's
accepted-code predicate over all 256 byte values. A nonempty state array must
have the same count as the links. Selection requires both an accepted converted
code and a state byte at least 128. Empty state bytes select no entries even
when links exist. This is a high-bit test, not a nonzero boolean test.

Derive sequential selected indices in physical link order, retaining absent
entries separately from selected zero-valued patterns. Bind the derived context
to its immutable input revisions and the explicit fresh-map assumption. Later
input edits must not silently reuse or update that context. No automatic binding
between different native files or runtime objects is established by the source.

Freeze complete record-316 and context/record-112 results before implementing
their public views. Cover every code/state-byte combination, empty contexts,
version-zero code conversion, malformed dense maps, context-count mismatches,
all raw wide-bit patterns and exact EOF. Native acceptance must verify the
dedicated three-input manifest: record 402 in CPT and records 316/112 in FLD.
Native owner binding, runtime scatter and engineering meanings remain excluded.

Existing-value edits must preserve the derived selection, fully reopen under
the same immutable context and return complete carrier history. Reuse the
current wide-array view and carrier edit contracts. Test independent byte
patches, no-ops, exact decoded reversals, compressed no-op preservation, source
isolation, resource bounds, concurrent operations and fresh-process equality.
