# Stock and mirrored-stock records

Bessemer composes the full ordinary stock body from its body-feature base,
derived-feature collections and stock-specific members. The stored u32 mode
selects a kernel body only for values 1 and 2. Kernel suppression is a separate
byte within that selected body. Other mode values retain their complete word
and omit the body without a placeholder.

The configuration is a direct name/id pair. Arrays at `+A10`, `+A88` and `+A30`,
and the list at `+CF8`, are direct collections without container tags or map
slots. The `+C30` transform uses the existing fixed 30-word codec; component-body
transforms use the compact codec. Mirrored stock appends a tagged plane pointer.

The final dynamic array retains its element width, metadata and unsigned words.
A zero width ends the array immediately. A nonzero width must be divisible by
four; a huge width with zero elements remains valid. Derived pair counts are
unsigned, while stock pair and hole-record counts are signed. Hole records move
from the stock tail to the inherited layer at `0x373C`. Duplicate values and
ordered occurrences are preserved before native runtime set normalization.

Body references retain their wrapper, signed control, independent kernel bodies,
direct attribute lists and final external pointer. Component-body references
retain aliases, both compact transforms and all ten stored box doubles. These
readers require the existing ordinary modern context; special smart-copy modes,
unsupported nested classes and unimplemented kernel forms remain explicit
failures.

The independent fixtures cover 216 graphs across four classes, nine versions
and six value styles, with 51,378 primitive observations. Twenty-eight owned
document fixtures include four independent edit goldens. Three immutable-reference
fixture tests, ten focused Rust tests and four public save/reopen, edit and
source-isolation tests passed. The actual native stock/component reference test
also passed, checking both stock classes, retained endpoints, box fields,
configuration values and fixed-transform words.

The shared wrapper family now contains 21 reviewed classes. An additional 270
graphs and 30 owned documents exercise body references at extrusion/end,
starting-end, revolution-end, four sketch-intersection fields and sketch external
references. Their 51,809 primitive observations supplement the stock fixtures;
the earlier wrapper corpora remain unchanged. Exact inventory tests decode the
new target through each affected caller and compare the full family. Combined
coverage is 486 graphs, 103,187 observations and 58 owned document fixtures.

Two fresh native processes agreed on all 906 statuses and failure causes.
Complete resolved configurations increased from 842 to 848, preserving every
previous success. Fifty-eight inputs remain explicitly rejected, so both
aggregate processes exited 101. Exact stream reconstruction does not establish
full document semantics or geometric validity.

Independent readers reached EOF for all 848 accepted configurations, totaling
64,364,207 resolved-stream bytes, and matched all four input-stream hashes.
`accepted-reference.json` identifies the source snapshot and accepted indices.

The JSON reports retain the schema, fixture, reader and native binding hashes.
The humanizer skill was unavailable; its prose review is left to the maintainer.

Local `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: formatting, Clippy,
build, 996 library tests, all integration tests and 274 doctests. Three external
fixture tests remain intentionally ignored by the default gate; native corpus
verification ran separately as described above.
