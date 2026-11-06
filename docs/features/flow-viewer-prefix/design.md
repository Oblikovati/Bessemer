# Record 121 viewer-prefix interoperability

The API will require the viewer-query profile explicitly. It will preserve the
signed outer version, validate a complete zlib stream under a decoded-byte bound,
and require complete positive-length arrays and the remaining known prefix fields.
An independent known-wide-value bound will include the three discarded tuple
values. All suffix bytes will remain opaque and count against the decoded-byte
bound.

Coverage will distinguish exact outer/compression framing and exact known-prefix
framing from incomplete full-body validation. An empty suffix will not promote a
prefix result to a complete body result. Diagnostics will retain the explicit
profile assumption, unresolved suffix, native runtime and engineering exclusions.
No general record-121 reader or graph-profile fallback will be implied.

Views will expose all known raw fields, array intervals, discarded tuple values,
the metadata projection and the complete opaque suffix. Edits will replace
existing known fields while preserving array lengths, outer version and suffix
bytes. They will operate on a copied carrier and reparse the same explicit prefix
profile. No-op edits will preserve stored compressed bytes. Fresh reversals will
restore decoded bytes without promising another compressor's original encoding.

Acceptance requires all owned prefix reads, independent byte patches, recomputed
metadata outputs, no-op preservation, complete physical history and opaque-tail
identity after edits and reversals. Resource boundaries, malformed compressed and
known-prefix input, source isolation, public thread traits, concurrent operations
and two-process agreement are also required. Incomplete body checks must remain
visible in every successful prefix result.
