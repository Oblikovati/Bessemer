# Direct Simulation manager Property headers

Feature issue: [#131](https://github.com/Oblikovati/Bessemer/issues/131).
Behavior profile: `simulation-manager-property-v1`. The typed implementation,
focused corpus, fresh-process checks and full local gate passed. Platform
integration gates remain pending.

## Complete declared profiles

Clause 14.345 and the actual `cosworkspersist.read_manager_property` reader
define the direct BeamManager and MassElementManager `Property` streams.
BeamManager requires an explicit nonnegative signed-i32 version. Its first
signed word targets runtime member +0x14c. Version 810024 adds +0x168;
version 820006 then adds +0x148. Preserve that order and require exactly
four, eight or twelve bytes. These destinations do not assign physical meanings.

MassElementManager retains one signed word at runtime member +0x28 and
requires exactly four bytes. No version threshold applies to that word.
Retain explicitly supplied optional version context without inventing one
from the input size. Unsupported kinds or contexts must fail explicitly.

Construction must supply the complete selected word sequence. Isolated
same-width word edits must retain complete source/replacement parents,
revision-bound normalized field IDs, ordered copy/derivation history,
completion, diagnostics, exact validation scope and achieved pattern error.
Authentic no-ops must retain original identities. All published archives and
results must be immutable, Send and Sync, with source isolation after success,
failure and caller/drop/returned-value mutation.

## Evidence and acceptance

C-721 retains 353 Beam headers totaling 4,228 bytes and 161 Mass headers
totaling 644 bytes. Beam has 351 twelve-byte and two eight-byte native
headers. Its four-byte profile has synthetic boundary evidence only.
The retained contexts comprise 357 accepted Beam candidates, 30 rejected
Beam candidates and 161 accepted Mass candidates. Preserve every context;
candidate acceptance does not prove runtime version dispatch.

Freeze the complete actual reader modules, clause and native inventory.
Check independent signed-word goldens at both thresholds and their neighbors,
signed extrema, missing/extra bytes, unsupported contexts and foreign IDs.
Compare all accepted constructions and edits byte for byte, retain every
history relation, and run isolation, compile-time Send/Sync, concurrent/serial
equivalence and two fresh-process full-export checks. Run `cargo xtask pr`
and require all five platform checks before integration. Keep one integration
PR open and merge it when green.

Nested folders, beam/mass child objects, materials, storage enumeration,
object-count inference and native writer/application equivalence remain
outside these direct headers. No dependencies, version bump or release is
planned. The maintainer humanizer skill is unavailable; its prose pass remains
for review.

## Reference preflight checkpoint

Five checks passed both during initial evidence creation and in an unchanged
baseline replay. All 514 native payloads reconstructed exactly. Every retained
candidate outcome matched the independent selected-size predicate and actual
reader, including all 30 rejected Beam contexts. Owned tests covered both
thresholds and their neighbors, signed extrema, optional Mass version context,
all wrong sizes through sixteen bytes, unsupported kinds and invalid version
contexts. The complete clause, inventory identity and actual frozen module
hashes are retained here. No installed target code executed.

## Implementation checkpoint

The public profile requires a normalized version for Beam and retains an
explicit optional version for Mass. Both archives require exact EOF and
construct the complete selected signed-word sequence. Physical field IDs bind
to their original revision. Isolated edits retain source/replacement parents,
every ordered field relation, exact coverage and authentic no-op identities.

All ten focused tests passed, including the native export. The complete corpus
covers 8,553 owned and 7,928 native read/edit contexts. Every accepted image was
reconstructed through the complete constructor. Literal independent splices
checked every replacement and all physical/runtime offsets, full signed words,
copy/derivation history, profile context, completion and diagnostic coverage.
All thirty retained candidate mismatches remained typed errors.

The full 5,257,724-byte export matched the frozen reader. Two fresh processes
each passed all ten tests; their complete outputs matched the reference and
each other byte for byte. All thirteen public types have Send/Sync assertions;
eight threads repeated complete edits and checked every history relation.
Caller/drop/returned-field/buffer isolation and foreign-ID rejection passed.
Clippy and the function/file-size audit passed. All seventeen earlier final-LBC
tests passed with their native export, whose complete 6,599,636 bytes remained
unchanged. The expanded frozen-reader corpus and strict unchanged-baseline
replay passed six checks. The full local gate passed 3,114 tests with zero
failures and twenty-nine opt-in tests ignored by the standard gate. The native
export was run explicitly. `complete-verification.json` records the full gate,
indexed fixture/source identities, exact corpus hashes, process observations
and acceptance exclusions.
