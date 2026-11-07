# Record 444 typed interoperability

The implementation reuses bounded whole-zlib decoding and complete carrier
edits. It admits versions zero and one, preserves the optional discarded signed
member and exposes the counted array through the existing raw wide-array view.
Decoded-byte and actual-value limits are independent, with a zero-value limit
admitting stored-empty arrays.

Edits replace existing wide patterns or the physically stored legacy member.
Version, count and member presence remain fixed. Each edit copies the
carrier and reparses the complete replacement, returning exact validation coverage,
diagnostics and physical record history. A no-op preserves the original
compressed bytes. Fresh reversals restore decoded bytes; they do not claim
to reproduce another compressor's original encoding.

Acceptance requires all owned reads, literal expected patches, no-op preservation,
fresh decoded reversals, exact resource boundaries, malformed compressed and
decoded framing rejection, input isolation, public thread traits, concurrent
operations and two-process byte agreement. Native value discard remains an
explicit observation rather than a request to drop serialized values.
