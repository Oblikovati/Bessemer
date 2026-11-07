# Explicit timestamp packing and entry edits

Issue: #250. The authority is specification clauses 7.3.1 and 7.3.2 and
`../inventor-interface/solidworks/ziptimestamps.py`. Existing source timestamp
preservation remains the default container behavior.

## Kernel decision

The new operation uses native timestamp behavior profile 1: six explicit
unsigned 32-bit words in seconds, minutes, hours, day, zero-based month and year
order. A nonzero explicit packed override wins. Otherwise absent metadata
produces zero. Year adjustment uses the documented strict thresholds at 1,980
and 80. Arithmetic wraps at 32 bits; the time term retains overflow above its
low 16 bits before OR with the date term.

Do not read a clock, infer a timezone, validate calendar ranges or predict native
uninitialized stack contents. Both supported year conventions and noncanonical
patterns remain observable. This additive operation does not change the
existing container behavior version or authorize a package version bump.

## One complete operation

Inspect an existing entry and edit its timestamp on a private copy. Capture the
input Document's current serialized checkpoint once, including pending stream
payload replacements. Parse and bind header ranges to that immutable checkpoint;
keep byte offsets and identities from another revision out of the edit path.

Write the same packed word to the selected local and central header fields.
Preserve every unrelated byte, including stored payloads, filler, decoys, names,
compression and other entries' timestamps. Reparse the complete result, verify
the two stored copies and return immutable source/result checkpoints with
completion, exact structural coverage, diagnostics, ordered copied/derived byte
history and zero approximation error. A noop retains the original checkpoint;
an inverse edit must recover the entire original input wire.

Use existing parsers, revision identities and editing machinery where their
contracts fit. Keep payload CRC and model/application validation explicitly
separate from metadata framing. No new-container key, signature, release-date,
wall-clock or timestamp reconstruction policy is part of this operation.

## Acceptance

First preserve the frozen Source requests, native fixture metadata and current
container tests. The runtime profile must match every frozen arithmetic result
and all expected header offsets. Check every single bit and complement, both
year boundaries, overrides, time overflow, missing entries, foreign/stale
identities, exact limits, noops and inverses. Verify input isolation after success,
failure, caller mutation and destruction; compare serial and concurrent results
and two fresh processes byte for byte. Include a Document with an existing
payload edit so the captured checkpoint is its current state.

Run focused tests, clippy, formatting and the exact-source PR gate before
publication. Finish PR #249 before opening another PR, then require every hosted
check before merging. Native writer execution, application acceptance and
whole-schema parity remain separate requirements. The humanizer skill is
unavailable; maintainer prose review remains pending.
