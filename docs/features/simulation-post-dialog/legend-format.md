# Legend number-format evidence

Feature [#166](https://github.com/Oblikovati/Bessemer/issues/166) includes the
four raw legend words and bounded format construction documented in specification
14.466, C-846. Typed local plans expose construction and alternate selection
with their inputs, completion, history, diagnostics and exact local coverage.

The original Source model constructs a format string from signed mode B8 and
precision BC. Mode 1 selects `f`, mode 2 selects `g`, and every other raw mode
selects `e`. Width is precision plus seven with signed 32-bit wrapping. Neither
mode nor precision is validated or rewritten. Generating a string does not prove
that a native formatting library can render an arbitrary precision successfully.

C4 and C8 retain raw signed values. When present, C8 equal to zero selects `f`
for the alternate format; every other value selects `g`. Their absence under
older versions remains absence. The installed alternate method also tests an
output pointer, mode zero, nonzero C4 and a bounded input magnitude. It clears
the binary32 sign, compares against binary32 1000, widens to binary64 and
compares against the stored binary64 0.001. The lower comparison rejects
unordered values. Actual pointer state and input values are not supplied by the
archives.

The installed verifier checks both complete methods: 67 primary and 79 alternate
instructions, their exact body hashes, named exports, five UTF-16 literals,
three constants and both pinned decompilations. Independent Ghidra captures
cover the same 146 locations and both contiguous body extents. The temporary
analysis session closed; no target code executed.

One captured COMISD operand has the same width typo tracked by
[Bug #169](https://github.com/Oblikovati/Bessemer/issues/169). Qualification
requires the exact address, original text and eight bytes. Tests reject each
single-byte mutation and changed register text, and leave unknown addresses
unqualified. The original Source report remains unchanged.

All 284 original report rows remain retained. Complete native replay matches
each of the 276 accepted rows, preserving stream multiplicity, selected versions,
original prefix or EOF scope, field offsets, raw values and derived strings.
The eight rejected rows still have only failed helper attempts. The replay also
checks all 425 owned contexts; twenty older opaque styles expose none of these
named fields. Missing words are never filled from initialization or observation
counts. No stored-byte coverage credit is added.

The [evidence verification record](legend-format-evidence-verification.json)
records the installed bindings and the full221 Source gate, which passed in
431.660seconds and exited successfully after snapshot recording.

## Typed local operations

`SimulationPostDialog::legend_format_inputs` and
`SimulationPostStyle::legend_format_inputs` return the four optional raw words.
`SimulationLegendFormatInputs::primary_format` requires only B8 and BC, in that
order, and returns an immutable `SimulationLegendFormatPlan` with a constructed
ASCII string. Unknown modes retain their original value while selecting the
local `e` fallback. Negative precision and signed-width overflow remain literal
string construction, without a native-rendering claim.

`alternate_format_masked_exceptions_daz_off` takes explicit
`SimulationLegendFormatRuntime` facts. Unknown pointer presence fails before
any word is required. A known null pointer skips all remaining dependencies.
A nonzero mode skips C4, the input value, C8 and BC; a zero C4 skips the latter
three. Only a reached magnitude test requires the original binary32 input.
An admitted magnitude then requires C8 before BC. These guards retain every
unconsumed supplied parent without treating it as a read dependency.

The exact magnitude classifier uses integer ordering of sign-cleared binary32
patterns. The first admitted positive value is `0x3a83126f`, whose binary64
widening exceeds the stored lower bound; its predecessor falls below it.
`0x447a0000` is exactly1000 and its successor exceeds it. Both signs, zeros,
subnormals, infinities and both NaN classes therefore have defined local
outcomes without depending on this process's floating environment. The check
mode explicitly assumes masked native FP exceptions and DAZ disabled; actual
native FP state remains unknown.

A skipped plan returns `None` from `format_string`, meaning no local string was
produced and no native assignment is modeled. It does not represent an empty
output string. Primary and alternate construction return a derived string,
the complete supplied parent words/runtime facts, the consumed-member mask,
pointer/value read coverage and zero achieved local pattern error. Plans own
their strings and expose no mutation path. Archive bytes remain unchanged.

The independent oracle combines the unchanged copied Source constructor with
binary32 decoding, binary64 widening and the pinned native constants. It covers
79,488 operations, all optional-word combinations, guard ordering, wrapping
precisions and340 signed magnitude patterns. Seven hypothetical calls for each
of425 owned and276 native complete archive contexts preserve actual field
absence and full body bytes. These calls do not establish actual invocation.

All seven Rust checks passed, including native contexts and eight-worker
comparisons. Clippy passed. Two fresh processes matched all84,395 expected
results, totaling4,388,540bytes. The [operation record](legend-format-operation-verification.json)
and [process proof](legend-format-process-verification.json) retain the exact
gates, hashes and exclusions. The full Source suite passed223 checks in
711.138seconds and exited successfully after snapshot recording.
Exact publication validation, platform gates and
remaining documented feature166 models remain outstanding. Humanizer is
unavailable in this checkout; maintainer prose review remains pending.
