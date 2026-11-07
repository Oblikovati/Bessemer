### 11.221 Thickening features and surface-reference layers

*Status: B / ? — three thickening features, four constant-surface references and one component surface reference verified inside complete native roots; raw meanings and full-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature archive, version >0x27CB. sldMoPartu.dll moThicken_c primary 0x1804C0B60 / COL 0x1804F27C0 selects 0x18035B030. RTTI establishes moBodyFeature_c ancestry, and the native serializer calls that imported base. Hooks +4B8/+4C0 resolve to the ordinary moConcept_c lightweight/resolved pair. After the complete body-feature prefix:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +810 | f64 | always | Preserve stored scalar. | B / ? |
| 2 | raw +818,+81C | two i32 | always | Signed stored words in order. | B / ? |
| 3 | list +808 | tagged suObList | always | Object pointer with normal class/object map allocation, not an embedded direct collection. | B |
| 4 | raw +820 | i32 | always in scope | Native load gate >0x3D9 predates scope. | B / ? |
| 5 | dimension +800 | tagged moDisplayDim_c | always in scope | Native load gate >=0x3E2 predates scope. | B |
| 6 | raw +824,+828 | two i32 | always in scope | Native gates >0x6A0 and >0x708 predate scope. | B / ? |
| 7 | direction +830 | tagged moDirectionSpec_c | version >=0x2F74 | No occurrence below the inclusive boundary. | B |

The complete empty feature body with null/cached targets is 283 bytes below 0x2F74 and 285 thereafter. New tagged targets contribute their own bodies and map slots. No operation/direction enum is assigned from field offsets or observed values.

The nested constant-surface reference is independently bound in sldmodu.dll: moConstSurfRef_w primary 0x4CED4C68 / COL 0x4D31FFE8 selects 0x4B90DBB0 and has moRefWrapper_c ancestry. It calls shared helper 0x4B90DB10, restoring the wrapper followed by u32 raw +A8. Its derived body reads i32 body presence; zero ends it, nonzero selects the existing strict kernel body and then a direct suObList of attributes. The attribute-list gate >=0x46E predates scope, so the modern wire grammar matches 11.220's solid reference. The implementation reuses that grammar only in the copied scoped reader map. Runtime partition changes differ from solid-reference behavior but add no archive fields. Unsupported kernel contexts retain explicit stops; an absent-body latest reference is 70 bytes.

moCompRefSurface_c primary 0x4CD8F7D8 / COL 0x4D2ECAF0 selects 0x4BBBA630. Its moCompRef_c ancestry and call to 0x4BBBA440 establish the component-feature body plus tagged reference geometry described in 11.219. It appends **i32 raw +100**, without a version gate in scope; this is not the coordinate-system class's fixed transform at the same runtime offset. Its latest empty body is 96 bytes. Native RTTI additionally establishes moCompSurfaceBody_c's moCompRef_c ancestry, admitting that already decoded component class and moCompRefSurface_c to the wrapper's accepted contexts.

Five focused tests verify exact scalar/word/pointer ordering, signed extremes, cached list/dimension/direction/geometry identities, inclusive direction boundary, tagged empty minima and self-map accounting, following-byte alignment, truncation/context rejection, wrapper component identity and native source-hash-bound complete containing-root spans and fields for all three classes. Present constant-surface bodies use the strict shared kernel grammar; populated native bodies in these references remain unobserved.

The three native thickening features each have one list item and a moDisplayDistanceDim_c, with null direction pointers. Raw +810 is 0.002032 once and 0.02 twice; other words are retained without inferred enums. Four constant-surface references have body presence zero and raw +A8 values 0,238,0,145. The one component surface reference has raw +100 zero, feature ID 19, feature time 1223400751 and null reference geometry.

Across 906 complete modern contexts the frontier has **24,377 complete roots**, **771 complete lists** and **135 explicit stops**: five additional roots and two additional complete lists relative to Edition 0.226. **698 lists reach exact EOF**, spanning **46,532,839 decoded bytes**, an increase of 25,245 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. `resolved-feature-list-frontier-0.226.json` preserves the preceding checkpoint. These feature traversal counts add no whole-document coverage credit. Complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-249.
