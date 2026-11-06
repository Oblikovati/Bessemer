# Record 444 typed interoperability

The implementation will reuse bounded whole-zlib decoding and complete carrier
edits. It will admit versions zero and one, preserve the optional discarded signed
member and expose the counted array through the existing raw wide-array view.
Decoded-byte and actual-value limits will be independent, with a zero-value limit
admitting stored-empty arrays.

Edits will replace existing wide patterns or the physically stored legacy member.
Version, count and member presence will remain fixed. Each edit will copy the
carrier and reparse the complete replacement, returning exact validation coverage,
diagnostics and physical record history. A no-op will preserve the original
compressed bytes. Fresh reversals will restore decoded bytes; they will not claim
to reproduce another compressor's original encoding.

Acceptance requires all owned reads, literal expected patches, no-op preservation,
fresh decoded reversals, exact resource boundaries, malformed compressed and
decoded framing rejection, input isolation, public thread traits, concurrent
operations and two-process byte agreement. Native value discard will remain an
explicit observation rather than a request to drop serialized values.
