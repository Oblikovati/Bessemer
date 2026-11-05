### 11.156 Ordinary modern sweeps, wire contacts and embedded reference bodies

*Status: B — native-derived sweep/data/contact framing with populated native collections and compressed reference bodies; optional native dimension branches and remaining semantics stay partial.*

moSweep_c primary table `4D2DF5D0`, locator `4D380F00`, selects serializer `4CB41AF0` in sldmodu.dll. Runtime descriptor `4DB07BC0` selects factory `4CB41260`, calling constructor `4CB41380`. The constructor calls moSketchedFeature_c and initializes direct moSweepData_c +818 via `4CB53250`. The serializer explicitly delegates to moBodyFeature_c, then direct data serializer `4CB57060`. Allocation size 0xC60 is not serialized length.

For ordinary non-DB external-XML archives with v > 0x27CB, read the complete body-feature base, followed by the direct data body below. Data offsets are relative to moSweepData_c, whose containing feature offset is +818. Lists of offsets denote separate fields in the listed wire order.

| # | Data field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | arrays +2C0, +2E0, +300 | Three direct suObArray bodies | Always in modern scope | Each has independent ReadCount/tagged items | B |
| 2 | raw +270, +274; raw +278; raw +70 | Two u8; i32; f64 | Always | Preserve full byte/numeric values | B |
| 3 | raw +2AC, +2B0; raw +7C, +80 | Two u8; two i32 | Always | The two i32 words independently control field 4 | B |
| 4 | direction +88, +90 | Tagged moDirectionSpec_c pointers | Respectively raw +7C == 2, raw +80 == 2 | Omit the entire tag when the controlling value is not 2 | B |
| 5 | lists +98, +E0 | Two direct suObList bodies | Always | Independent ReadCount and tagged items; no list object tags | B |
| 6 | raw +230; raw +78, +234 | i32; two u8 | Always | +234 controls feature dimensions +BD0/+BD8 | B |
| 7 | raw +238, +240; raw +248, +24C, +2B4 | Two f64; three i32 | Always | +248 provides the second dimension's additional condition | B |
| 8 | raw +280, +288; raw +290 | Two f64; i32 | Always | Preserve original values | B |
| 9 | raw +27C, +2A8; raw +258; raw +260 | Two u8; i32; f64 | Always in modern scope | +258 controls the feature's radial dimension pointer | B |
| 10 | twist +298; endpoint +2A0 | Tagged moTwistDataForBiDirectionalSweep_c pointer; tagged moEndPointRef_w pointer | Always in modern scope | Unsupported freshly serialized nested bodies remain explicit stops | B |
| 11 | raw +2B8 | u8 | Always in modern scope | No inferred boolean normalization | B |
| 12 | arrays +320, +340 | Two direct suObArray bodies | Always in modern scope | Independent ReadCount/tagged items | B |
| 13 | raw +3B0 | i32 | v > 0x27D9 | Absent at v == 0x27D9; present at v == 0x27DA | B |

Three selected sweep-data roles are bound through exact native methods and the canonical direct-data serializer:

| Saved data member | Native association | Wire |
|---|---|---|
| 78, mask 0x2 | UseBlocks request component | u8; other bits remain unassigned |
| 234 (feature A4C) | isThinSweep full 32-bit integer property | u8 |
| 230 | declared gcSheetEndCappingType initializer parameter | i32 |

The no-argument getUseBlocks returns (current word 0x78>>1)&1. Its integer overload returns that component only when its supplied signed caller argument is at least 2000, otherwise zero; this is not an archive-version query. setUseBlocks writes (old word 0x78&~0x2)|((full 32-bit input<<1) modulo 2^32). It does not first clamp the input to zero/one, so noncanonical inputs can set other bits and wrap. The archive STORE takes the low byte and reached modern LOAD zeroextends it to the current word. Offline raw_78 retains all saved bits; no field-wide Boolean domain follows.

The exact feature getter isThinSweep reads full 32-bit member 0xA4C. The original constructor's embedded moSweepData_c at feature 0x818 joins that member to data 0x234. Its STORE low-byte/LOAD zeroextension policy differs from a native Boolean conversion. The existing nonzero 0x234 conditional dimension grammar remains unchanged; neither the name nor the current integer guarantees a final thin-wall operation.

Named moSweepData_c::initGcSweepData forwards current raw32 member 0x230 to the declared gcSheetEndCappingType parameter of the exact typed gcSweepLoftCommonData_c::initialize call. This supplies an internal parameter-type role, not numeric enum labels, a closed valid domain or a mapping to public API enum values. Other opaque formal inputs and backend member copies receive no engineering meanings here. Twist-angle getters access members of the separate 0x298 helper object; their offsets do not identify direct sweep-data scalars. Runtime IsTolerantBody at 0x2BC is not a saved field in this body. Existing nested-class/context limits, raw keys and order remain unchanged. Evidence: C-3522.

Constructor evidence establishes the five direct suObArray members and two direct suObList members. Other initialized collections, including lists +08/+158/+1A0/+1E8, dynamic array +360 and arrays +370/+390, are not silently appended to this serializer. Direct collection counts SHALL fit the remaining minimum tagged-pointer slots. New pointed-to objects share the enclosing MFC map; collections themselves add no object-map slots. The empty data body is 111 bytes through v 0x27D9 and 115 bytes thereafter. Earlier data conversions, a discarded byte in versions 0x1BC6–0x2013 and pre-0x2014 twist/endpoint behavior are outside modern scope.

**Nested twist record.** The tagged data 0x298 pointer can identify moTwistDataForBiDirectionalSweep_c. Actual descriptor 4DB07C28, versionable schema 0x80000001, factory 4CB53200, constructor and zero-offset su_CObject ancestry bind canonical primary 4D2E4410 to serializer 4CB53980. Runtime allocation 0x20 is not a file length.

In ordinary non-DB binary archive mode with archive mode byte +0x20 flag 0x10 clear, and reached LOAD version query >=0x2014, its schema 1 body is exactly:

| Order | Runtime member | Wire | Native current getter through SweepData298 |
|---|---|---|---|
| 1 | 08 | u8 | getTwistAngleReverseDir2, full 32-bit integer return |
| 2 | 10 | f64 | getTwistAngleDir2 |
| 3 | 18 | f64 | getTwistAngleTurnsDir2 |

The imported su_CObject::Serialize at 31D3F030 follows its verified zero-payload ordinary branch, so the complete scalar body is 17 bytes with no padding. Tagged-object/class headers and map entries remain outside that length. External model XML storage does not itself select archive-XML flag 0x10. Typed object operators bind the expected runtime class and separately select regular or DB ReadObject/WriteObject; DB and archive-XML bodies are not admitted by this 17-byte rule.

Selected STORE always writes these three members, taking only the low byte of current 0x08. Reached LOAD zeroextends that byte to the full 32-bit member, then reads the two doubles. It does not Boolean-normalize the byte; all 256 saved byte values remain raw offline. The older local LOAD branch below 0x2014 skips these scalar reads without directly resetting the members; constructor defaults do not substitute for archived values. The enclosing SweepData getters read through current 0x298 and return zero when it is null. Their names establish no angle units, integral turn-count domain, normalization or final geometry.

This documents the native nested body and its qualified field roles. The production reader still has no fresh-object implementation for this class, so its explicit unsupported-nested-body stop remains. No reader admission, populated source replay or coverage credit is added. Evidence: C-3524.

After the complete direct data body, the feature suffix is:

| # | Feature field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +C04 | i32 | Always in modern scope | Preserve signed value | B |
| 2 | raw +C08, +C0C, +C10, +C14, +804 | Five u8 | Always in modern scope | Stored independently despite wider runtime members | B |
| 3 | dimension +BD0 | Tagged moDisplayDim_c pointer | Data raw +234 nonzero | Original native feature control at +A4C | B |
| 4 | dimension +BD8 | Tagged moDisplayDim_c pointer | Data raw +234 nonzero and raw +248 == 3 | Original feature control at +A60 | B |
| 5 | raw +C18, +C1C | Two u8 | Always in modern scope | Follow the optional dimension pair | B |
| 6 | angular dimensions +BE0, +BE8 | Two tagged moDisplayAngularDim_c pointers | Data raw +278 is 8 or 9 | Both pointers occur in modern scope | B |
| 7 | radial dimension +BF0 | Tagged moDisplayRadialDim_c pointer | Data raw +258 nonzero | Original feature control at +A70 | B |

The leaf predicate `4CB551A0` reads data +278 and returns 1 for 8, 2 for 9 and 0 otherwise. Both nonzero return values select field 6; the return value is not a pointer count. Older gates for +BE0 (v > 0xA27), +BE8 (v >= 0x2017) and +BF0 (v >= 0x1FD3) are all satisfied in modern scope. With empty direct collections and no conditional pointers, the data/feature payload is **126 bytes** after v 0x27D9 and 122 bytes before or at that gate. Runtime +BF8/+BFC/+C00 state and post-load ownership updates do not add archive fields. Raw fields retain their serialized values; member meanings remain open.

The dependency moCompProfile_c has primary table `4CE97D18`, locator `4D319D60`, selecting `4BBBA2E0`. It delegates to shared component-feature serializer `4BBB9A60` and adds no fields. RTTI establishes moCompFeature_c/moCompRef_c ancestry. Its modern body is therefore the component-reference body followed by feature_id i32 and feature_time u32, as in 11.17. Scoped dispatch and reference-wrapper validation now accept this proven derived class while preserving other unsupported-context checks.

moWireContact_c primary table `4D2E4870`, locator `4D381210`, selects serializer `4CB6EF10`. It calls contact-base serializer `4CB6E990`, then appends tagged moRefWrapper_c +E8. In modern scope, its body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | array +50 | Direct suObArray body | Always | Copy-constructor `4CB6D8A0` proves the embedded type | B |
| 2 | raw +90; raw +98; angular dimension +A0 | f64; i32; tagged moDisplayAngularDim_c pointer | Always in modern scope | Preserve the numeric values before native legacy repairs | B |
| 3 | raw +A8; raw +B0, +B8, +C8, +CC | f64; four i32 | Always in modern scope | Non-contiguous order is intentional | B |
| 4 | synchronization +E0 | Tagged moLoftSynchRef_c pointer | Always in modern scope | Unsupported new nested bodies stop explicitly | B |
| 5 | raw +40, +44, +48; raw +C0 | Three i32; f64 | Always in modern scope | No hidden base-field block before array +50 | B |
| 6 | reference +E8 | Tagged moRefWrapper_c pointer | Always | Existing wrapper/derived reference grammar | B |

The empty wire-contact body is **62 bytes** with short null tags and a short zero array count. Historical numeric normalization and earlier missing-field gates remain outside this modern reader. Constructor-only pointer arrays and cached entities do not add fields.

Embedded general-curve and single-face reference bodies extend the partial layouts in 11.16. General-curve serializer `4C27E600` reads its existing **u8** presence value and, when nonzero, calls mo_loadBodyFromArchive. Single-face base serializer `4C2851F0` reads an **i32** presence value, calls the same body loader when nonzero, then reads a direct suObList attribute body in modern scope; final single-face serializer `4C2856E0` supplies the existing last word. In the selected Parasolid context both calls pass conversion flag zero. There is **no attribute list after a general-curve body**. Presence-field widths SHALL remain distinct.

The shared body reader preserves the suppression byte and accepts the fully validated size/GUID/zlib/single-body Parasolid form established by the existing embedded-kernel evidence. Unsupported forms remain explicit KernelBodyStop/errors rather than opaque-byte skips. After a single-face body, read its complete attribute list before the existing +C8/+D8 suffix; after a general-curve body, continue with its existing +C8/+CC words and three object pointers. Attribute assignment, partition changes and handle-owner updates are runtime operations; the offline reader preserves serialized references and does not mutate cached owners. Forty native embedded references now decode: **22 general-curve and 18 single-face**, all unsuppressed compressed Parasolid, with empty attribute lists in the face cases. Populated attribute lists are tested synthetically.

The frontier contains **21 fresh native sweeps**. Nineteen have initial array counts (1,1,0), two have (1,1,1); both direct lists and the last two arrays are empty. All six recorded controls (+7C/+80/+234/+248/+258/+278) are zero, so positive optional direction/dimension bodies have synthetic evidence only. Five focused tests cover populated arrays/lists, shared identity, all modern optional dimension predicates, +3B0 boundary, component-profile dispatch/wrapper acceptance, wire-contact fields, compressed curve/face continuations, attributes, malformed/context rejection and a hash-bound native sweep/reference containing-root end. The old unsupported-reference-body regression now checks that genuinely unrecognized kernel framing remains an explicit stop.

Edition 0.167 records **20,944 complete roots**, **514 list-complete configurations** and **392 explicit stops** across all **906 complete modern model contexts**. **475 lists reach exact EOF**, spanning **22,479,381 decoded bytes**; **39 complete lists retain four-byte unknown tails**. Leading dependencies include face/point reference-plane data (16), blend cuts (15), domes/component axes (14 each), 3D profiles/sketch regions (13 each) and flat patterns (12). Full part, assembly, drawing and installed-project bit coverage remains incomplete. Unknown raw meanings and remaining kernel contexts do not gain whole-document coverage credit from these traversal results.

Evidence: C-184.

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

Exact own-class native getters/setters bind full 32-bit members 0x818, 0x820 and 0x828 to ReverseDir, IsFillVolume and Merge respectively. The full 32-bit setIsProtrusion input is stored at 0x81C; this is a setter-only association. These values use signed32 archive codecs, with no closed Boolean/enum file domain or guarantee of actual fill, merging or protrusion. Raw 0x824 remains unnamed.

Member 0x810 is the saved binary64 Thickness fallback/input scalar. Named getThickness returns raw 0x810 only when current dimension 0x800 is null; otherwise it tail-dispatches dimension virtual 0x868(0). Named setThickness preserves the original receiver and input, optionally invokes current dimension 0x800 virtual 0x860 with the double and integer arguments 1/0, and only then writes the supplied double into original 0x810 on normal continuation. The saved scalar and current getter result are therefore distinct sources. Callback exceptions/faults can prevent the store; opaque targets, aliases and lifetime remain separate. Units, finite-value restrictions and final geometry are not inferred.

The selected suffix latches one effective version result after base/helper work; its threshold branches use signed comparisons, not unsigned. LOAD reads 0x820 from signed version 0x3DA and 0x828 from 0x709; the earlier reached 0x828 branch assigns literal 1. Their selected STORE paths have no corresponding LOAD gates. LOAD includes 0x824 from 0x6A1, otherwise locally assigns 0, and dimension 0x800 from0x3E2, otherwise null. Direction 0x830 LOAD begins at 0x2F74, while selected STORE emits its pointer without that LOAD gate. These native historical branches do not admit old contexts to the modern reader, impose a closed raw domain or establish universal saved/current identity. Offline fields preserve their actual saved values. Evidence: C-3523.

The complete empty feature body with null/cached targets is 283 bytes below 0x2F74 and 285 thereafter. New tagged targets contribute their own bodies and map slots. No operation/direction enum is assigned from field offsets or observed values.

The nested constant-surface reference is independently bound in sldmodu.dll: moConstSurfRef_w primary 0x4CED4C68 / COL 0x4D31FFE8 selects 0x4B90DBB0 and has moRefWrapper_c ancestry. It calls shared helper 0x4B90DB10, restoring the wrapper followed by u32 raw +A8. Its derived body reads i32 body presence; zero ends it, nonzero selects the existing strict kernel body and then a direct suObList of attributes. The attribute-list gate >=0x46E predates scope, so the modern wire grammar matches 11.220's solid reference. The implementation reuses that grammar only in the copied scoped reader map. Runtime partition changes differ from solid-reference behavior but add no archive fields. Unsupported kernel contexts retain explicit stops; an absent-body latest reference is 70 bytes.

moCompRefSurface_c primary 0x4CD8F7D8 / COL 0x4D2ECAF0 selects 0x4BBBA630. Its moCompRef_c ancestry and call to 0x4BBBA440 establish the component-feature body plus tagged reference geometry described in 11.219. It appends **i32 raw +100**, without a version gate in scope; this is not the coordinate-system class's fixed transform at the same runtime offset. Its latest empty body is 96 bytes. Native RTTI additionally establishes moCompSurfaceBody_c's moCompRef_c ancestry, admitting that already decoded component class and moCompRefSurface_c to the wrapper's accepted contexts.

Five focused tests verify exact scalar/word/pointer ordering, signed extremes, cached list/dimension/direction/geometry identities, inclusive direction boundary, tagged empty minima and self-map accounting, following-byte alignment, truncation/context rejection, wrapper component identity and native source-hash-bound complete containing-root spans and fields for all three classes. Present constant-surface bodies use the strict shared kernel grammar; populated native bodies in these references remain unobserved.

The three native thickening features each have one list item and a moDisplayDistanceDim_c, with null direction pointers. Raw +810 is 0.002032 once and 0.02 twice; other words are retained without inferred enums. Four constant-surface references have body presence zero and raw +A8 values 0,238,0,145. The one component surface reference has raw +100 zero, feature ID 19, feature time 1223400751 and null reference geometry.

Across 906 complete modern contexts the frontier has **24,377 complete roots**, **771 complete lists** and **135 explicit stops**: five additional roots and two additional complete lists relative to Edition 0.226. **698 lists reach exact EOF**, spanning **46,532,839 decoded bytes**, an increase of 25,245 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. `resolved-feature-list-frontier-0.226.json` preserves the preceding checkpoint. These feature traversal counts add no whole-document coverage credit. Complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-249.
