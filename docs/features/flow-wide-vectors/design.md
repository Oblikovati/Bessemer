# Record 148 typed interoperability

The reader will admit only raw version-zero bodies and require complete EOF. It
will bound payload bytes, actual vector count and total raw-wide values before
allocating vector metadata. All signed lengths will be validated before exposing
the immutable view. Zero vectors and present-empty vectors will remain distinct;
a zero total-value budget will still admit an all-empty length table within the
vector-count and payload-byte bounds.

Each vector will expose its exact original interval through the existing raw
wide-array view. Edits will replace existing patterns while preserving the full
length table, vector count and physical order. They will copy the carrier, reparse
the complete replacement and return validation coverage, diagnostics and physical
history. No-op edits and fresh reversals will preserve the raw carrier bytes.

Acceptance requires every owned vector read, independent expected byte patches,
no-op preservation, full history, fresh reversals and exact resource boundaries.
Malformed lengths, impossible totals, prefixes, suffixes and unsupported versions
must fail without exposing partial successful views. Input isolation, public
thread traits, concurrent operation agreement and two-process byte agreement are
also required. Native runtime population and engineering interpretation remain
separate from serialized framing and raw-pattern preservation.
