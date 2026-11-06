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

These are independent fixtures before typed metadata APIs. Native map allocation,
runtime state, engineering meanings, backend decoding and native application
acceptance remain separate work. The fixture reader reuses the existing binary
packet decoder; the field-value-specific 64-bit helpers remain with that feature's
fixture reader. Both fixture suites run after that internal extraction.
