# Complete mixed-kind medium records

Issue: #241. Reference: `../inventor-interface/solidworks/flowmediumliquids.py`.

The FML1 packet freezes 3,753 complete record-424 profiles. It includes the
installed kind-one record and every earlier complete kind-one case, all active
outer versions 2 through 127, all six kind-two model layouts, both first-table
orders and duplicate physical entries. Additional cases cover every stored
state byte, the metadata string corpus, populated nested arrays, kind-two-only
bodies, first tables without references and empty reference lists of either kind.

Each first-table entry stores a kind, a sixteen-byte table key and its complete
body. The packet embeds independently decoded FMO1 or FMO2 fields rather than
only recording opaque body bytes. Rust and Python rebuild all nested fields,
check every entry boundary and compare the complete record byte for byte.

Reference bodies select the first-table map by their declared kind. Equal keys
may occur in both maps, but a key present only in the other kind cannot resolve
a reference. Kind-one reference bodies store a state byte; kind-two bodies do
not. Outer references resolve against the combined second table. Duplicate
physical entries and repeated ordered references remain present; these checks
do not assign a native duplicate-key lookup identity.

Python checks all proper prefixes of compact examples for each model in both
table orders, appended bytes, cross-kind and missing outer references, invalid
count/tag/version framing, populated final collections and explicit fresh-state,
active-outer and string-mode requirements. The packet is regenerated in a
temporary directory to verify reproducibility.

This baseline adds no production mixed-record API. The existing complete
kind-one profile and both individual body APIs remain separately supported.
Installed kind-two acceptance, engineering/model evaluation, native runtime
behavior and application acceptance remain unverified. The complete profile
requires an explicit fresh backend and zero final collection counts, matching
the documented reader's accepted scope.
