# Primary and seven-line text controls

Section14.508 and C-890 bind stored23C and240 to zero/nonzero source selection.
The primary branch copies the runtime CString at PostStyle248 when23C is
nonzero and constructs an empty CString when it is zero. Word240 independently
selects seven runtime CStrings at250 through280 or seven local empty CStrings.
These are selection roles, without recovered UI labels or complete layout.

The seven runtime strings are not seven additional wire fields. The existing
counted `style.utf16` payload supplies the primary CString on positive-count
assignment. Embedded zero units truncate that assignment prefix, while every
counted byte remains preserved. Count0 skips assignment and therefore retains
an earlier runtime CString; it does not establish an empty runtime primary.
Lone surrogate units remain raw units without Unicode repair.

The evidence replay preserves all276 C890 accepted identities, complete bodies,
raw controls, character counts, text bytes and conditional source descriptors.
Every native set has23C=0,240=1 and count0. Actual runtime line contents remain
unknown. All425 owned contexts are also parsed through exact EOF;405 contain
the named controls and20 retain missing older groups without initialization.
The owned matrix contains112 positive-count payloads; these remain synthetic
evidence and do not imply populated text in the supplied native files.

C855 retains284 identities:224 original parent-prefix observations,52 strict
helper observations and8 unaccepted interpretations with no text views. All276
accepted views are independently matched to complete current bytes, including
text offset/end, counted bytes, assignment-performed flag, NUL prefix and code
units. The earlier prefix-only qualification remains attached to its source
record. The eight original explicit interpretations retain their failed status; empty views do not
become successful empty text results.

Two exact historical color-report hash transitions are qualified by this replay.
Every other C890 and C855 source, probe, helper, original unit test and retained
decompilation pin must match. The original evidence records remain unchanged.

The installed verifier matches the complete315-instruction display consumer
and all three earlier C855 assignment/helper bodies. It binds both conditional
source branches, the seven empty-string constructors, their shared zero UTF-16
literal and actual CString IAT slots. The imports are `mfc140u.dll` ordinal286
at1802CCB90 and ordinal280 at1802CCB98; imported symbol names are absent.
The constructor descriptions come from the documented static evidence, not
invented named import entries or observed execution.

Independent Ghidra12.1.4 captures retain all315 display-consumer instruction
locations and the two zero bytes at1802D7C10. Provider limitations, evidence IDs
and full original records are retained. The source DLL was not executed or
edited, and the temporary analysis session was closed.

The strict targeted suite passed14 checks in4.987seconds. Its process exited
successfully after snapshot recording. The [evidence proof](text-control-evidence-verification.json)
records exact revision, hashes, decoder facts, preserved qualifications and
outstanding operation/full-suite/publication requirements.

## Local assignment and selection

`text_display_controls()` reads canonical parent/style words23C/240 and the
complete counted text frame. Older missing controls remain typed missing
values. The getter never initializes a runtime primary or invents line text.

`assign_primary(previous, maximum_code_units)` explicitly resolves known primary
state after load. Positive count assigns the first-NUL prefix; zero count
preserves a known prior CString. An unknown prior at count0 returns typed
missing state, while a positive leading-NUL payload completes an empty
assignment. The result retains complete wire bytes, optional previous text,
assignment kind, known output, diagnostics and per-stage coverage.

`plan_selection(runtime, maximum_code_units)` reads actual current runtime
CStrings. It does not reapply stored text. Each zero display word selects empty
literals without requiring unknown runtime contents; each nonzero word requires
its primary or seven line inputs. Explicit runtime units after embedded zeros
remain preserved. Array positions retain all seven line indices and selected
origins identify each runtime parent.

`plan_after_assignment(previous_runtime, maximum_code_units)` explicitly composes
both stages and retains the assignment result with its parents and coverage.
It first requires a known assignment result, even when a display control is
off. Call `plan_selection` directly when selecting current state without that
load operation. Each stage records its own reached CString dependencies and
budget; the combined result retains both stages rather than presenting the
selection count as the complete assignment footprint.

The supplied bound limits counted input and known output for assignment, and
the sum of eight selected strings for selection. It is a resource policy, not
a native validity rule. Current selection does not inspect unused stored text
against its output budget. Missing selected state and budget/overflow failures
return typed errors before publication, with no successful partial selection.

The existing text type owns immutable private byte storage, copies borrowed
input on construction and exposes only immutable payload slices. Plans and
assignment results retain those immutable values; no mutable payload can be
reached through a published result. Tests mutate the original constructor
buffer, replace controls and runtime strings after publication, and compare
all result parents and bytes. Success and failure also leave input contexts
unchanged. All ten public operation types assert Send/Sync and eight workers
replay the complete matrix against serial bytes.

An independent oracle covers18,200 cases over arbitrary signed zero/nonzero
controls, unknown versus known-empty state, both assignment kinds, every
missing-line position, raw NUL/surrogate/supplementary units, explicit
composition and resource boundaries. The original Source view remains bounded
at512 units. Additional513-unit programmatic-budget cases use the reviewed
native bounded-copy loop as a separate oracle; they do not establish native
allocation behavior, oversized-file acceptance or a changed Source-view limit.
Complete projections retain all425 owned and276 native archive contexts.
Native projected operation inputs deliberately mark runtime strings unknown;
their typed missing results are local dependency checks, not observed native
failures or invented runtime contents.

No empty selected string establishes skipped font, measurement, line-space or
output work. Font names, UI labels, layout coordinates, runtime production and
complete rendered text remain unresolved. All7 Rust integration checks and the overflow regression passed. Clippy passed
with warnings denied, and the strict Source suite passed16 checks. Two fresh
processes matched18,901 contexts and609,580 output bytes against independent
expectations. The [operation proof](text-selection-operation-verification.json)
records revisions, log hashes and exclusions. The expanded full Source suite
and publication gates remain pending. All other named
schema models and publication gates remain required for feature166.
