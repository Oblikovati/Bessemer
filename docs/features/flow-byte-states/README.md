# Flow record 316 byte states

Issue: #241. Reference: `../inventor-interface/solidworks/flowauxrecords.py`.

Record 316 is one exact zlib stream. Its decoded body stores an arbitrary
signed 32-bit header, a nonnegative signed 32-bit count and exactly that many
raw bytes. The header does not select a layout or execute native dispatch.
Empty arrays are complete results. Bytes retain all 256 values without boolean,
enum or engineering interpretation. This differs from record 12's raw framing
and record 413's versioned byte layouts.

The FBS1 packet freezes 965 complete schema results. It retains stored compressed
bytes, the complete decoded body, header, count, end and every raw value. Cases
cover all header single bits and complements, arbitrary negative headers,
every raw byte, boundary lengths through 65,536, compression levels zero through
nine, five deflate strategies and window sizes nine and fifteen. The independent
decoded oracle is the literal header-plus-count-plus-bytes image.

The dedicated auxiliary manifest contains one native FLD record-316 stream:
265 stored bytes expand to 5,540 bytes, with header zero and 5,532 values.
The complete value census is 90 bytes equal to `3`, 90 equal to `4`, 1,476 equal
to `236` and 3,876 equal to `253`. The freeze verifies the manifest against its
native evidence hash, then checks the whole source file, selected carrier
occurrence, exact record extent, stored payload hash and complete projection.
Hashes are retained in `baseline-verification.json`.

Run `python -m unittest discover -s docs/features/flow-byte-states -p test_byte_states.py`
and `cargo test --test flow_byte_state_fixtures`. Checks cover exact stored and
decoded reconstruction, both kinds of compact truncation, outer and inner
extensions, concatenated streams, header/checksum corruption, raw deflate, gzip,
invalid counts and carrier guards. Fixture regeneration uses a temporary root.

`FlowCarrier::record_316_byte_states` returns an immutable `FlowByteStates`
under explicit `FlowByteStateOptions`. The decoded byte bound must admit the
eight-byte header/count prefix. The actual-value bound may be zero for an
empty array. Inflation is bounded before parsing; exact count/EOF agreement and
the value limit are checked before publication. No per-byte descriptors are
allocated. The header is retained as a signed word without version selection.

`replace_header` and `replace_byte` edit copies, reopen the complete compressed
payload under its original bounds and return `FlowSimpleTableEdit` checkpoints
with physical carrier history. Counts stay fixed. No-ops preserve the original
compressed bytes, including native and alternative compression representations.
Other edits preserve untouched carrier payloads and exact decoded reversibility;
they do not promise to reproduce the original compressor's stored bytes on a
fresh reversal. Coverage reports exact zlib framing and actual transfers, while
diagnostics retain the runtime, engineering and application exclusions.

Typed reads cover every raw byte in all 965 owned cases. Edit checks cover every
header and every byte in cases of at most 257 values, every native position,
and distinct first/middle/last positions in larger generated cases. This selects
32,851 byte positions, including all 5,532 native positions. Each has an
independent literal patch, a compressed no-op check, complete physical history
and a fresh decoded reversal. Two fresh processes compare the same source-checked
corpus and selected edits byte for byte. Eight workers compare mixed header/byte
edits against serial results. Compile-time checks cover both new types and the
three reused edit, coverage and diagnostic types. Selected edit checks do not
claim an edit test for every large generated position.

Record 112 combines these bytes with converted record-402 codes under an
explicit fresh-map assumption. Selection requires a high-bit state and an
accepted code; neither a nonzero state nor its high bit alone establishes
selection. Native owner binding, runtime scatter, engineering meanings and
SolidWorks application acceptance remain outside this byte-state baseline.
