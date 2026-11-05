# Supplied-page database overflow interoperability

Feature #229 tracks complete overflow key/value storage operations.
`database_overflow::OverflowPayload` reads complete explicitly supplied chains
under the `InstalledWriter` profile. `database_hash::HashWriterBucket` reads mixed
inline/overflow buckets under that explicit profile. The earlier `HashPageBuffer`
API keeps its strict inline-only behavior.

Callers supply nonzero provider page identities, complete images, an explicit
engine extent and the cell's key/value lengths. No page-to-file mapping is
inferred. Every supplied page must belong to the one complete chain; duplicate,
missing, cyclic, extra and prematurely terminated pages fail. The value-start
checkpoint must equal the exact key end, including the untruncated cursor.
Byte, page and payload budgets are checked before owned payload allocation.

`edit_value` replaces an existing same-length value on private copies and
reparses the complete result. It returns immutable source/result checkpoints,
changed/no-op completion, exact local coverage, zero reparse error, exclusions
and page-ID/byte-ordered history covering every supplied page. Keys, metadata,
links and opaque tails remain unchanged. Length changes and page allocation
require further work; this operation does not simulate an engine update.

The independent fixture corpus follows the stored-page writes recovered from
installed `sldDBImplu.dll`, rather than the distinct native key-reader callback
requests tracked in #226. The retained assembly, binary hashes and calling
convention are recorded in
[the writer investigation](../database-hash/overflow-writer-storage-investigation.json).
No installed target or original producer was executed to construct these images.

The 117 owned images cover empty keys and values, exact first-page and continuation
boundaries, multiple page extents and noncontiguous unsigned64 page identifiers.
Each image includes a bucket cell, complete overflow pages and separately stored
expected key/value bytes. Closed-form key-end positions and a separate
metadata-directed value reader check the construction.

At a supplied extent of 65,536, a key ending at the page boundary produces a
16-bit stored cursor of zero. Six images retain this conditional arithmetic
case. They are rejection controls, not successful value-decoding examples. The
fixtures do not establish that this extent is reachable in the installed engine,
or that any native sample contains this state.

The first overflow page stores a BE64 next link at byte 0, the BE64 value-start
page at byte 8 and the BE16 value-start cursor at byte 16. Key bytes start at
18; later pages reserve only their eight-byte next link. Values resume at the
recorded cursor and use byte 8 on subsequent pages. Unused bytes remain opaque.

The `HOV1` packet starts with its four-byte magic and a little-endian u32 record
count. Each record has a little-endian u32 byte extent, followed by
`<IIQBI`: page extent, key length, value length, cursor-wrap flag and page count.
Then come the complete bucket page, each `<Q` page identifier and complete page,
and the expected key and value bytes. The packet framing is a test format.

```text
python -B -X utf8 -m unittest discover -s docs/features/database-overflow -p test_*.py -v
python -B -X utf8 docs/features/database-overflow/freeze_writer_pages.py
```

The native search extracted 2,486 database streams from all 842 distinct files
in the 906-case corpus without extraction failures. A structural candidate scan
found 5,140 overflow cells with complete key/value chains and matching value-start
checkpoints. Keys have lengths 3 or 36; none cross a page boundary. One 4,380-byte
value crosses two pages.

Three complete native database streams are retained with literal reviewed key
and value ranges and separate expected payload files. The two-page value uses
stream ranges `[16438,20480)` and `[20488,20826)`. The tests compare these direct
ranges with metadata-directed reconstruction. All eight fixture tests passed.
The compressed census and `native-census-verification.json` retain the wider
search evidence. Physical page identity times 4,096 matches these selected
streams; this does not establish a universal engine/provider mapping.

Fourteen focused Rust tests passed, including all 111 admitted owned chains,
six wrapped-cursor rejections, three native chains, exact/one-below budgets,
malformed pages and graph checks, input isolation, shared concurrent edits and
page-order independence. The two-page native value was edited inside a complete
owned containing document, reopened and reversed to exact original file bytes,
preserving unrelated streams. Two fresh processes emitted identical 2,728,863-byte
packets; all original and edited pages matched independent expected spans.

The complete native candidate cohort now has a separate frozen regression packet:
5,140 chains, including the one two-page value. Every database image was checked
against its retained source hash; every expected key and value matched the
earlier independent span hashes. Complete regeneration reproduced all 23,174,343
packet bytes. The deterministic gzip image occupies 467,910 bytes.

The expanded Rust suite passed 15 tests. Its native census checked every stored
key/value, independently expected edited page, fresh reparse/reversal, source
isolation, coverage and ordered history. A second fresh process repeated the
complete census against the same expected bytes. Twelve Python tests also passed,
including changed-source, truncated-chain and cycle rejection. This is page-level
native corpus acceptance; the complete-document census below extends its scope.

`native-packet-reference.json` and `native-rust-census-verification.json` retain
the packet identity and executed checks. `BOV1` is a test format: magic and u32
record count, then length-prefixed records. Each record starts with little-endian
`<IIIQQI`: source ordinal, extent, key length, value length, first page ID and page
count. Complete `<Q`-identified pages, expected key bytes and expected value bytes
follow. Source ordinals refer to the retained candidate census.

```text
python -B -X utf8 docs/features/database-overflow/freeze_native_packet.py
cargo test --test database_overflow
```

The complete-document native census also passed: all 5,140 edits across 440
streams in 357 original documents reopened successfully and preserved unrelated
stream names, order and bytes. Fresh typed reversals restored the selected
database streams and exact complete original file bytes in 62.48 seconds.
Each output retained the original document checkpoint to restore its compression
provenance; the reverse payload came from a separately reopened document. This
does not recover missing provenance from an arbitrary reopened file.

All 357 complete source files matched their frozen lengths and SHA256 hashes
before and after the run. Installed files were read only; edits and reopened
documents stayed in memory. `native-document-census-verification.json` records
the executed test, exact census-log agreement and source checks.

The ignored native test requires an explicit samples root. The source-root
preflight authenticates the full original file hashes; the Rust trial checks
the selected frozen pages, whole-document reconstruction and input isolation.
`BDD1` is its test manifest: u32 stream count, then each u32 ordinal, length-prefixed
UTF8 relative path and stream name, u64 file length and 32-byte SHA256 digest.

```powershell
python -B -X utf8 docs/features/database-overflow/freeze_native_documents.py --source-root "C:/Users/Public/Documents/SOLIDWORKS/SOLIDWORKS 2026/samples"
$env:BESSEMER_OVERFLOW_SAMPLE_ROOT="C:/Users/Public/Documents/SOLIDWORKS/SOLIDWORKS 2026/samples"
cargo test --test database_overflow every_native_overflow_edit_reopens_original_document_and_reverses_with_retained_checkpoint -- --ignored --nocapture
```

The full supplied-overflow gate passed 4,331 tests at `83f3aacf`; later native
census, shared-reader and mixed-bucket additions need their own full gate. Native keys spanning
pages still need independent evidence. These operations do not validate
file-wide indexing, allocation, transactions, commits, geometry or application
opening. The humanizer skill is unavailable; maintainer prose review is pending.

The mixed-bucket API reads cell lengths and first overflow identities from the
stored 26-byte headers. Callers supply complete bucket bytes, the exact union of
referenced overflow pages, an `OverflowStorageContext` and the aggregate free/cell
record limit. Page and byte limits include the bucket; the payload limit includes
all cell keys and values. Shared, cyclic, missing, duplicate or unused overflow
images fail without publishing a partial bucket. No file-offset mapping is inferred.

Mixed edits retain immutable source/result snapshots, validate all other cells and
metadata, and return bucket spans followed by overflow spans in page-ID/byte order.
The shared metadata reader and inline history policy are reused. Raw free sizes
still carry no allocator-fit guarantee; outgoing bucket links are preserved
without file-graph validation. Values remain opaque.

The mixed corpus contains 122 complete images: 117 owned inline/overflow
compositions, empty/inline-only controls and three native buckets with 36 cells.
Six conditional cursor-wrap cases reject. All 259 admitted cell edits matched
independent complete images and reversed from fresh buckets. Every cell in the
12-cell native mixed bucket also reopened and reversed in a complete owned
containing document. The expanded suite passed 24 Rust tests and 16 Python tests;
all 11 earlier inline tests passed unchanged. Two fresh processes emitted identical
4,518,654-byte packets matching every independently expected edited bucket/page.

`bucket-fixture-reference.json` binds the `BWB1` fixture, and
`bucket-local-verification.json` records executed checks and exclusions. Each
length-prefixed test record contains `<IBIII` extent/rejection/page/free/cell
counts, a complete bucket, `<Q`-identified complete pages, `<HHH` free records
and `<HIHQIQ` cell offset/hash/next/first/key-length/value-length followed by the
expected key and value bytes. This test framing is separate from stored database bytes.
