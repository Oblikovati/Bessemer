# Record 304 typed interoperability

The reader accepts complete raw payloads for versions 0 through 7. It bounds
payload bytes and stored rows before constructing views, and rejects missing
trailers or trailing bytes. The legacy trailer has its own view rather
than increasing the stored row count. Optional fields distinguish absence
from a stored zero or empty block.

Edits replace existing signed members, discarded words and raw block bytes.
They preserve version, row count, conditional layout and trailer presence.
Each edit copies the carrier, reparses the complete replacement, and returns
validation coverage, diagnostics and physical record history. A no-op retains
the original bytes. Revision-bound identities reject foreign or stale input
without expanding retained carriers into error messages.

Acceptance requires typed reads of every frozen field; independent expected byte
patches; no-op and fresh reversal checks; exact resource boundaries; malformed
framing rejection; input isolation; public-type thread traits; concurrent edits;
and deterministic replay in two processes. Native runtime population and
identity-list resolution remain explicit exclusions until separately proven.
