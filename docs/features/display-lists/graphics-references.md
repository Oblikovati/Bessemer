# Graphics-reference archive records

Issue #28 adds the manager, wrapper and magnetic-reference MFC classes to the
existing global object map. Presence flags retain their raw signed values;
point counts are nonnegative and consume the shared entry budget. Nulls, aliases
and cycles retain their identity. Magnetic references carry the final double only
when the display version exceeds `0x2BC3`.

The wrapper's direct `mgPlane_c` body contains six doubles: root point followed
by normal, exactly 48 wire bytes. The approved sibling specification and retained
native restore evidence establish this layout. Its 56-byte native allocation
includes the vtable. This body differs from the section-plane serializer.

The sibling display-list module lacks a registered plane hook. The fixture
reference parser temporarily supplies that six-double reader, then restores the
hook; it does not modify sibling files. `graphics-references.json` records the
reference source hashes, native evidence hash, module hash and restore address.

Five independently encoded complete roots cover version 19000 part and assembly
callers, an empty manager, an independently edited plane, and a version 11000
manager reached through the generic scene object list. They provide 48 independent
semantic field comparisons, including Unicode, embedded NULs, signed zero,
infinities and both NaN encodings. These graphs establish wire identity only;
they do not claim application-valid CAD relationships.

Eight integration tests check forced reconstruction, the independent field
oracle, raw flags, version omission, cycles, aliases, nulls, exact part budgets,
explicit rejection of unresolved faceted bodies, all truncated prefixes, invalid counts, changed presence flags, trailing bytes,
container edit/reopen/revert, preservation of other streams, source isolation and
eight concurrent edits against independently encoded bytes.

New `moFacetedData_c` bodies remain unsupported because their complete reader is
unresolved. This checkpoint does not complete clause 13 or expand the existing
1,086-stream native corpus count. Full schema and native application acceptance
remain separate gates in the feature README.

The maintainer-side humanizer skill is unavailable; its prose pass remains for review.
