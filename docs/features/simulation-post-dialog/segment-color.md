# Element-segment RGB consumers

Section14.510 and C892 bind stored1BC RGB to the element-segment primitive
path. The canonical color still retains allfour original words. Neither the
three-channel read nor the graphics calls establish the fourth word's purpose.

The complete275-instruction consumer at18015E830 addresses core+55C and reads
three channels into a twelve-byte local through `COGLColor::GetColor3fv`.
The same local reaches136-instruction helper1800BEAA0 inR8 or145-instruction
helper1800BED10 inR9. The helpers pass the original pointer to `glColor3fv`
at three call sites across the two helpers. Complete byte hashes, instruction
text and exact import slots remain checked against the installed image.

Independent Ghidra12.1.4 captures retain555 analyzed instruction locations.
Its no-return assumption at18015ECDE excludes the trailing NOP at18015ECE3;
the complete GNU byte verifier retains that instruction and its original byte.
This provider difference remains explicit. Six recorded COMISD width qualifiers
are qualified only by their exact original text, address and eight bytes,
with wrong-text, wrong-byte and unknown-address regressions. Ghidra independently
reports their scalar widths and resolved memory addresses. Bug169 tracks the
decoder discrepancy. The target never executed and the temporary session closed.

Complete archive replay rematches all276 original stream identities and four
RGB encodings, retaining stream multiplicity. Alpha purpose, actual runtime
mode and actual primitive invocation remain unknown. Every425 owned parent/style
context also retains complete EOF.405 have exact four-component bytes;20 older
contexts retain an opaque legacy region with no inferred1BC color. The historical
color-report hash transition is qualified only for its exact recorded pair;
allother source dependencies must match unchanged.

The strict targeted suite passed15 checks in5.214seconds and its process exited
successfully after snapshot recording. The [verification proof](segment-color-evidence-verification.json)
records exact hashes, the corrected legacy boundary and remaining gates.
The full Source suite passed194 checks in517.964seconds and exited successfully.

## Local RGB read

`element_segment_color_inputs()` retains the optional canonical1BC RGBA from
dialog and standalone style snapshots. `plan_rgb_read(runtime)` evaluates
mode, first index, last index, ordered range and flags in the verified order.
Unknown facts fail only when reached. Modes outside3/4, nonpositive indices,
a reversed range and nonzero flags without mask0x4 return completed empty
local plans. The stored color is required only after those guards admit a read.

An admitted normal-flow read returns the first three raw words without numeric
conversion or clamping. Its immutable provenance retains allfour original words
and every supplied runtime fact. Completion, diagnostics and exact local coverage
remain attached. Earlier state calls and their exceptional paths are outside
this local read model. The result grants no endpoint or draw validity.

Independent expectations cover25,434 guard/bit-pattern cases and four separately
labeled runtime scenarios for every425 owned and276 native archive contexts.
Allseven Rust checks passed, including known-zero versus missing colors, alpha
independence, source mutation isolation and eight-worker comparisons. Two fresh
processes matched all1,030,588 expected bytes. Clippy passed for alltargets and
features with warnings denied. The strict17-check Source suite passed in5.189seconds.
The [operation proof](segment-rgb-operation-verification.json) and
[process proof](segment-rgb-process-verification.json) retain exact scope and hashes.

## Remaining primitive operation work

The consumer visits endpoint geometry after its color read. Endpoint node
identifiers, coordinates, possible assembly transforms and helper separation
checks remain required inputs to a complete primitive call plan. A stored color
or completed local read is insufficient to establish primitive invocation,
valid geometry or successful drawing. No UI label, physical unit or global
meaning for every1BC consumer is inferred.

A complete primitive operation still needs ordered typed endpoint dependencies,
copied geometry provenance, explicit geometry coverage and independent boundary,
ownership, concurrency and process checks. The full196 Source gate and exact
publication gates remain required for this local-read addition before integration
of feature166, together with the remaining named schema models.
