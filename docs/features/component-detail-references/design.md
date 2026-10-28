# Component-detail references

Feature #54 continuation after the display-weld reader. Native corpus cases
547, 615 and 905 now stop at `component_reference` containing
`moCompDetailItem_c`. No implementation or completion credit is claimed here.

Clauses 11.345 / C-387 and 11.348 / C-390 bind the component-detail class to the
component-reference prefix followed by two tagged detail pointers (+E8/+F8)
and raw signed word +100. The latter pair's historical gate holds throughout
the supported ordinary modern context. The native external/internal predicate
controls the body; class identity alone does not select its internal branch.

The approved `weldinternal.verified_current_unit` requires a verified mapped
`moPart_c` root, an exact `moUnitComponent_c`, stored external-model presence
zero and owner identity equal to that root. A null component follows the native
internal branch directly. Nonnull components without this proof must fail
explicitly; they must not fall back to the null-component layout.

Bessemer already preserves the complete part's shared map and passes its decoded
fields into resolved-feature parsing. `lightweight::root_tag` verifies a new part
root, while `part_objects::unit_component` retains its owner pointer and rejects
external models. The next implementation should carry the verified root's typed
map identity from the completed part archive into the operation-local decoder.
It should reuse `object_values` for decoded stored facts, including inherited
and newly created objects. A class name, guessed slot or owner-shaped field is
insufficient evidence of the selected current part.

Complete the component-detail reader and its typed display-object role together.
Its dependencies need complete detail-item bodies and exact finite subtype roles;
no admitted class should dispatch to a stub. Test null, fresh and cached units,
actual current-part ownership, a different mapped owner, external-model presence,
missing root provenance, shared detail pointers and isolated signed-word edits.
Use complete owned archives for provenance tests and independent primitive/EOF
oracles for the wire. The reference MFC reader does not enforce its `expect`
argument, so it cannot prove pointer-role rejection; retain native bindings and
focused Rust negatives for that boundary.

This note records the next required implementation, not a narrower supported
replacement. Historical/internal contexts, external component layouts, runtime
repair and full schema parity remain incomplete. Humanizer is unavailable;
maintainer prose review remains open.
