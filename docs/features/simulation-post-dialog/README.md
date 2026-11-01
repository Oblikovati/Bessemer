# Dialog and standalone post-processing interoperability (#166)

This feature covers the complete DGXPOST dialog grammar, standalone PostInfo and
PostStyle carriers, and their documented stored-field roles and bounded derived
views. C-766 through C-770 establish the staged dialog framing. C-840 establishes
the separate legacy helper routes. Later PostInfo, cut-primitive, color, style
and legend evidence remains part of the required scope, including the field
catalog and detailed legacy view; this feature does not stop at prefix framing.

The original modern inventory has 224 dialog streams and 496,979 logical bytes.
Every original candidate and all five staged outcomes are replayed exactly,
including values, spans, prefix/complete status and hashes. Prefix consumption
is repeated evidence over the same bytes and receives no additional byte credit.

The separate standalone inventory has 120 streams and 44,640 bytes. PostInfo has
60 helper-complete blocks, 52 with a caller-compatible retained version.
PostStyle has 52 helper-complete/caller-compatible blocks and eight failed
records. All 240 original arguments remain present: 172 helper successes and 68
failures. The sixteen record/context exceptions remain separate from the 104
caller-compatible records. Helper EOF does not establish that its parent chose
the standalone route.

The full corpus scan also found 251 `cwPostDlgParam` streams under legacy study
paths, totaling 275,470 bytes, outside C-770's Results inventory. Their complete
payloads and primary/coexisting root version sources are retained separately.
All 251 fail under those retained candidates. No lower version is selected to
make them fit; their grammar/context mismatch remains unresolved. The original
224-stream denominator remains unchanged.

Three older reports pin PostStyle helper hash `a13ebac8…626ad3f`; the current
helper hash `3bbb7feb…1982145` is independently pinned by C-770 and C-867. The
single approved source transition adds C-867's detailed below700006 view.
Every other original dependency pin must match. Fresh replay requires every
older native output unchanged and the complete C-867 synthetic output, including
its overlapping outer raw span and detailed view. The C-867 audit is retained as
provenance and is never executed: it would regenerate evidence in the sibling
repository. Its historical color-report dependency is not consumed by these
checks; imported implementation origins and hashes are captured in the immutable
source snapshot.

The field refinement now retains C-841's original PostInfo projection and the
complete C-843 primitive/direction/contour view for all sixty native blocks and
720 slots. Only the three documented annotation keys are removed when checking
the older projection; every original raw word, finite view and sign-flipped bit
view must remain identical. Current RGBA replay preserves all 359 original
candidate outcomes across 284 style/dialog carriers, including failures and
the modern prefix-only qualification.

The current machine-readable catalog has 118 labels: 103 bound consumer roles,
one framing role and fourteen unresolved purposes. This reflects its later
evidence provenance beyond the older counts recorded in section 14.479. Every
descriptor, unknown purpose, eighteen-version probe, native occurrence and byte
category is recomputed exactly. All contributing evidence packet hashes match.
Core primitive, contour and RGBA method spans, exports, tokens and the float
sign mask are bound to installed bytes without target execution. Four further
ancestor source transitions are explicit in
[the field adoption record](field-source-adoption-reference.json); no general
source-hash exception is allowed.

Installed checks read all five complete caller/provider images without loading
target code. They bind dialog and legacy helper imports/exports, legacy stream
literals, all recorded serializer function spans, color/reference/shared-text
and equation delegates, and the recorded IAT slots.

Run the foundation with:

```powershell
python -B -X utf8 docs/features/simulation-post-dialog/freeze_post_dialog.py
```

Expanded initialization and strict replay each passed all 24 checks: seventeen
original tests and seven foundation checks. Both processes exited successfully
after cleanup. [Foundation verification](foundation-verification.json) records
the source snapshot, exact packet/log hashes, source adoption and exclusions.

Field initialization and strict replay each passed all 43 checks and exited
successfully after cleanup. [Field-model verification](field-model-verification.json)
records the exact current evidence, full-view comparisons, catalog and log hashes.

Independent complete wire literals now cover 495 contexts and 175 distinct
versions: 225 parent dialogs, seventy standalone PostInfo records and two hundred
standalone PostStyle records. The 1,462,773 logical bytes are assembled before
calling the reader. Every source span, endpoint and reconstruction hash matches.
Adjacent gates include populated references, sensors, integer/string lists,
equation variables and the byte-list/character-text transition; newest controls
include duplicate reference records and both equation selector states.

PostInfo literals keep all twelve physical slots even when active-count words
are negative or extreme. Five bit rotations additionally check known/unknown
primitive and direction views, exact normal/center/shared-scalar bits, closed-form
zero/subnormal values, nonfinite handling and the D-view sign XOR. Empty text,
surrogates, embedded NULs, odd-byte persisted reference text and both embedded
archive dialects remain distinct. Initialization and strict replay each pass
all 45 checks and exit successfully. [Owned-literal verification](owned-literal-verification.json)
records the exact frozen outputs and exclusions.

Complete primitive and edit-history expectations now accompany the 495 owned
inputs. The fixture packets retain each physical field's path, type, byte span
and exact value, plus a literal first-word edit and its ordered outer/nested
history. Counted character text and byte payloads stay atomic. The legacy
368-byte style view is partitioned once rather than counted again as an
overlapping raw block. All 396 successful native helper contexts have separate
projection/history exports; this does not turn helper acceptance into caller
acceptance or cover the additional legacy mismatch cohort.

The parent scalar partition follows schema14.492–493: the stored min/max code
and deformation selector are signed integers, and the two scales and four
range values are float32 bit patterns. Remaining unresolved words retain their
raw bits. These stored roles do not establish units, normalization, runtime
activation or geometry. [Packet hashes](operation-packets-reference.json)
identify the complete expectations; Rust codec acceptance remains pending.

The complete C-874/C-875 parent-view reports are retained and replayed against
all 224 native parent contexts. Installed export names, complete accessor bytes
and the constant-one literal match the hashed provider. The two historical color
replay-input hashes differ from the current report; the exact transitions are
recorded, and every original parent row is checked directly against recovered
native bytes. No historical audit is executed or rewritten.

All 225 owned parent inputs also carry range/scale expectations. Another 210
literal controls cover all min/max-code and deformation-selector branches,
unknown signed extremes and five exact float32 patterns. Stored values and
selected views remain separate: no units, range ordering, UI mode names or
geometry activation are inferred. Initialization passes 57 checks, including the
five original range/scale tests. Later bounded consumer views remain pending.

Owned complete operations, full later field-role/view adoption, typed Rust
read/construction/edit, atomic text profiles, scoped embedded archive acceptance,
independent fields/history/resources, adversarial limits and extents, isolation,
concurrency, process exports and full local/platform acceptance remain pending.
Imported-helper text, PostStyle's separate defensive text bound and standalone
700002 PostInfo safety limits must remain distinct. Unknown bits and enum words
must survive without runtime normalization or invented physical meanings.

Native application opening, the additional legacy mismatch cohort, remaining
carrier exceptions, universal runtime version selection, unknown physical
meanings and whole-format parity remain unproven.

Humanizer was unavailable; maintainer prose review remains pending.

The standalone `SimulationPostInfo` operation now reads, constructs and edits the
complete 460-byte helper body and its exact version700002 suffix. Its 115 fixed
fields retain enum words, unresolved word bits and float32 patterns; all twelve
slots remain present for negative or extreme active counts. Counted character
text is an atomic value, preserves raw code units and uses an explicit configurable
character safety budget rather than the imported helper's separate 1024-character
profile. Reads require EOF, and edits return immutable inputs/results, exact
coverage and ordered field history through the existing load adapters.

Thirteen focused Rust checks pass, including all 70 owned contexts, all 120 native
PostInfo helper arguments from 60 streams, complete field and literal edit-history
oracles, every-field changed/no-op edits, input isolation, five resource axes,
full 0x100000-character safety controls and eight-worker replay. This helper
operation does not infer named-stream dispatch compatibility. Parent-dialog and
PostStyle codecs, remaining bounded views and full issue166 acceptance remain
pending. Humanizer prose review remains pending.

`SimulationPostStyle` now reads, constructs and edits complete historical and
modern helper profiles. The legacy 368-byte body has 20 preserved raw words and
72 float32 color components. Modern fields follow every documented legend/style
gate, including the two legend-position float32 values. The operation shares
PostInfo's four stored value kinds, character frame and budget representation;
its reader applies the selected PostStyle layout and character cap. The reviewed
style safety profile uses 512 characters, with independent payload and aggregate
budgets. Loading preserves all stored bits and does not apply rendering defaults.

Fourteen focused Style checks and all thirteen Info checks pass together. Style
coverage includes 200 owned contexts / 74,492 bytes, every proper prefix plus a
trailing byte, 18,560 changed edits and the same number of no-ops, complete field
and ordered history oracles, five exact/one-below resource axes, raw text ownership
and eight-worker replay. Native replay matches all 120 retained Style candidates:
52 accepted / 14,768 bytes and 68 rejected; all 52 successes are caller-compatible
in this cohort. The complete native field and literal edit-history expectations
match byte for byte. Full Style PR/platform acceptance remains pending, as do the
complete parent-dialog acceptance, remaining bounded views and expanded legacy mismatch cohort.

The parent edit oracle now records every one of 117,435 physical field mutations
across all 225 owned dialog contexts. Both complete grammar acceptance and
preservation of unselected field paths, kinds, values and rebased spans come
from fresh pinned Source parsing. The resulting matrix contains 115,312 valid
isolated edits and 2,123 rejected structural mutations across all eight value
kinds. The earlier 662 missing-terminal-NUL PersistText mutations remain separate
negative controls; valid odd-byte replacements retain the required terminator.

Initialization and strict replay each pass all 58 reference checks. The compact
matrix stores complete replacement frames and independent outcome/span facts;
complete original inputs and primitive expectations remain frozen separately.
[Edit-oracle verification](edit-matrix-verification.json) records input/output
hashes, exact counts, completed Rust checks and remaining acceptance work. Parent
prefix and expanded concurrency acceptance are tracked separately.


The full strict replay now passes 59 checks, including independent resource
counts for all 225 owned parent contexts. The resource packet records outer
bytes, fields, variable entries and text payloads, plus per-root embedded byte,
field and new-object maxima. Nested coverage totals remain separate from those
per-root limits. Rust accepts each exact budget and rejects one below during
both reading and construction. [Resource verification](resource-verification.json)
records packet, snapshot and verification-log hashes. These checks do not claim
application acceptance or geometry validation; complete parent acceptance is
still in progress.


A separate pinned Source replay checks complete boundary inputs for five
collection roles and every reached opaque-byte role. It replaces existing
collection records before inserting literal records, retaining the complete
suffix. Negative lengths and counts, exact caps, and one above each cap have
independent outcomes in [the boundary reference](cap-reference.json). The
collection cap is 10,000; reference blobs allow 1,000,000 bytes and legacy list
payloads allow 1,048,576 bytes. These targeted checks supplement the full gate.


The parent codec uses the complete versioned grammar and requires EOF. It reuses
the Info and Style field readers while applying the parent text caps and scoped
reference archive dialect. Published inputs and results own immutable bytes and
fields. Construction reparses the complete supplied shape; isolated edits return
the selected field result together with unchanged inputs, diagnostics, coverage
and ordered outer/nested provenance. All 21 checks in the first expanded parent
run passed, alongside 13 Info and 14 Style checks. The exhaustive parent checks
covered 117,435 changed requests, the same number of no-ops, every proper prefix
of the owned bodies, and eight-worker replay. [Focused verification](parent-focused-verification.json)
records the counts and remaining acceptance work. Later bounded consumer views
remain part of issue #166.


Nine populated reference roles also have independent whole-record count
controls. Each complete 10,000-record body is accepted by both readers and can
be reconstructed by Rust; negative counts and 10,001-record bodies fail. The
controls repeat an accepted complete record, including its scoped embedded
archives, and retain the original suffix. Their outer byte and field budgets
allow the full input so the count check is tested independently. See
[the reference-count evidence](reference-cap-reference.json).


Two fresh processes produced identical complete field and edit-history packets
for all 449 parent contexts: 225 owned and 224 native. Both outputs also match
the independent Source expectations byte for byte. The history packet retains
other helper contexts as passthrough framing; those add no parent runtime
coverage. [Process verification](process-pair-verification.json) records sizes,
hashes and exclusions.


The full strict reference replay passes all 61 checks after integrating both
resource denominators and complete reference-count boundaries. The
[resource verification record](resource-verification.json) retains the latest
snapshot and log hashes alongside the earlier 59- and 60-check runs.


The parent codec passed the local PR gate: 3,561 tests passed, none failed,
and 118 were ignored. Two later parent archive-boundary tests and the expanded
public-type thread-safety assertions passed separately; the final all-target
Clippy run also passed. [Operation verification](parent-operation-verification.json)
records the gate scope, logs and remaining cross-platform acceptance. The
consumer views remain a separate implementation step under the same issue.


The range and deformation-scale accessors implement the bounded prefix views in
specification 14.492–14.493. Their complete raw-bit expectations cover all 225
owned contexts. Another 210 cases cross every documented state and selector
branch with zero, signed zero, subnormal, infinite and NaN patterns. A state edit
refreshes the selected endpoints while the source view stays unchanged. Unknown
state codes remain visible and select baseline endpoints; unknown scale
selectors remain visible and select literal float32 one. Stored values remain
available alongside the selections. The pinned helper replay also checks the
225-context packet and its input provenance.


The integrated range/scale packet passed the full 61-check strict Source replay.
Four focused Rust tests and all-target Clippy passed. All five full/partial
snapshot-selection modes were checked separately; partial runs retain their own
evidence files. [Prefix-view verification](prefix-view-operation-verification.json)
records the executed checks and remaining publication work.


PostInfo named views expose all twelve physical slots independently of their
stored active counts. Known primitive and direction codes receive typed names;
unknown codes retain their original integers and no inferred name. Normal,
center and shared-scalar patterns remain unchanged. The derived D pattern flips
only the scalar sign bit. The header view retains all seven words, signed count
values and the noninvertible forward contour preference conversion. Five frozen
Source blocks cover sixty slots, including unknown codes and non-finite patterns.
All 17 PostInfo tests passed with native checks included, and all-target Clippy
passed. Ten targeted pinned Source checks passed. Reverse/default contour policy,
runtime comparison and the other consumer views remain separate obligations.


The named PostInfo views passed the full 61-check Source replay and all 18 final
Rust tests, with native checks included. The native named-role check covers 60
physical blocks and 720 slots under both retained helper arguments. Its 1,440
slot observations repeat those same stored bytes and receive no extra byte
credit. All-target Clippy passed after that check was added. See
[named-view verification](postinfo-named-view-verification.json).
