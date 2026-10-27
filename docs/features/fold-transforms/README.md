# Fold folders and transform records

Bessemer reads, edits and writes complete ordinary modern fold/unfold sketch
folders, sketch-transform records, feature-transform records and sketch-transform
property records above version `0x27CB` in non-DB external-XML archives.
Folders reuse the complete feature-folder base before their two tagged transform
pointers. Direct lists and transforms add no enclosing object-map slots.

Sketch and feature records retain their distinct list order. Signed transform
counts are bounded by the 34-byte minimum restore body and aggregate entry limits.
The existing typed transform codec preserves the original matrix-presence byte,
matrix values, translation, scale, flags and flag-selected extensions. Unknown
flag selectors retain the documented load framing without inferred payloads.
No transform is executed or normalized.

Sketch property associations remain ordered pairs, including duplicate feature
keys, repeated property identities and null targets. Declared roles reject
unrelated cached objects. Property records retain three signed words, one unsigned
word and two independent CStrings. Their engineering meanings remain unresolved.
The direct byte array uses ReadCount and preserves every byte, including zero and
values above 127; it does not use a signed fixed count or stride descriptor.

Independent readers verified 105 complete cases with 11,280 original primitive
observations. Three versions cover the ordinary lower boundary, current fixtures
and the full u32 version domain. Five flag values cover absent/base/extended/matrix5
extensions and an unknown selector, with both matrix-presence forms. The matrix
also includes empty bodies and complete signed/unsigned property extrema.

The reference reads a sketch byte array as one raw span. Observation retains the
original source reads and splits only those spans identified by their actual
fold serializer callsite and checked against independently decoded byte arrays.
This produces 11,460 nonempty typed observations. Source file/function/line
provenance remains on every byte. Class-name reads and empty raw reads are not
projected into array fields. Projection tests check exact bytes and reject
mismatched expected spans.

Four owned streams import the blank part's actual 97 slots and finish with
105, 109, 109 and 109 slots. Independent edits change a shared property's second
CString and the first sketch transform's x translation. Public tests compare both
edits against complete expected streams and verify source isolation after success
and rejected transform-shape changes. Private tests compare every field/byte/slot,
reject every truncated prefix, check malformed counts and pointer roles, and
require exactly four entries for a four-byte direct array. Python tests regenerate
all fixtures and verify the branch matrix and observer boundaries.

`reference.json` binds stable before/after sibling reader hashes, generators,
reused builders, primitive inventories and owned fixtures. The schema snapshot
retains the folder, transform and property clauses.

Two fresh native processes retain all 638 prior complete configurations and add
twelve, reaching 650/906. Their outcomes and causes agree exactly; 256 remaining
configurations still stop explicitly. An independent reader audit reaches exact
EOF for all 650 accepted streams, totaling 37,320,942 feature bytes. Every case
matches all four original input hashes. The audit used a verified immutable copy
of 1,114 approved sibling Python files and checked its hashes before and after
decoding; the temporary copy was removed after its process exited.

The staged Windows x64 `cargo xtask pr` gate passed with Rust 1.99.0: 521 library
tests, four xtask tests, every integration suite and 265 doctests. Three external
corpus gates remain ignored by default; both native replays above ran explicitly.

Native application acceptance,
geometric validity, DB/deferred archives, historical formats and full documented
schema parity remain unproven. No dependency or package version changed. The
humanizer skill is unavailable; its prose pass remains with the maintainer.
