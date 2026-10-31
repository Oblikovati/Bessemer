# Binary inertial and base-excitation collections

Feature [#136](https://github.com/Oblikovati/Bessemer/issues/136) covers complete
Gravity, CentriFugal and BaseExcitation collections. Implementation follows the
thermal acceptance gate in #134. Complete reads, construction and typed edits
are implemented locally; remaining corpus and integration gates are unfinished.

The initial baseline retains complete clauses 14.338 and 14.341, hashes of the
actual readers, and every native report row and candidate outcome from C-714
and C-717. Fresh extraction through the actual document/CFB adapters recovered
all 18 streams and 17,928 bytes, checking envelope and nested hashes. Every
retained outcome matches both the original callback and the full actual C-716
embedded registry. All 18 retained candidate attempts succeed; their complete
input/span packet is 24,304 bytes.

Another 135 independently built two-record cases cover every shared and derived
threshold and neighbor, populated optional bodies and both nonnull embedded
profiles. The full actual readers accept each complete image, including the
high-bit unsigned BaseExcitation header. Their input/span fixture is 481,704
bytes. The Rust reader and constructor match these complete input/span packets.

Reuse the shared load base and explicit embedded profiles from #134, preserving
every existing thermal field, identity, byte and public API. Keep any structural
extraction separate from the new derived grammars. Gravity and CentriFugal add
their word/string/double groups and the 800037 tail gate. BaseExcitation retains
its high-bit unsigned header word and the 820012 three/four-string and mixed-tail
gate. The older BaseExcitation branch has synthetic/static evidence only.

Run `python -B -X utf8 docs/features/simulation-inertial-loads/freeze_inertial.py`
for strict replay from a consistent copied source tree. The reference-only
dependency setup is shared with #134. `--write` creates initial baselines for
review; strict runs compare all existing evidence and fixture bytes.

The private shared reader, complete collection loop and common load base have
been extracted separately from new load behavior. The public thermal API stays
unchanged. All 58 focused/native regressions passed, clippy passed, and all five
complete exports matched the pre-extraction revision and independent oracles
byte for byte across 26,626,482 bytes. See the
[extraction evidence](shared-base-extraction-verification.json).

Every outer path, type, offset and raw value also matches an independent source
projection for all 135 owned and 18 native contexts. Two fresh processes produced
identical 2,412,666-byte exports matching that oracle. Embedded archives are
retained as complete atomic images in this outer projection. Field/value/limit
types reuse the existing load wire representations; the new family kind and
coverage remain explicit. Nine focused tests, four new reference checks and
the thermal projection regressions passed; clippy passed.

Set `BESSEMER_INERTIAL_NATIVE_INPUT` to
`target/simulation-inertial-native-cases.bin` for the native input/span test.
Set `BESSEMER_INERTIAL_NATIVE_PROJECTION_INPUT` to
`target/simulation-inertial-native-projection.bin` for its field projection test;
`BESSEMER_INERTIAL_PROJECTION_OUTPUT` optionally exports the full checked packet.

Typed edits now retain revision-bound original/result identities, complete source
and replacement values, exact coverage, diagnostics and every outer/nested
history relation. All six value kinds have isolated replacement tests across
all three families. No-ops preserve outer and embedded identities. Embedded
replacement tests cover deletion and creation parents. Eight threads perform
32 mixed edits per family, comparing full bytes and ordered parents with serial
results. All sixteen public types/aliases have compile-time Send/Sync assertions.

Independent actual-reader expectations also verify complete scalar-edited images
for every owned and native context. Set `BESSEMER_INERTIAL_NATIVE_EDIT_INPUT` to
`target/simulation-inertial-native-edits.bin` for the native edit oracle test.
Bug #137 adds a framing regression: semantically equal embedded CStrings with
different retained prefix widths must produce a changed wire image.

The malformed corpus retains all 15,927 actual reference outcomes; each rejects
in both readers. All 135 owned contexts pass independently measured exact
outer and embedded limits and reject each applicable one-below limit. Every
proper prefix and an appended byte also reject. The full focused run passed
25 tests, including explicit native inputs; seven strict reference checks and
clippy passed. See [adversarial verification](adversarial-verification.json).

Complete scalar-edited fields, every outer parent and every nested-copy parent
now match independent source projections in all 153 owned/native contexts. Two
fresh processes matched those full expectations across 7,242,144 bytes. Opaque
source/result handles and physical occurrence order are checked throughout. See
[history verification](history-verification.json). Set
`BESSEMER_INERTIAL_NATIVE_HISTORY_INPUT` and
`BESSEMER_INERTIAL_NATIVE_NESTED_HISTORY_INPUT` to the corresponding
`target/simulation-inertial-native-history.bin` and
`target/simulation-inertial-native-nested-history.bin` packets. Optional
`BESSEMER_INERTIAL_HISTORY_OUTPUT` and `BESSEMER_INERTIAL_NESTED_HISTORY_OUTPUT`
paths export the full owned/native checked results.

The final local PR gate and all five exact-head platform checks are required
before merging. Native application acceptance remains separate.

Native runtime dispatch, study-index association, model conversion, unit
semantics, DLL/COM execution and native application acceptance remain outside
the byte-framing evidence. This feature does not complete installed-schema parity.
