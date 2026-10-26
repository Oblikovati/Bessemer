# Folders, parameters, sheet-metal bridges and relation graphs

The private ordinary-part codec now reads the documented feature-group folders,
position groups, older per-body sheet-metal bridge and component-feature
references. Notes-area folders retain their extra signed word selected by the
documented final vtable; ordinary groups omit it. Position groups retain both
direction arrays, packed reference counts and late raw bytes. Direction counts
above three fail explicitly before iteration. The older sheet-metal bridge
preserves four typed pointers and two raw words, including aliases and component
body-feature variants.

Length and angle parameters reuse the existing density-parameter serializer,
as the reference specifies. Stored class names, node/name identities, negative
zero, NaN payloads and noncanonical alternate-value presence bytes remain exact.
No numeric evaluation or unit conversion is introduced.

Relation, equation-side and selected element records now compose with shared
reference wrappers. Cyclic side-to-relation pointers, repeated sides/elements,
Unicode strings, signed fields and transform payloads retain their stored form.
Dimension/global-variable elements preserve their pointer prefixes before the
common element body. The shared wrapper handles its u16 presence words and
versioned tail once for ordinary, dimension, global-variable and component
transform wrappers. Its component family is an explicit class list from the
scoped reference, rather than an inferred name prefix. A negative fixture
rejects an unrelated runtime class. Expression evaluation, solver behavior and
geometry validity are not claimed.

The exact `moRadiateSurfIdRep_c` alias from clause 11.115 reuses the existing
two-word entity representation. Its selected model family now has 41 known
variants. No other radiate or similarly spelled class is admitted by inference.
The original 40-class fixture packet remains unchanged; the scoped alias has
its own new packet and retained native-identity evidence hash.

| New owned suite | Cases | Independent scalar values |
|---|---:|---:|
| Feature folders and floating parameters | 63 | 1,938 |
| Position groups and sheet-metal bridges | 40 | 1,872 |
| Relations, equation sides and elements | 40 | 304 |
| Shared model reference wrappers | 64 | 3,576 |
| Scoped radiate representation | 12 | 36 |

All cases require exact reconstruction and reject every shortened prefix and
extra trailing byte. Direction cardinality and unrelated wrapper-class mutations
have focused negative tests. Source-overwrite tests cover folders, parameters,
relations and wrappers. Two fresh reference processes reproduced all five wire
packets exactly. Existing frozen fixture bytes and scalar baselines were not
changed.

Native exact reconstruction increased from 768 to 834 of 906 configurations.
Intermediate runs reached 787 after folders/parameters, 804 after position and
sheet-metal bodies, and 827 after shared wrappers. Relation bodies first moved
their 48 cases to wrapper dependencies; those partial traversals earned no
acceptance credit. The scoped radiate alias completed another seven cases.
The complete result rows and input packet hashes are retained in
`native-corpus-checkpoint.json`.

The remaining 72 first failures are 21 general-curve references, 15 feature
dimension handles, nine single-face references, seven circle references, seven
cylinder references, five G2 handles, four radial handles, one offset handle,
one part exploder and two derived pointer-role failures. Further dependencies
may appear after these bodies are implemented. The required 906-of-906 gate
still fails. #46 remains open for derived component/dimension roles, and #47
for selected-configuration error attribution. Corpus-wide independent scalar,
edit/history/document and native-application acceptance, the public model API
and the platform matrix remain open under #44 and #2.

`groups-relations-verification.json` records final sources and executed checks;
the schema snapshot and per-suite manifests bind the reference and generator
versions. All 145 model tests and all-target Clippy passed. All 44 Python tests
passed, including eight #48 regressions for schema snapshot output. That fix
removed one Windows text-translation layer from three earlier snapshots only
after every original section hash matched. Source/section digests and wire
fixtures stayed unchanged; `schema-snapshot-newlines.json` retains before/after
artifact hashes. Future snapshot output uses the hash-checked byte writer in
`schema_snapshot_bytes.py`. Both #48 and the earlier #45 remain open until
integration with #44. No extra branch, PR, version or release is included. The humanizer
skill is unavailable; the prose pass remains for maintainer review.
