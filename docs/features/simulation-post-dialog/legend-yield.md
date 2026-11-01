# Legend gray-above-yield color and runtime split

Specification14.470 and C-850 identify serialized `legend.color_d8` as the
gray-above-yield base color. Its four binary32 words appear only after version
810018. They remain raw color words: equal RGB channels and alpha1 in the
observed corpus establish neither a validation rule nor a default. The runtime
yield double at170 and signed split index at178 are separate, unserialized state.

The native gray mapper always copies D8 and constructs packed white before
reading the split index. Index-2 forwards the entire requested signed count to
the mapping helper, with the copied D8/white pair and raw reversal word. Other
negative indices, or indices at least the requested count, return0 without a
mapping call. This fallback still performs the initial color construction.

A remaining index maps the suffix first, then the ordinary prefix. The suffix
pointer displacement uses sign extension of the wrapped signed32 value3*index,
then multiplication by4. It is not an unbounded index*12 calculation. Index0
still makes the second call with a zero prefix count. Reversal, ordinary base
count and index are read again after the suffix call. These dependencies matter
for any future call planner: a snapshot alone cannot establish helper-induced
state changes or actual output. Return1 follows completion of the mapping calls;
it does not validate their interpolation or memory access.

The complete adjustment method has52 instructions and three return sites. It
uses the requested count, D4, runtime yield and caller tick/range arrays. Its
selected index and possible tick replacement depend on floating-point comparisons
and search behavior. The stored flag or D8 alone cannot supply that index. This
evidence binds the complete method without claiming a complete adjustment model.

The verifier compares all67 mapper instructions and52 adjustment instructions
against the installed, whole-module-pinned bytes. Independent Ghidra captures
cover all119 locations and both complete contiguous extents,450bytes in total.
Both temporary analysis sessions closed; no target code executed. Exact named
imports bind the D8 copy constructor and packed-white constructor. The copy
constructor shares the already pinned four-word copy-assignment body; no linker
cause is inferred. The packed constructor retains its exact binary32 inverse
scale and complete provider bytes.

One instruction at1800DB110 has the signed immediate spelling `-2` in the
original probe and `0xfffffffe` in GNU disassembly. Bug172 records the exact
address, text and four bytes. The local qualification rejects every single-byte
mutation, changed register, immediate or operand width; unknown addresses remain
unqualified. Global disassembly normalization remains unchanged.

All284 original rows remain retained:276 accepted contexts,212 stored colors,
64 accepted color absences and eight rejected streams. Complete current parent
parsing preserves the original prefix/EOF qualifications and identities. All425
owned contexts retain their actual version-conditioned presence, including116
absences. No new stored-byte coverage or actual native gray-path observation is
claimed. The exact historical color-report transition is admitted only beside
complete current replay.

## Typed stored views and ordered call requests

`SimulationPostDialog::legend_yield_inputs` and
`SimulationPostStyle::legend_yield_inputs` retain optional D8 and entry reversal.
`plan_gray_calls_nearest_even` copies all four D8 words and constructs white
before requiring the runtime index. A missing color therefore fails even when
the supplied index would fall back. Entry reversal is required only for a reached
mapping call; a missing post-suffix state fails only on the partial path.

`SimulationLegendYieldRuntime` supplies the requested signed count and entry
index. Its optional `SimulationLegendYieldPrefixState` supplies the three actual
post-suffix rereads. Those values may differ from entry state, including a changed
negative prefix count or reversal. The plan forwards them without revalidating
the earlier index test. Neither an unchanged state nor an actual helper return
is inferred.

Completed immutable plans retain the original stored/runtime parents, a derived
history relation, the copied color pair and all ordered calls. Each call retains
the exact signed count, base count, raw reversal, base-area identity and signed64
relative byte displacement. Fallback produces a completed empty call list;
mapping plans expose a conditional return1 only if every requested call completes.
Coverage and diagnostics distinguish exact local integer/color-pattern planning
from unmodeled interpolation, tick adjustment and pointer validity.

Independent unbounded-integer expectations cover13,632 boundary/generated cases.
Complete parent replay covers425 owned and276 native contexts plus210 arbitrary
four-channel/reversal mutations. Every parent receives eight explicitly
hypothetical runtime contexts. The generated presence mask uses high LCG bits
to avoid low-bit parity repeating the same missing-input path.

All thirteen Rust checks passed, including every native case. Eight-worker
comparisons match serial operation and parent results, and all ten public types
satisfy `Send + Sync`. Clippy passed with warnings denied. Two fresh processes
matched all20,920 independent expected results, totaling1,422,560bytes each.
The final strict Source suite passed17 checks in6.163seconds and exited
successfully after snapshot recording and cleanup.
The [operation record](legend-yield-operation-verification.json) and
[process proof](legend-yield-process-verification.json) retain exact hashes and
remaining integration gates.

The [evidence record](legend-yield-evidence-verification.json) records executed
checks and remaining gates. Complete interpolation, runtime split derivation and
remaining feature166 models remain outstanding.
Humanizer is unavailable in this checkout; maintainer prose review remains pending.
