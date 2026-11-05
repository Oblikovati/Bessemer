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
