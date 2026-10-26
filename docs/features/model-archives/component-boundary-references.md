# Ordinary component and boundary references

The private model codec now reads the supported ordinary component-reference
base and its face, vertex, edge, loop, silhouette-edge and atom/solid/surface-body
forms. Optional strings, compact transforms, aliases, faceted-data pointers and
versioned raw fields retain their stored values. Nonzero old-payload and direct
reference-info branches remain explicit unsupported failures, matching the
reference's unresolved branches. No layout is selected from nearby byte patterns.

The boundary-reference base, edge, face and vertex forms are also implemented,
with direct entity IDs and the base entity-ID representation. Direct IDs consume
no extra MFC class/object slots. Repeated representation pointers preserve the
shared map identity. Entity counts remain unsigned and are checked against the
remaining bytes and aggregate entry budget before iteration. Temporary chooser
and direct model-data bodies remain explicit unsupported branches. Additional
derived representation serializers and their assignability checks remain open.

| Owned suite | Cases | Independent scalar values |
|---|---:|---:|
| Component references | 188 | 5,402 |
| Boundary references and entity IDs | 48 | 412 |

Both suites require exact reconstruction and reject every shortened prefix and
extra trailing byte. Component cases cover ten version boundaries, noncanonical
presence values, self aliases, both matrix-presence forms and extended perspective
payloads. Edge cases exercise all outcomes of the stored composite-reference
flag condition. Another 388 mutations check unsupported base branches and
negative edge counts. A boundary regression rejects `u32::MAX` entity counts as
unsigned values before iteration. Two fresh reference processes reproduced both
wire packets exactly. The schema snapshot binds edition 0.326 clauses 11.17 and
11.18; each fixture manifest separately binds its source and generator hashes.

The native corpus still reconstructs 632 of 906 configurations exactly. The
component readers first advance 109 former component-class stops to face
references. Boundary readers then advance those records to their entity-ID
representation pointers. The current first-stop distribution is 160 derived
representation role failures, 44 mate-reference folders, 36 relations, 33 length
parameters and one derived component-reference role failure. This is deeper
typed traversal, not an increase in accepted configurations. No opaque tails,
record skipping or broad role relaxation were added.

Bug #46 now has concrete derived-representation cases as well as the original
component-plane case. Its fix needs reviewed inheritance evidence, complete
nested bodies and positive/negative alias tests. Bug #47 still tracks selected
configuration error attribution. Neither issue is closed by these serializers.
The corpus gate continues to fail its 906-of-906 requirement. Full feature
contracts, native application acceptance and the platform matrix remain open
under #44. No new branch, PR, version or release is included.

Verification commands and source hashes are recorded in
`reference-codec-verification.json`. The humanizer skill is unavailable; the
prose pass remains for maintainer review.
