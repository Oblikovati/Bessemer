# Flow legacy link records

Issue: #241. Reference: specification section 14.46 and
`../inventor-interface/solidworks/flowlinkrecords.py`.

Record 402 versions 0–9 contain ordered entries with two signed reference
indices, seven raw binary64 patterns, a code byte and a byte-counted integer
array. An explicit pointer-count context accepts `-1` as a sentinel, including
when the count is zero. Other negative values remain available only without
range checking. Version zero converts six-byte bands beginning at 30, 40, 50
and 60 in pairs; the original code byte remains stored.

The tail contains eleven signed members in documented physical order. Versions
two and three retain one and three discarded words. Version five adds a control
word; both that word and member `0x290` must be positive to consume conditional
integers. Version six adds raw numeric words. Their parallel indices are stored
from version eight; earlier versions expose a runtime default of `-1` without
pretending that an index array was serialized. Version seven adds a final array,
whose elements widen from signed 32-bit patterns to raw 64-bit patterns in
version nine.

The independent baseline has 954 views, including the native CPT payload with
and without pointer-range context, all legacy versions, all 256 code bytes,
raw/zlib transport, empty tails, sentinel-only references, signed extrema,
maximum byte-counted arrays, nonfinite numeric patterns and each control branch.
Rust reconstructs every decoded byte and checks all recorded intervals. Python
checks regeneration, fixed hex bytes, conversion and context gates. Native
totals are 5,532 entries, 47,602 variable integers and 4,056 numeric-tail words.

`FlowCarrier::record_402_links` reads a selected revision-bound occurrence with
explicit raw/zlib transport, an optional validated pointer-count context and
independent decoded-byte, entry and total dynamic-array limits. Entry and tail
views borrow exact spans without allocating per numeric value. Coverage reports
the complete serialized extent, entry/array totals and supplied range separately.
Diagnostics retain the native and semantic exclusions.

`replace_field` edits an existing byte, signed scalar or raw wide pattern on an
isolated carrier copy. The complete payload is reparsed under the original
options, and all structural spans must remain identical before publication.
Thus member or control edits cannot silently change conditional array shape.
Source/result checkpoints and every physical history relation accompany edits.
Exact no-ops preserve stored bytes and revision identity. Fresh raw reversals
restore whole carrier bytes; fresh compressed reversals restore decoded bytes
without assuming a canonical zlib representation.

Native runtime state, object-reference resolution, engineering roles and
application acceptance remain unproved.
