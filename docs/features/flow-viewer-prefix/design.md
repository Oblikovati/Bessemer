# Record 121 viewer-prefix interoperability

The API requires the viewer-query profile explicitly. It preserves the
signed outer version, validates a complete zlib stream under a decoded-byte bound,
and requires complete positive-length arrays and the remaining known prefix fields.
An independent known-wide-value bound includes the three discarded tuple
values. All suffix bytes remain opaque and count against the decoded-byte
bound.

Coverage distinguishes exact outer/compression framing and exact known-prefix
framing from incomplete full-body validation. An empty suffix does not promote a
prefix result to a complete body result. Diagnostics retain the explicit
profile assumption, unresolved suffix, native runtime and engineering exclusions.
No general record-121 reader or graph-profile fallback is implied.

Views expose all known raw fields, array intervals, discarded tuple values,
the metadata projection and the complete opaque suffix. Edits replace
existing known fields while preserving array lengths, outer version and suffix
bytes. They operate on a copied carrier and reparse the same explicit prefix
profile. No-op edits preserve stored compressed bytes. Fresh reversals will
restore decoded bytes without promising another compressor's original encoding.

Acceptance requires all owned prefix reads, independent byte patches, recomputed
metadata outputs, no-op preservation, complete physical history and opaque-tail
identity after edits and reversals. Resource boundaries, malformed compressed and
known-prefix input, source isolation, public thread traits, concurrent operations
and two-process agreement are also required. Incomplete body checks must remain
visible in every successful prefix result.
