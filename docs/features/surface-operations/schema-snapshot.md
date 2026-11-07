### 11.222 Surface-cut and trim-reference-surface features

*Status: B / ? — five surface cuts and one trim verified inside complete native roots; raw meanings, unobserved pointer combinations and full-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature context, version >0x27CB. sldMoPartu.dll primary RTTI binds moSurfCut_c (0x1804BBD88 / COL 0x1804F2548) to 0x180357290, and moTrimRefSurface_c (0x1804C54B8 / COL 0x1804F29D0) to 0x180360030. Both have moBodyFeature_c ancestry, call its imported serializer and use the ordinary inherited moConcept_c lightweight/resolved hooks. Surface-cut follows the complete body-feature prefix with:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | atom feature +808 | tagged moAtomFeature_c | always in scope | Preserve pointer identity. | B |
| 2 | raw +818,+81C | two i32 | always | Separate signed words. | B / ? |
| 3 | face +820 | tagged moFaceRef_c | always in scope | Nullable tagged occurrence. | B |
| 4 | surface +810 | tagged moConstSurfRef_w | always in scope | Existing constant-surface grammar. | B |
| 5 | plane +900 | tagged moPlaneRef_w | always in scope | Existing plane-reference grammar. | B |
| 6 | arrays +8C0,+8E0 | two direct suObArray | always in scope | Each ReadCount and tagged items, in order. | B |
| 7 | chooser +838 | direct moBodyChooser_c | always in scope | Direct object list then tagged moAtomFeature_c owner; no extra tagged chooser object. | B |

Native load gates >=0xD6, >=0x3AB, >0x4CA, >0x7F9 and >=0x70F all precede scope. Constructor 0x180356F70 binds the two suObArray members and embedded body chooser. With null/cached pointers and empty arrays/chooser, the complete body is **275 bytes**. New targets contribute normal tagged bodies and map slots.

Trim constructor 0x18035F590 allocates a moTrimSurfData_c at runtime +800 through constructor 0x18035F840 and constructs the direct body chooser at +808. The serializer does not write a tagged trim-data object: it dispatches selected members of that allocation directly. Runtime offsets prefixed data below refer to that allocated data object. After the complete body-feature prefix the modern trim sequence is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | data arrays +20,+48 | two direct suObArray | always in scope | Direct collections in exact order. | B |
| 2 | data profile +18 | tagged moProfileFeature_c | always in scope | Normal archive map identity. | B |
| 3 | data direction +40 | tagged moDirectionSpec_c | always in scope | Nullable target. | B |
| 4 | data raw +10 | u32 | always in scope | Native gate >0x4DF predates scope. | B / ? |
| 5 | feature chooser +808 | direct moBodyChooser_c | always in scope | Direct object list then tagged owner. | B |
| 6 | data solid +68 | tagged moSolidRef_w | always in scope | Existing solid-reference grammar. | B |
| 7 | data keeper +78 | direct allocated moEntRefKeeper_c body | always in scope | suObArray serialization; no pointer tag or separate count prefix beyond that array grammar. | B |
| 8 | data array +A0 | direct suObArray | always in scope | Separate direct array. | B |
| 9 | data raw +08,+0C | two i32 | always in scope | Preserve signed words. | B / ? |

The trim proxy/transfer path below version 0xDF7 is excluded. Later native gates >0xDF8, >0x1FE4 and >=0x2000 also precede scope, so all rows are present throughout the supported context. Constructor evidence identifies direct arrays +20,+48,+80,+A0 and an allocated entity-reference keeper +78. Array +80 is constructed but is not separately dispatched by this serializer; it must not be inserted into the wire sequence. The keeper's primary serializer thunk 0x1803AD148 resolves to sldmfcu.dll suObArray::Serialize. The body chooser independently binds through sldmodu.dll primary 0x4CE879D0 to 0x4C27AAA0: ordinary zero-byte su_CObject base, direct object list and tagged owner. Runtime allocation sizes 0xC0 for trim data and 0x8B8/0x908 for feature objects are not serialized lengths. The complete empty trim body is **281 bytes**.

Both readers are installed only in the copied modern feature-context map. Four focused tests cover full pointer/collection ordering, duplicate and cached identities, signed/u32 extremes, populated arrays/keeper/chooser, direct-data map accounting, tagged minima at 0x27CC and version 19000, cached self/following-byte alignment, truncation/context rejection and source-hash-bound complete native containing-root spans/raw fields/counts/target classes for both features.

Five native surface cuts have raw +818 = -1; raw +81C is zero in three and one in two. Both arrays and chooser lists are empty. Each has a plane-reference pointer and null atom/face/constant-surface pointers. The complete native trim has array counts (1,1,0,0), one chooser item, profile moProfileFeature_c, null direction/solid, u32 raw +10 = 458760 and raw +08/+0C zero. These observations do not define valid domains or restrict other pointer combinations.

Across 906 complete modern contexts there are **24,399 complete roots**, **774 complete lists** and **132 explicit stops**: 22 additional roots and three additional lists relative to Edition 0.227. **701 lists reach exact EOF**, spanning **46,743,994 decoded bytes**, an increase of 211,155 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. Later affected parsing reaches moOffsetRefSurface_c or trim-edge entity representations rather than skipping them. `resolved-feature-list-frontier-0.227.json` preserves the preceding checkpoint. Whole-document coverage credit is unchanged; complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-250.
