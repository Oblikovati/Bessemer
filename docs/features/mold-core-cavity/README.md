# Mold core/cavity feature archives

Issue #215 fills the missing dispatch for `moMoldCoreCavitySolids_c` and
`moMoldCoreCavitySolidsData_c` in ordinary modern part archives. The feature
contains the existing body-feature base and one data pointer. The data body
retains its owner, three direct collections, profile pointer, two signed words
and two distance dimensions followed by an angular dimension.

The independent graphs at versions `0x27CC` and `19000` exercise empty and populated collections, null
items, duplicate aliases, a feature-owner back-reference, a complete profile,
shared distance dimensions and an angular dimension. The two raw words use
distinct values to pin their non-offset wire order. A separate literal edit
changes SetbackSrf without normalizing it to Boolean or introducing backups.

Static native bindings and retained measured legacy streams establish the
field order. The new fixtures use the already supported modern inherited
codecs; they do not admit the measured legacy contexts. Runtime backup doubles,
unchecked dimension-provider calls, post-load processing and mold geometry are
not serialized here and are not emulated.

Nine private tests and four document tests cover exact graphs, whole-carrier
reconstruction, independent edits and fresh reversals, revision-bound history,
eight-worker isolation, source mutation, resource limits and typed-pointer
rejection. The full local gate passed at `0acde5cb`: 4,189 tests passed and 231 were
ignored. Exact-head remote checks remain required before integration. It is excluded from PR212 and the
completed material gate at7f6fe735. No new native populated-file, geometry or
SolidWorks application acceptance is claimed. The humanizer skill is
unavailable; its prose pass remains with the maintainer.
