# Flow ID/state updates and metadata prefix

Issue: #241. Reference: specification section 14.52 and
`../inventor-interface/solidworks/flowmetadataprefix.py`.

Raw record 410 stores an uninterpreted unsigned four-byte header, a nonnegative
signed entry count, and ordered five-byte entries. Each entry retains its signed
identifier and complete state byte. The header does not select a version layout.
Repeated identifiers remain visible in the wire view. Applying the updates
requires an explicit existing map: the last duplicate wins, and unmentioned keys
remain unchanged. Native hash-table allocation, ordering and identifier roles
are not reproduced.

The independent FIS1 baseline has 1,880 views covering 67 full-width header
patterns, every state byte, repeated/negative/boundary identifiers, completed
empty updates and four existing-map contexts. It retains every row interval,
canonical sorted map result and an independent assignment chain, including the
previous value at every duplicate update. The complete native CFLD payload is
18 bytes and contains `(410, 1)` followed by `(413, 1)`. These observed values do
not establish a general carrier-record meaning for identifiers.

Raw record 424's metadata prefix uses a signed version-byte gate. Raw bytes
2–127 read integer vectors and string/state objects; other bytes stop after the
first byte and retain all remaining bytes as opaque. Native string mode zero is
explicit. Strings retain their original UTF-16LE units, including terminal and
embedded NULs, unpaired surrogates and noncharacters. The native text projection
ends at the first NUL. Version two derives member-90 text from the first string;
later active versions store a fourth string, including a distinct stored empty.

The FMP1 baseline has 537 views covering every version and state byte, empty
collections, zero-unit versus stored-NUL strings, surrogate/noncharacter windows,
long strings and large vector collections. Rust reconstructs every known prefix
byte and separately checks the exact opaque tail. The native version-two record
has 112 known prefix bytes and 13,079 opaque backend bytes. The baseline always
retains `full_payload_schema = false`, including when no tail bytes remain.
This prefix evidence does not establish complete record-424 acceptance.

The initial fixture baseline remains independent of the typed APIs. Native map allocation,
runtime state, engineering meanings, backend decoding and native application
acceptance remain separate work. The fixture reader reuses the existing binary
packet decoder; the field-value-specific 64-bit helpers remain with that feature's
fixture reader. Both fixture suites run after that internal extraction.

## Record 410 API

`FlowCarrier::record_410_states` validates the complete raw payload under
independent byte and row limits. `FlowIdStates` retains the full unsigned header
word, signed identifiers, every state bit and exact row intervals. The header
does not select a native version. Empty tables are completed results.

`replace_header_word`, `replace_identifier` and `replace_state` edit existing
fields on an isolated carrier, reparse the entire result and retain typed source
and result checkpoints plus every physical history relation. Counts remain
structural. Exact no-ops preserve the original wire identity.

`apply_to` requires a `FlowIdStateMap` containing the explicit existing state.
It applies every row in wire order and preserves unmentioned keys. The immutable
projection retains both maps, its source record and all assignments, including
equal-value updates and the value immediately before each duplicate assignment.
Created and derived relations distinguish absent keys from existing zero values.
Sorted result iteration provides deterministic output without modeling native
hash allocation or iteration order.

The focused suite checks all 1,880 views, malformed boundaries and independent
limits. Every existing field in all distinct wire layouts, including the complete
native payload, receives no-op, patch, history and fresh reverse checks. Eight
workers compare complete map and assignment transcripts after input isolation
checks. Two fresh processes reproduce the entire independent FIS1 packet byte
for byte. Native identifier roles and application acceptance remain unverified;
this API adds no record-424 backend coverage.

## Backend boundary baseline

The section-14.53 backend reader consumes version zero, then up to two
version-zero table headers. An empty first table continues to the second header.
A populated table stops after its first signed kind tag; only first-table kind
one also stores a 16-byte key before that boundary. The declared populated count
need not fit the remaining bytes because object bodies have not been decoded.
Other kind tags remain raw, including negative and boundary values.

The FMB1 baseline freezes 1,039 views: the native backend, empty framing with and
without a tail, both table positions, 259 kind tags and counts one and signed
maximum. Rust reconstructs every known byte and retains the exact opaque tail.
The native backend contains 13,079 bytes and stops at byte 26 before the first
kind-one body. Completed empty backend framing still does not establish complete
record framing; following bytes belong to the outer record and remain opaque.
Python checks every proper prefix of each known span, every unsupported version
byte at all three version positions and negative counts at both table positions.

This is fixture evidence for the next backend implementation. The documented
kind-one and kind-two bodies, typed references, outer collections and finite
curve evaluation still require separate implementation and verification.

## Record 424 metadata-prefix API

`FlowCarrier::record_424_metadata_prefix` reads the raw prefix under an explicit
`NativeUtf16` string mode. Independent limits bound complete payload bytes,
vector headers, object headers, total vector values and total stored string
units. Counts are checked against the physical payload before allocation.

`FlowMetadataPrefix` retains its carrier and occurrence identity, every vector
and object boundary, and all undecoded bytes. Borrowed strings distinguish an
absent fourth field from a stored empty string and distinguish zero units from
a stored NUL. Raw bytes retain invalid Unicode scalar sequences. Native text
projections borrow UTF-16LE bytes before the first NUL; version two derives the
fourth text from the first stored string. State bytes retain all bits beside
an explicit Boolean projection.

Existing vector values, object state bytes, signed member-C4 words and stored
UTF-16 units can be edited. A terminal NUL cannot be removed. String-unit edits
recompute native first-NUL text and its version-two fallback. Version edits are
accepted only when reparsing preserves all collection and stored-string layouts.
Every edit reopens the complete carrier, checks the full patched payload and
retains source/result checkpoints and all physical history relations. Exact
no-ops retain wire identity; fresh reversals reproduce the original carrier.

All 537 frozen views match the typed reader. Every small existing field and the
endpoints of larger vectors/strings receive independent patch, no-op, history
and reverse checks. The complete native prefix is included. Tests cover all
small known-prefix truncations, missing terminators, wrong/foreign/stale IDs,
invalid indices, absent fourth fields, exact/one-below/zero resource limits,
input isolation and matching results across eight workers. Two fresh processes
reproduce the complete independent FMP1 packet, including every native text
projection and opaque byte.

Coverage always reports `full_payload_schema = false`, even with an empty tail.
The explicit stop identifies either the inactive signed-version branch or the
backend virtual-call boundary. Backend framing and medium bodies have separate
reference schemas; they are not decoded by this prefix API. Native runtime
state and application acceptance remain unverified.
