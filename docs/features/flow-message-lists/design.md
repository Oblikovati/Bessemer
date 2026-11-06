# Typed message-list implementation plan

The complete FM31 baseline is the acceptance corpus for a future typed API.
No partial-prefix or placeholder public methods are shipped with it.

Normalize the supported version once into either legacy raw-byte transfer or
modern mode-zero UTF-16 transfer. Require the modern native-mode assumption
explicitly. Retain version one's unused context without deriving a string
interpretation or doubling the byte count.

Publish immutable item, list, string and field views over owned payload bytes.
Retain physical ancestry, exact intervals, all count prefixes, raw member words
and second-list absence. A legacy string has no native text projection; a
modern string retains full units and a separate first-NUL prefix. Unpaired
surrogates and hidden suffixes must survive fixed-length edits.

Bound actual rows, strings, text bytes, fields, scopes and stored values before
allocation or access. Structural counts and versions stay read-only in existing
field edits. Every edit must reopen the complete payload under the original
mode and bounds, enforce final-NUL rules when applicable, preserve no-ops and
return source/result checkpoints with physical carrier history.

Validate all source-frozen fields and projections before adding operations.
Then check independent byte-length oracles, invalid contexts and counts, every
existing-value edit selected by a recorded corpus policy, source isolation,
fresh reversals, deterministic processes and concurrent operations. Keep native
runtime acceptance separate from exact serialized framing.
