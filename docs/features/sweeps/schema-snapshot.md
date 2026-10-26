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

Constructor evidence establishes the five direct suObArray members and two direct suObList members. Other initialized collections, including lists +08/+158/+1A0/+1E8, dynamic array +360 and arrays +370/+390, are not silently appended to this serializer. Direct collection counts SHALL fit the remaining minimum tagged-pointer slots. New pointed-to objects share the enclosing MFC map; collections themselves add no object-map slots. The empty data body is 111 bytes through v 0x27D9 and 115 bytes thereafter. Earlier data conversions, a discarded byte in versions 0x1BC6–0x2013 and pre-0x2014 twist/endpoint behavior are outside modern scope.

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


### 11.164 Endpoint references and spline tangency controls

*Status: B — native modern layouts, direct-handle types and populated corpus boundaries verified; raw scalar meanings and special contexts remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`, with sketch-control archive conditions 1, 2 and 4 clear. `moEndPointRef_w` primary table `0x4D17D008` selects `0x4C27D8F0`; the serializer calls point-reference helper `0x4C281C60`, which calls common reference wrapper `0x4C2842D0`. Thus the endpoint inherits the complete wrapper from 11.16 and its three-f64 point at +A8 before the following suffix. The runtime class and cached pointer identities remain distinct from the base point-reference class.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited point reference | reference wrapper; point | always | Existing common wrapper, including conditional transforms and versioned raw words, followed by three f64 point coordinates. No additional class tag for the inherited layers. | V / ? |
| 2 | raw +E8 | f64 | always in this scope | Preserve the stored value. Historical load before `0x1FB` reads an i32 discriminator instead; that conversion is excluded. | V / ? |
| 3 | raw +F0 | i32 | always in this scope | Preserve complete signed word. Historical load omits it through `0x726`. | V / ? |

The latest-version null-component, no-transform endpoint body is 98 bytes before an outer object/class tag: 62-byte wrapper, 24-byte point and 12-byte raw suffix. Wrapper version gates still apply; this fixed example is not a replacement for their grammar. The native base resets a runtime +E0 pointer without consuming an archive field. The suffix values are not assigned parameter-range or boolean semantics solely from the runtime class name.

`moSplineTangencyControl_c` primary table `0x4D1CF2A0` selects serializer `0x4C5C2690`. It calls derivative-control helper `0x4C5B2260`, which calls sketch-control helper `0x4C5C4690`. In the stated ordinary mode the common sketch-control helper reads exactly one tagged sketch pointer. The derivative helper then serializes two embedded handles directly, before the tangency point/vector suffix.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | sketch +08 | tagged sgSketch pointer | always in this scope | Preserve null or actual cached/new object identity. Special conditions can source the owner from runtime context instead and are excluded. | V |
| 2 | spline handle +40 | direct sgSplineHandle body | always | Existing entity-handle grammar: unsigned u16 index, signed i32 replacement only for sentinel `0x777F`, then i32 reference ID and i32 DimOnCM. No class tag or object-map entry. | V / ? |
| 3 | point handle +18 | direct sgPointHandle body | always after row 2 | Same handle grammar, read as a separate record even when values match the spline handle. Native member-offset order does not determine wire order. | V / ? |
| 4 | point +128 | three f64 | always | Preserve all coordinates; geometric role beyond the native point type remains partial. | V / ? |
| 5 | vector +140 | three f64 | always | Preserve all coordinates without normalization. | V / ? |

The null-sketch, short-index tangency body is 70 bytes. Each escaped handle index adds four bytes, so a two-escaped-handle body is 78 bytes. Neither direct handle allocates a map slot; only an outer tagged control and its tagged sketch pointer can do so. Native runtime-class descriptor `0x4DAF6970` selects factory `0x4C5C2130`, which allocates `0x160` bytes and calls derivative constructor `0x4C5B2020`. That constructor establishes sgPointHandle at +18 and sgSplineHandle at +40. Both handle primary serializer slots select `0x4C591ED0`, establishing the shared direct-body grammar. The factory's other points/defaults, the sketch watcher and runtime validity state add no wire fields to this serializer.

Native load assigns the sketch to the embedded handles, checks runtime entity resolution and creates watcher state. Offline reading preserves the two stored handle records without resolving sketch entities or executing watcher updates. Historical tangency coordinate transformation before `0x96C` and nonordinary sketch-owner modes are excluded. Complete pointer framing does not establish validity of arbitrary synthetic index/reference combinations.

The complete-root audit retains **14 endpoint references** and **40 spline tangency controls**. All endpoints reference moCompEdge_c; +E8 is 0.5 in 13 cases and 0.0 in one, while +F0 is zero in all 14. All 40 controls have sgSketch owners. Per-root summaries retain both direct handle records and point/vector coordinates; synthetic tests exercise escaped indices, signed extrema, equal-but-separate direct records, map accounting and exact/truncated boundaries.

Edition 0.175 records **22,083 complete roots**, **618 list-complete configurations** and **288 explicit stops** across **906 complete modern model contexts**. **564 lists reach exact EOF**, spanning **29,438,247 decoded bytes**; **54 complete lists retain four-byte unknown tails**. Remaining dependencies include flat patterns, connection points, weldment features, coordinate systems and draft features. Part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these record boundaries.

Evidence: C-192.


### 11.177 Modern sweep cuts

*Status: B — native inheritance, modern suffix absence and populated containing-root boundaries verified; shared sweep meanings and runtime cut geometry remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moSweepCut_c primary table `0x4D2E1C90` selects serializer `0x4CB41A20`. RTTI establishes moSweep_c ancestry. The serializer first calls moSweep_c serializer `0x4CB41AF0`, selected by moSweep_c primary table `0x4D2DF5D0`. These are different entry points; native control flow, rather than their names, establishes that the complete shared sweep body is reused.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited sweep body | complete body-feature base, direct sweep data and sweep suffix from 11.156 | always | Preserve every existing version gate, direct array, tagged pointer, raw scalar/byte and optional dimension. | V / ? |
| 2 | sweep-cut-specific suffix | zero bytes | supported modern scope | Native extra load fields are restricted to versions greater than 0x11CB and less than 0x142A, below the documented minimum. | V |

The historical extra load path reads an i32 presence word and a moRefWrapper_c pointer when nonzero. It is absent throughout this modern scope, so the reader does not add a zero presence word, alignment padding, cached reference or cut option after the shared sweep body. Native store likewise adds no sweep-cut-specific body in the recovered serializer. Older versions remain unsupported by this alias.

The scoped registry uses the existing complete sweep reader for moSweepCut_c. That reader still requires the ordinary context and version above `0x27CB`; no global registration or legacy-archive broadening is introduced. In particular, the shared direct-data +3B0 word remains gated after `0x27D9`. Optional dimensions controlled by sweep data retain their existing presence rules. The latest empty/null shared body occupies **377 bytes**, including its body-feature base. Populated synthetic fixtures verify paired optional dimensions, shared direction/angle references, tagged sweep-cut/cached identity and an exact following-record boundary without introducing a new class-specific suffix.

The complete-root audit verifies **six native sweep cuts**. Each has one item in direct data arrays +2C0 and +2E0, and zero items in +300/+320/+340. Their +234/+248/+278/+258 control values are zero and no optional sweep dimension fields occur in these samples. These are observed raw values, not a complete interpretation of sweep-cut options. Hash-bound native tests verify complete containing-root endpoints. Populated native optional-dimension branches, further nested reference/chooser classes and runtime cut construction remain open.

Edition 0.188 records **22,786 complete roots**, **691 list-complete configurations** and **215 explicit stops** across **906 complete modern contexts**. **629 lists reach exact EOF**, spanning **35,023,726 decoded bytes**; **62 complete lists retain four-byte unknown tails**. Annotations, helices, additional sketch dimensions and feature/chooser dependencies remain unresolved. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these record boundaries.

Evidence: C-205.


### 11.204 Loft synchronization references

*Status: B / ? — native ancestry/base call and 36 populated references in complete containing roots verified; raw option/parameter meanings and full document coverage remain partial.*

moLoftSynchRef_c primary table `0x4D2D51C0`, complete-object locator `0x4D37FE58` and serializer slot +28 select `0x4CAEEB60`. RTTI establishes moEndPointRef_w ancestry, and the serializer first calls moEndPointRef_w::Serialize at `0x4C27D8F0`. The complete ordinary non-DB external-XML body for versions greater than `0x27CB` is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | endpoint reference | direct moEndPointRef_w body | always | Existing point-reference/wrapper body plus raw +E8 double and +F0 word. No extra tagged base object or map slot. | B |
| 2 | raw +F8 | i32 | always | Preserve signed word. | B / ? |
| 3 | raw +FC/+100 | two u32 | always | Separate complete words, including high bits. | B / ? |
| 4 | raw +108 | f64 | always in scope | Historical load gate `v > 0xA1E`. | B / ? |
| 5 | reference +1A0 | tagged moRefWrapper_c pointer | always in scope | Historical gate `v > 0xBA8`; observed targets include moGeneralCurveRef_w. Retain standard null/cached/new identities. | B |
| 6 | raw +19C/+1A8 | two i32 | always in scope | Historical gate `v > 0xC71`. These runtime offsets do not determine the wire sequence. | B / ? |
| 7 | point reference +1B0 | tagged moPointRef_w pointer | always in scope | Follows both row 6 words. | B |
| 8 | raw +1B8/+1C0 | two f64 | always in scope | Historical gates respectively `v > 0xC71` and `v > 0xC79`; both precede this scope. | B / ? |
| 9 | raw +110/+118 | two f64 | always in scope | Historical gate `v >= 0x11B7`; follows +1C0 on wire despite lower runtime offsets. | B / ? |
| 10 | points +120/+138 | two × three f64 | always in scope | Separate native point archive operations under the same historical gate as row 9. | B / ? |
| 11 | curve references +1C8/+1D0 | two tagged moCurveRef_w pointers | always in scope | Historical gate `v >= 0x1213`; preserve repeated cached pointers and null occurrences independently. | B |

The complete suffix after the inherited endpoint occupies 116 bytes with null/cached pointer occurrences. An empty latest inherited endpoint occupies 98 bytes, making the latest empty complete body **214 bytes** before outer tagged framing. Earlier supported wrapper gates yield empty bodies of 202, 206 and 210 bytes at the tested versions `0x27CC`, `0x2B73` and `0x2FB0`. The historical pre-`0x11B7` point/default initialization is runtime behavior and adds no modern placeholder fields.

`splinecontrols.loft_synch` is installed only in the copied feature-context reader map. Three focused tests cover raw signed/high-bit words, all suffix offsets in wire order, both points, independent point/reference pointers, duplicate cached curve pointers, exact following-byte alignment, tagged identity/map accounting, wrapper gates, truncation/context rejection, and a source-hash-bound complete native containing root with stored raw/point/target-class equality.

The audit retains **36 complete loft synchronization references** in completed roots. In 32, +1A0 targets moGeneralCurveRef_w; four have a null +1A0 pointer. All observed +1B0/+1C8/+1D0 pointers are null. Raw +F8 values are 0 (14 records), 7 (10), 4 (6), 6 (3) and 5 (3); no option meaning or closed domain is assigned from these observations.

Across 906 complete modern contexts, the frontier now has **23,679 complete roots**, **736 complete lists** and **170 explicit stops**: 65 additional roots and four additional complete lists relative to Edition 0.209. **667 lists reach exact EOF**, spanning **38,435,564 decoded bytes**; **69 complete lists retain four-byte unknown tails**. One additional list reaches EOF, adding 272,233 exact-EOF bytes. Completed-list endpoints and tail lengths pass the recorded stream-size invariants. `resolved-feature-list-frontier-0.209.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-232.

