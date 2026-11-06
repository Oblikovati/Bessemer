# Record 304 typed interoperability

The reader will accept complete raw payloads for versions 0 through 7. It will
bound payload bytes and stored rows before constructing views, and reject missing
trailers or trailing bytes. The legacy trailer will have its own view rather
than increasing the stored row count. Optional fields will distinguish absence
from a stored zero or empty block.

Edits will replace existing signed members, discarded words and raw block bytes.
They will preserve version, row count, conditional layout and trailer presence.
Each edit will copy the carrier, reparse the complete replacement, and return
validation coverage, diagnostics and physical record history. A no-op will retain
the original bytes. Revision-bound identities will reject foreign or stale input
without expanding retained carriers into error messages.

Acceptance requires typed reads of every frozen field; independent expected byte
patches; no-op and fresh reversal checks; exact resource boundaries; malformed
framing rejection; input isolation; public-type thread traits; concurrent edits;
and deterministic replay in two processes. Native runtime population and
identity-list resolution will remain explicit exclusions until separately proven.
