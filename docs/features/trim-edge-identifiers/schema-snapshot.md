### 11.224 Left/right trim-edge entity representations

*Status: B / ? — 23 left-trim and 18 right-trim representations verified inside complete native roots; final-word meanings and full-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature context, version >0x27CB. sldmodu.dll moLeftTrimEdgeIdRep_c primary 0x4CF07810 / COL 0x4D32A270 and moRightTrimEdgeIdRep_c primary 0x4CF07AB0 / COL 0x4D32A320 both select serializer **0x4BBBBBD0**. RTTI establishes shared moRef1EdgeIdRep_c and moEntityIdRep_c ancestry. The serializer calls the existing common entity-representation helper 0x4BBBBF50, dispatches the direct member at +38 and restores a signed word at +48:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited representation | direct moEntityIdRep_c body | always | Feature pointer and tagged moFR_c external-feature reference, with normal object identity. | B |
| 2 | source entity +38 | direct moEntityId_c | always | Contains a tagged moEntityIdRep_c pointer; the entity-ID wrapper itself has no object tag or map slot. | B |
| 3 | raw +48 | i32 | always | Preserve the exact signed word without assigning an option/index meaning. | B / ? |

No version gate occurs in the derived serializer. A body with null/cached pointer occurrences is **10 bytes**, before outer tagged framing. New feature/FR/source-representation targets contribute normal object/class framing and slots. A direct source entity contributes only its tagged representation occurrence, not a separate entity-ID object identity.

The left-trim runtime descriptor at 0x4DAD6AF0 binds creator 0x4BBA7FE0 and runtime allocation 0x50. The creator calls 0x4BA954F0, whose constructor chain installs the reference-edge base and constructs moEntityId_c at +38. The creator then installs the left-trim primary table through moRef1EdgeIdRep_c. This proves the direct-member type independently of the field's size or name. The separately verified moEntityId_c primary serializer 0x4BBBC390 supplies its existing tagged-representation grammar. Runtime allocation sizes, constructor defaults and native code-range lengths are not serialized lengths.

Both names are bound to the existing derived-surface grammar only in the copied modern feature-context reader map. Shared native serializer dispatch and ancestry justify that reuse; the class spelling alone does not. Native evidence retains both primary COL/type/ancestry bindings, the serializer/common helper/direct entity-ID body, factory/constructor, descriptor and complete module hash. Pdata can describe fragments, so retained decompilation and full module hashing bind out-of-line code.

Four focused tests cover cached feature/FR/source identities, both signed-word limits, new source-representation class/object map accounting without a direct entity-ID slot, tagged minima/self identity for both classes at 0x27CC and version 19000, following-byte alignment, truncation/context rejection and source-hash-bound native complete containing-root spans and raw/target-class equality for both classes.

The audit retains **23 left-trim** and **18 right-trim references**. Their source representations are moFromSktEntSurfIdRep_c or moDerivedSurfIdRep_c, external references are moFR_c, and feature pointers are null or moTrimRefSurface_c. Observed raw +48 values are 0 or 1; these observations do not establish a Boolean, enum or index domain.

Across 906 complete modern contexts there are **24,506 complete roots**, **777 complete lists** and **129 explicit stops**: 14 additional roots and two additional lists relative to Edition 0.229. **704 lists reach exact EOF**, spanning **48,001,317 decoded bytes**, an increase of 51,992 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. The next affected identifier dependency is moDraftSurfIdRep_c, which remains an explicit stop. `resolved-feature-list-frontier-0.229.json` preserves the preceding checkpoint. Whole-document coverage credit is unchanged; complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-252.
