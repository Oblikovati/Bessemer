# Reference-curve body features

Bessemer reads, edits and writes ordinary non-DB external-XML `moRefCurve_c`
records newer than `0x27CB`, following 11.338 / C-380 and the guarded integration
in 11.339 / C-381. This body-feature class is distinct from the reference-geometry
prefix used by helices. Native primary dispatch, complete code ranges and
constructor-bound collection types establish its wire order.

The reader preserves a tagged reference array, strict kernel body, direct
attributes, raw words and DWORD arrays. Flag bit 1 selects the extra DWORD array.
Otherwise a nonempty reference array selects a direct object array; null or
empty references select a plane only after `0x2B94`. The reader uses existing
decoded-object facts for fresh and cached arrays and looks up their counts only
when the flag branch needs them. Missing required facts remain errors.

After `0x2B87`, a fixed-width u16 count introduces kernel bodies, each followed
by its own direct attribute list, including suppressed bodies. Further raw words,
plane and direction occurrences follow. A curve reference appears only after
`0x3EFB`. Constructed runtime array +800 adds no derived wire field. DWORD arrays
use packed counts; the body count and raw +858 stay plain u16 values. No flag,
node-ID, DWORD or scalar domain is inferred from native examples.

The independent reader consumed 60 complete graphs at ten versions with 20,774
primitive observations. Fixtures include version 11000 and both sides of all
three later gates, null/fresh-empty/fresh-nonempty/cached-empty/cached-nonempty
arrays, flag priority, complete shared references, strict serialized and
suppressed bodies, positive extra-body counts, attributes, signed extremes,
DWORD high bits, distinct equal features and repeated aliases. Seven containing
streams use the actual 97-slot document map and an independent one-word edit
golden. Bare cursors select the exact public native reader in copied maps;
containing streams use the unmodified supported reference path.

Seven private Rust tests compare every field, byte and map slot; reject every
prefix, trailing byte and old context; check branches and exact suffix sizes;
reject wrong pointer roles and impossible counts; and require explicit schemas
for serialized bodies. The focused run also preserves four existing
component-reference-curve tests. Three public tests reconstruct the containing
documents and check successful/rejected edit isolation. Two Python tests
regenerate every fixture and parsed identity check.

Two fresh native processes agree on every outcome and cause across 906
configurations with a stable executable hash. All 749 previous complete cases
remain complete and one more completes, reaching 750. Four other files advance
to later explicit dependencies. The broad corpus assertion still fails for
the remaining 156 configurations. An immutable, verified copy of 1,253 reference
Python modules independently consumed all accepted feature streams through
exact EOF, covering 46,973,703 bytes, with input hashes and unknown-byte totals.
The newly completed hemisphere sample also independently matches C-380's
49,122 bytes, 39 roots, eight reference curves and 16,421 unknown bytes. These
checks establish archive coverage without adding geometry/application credit.

Positive serialized extra bodies are synthetic structural fixtures; the retained
native examples have zero extra bodies. Runtime repairs, topology and geometry
validity, SolidWorks application acceptance, historical/DB/special contexts and
full documented-schema parity remain unproven. No dependency, exported Rust API
or package version changed. The humanizer skill is unavailable; its prose pass
remains with the maintainer.
