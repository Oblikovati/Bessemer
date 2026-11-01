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

## Remaining operation requirements

Project23C,240 and counted text from the canonical parent/style snapshot.
Keep unknown previous primary state distinct from a known empty CString.
Compose positive-count NUL-prefix assignment and count0 preservation explicitly;
do not silently reapply assignment when selecting an already-mutated runtime
object. Accept actual runtime strings as explicit inputs, including all seven
line positions. Do not infer them from stored control values.

A complete local selection plan must retain original wire bytes and raw controls,
its actual selected string parents and every line index. Require unknown runtime
values only when selected; disabled sources need no invented runtime contents.
Preserve all raw UTF-16 units, including embedded zeros in explicitly supplied
runtime CString state. Resource limits and missing required state return typed
errors before publication. Published results must own their reachable strings,
retain completion, diagnostics, coverage and exact-error scope, and pass mutation,
concurrency and fresh-process comparisons against an independent oracle.

No empty selected string establishes that all font, measurement, line-space or
output operations are skipped. Font names, UI labels, layout coordinates and
complete rendered text remain unresolved. This evidence step introduces no
incomplete public operation API. Feature166 still requires the complete local
selection/assignment model, its full tests and all publication gates.
