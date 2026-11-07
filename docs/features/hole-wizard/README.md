# Hole Wizard interoperability

Bessemer reads, reconstructs and edits complete ordinary-part Hole Wizard
features through the existing resolved-feature API. This advances Feature #54
and goal #2. The reader consumes the complete inherited body, the compatibility
word present only from `0x2F46` through `0x2F67`, and the suffix documented in
section 11.151.

Raw numeric parameters, full flag bytes, strings and pair elements retain their
serialized values. Native post-load code can overwrite selection words,
normalize type codes and convert a byte to boolean; Bessemer preserves the
original wire representation, including NaN payload bits. Unknown meanings
remain explicit rather than receiving semantic coverage credit.

Nonzero slot enablement selects a complete distance dimension, direct slot-axis
bodies and the remaining slot fields. Before `0x2B5E`, the pair count also gives
the number of slot bodies, with no second count on wire. Later versions carry an
independent signed count. Every axis contains two points and a vector, totaling
72 bytes. Direct axes allocate no object-map slots; separately tagged
`moSlotCenterAxisData_c` objects use the same complete body with normal map
insertion. The reader retains the old conditional and new unconditional
placements of word +83C on either side of `0x42EA`.

Independent fixtures cover 72 complete Hole Wizard objects across 18 version
contexts, plus two separately tagged slot axes. Their 28,544 primitive
observations partition every byte. The reference reader reaches exact EOF and
records final object-map slots. Tests compare every primitive, byte and slot,
reject every truncated prefix and trailing byte, reject old contexts, check
negative/oversized pair and slot counts, and reject unrelated dimension classes.
Additional assertions check the independent compatibility gates and values that
native runtime normalization would change. Positive slot bodies and populated
arrays are synthetic; native examples have slot enablement zero and empty arrays.

Five owned streams use the blank part's actual model and CMgr state with 97
inherited slots. They cover enabled/disabled slots, shared dimensions and
references from both collection framings back to the feature. Public tests
reconstruct whole documents byte for byte. A longer Unicode label edit matches
a separately constructed expected stream, preserves all other fields and
object identities, records one field-history change and leaves source archives
and documents unchanged. Three Python tests check deterministic regeneration,
all gate boundaries, invalid inherited context and the independent label edit.

The staged `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 445 library
tests, all integration suites and 265 doctests. Three external corpus gates
remain ignored by default. Six private reader tests, two public operation tests
and three Python fixture tests passed. `local-verification.json` records the
executed scope and runtime/test source hashes.

Two fresh native corpus processes reconstruct 516 of 906 feature configurations
exactly, with identical outcomes and failure causes. Every previous 483 success
is preserved. The broad parity gate still fails for 390 explicit stops. The
independent reader verifies all 516 accepted streams, totaling 27,665,368 bytes,
against four-stream input hashes. `frontier-checkpoint.json` and
`accepted-reference.json` retain those observations; full documented schema
parity remains incomplete.

`reference.json` binds fixtures, generators, reused builders and reference
readers. `schema-snapshot.md` preserves the relevant specification sections with
raw-byte hashes in `schema-reference.json`. Runtime file version 19000 remains
distinct from model wire version. Database paths, historical contexts outside
the modern scope, geometry, topology and native application acceptance remain
outside stored-schema validation. No dependency or package version changed.
The humanizer skill is unavailable; its prose pass remains with the maintainer.
