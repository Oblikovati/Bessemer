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

The [verification record](legend-yield-evidence-verification.json) records executed
checks and remaining gates. Typed local yield operations, complete interpolation,
runtime split derivation and remaining feature166 models remain outstanding.
Humanizer is unavailable in this checkout; maintainer prose review remains pending.
