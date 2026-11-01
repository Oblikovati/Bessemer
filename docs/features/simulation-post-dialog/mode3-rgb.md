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
records exact hashes, preserved unknowns and remaining gates.

## Remaining local operation work

Project actual optional word1B8 and complete1DC RGBA from canonical snapshots.
Retain unconditional caller copying separately from conditional candidate RGB,
including original alpha and older unavailable facts. Keep group eligibility,
face-dependent replacement and actual array-update acceptance as distinct stages.
Unknown reached runtime facts cannot become assumed false or zero.

A complete local operation needs immutable provenance, typed completion and
failure, explicit exact bit-copy coverage, independent branch/array-offset oracles,
raw-pattern boundaries, input isolation, concurrency and fresh-process comparisons.
Array copying does not validate geometry or prove rendering. The complete Source
and publication gates remain required alongside the remaining feature166 schemas.
