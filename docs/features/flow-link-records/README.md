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

This is the fixture baseline for the forthcoming typed reader/editor. Native
runtime state, object-reference resolution, engineering roles and application
acceptance remain unproved.
