# Record 122/434 typed interoperability

The readers enforce the exact 200-byte legacy and 29-byte modern layouts.
The legacy gate word remains raw data, with no version dispatch. Modern
versions are restricted to zero and one. Views expose every stored field,
including legacy words that do not contribute to the native update projection.

A typed update projection preserves the signed member values, unsigned state
byte, legacy constants and disabled-gate absence. It reports stored input and
documented update semantics without mutating any native object. No-record absence
remains separate from a selected record with an empty update projection.

Selection uses the latest occurrence of record 434 when present, otherwise
the latest record 122. It parses only that occurrence and returns its typed
failure without a fallback. Selection tests must include malformed preferred
records, shadowed malformed records and duplicate occurrences.

Edits replace existing raw words, modern members or the stored state byte on
a copy, then reparse the complete replacement and recompute the update projection.
Version and layout remain fixed. The result includes exact coverage,
diagnostics and full carrier history. Acceptance requires independent raw byte
patches, gate transitions, no-op preservation, fresh reversals, resource boundaries,
source isolation, public thread traits, concurrency and two-process agreement.
Native runtime application and engineering interpretation remain separate checks.
