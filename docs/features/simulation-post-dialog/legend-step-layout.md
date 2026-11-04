# Local legend step-layout integers

Specification14.471 and C-851 bind the renderer's two FC-dependent layout
calculations. `SimulationLegendStepInputs::project_step_layout` projects those
integer values from explicit caller facts. It does not execute the renderer or
prove that its text helpers return. Post-helper values are conditional on the
supplied state at those later reads.

An explicit enable0 completes before reading FC, coordinates, size or helper
state. The plan contains no scalar values and reports a completed early return.
Other raw enable values require the reached facts. Missing values return typed
errors naming the first missing native dependency; no partial scalar result is
published.

Entry FC zero reads pointY then sizeY and computes their low32 difference.
Nonzero FC reads runtime12C then124, wraps their difference to signed32, divides
by20 toward zero, subtracts sizeY and adds pointY with low32 wrapping. Both
paths then read pointX before the text helpers. Unknown range words are not
required on the zero-FC path, but unknown pointX remains an error.

`SimulationLegendStepLayoutAfterHelpers` supplies the later spacing argument,
second FC read and runtime1B0/1B4 reads. These facts may differ from entry state.
Spacing is read before FC. Nonzero second FC replaces it with2*spacing-4.
Nonzero1B0 then triples spacing without reading1B4; zero1B0 requires1B4 and
triples spacing only if that value is nonzero. Finally the local vertical
result subtracts adjusted spacing from initialY. Every operation retains the
native low32 pattern, including negative and overflowing values. Algebraically
neutral supplied values do not remove required reads.

The divide-by20 implementation is independently certified against the complete
bound instruction sequence. With multiplier M=0x66666667 and D=2^35,
20M-D=12. For each signed32 magnitude n=20q+r,0<=r<=19, the remainder numerator
12q+Mr is positive for nonzero n and at most33,930,241,645, below D=34,359,738,368.
The native signed high product followed by SAR3 therefore gives the required
floor or negative ceiling; its logical sign correction gives truncation toward
zero. The product also fits signed64 across the entire domain. Additional
regressions compare8,359 boundary/generated instruction-emulation results with
independent unbounded division. The
[certificate](legend-step-layout-division-reference.json) retains the exact
bounds; all373 renderer instructions remain part of the complete C851 binding.

Completed immutable plans retain entry FC, every supplied runtime fact, a
derived history relation, optional complete scalar values, completion,
diagnostics and exact reached-read coverage. The coverage distinguishes both
entry FC paths and the short-circuited second runtime flag. Local integer
pattern error is zero. No text labels, helper arguments, runtime160 write,
font/drawing effects, pointer safety or pixel validity are inferred.

The independent corpus covers35,468 cases:24,576 optional-input/enable/FC/flag
combinations,2,700 signed arithmetic boundaries and8,192 generated contexts.
Complete parent replay retains425 owned and276 native contexts plus268 full FC
mutations. Each receives eight explicitly hypothetical runtime contexts rather
than an inferred native invocation.

All fifteen Rust checks passed, including complete native replay and eight-worker
comparisons. All eight public layout types satisfy `Send + Sync`. Clippy passed
with warnings denied. The final strict Source suite passed23 checks
in9.456seconds and exited successfully after snapshot recording and cleanup.
Two fresh processes matched all43,220 independent expected projections,
totaling1,383,040bytes each. The
[operation record](legend-step-layout-operation-verification.json) and
[process proof](legend-step-layout-process-verification.json) retain exact hashes
and remaining publication gates. The full integrated Source suite then passed
268 checks in500.038seconds and exited successfully after snapshot recording.

Full renderer output remains outside this local integer projection. Other
required feature166 models and exact publication gates remain outstanding.
Humanizer is unavailable in this checkout; maintainer prose review remains pending.
