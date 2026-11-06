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

This baseline precedes the typed read/edit API. That API must bound inflation
and actual values independently, retain the signed header without dispatch,
edit existing bytes and header values on isolated copies, and reopen the whole
stream under the original bounds. No-ops must preserve stored bytes; other
edits must preserve untouched carrier payloads and exact decoded reversibility.
Complete history, immutable checkpoints, source isolation, thread safety and
fresh-process equality remain required before publication.

Record 112 combines these bytes with converted record-402 codes under an
explicit fresh-map assumption. Selection requires a high-bit state and an
accepted code; neither a nonzero state nor its high bit alone establishes
selection. Native owner binding, runtime scatter, engineering meanings and
SolidWorks application acceptance remain outside this byte-state baseline.
