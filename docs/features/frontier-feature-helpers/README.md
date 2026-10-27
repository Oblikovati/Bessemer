# Lock/environment features and pattern/fillet helpers

Issue #54 completes the five remaining class bindings from clause 11.350 / C-392
for ordinary external-XML parts above version `0x27CB`:

- `moLockFeature_c` uses the existing complete feature body without a suffix.
- `moSMEnvTableFeat_c` uses the existing model-feature body, including +354.
- `moLPatternItemId_c` preserves two signed words, one byte and one double.
- `edgeParamsObject_c` reuses the existing edge-radius layer, then reads two
  display-dimension pointers.
- `featRadiusObject_c` reads a display dimension and an unqualified object pointer.

Native evidence binds both feature serializers to five-byte direct jumps. Native
padding adds no wire bytes. The historical intermediate model-feature helper
branch lies outside this modern context. Pattern items have exactly 17 payload
bytes here; byte flags retain their original width and doubles retain all IEEE
bits. Null edge/radius helpers have 12 and four payload bytes respectively.
The radius object pointer accepts arbitrary supported objects and cached arrays;
its dimension pointer remains typed.

The independent fixture set contains 390 graphs over thirteen versions, 77,028
primitive observations and 34 owned streams. It covers component-edge and display
dimension version boundaries, null and repeated pointers, distinct equal objects,
shared dimensions, arbitrary node-name and component-edge targets, and a cached
reference to the containing array. Pattern values cover signed extremes, all six
selected byte flags, signed zero, both infinities and two distinct NaN payloads.
Four independently built word edits cover the lock feature, environment feature,
pattern item and edge helper.

Rust checks complete primitive partitions, exact map slots, all truncated
prefixes, appended bytes, old context, unbound class spellings and pointer roles.
Public document tests check unchanged storage, ownership after source mutation,
successful edits and rejected type edits. The existing saved-model/CMgr context
and exact-EOF publication requirements remain in force. Complete Rib preludes
install the source reader's display dimensions through its ordinary path; the
public source reader is unmodified.

Run `python -B -X utf8 -m unittest discover -s docs/features/frontier-feature-helpers`
to regenerate fixtures in temporary directories. Run
`cargo test --lib model_archives::frontier_feature_helper_tests` and
`cargo test --test resolved_frontier_feature_helpers` for focused Rust checks.
`cargo xtask pr` runs the repository gate.

`schema-reference.json`, `native-binding-reference.json` and `reference.json`
bind the supplied specification, installed DLLs and independent fixture reads.
Native corpus reconstruction is reported separately from these synthetic graphs
and from geometry or SolidWorks application acceptance. The two edge identifiers
from the same clause are documented in `../frontier-edge-identifiers/README.md`.

The native corpus advances from 751 to 757 complete reconstructions, with 149
explicit stops. Cases 291, 534, 537, 543, 683 and 843 complete; two other cases
advance to later stops. All prior successes remain accepted. An independent
pinned source audit verifies exact EOF and all four stream hashes for every
accepted input, covering 47,982,941 feature-stream bytes. The six newly completed
sources also match the clause's documented sizes, root counts and unknown totals;
both electronic-housing sources retain their separate identities. See
`frontier-checkpoint.json`, `accepted-reference.json` and
`native-sentinel-reference.json` for those records.

The humanizer skill is unavailable in this session; prose review remains a
maintainer task.
