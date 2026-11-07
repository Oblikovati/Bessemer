# Strict control payload interoperability

Issue: #241. This is planned work; no typed control-payload API is shipped yet.

Readers will accept revision-bound physical occurrence IDs for the exact control
identifier. Ordinary and solver selector policies remain separate. Reading a
control payload must not make a reserved identifier selectable through the
ordinary solver API or adopt an index into the physical carrier automatically.

Saved-span readers will validate one complete checksum-valid zlib stream, a zero
marker, the full unsigned count and complete 16-byte entries through EOF.
Terminal-index readers will require an explicit typed four/eight-byte carrier
length context, validate the zero prefix, separate the exact compressed member
from its footer, and check footer extent, identifier and backward distance.
The decoded index count remains signed/nonnegative, independently of the raw
unsigned identifiers and target spans. Independent decoded-byte and entry bounds
must be checked before allocation or multiplication. Empty bodies retain their
required marker/count headers; they are complete empty results.

Views will preserve entry order, duplicates, raw fields and all transport/footer
intervals. Existing-field edits will operate on a copy and retain count and
context. Saved-span edits will re-encode only changed decoded bodies. Index edits
will also recompute the footer distance from the new compressed extent, reject
distances that do not fit the declared width, and reparse the complete result.
No-ops will retain original stored bytes; fresh opposite edits will restore
decoded entries without requiring the original compression encoding. Results
will retain both immutable checkpoints and complete physical carrier history.

Strict payload framing is separate from target consistency. A zero record span,
reserved target identifier, duplicate identifier or out-of-range target offset
must remain a raw accepted field when the documented profile accepts it. No
automatic adoption, target validation, saved-span semantic mapping or native
runtime execution is implied. Compression changes can move later physical
records; opaque indexes elsewhere in the carrier are not repaired implicitly.

The logical `flowindex.py` reader has a separate recognition/fallback policy.
Its invalid terminal candidates can fall back to physical scanning; direct
strict control-payload readers must reject malformed requested payloads. That
logical replay needs its own oracle and API after the strict payload operations.

Acceptance will cover all owned entries, independent byte patches, footer
recomputation, no-op preservation, decoded reversals, complete history, exact
resource boundaries, malformed frames, source isolation, public thread traits,
concurrent operations and two-process byte agreement. Installed native payload
and application evidence remain required before broader acceptance claims.

The humanizer skill is unavailable; maintainer prose review remains pending.
