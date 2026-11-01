# Conditional mode3 geometry RGB

Section14.511 and C893 bind stored1DC RGB to a conditional candidate in the
complete consumer at18015C230. The candidate starts as three raw0.5 words.
The caller reads signed word1B8, copies the complete1DC RGBA and replaces the
candidate with its first three words only when the mode equals exactly3.
The previously bound translucent-model alpha role remains separate.

The later admitted-group loop replaces the candidate whenever mode differs
from3 or the numeric face argument equals0x405. Numeric face0x404 selects the
group's first color; other face values select its other color. Group eligibility
and further runtime branches remain necessary. A stored mode3 therefore proves
only the stored-mode condition, without establishing which color reaches geometry.

The caller passes the selected triple to18018AE40 with count1 and index5.
Its complete five-byte thunk jumps to18018A870. The complete array helper copies
three raw words per entry at stride12 after validating or growing the destination.
For that call, the destination offset is5*12=60bytes. Rejection and reporting paths
remain possible; a static submission call proves no successful array update.
No fourth color component is consumed by this triple copy.

The installed verifier retains371 caller instructions,133 helper instructions,
exact method digests, the color-copy import and allfive thunk bytes. Independent
Ghidra12.1.4 captures retain all504 method locations and the thunk's one instruction,
including their complete contiguous body extents and forwarding target. Arithmetic
unit-scale spellings with negative offsets are normalized only as identities;
changed scales and offsets remain distinct. No target code executed, the original
binary remained unchanged and the temporary decoder session closed.

Complete replay preserved all276 original stream identities, two RGB encodings
and signed stored-mode counts1:255,0:9,2:9,3:3. Actual face codes and actual triple
submission remain unknown. All425 owned parent/style contexts retained complete
EOF, including20 older opaque styles with no named mode/color. Neither group RGB
nor a guessed initializer may overwrite original1DC bytes.
Complete EOF applies to retained bodies within unchanged candidate-version and
legacy/style or modern-prefix scopes.

The strict targeted suite passed15 checks in5.292seconds and its process exited
successfully after snapshot recording. The [verification proof](mode3-rgb-evidence-verification.json)
records exact hashes, preserved unknowns and remaining gates. The full Source
suite passed201 checks in524.555seconds and exited successfully.

## Local kernel decision

Both traversal passes retain the same conditional group-color replacement.
The primary pass suppresses its array call for plot54 when the unsigned
selector-minus1 result is at most1. The reflected pass has no corresponding
plot/selector suppression. Those two call sites must remain distinct in a local
operation, even when their selected RGB words happen to match.

For the fixed count1/index5 call, the last target index is5. The helper uses
a signed comparison: capacity above5 takes the existing destination; every
other signed capacity reaches the growth helper. Only an explicitly known
zero growth result takes the copy path. A nonzero result follows the status4
reporting path. An unknown result remains unknown. This establishes a branch
contract without simulating the native allocator or proving a successful update.

Before publishing a local copy, require an explicit destination snapshot with
at least six complete triples. Require the appropriate post-growth snapshot
only after an admitted growth result. Preserve the original destination and
all three copied words; do not invent missing cells or change serialized1DC.
Caller rejection must leave unknown destination facts unused. Allocation and
whole-scene acceptance remain separate from this exact local bit-copy contract.

The strict decision suite passed16 checks in5.277seconds, including15 reviewed
pass-filter cases and14 capacity/growth cases. The [kernel proof](mode3-rgb-kernel-verification.json)
retains the contract and remaining implementation gates.

## Local operation

`mode3_geometry_rgb_inputs()` preserves optional1B8, complete1DC RGBA and the
available parent plot code. Older opaque styles keep missing named fields;
standalone styles keep an unknown parent code. `plan_group_rgb(group)` requires
the reached stored mode and unconditional full color copy before inspecting
the declared primary or reflected group context.

A nonzero group status or zero item-test result completes with no array request.
Eligible mode3 groups require the face before deciding whether group replacement
is needed. Other modes copy both group RGBA parents before requiring the face
for selection. Both parents remain retained even when only one RGB is selected.
The primary plot54/selector1-or2 filter follows selection; the reflected pass
keeps its distinct submission rule. Every reached unknown produces a typed
missing-fact error. Known zero values remain distinct from missing values.

`apply_local_array(context)` completes empty requests without requiring unused
array facts. An admitted request requires the signed capacity and the appropriate
existing or post-growth snapshot. Nonzero growth results produce typed failures.
A known zero result still requires explicit destination cells. The operation
checks fixed index5, copies all observed destination cells and changes only
that cell's three raw words. Constructor budgets apply before copying supplied
storage. Successful results retain the original plan/context, selected destination
parent, before/after words, exact local coverage and achieved zero pattern error.
All published storage is immutable and owns its copied constructor input.

All eight Rust checks passed, including the explicit zero-budget boundary and
source mutation isolation. Clippy passed for alltargets/features with warnings
denied. The strict Source suite passed18 checks in5.458seconds. Eight workers
matched serial words, and two fresh processes matched all3,944,772 independently
expected bytes. The [operation proof](mode3-rgb-operation-verification.json) and
[process proof](mode3-rgb-process-verification.json) record exact hashes and scope.

Independent expectations cover15,342 group/array/raw-pattern cases and four
separately labeled scenarios for every425 owned and276 native context. The
scenarios preserve unknown runtime facts and distinguish primary filtering from
reflected accepted-growth copies. They do not assert native invocation or
allocator execution. Full retained bodies still round-trip unchanged.

The complete native allocator, renderer state, reflection admission, exceptional
paths, geometry validity and whole-scene rendering remain outside this local
word-selection/copy contract. Alpha remains in original copied parents. No group
selection or derived array write overwrites serialized1DC. The full204-check Source suite passed in441.091seconds and exited successfully.
Exact publication checks and
remaining documented feature166 schemas still require their own acceptance gates.
