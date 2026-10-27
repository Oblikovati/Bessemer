### 11.220 Combine/delete body features and solid references

*Status: B / ? — four combine features, three delete features/payloads and 16 solid references verified inside complete native roots; raw option meanings, populated solid-reference kernel bodies and whole-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature context, version >0x27CB. sldMoPartu.dll primary RTTI binds moCombineBodies_c (0x180466A48 / COL 0x1804ED890) to 0x18024EEE0 and moDeleteBody_c (0x18046B9B0 / COL 0x1804EDC28) to 0x18025B620. Both call the imported moBodyFeature_c serializer. Their +4B8/+4C0 hooks resolve to the ordinary moConcept_c lightweight/resolved-data pair. After the body-feature prefix, the combine suffix is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | body +800 | tagged moCompSolidBody_c | always | Target body component. | B |
| 2 | array +808 | direct suObArray | always | ReadCount and tagged objects; preserve order and duplicates. | B |
| 3 | raw +828 | i32 | always | Preserve signed stored option. | B / ? |
| 4 | raw +838 | u8 | version >0x3F07 | Preserve the exact stored byte before native nonzero-to-Boolean normalization. | B / ? |

Factory 0x18024EBD0 calls constructor 0x18024ED10, which constructs direct su_CObArray at +808 and installs suObArray's table. The runtime descriptor allocation 0x840 is not a wire size. With null/cached references and an empty array, the complete combine body is 259 bytes through version 0x3F07 and 260 thereafter.

The complete delete-body suffix is one tagged moDeleteBodyData_c pointer at +800, making its empty body 253 bytes. The payload primary 0x18046B920 / COL 0x1804EDBA0 selects 0x18025B540 and has su_CObject ancestry. Its ordinary zero-byte base is followed by a direct suObArray at +08 and i32 raw +28. The native +28 load gate (>0x1BC3) predates scope. Payload creator 0x18025B070 allocates 0x30 runtime bytes and constructs the direct array; the empty wire body is **six bytes**, not the allocation size. No operation enum or array-item restriction is inferred from observed values.

The nested moSolidRef_w is independently bound in sldmodu.dll: primary 0x4CED5050 / COL 0x4D320080 selects 0x4B90DED0, with moRefWrapper_c ancestry. Helper 0x4B90DB10 first calls wrapper serializer 0x4C2842D0, then restores u32 raw +A8 (native gate >0x1BBD, before scope). The derived body reads an **i32 body presence**. Zero ends the object. Nonzero selects one strict installed-kernel body followed immediately by a direct suObList of attributes. There is no body count or extra terminal pointer. Native partition/ownership/attribute fixups and condition 0x2000's store-time suppression do not add another field. The complete absent-body reference is 70 bytes at version 19000, or 58/62/66 bytes at wrapper checkpoints 0x27CC/0x2B73/0x2FB0. The present branch uses the existing strict kernel-body grammar and stops at unsupported kernel contexts; no opaque byte skip is used.

Native RTTI also establishes moCompSolidBody_c's moCompRef_c ancestry, admitting this already decoded component class to the wrapper's accepted component-reference contexts. All four new class bindings are confined to the copied modern feature reader map.

Six focused tests cover combine byte values 0/1/255, signed-word limits, direct-array ordering/duplicates, version boundaries and tagged minima, new payload-object map insertion and cached array identities, wrapper component identity, solid-reference raw word/full-width presence and conditional attributes, truncation/context rejection, and source-hash-bound native containing-root spans/fields for all four classes. Synthetic present-solid references exercise common suppressed-kernel framing; populated native non-suppressed bodies remain unobserved in these references.

The audit retains **four combine features**, with array counts 1/1/12/1, raw +828 equal to zero in three and 2 in one, and raw +838 zero throughout. **Three delete features/payloads** contain arrays of 2/2/1 moCompSurfaceBody_c targets and raw +28 zero. **Sixteen solid references** have body presence zero; raw +A8 values are preserved without assigning an identifier meaning. Across 906 complete modern contexts there are **24,372 complete roots**, **769 complete lists** and **137 explicit stops**: 66 additional roots and three additional lists relative to Edition 0.225. **696 lists reach exact EOF**, spanning **46,507,594 decoded bytes**, an increase of 316,244 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. `resolved-feature-list-frontier-0.225.json` preserves the prior checkpoint. Whole-document coverage credit is unchanged; complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-248.
