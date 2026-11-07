# Composer vector values

Feature #101 implements the declared binary32 reconstruction profile in clause
14.97 / C-473. This follows the complete packed-array wire operation in
`docs/features/composer-arrays/`. It supplies component patterns before later
geode snapping, transformations or rendering.

Call `block.reconstruct(ComposerVectorProfile::NearestEvenGradual79V1, maximum_components)`
on an accepted `ComposerVectorBlock`. The explicit profile selects behavior 1
for document version 7.9: round to nearest with ties to even, gradual underflow
and a separate binary32 result after each operation. The component budget is
checked before allocation, including expansion of all-constant vectors.

For each quantized code q, with Q equal to 255 or 65535, the operation evaluates:

```text
t = R(q / Q)
u = R(1 - t)
right = R(t * B)
left = R(u * A)
result = R(left + right)
```

The implementation uses integer arithmetic for every rounding step. It uses no
host floating-point arithmetic, fused multiply-add, mutable global cache or
native control-register access. Integer significands retain guard and sticky
bits for addition and quotient/remainder tie decisions for rounding. Tests pin
normal/subnormal boundaries, overflow and signed-zero behavior. Behavior 1
remains the replay contract for this profile until a recorded migration.

Direct mode and constant lanes copy raw binary32 words, preserving signaling
NaN payloads, infinities and signed zero. Quantized varying lanes require finite
endpoints when the row count is positive. Unsupported nonfinite arithmetic
returns a typed error without changing the original wire block. Zero rows
produce completed empty output even when retained endpoints are nonfinite.

`ComposerVectorValues` owns immutable row-interleaved `StoredFloat` patterns.
`patterns()` exposes their exact bits and `layout()` supplies the row shape.
`report()` returns completion, the selected behavior/profile, exact source and
component coverage, achieved profile-bit fidelity, derived revision history
and explicit exclusions together. Results survive source edits and drops and
are `Send`/`Sync`; six concurrent reconstructions must match sequential output.

Native MXCSR rounding, FTZ/DAZ modes, traps and exception flags were not measured
or reproduced. No error bound against unavailable original unquantized geometry
is established. Geometry, topology, self-intersection, tolerances, loaded meshes
and native application acceptance remain separate checks. These distinctions
are returned in the reconstruction diagnostics.

The owned oracle fixture contains 8,224 complete cases. It covers every one of
the 65,792 supported code weights, all byte codes and selected word boundaries
for 15 reviewed endpoint pairs, and 8,192 generated finite endpoint pairs from
a fixed seed. Expected words come from exact integer ratios and quotient/
remainder rounding in the independent reference test oracle. The actual frozen
profile reader must agree with those expectations before fixture publication.
The code-65 counterexample pins `0xBEFAFAF9`, distinguishing the required sequence
from the `0xBEFAFAFB` result of one binary64 interpolation and final rounding.

The installed audit checks all 38 archive identities, complete scene endpoints
and every original reference vector span. All 4,367 vectors must match their
retained reference pattern hashes. The Rust native gate compares every one of
16,562,558 components (66,250,232 bytes) with those full expected arrays. An
additional 32,286 first/middle/last-row varying components are checked with exact
rational arithmetic. These checks cover the recorded domains and samples, not
every possible binary32 endpoint pair or native floating-point control state.

Run `python docs/features/composer-vectors/freeze_values.py` to replay the
owned and installed reference checks. Add `--export` to write the local native
probe packet, then run `cargo test --test composer_vector_values_native -- --ignored`.
Use `--write` only for reviewed fixture updates. All reference Python modules
are copied and hash-pinned before imports; snapshots verify their origins and
unchanged contents. Two native Rust processes must write identical output bytes.

The maintainer-side humanizer skill was unavailable. A maintainer still needs
to review this prose.
