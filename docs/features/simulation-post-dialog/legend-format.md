# Legend number-format evidence

Feature [#166](https://github.com/Oblikovati/Bessemer/issues/166) includes the
four raw legend words and bounded format construction documented in specification
14.466, C-846. This evidence supplies the native and corpus prerequisites for the
typed operation. The Rust operation remains to be implemented and verified.

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

The [verification record](legend-format-evidence-verification.json) records the
executed gates and remaining work. Humanizer is unavailable in this checkout;
maintainer prose review remains pending.
