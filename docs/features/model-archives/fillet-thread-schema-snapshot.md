### 11.93 Cosmetic-thread feature and thread-data framing

*Status: Native serializer and synthetic branch evidence; subsequent native completion documented in 11.97 · Evidence: [C-108]*

The ordinary modern moCosmeticThread_c serializer uses the model-feature base (11.81), rather than the atom/body-feature stages. Its primary vtable hooks at offsets 0x1EF0/0x1EF8 resolve to 0x4C1FC4A0/0x4C1FC410; both consume no archive bytes when loading. The following suffix is scoped to ordinary zero-condition external-XML archives and versions above 0x27CB. Nonzero archive conditions 1/0x80 introduce other vector/word branches and are outside this reader.

The suffix begins with tagged cylinder reference (368), tagged component edge (370), tagged moCThreadData_c (6A0), CString (3B0), tagged display-distance dimension (3B8), tagged display-radial dimension (3C0), i32 cache word (written into the data object's +180), tagged surface reference (388), and three i32 words (360,418,41C). Native loading overwrites word_418 with 1; readers SHALL preserve its stored value. Next are i32 index count and that many i32 indices, tagged reference wrapper (398), f64 value_3A0 and tagged component feature (468). Readers SHALL preserve duplicate and unordered stored indices; native loading retains only successively increasing entries.

Version >0x27CF adds tagged edge (380) and surface (390); >0x27DE adds i32 word_378; >0x2F6A adds i32 word_424; >0x39D0 adds two i32 words associated with runtime bytes 699/69A. Readers SHALL retain these full stored i32 values, independently of native Boolean conversion. Unsupported nested objects stop the reader without a complete-object result.

The moCThreadData_c modern body has the following order. Offset labels describe native members and do not imply file offsets or established semantic names.

| Order | Fields | Representation |
|---|---|---|
| 1 | words 08,0C,10; byte 14 | i32 × 3; u8 |
| 2 | values 18,20,28; word 30; value 38 | f64 × 3; i32; f64 |
| 3 | points 68,70,78; cone F8 | three three-double points; eight f64 cone values |
| 4 | words 100,180; paired values 48,58,50,60; word 108 | i32 × 2; f64 × 4 in that interleaved order; i32 |
| 5 | instance count; transforms; instance IDs; instance words | i32 count; count compact mgXforms; count tagged moCThreadInstanceId_c pointers; count i32 values |
| 6 | vector 178 | three doubles, only if instance count is nonzero |
| 7 | caches | i32 count followed by count tagged moCThreadCacheData_c pointers |
| 8 | word 10C; value 1C8; optional values 1D0,1D8 | i32; f64; two f64 only when word_10C is nonzero |
| 9 | point presence; optional point 120; value 128 | u8; three doubles only when presence equals exactly 1; f64 |
| 10 | three string/word pairs | CString 190/i32 1A0; CString 1A8/i32 1B0; CString 1B8/i32 1C0 |

The cone helper reads six ordinary doubles, an angle double and a final double, for 64 bytes total. Runtime sign/flag interpretation of the angle consumes no further bytes. These bytes differ from the nine-double sketch cone entity in 11.36. Readers SHALL retain the eight stored values rather than substituting that sketch layout.

After the table, version >0x2B6A adds CString 198; >0x3366 adds f64 value_2D0; >0x3379 adds a three-double point at 80; >=0x3749 adds i32 word_118 and, when nonzero, another two f64 values associated with 1D0/1D8. Readers SHALL preserve both pairs if both conditional branches occur. Native string cleanup and word normalization do not remove archive fields. Instance/cache class bodies remain separate dependencies; the scoped reader stops on unsupported new objects rather than skipping them.

Primary RTTI proves moRevCut_c uses the same serializer 0x4BB322F0 as moRevolution_c; the opt-in thread audit admits this alias. `--complete-threads` retains 898/906 complete relation-manager records, 21 profiles, 11 extrusions, five revolutions and two ribs. No cosmetic-thread object or revolution cut returns complete: three native paths now stop on moLPattern_c inside the inherited feature graph. The other five stops remain fillets and present kernel-body framing. `complete-thread-corpus.json` retains source paths/streams for all eight failed cases. Suffix/data layout claims here have native-code and synthetic boundary evidence; whole-document bit coverage is unchanged.

### 11.98 Fillet feature, surface-ID keepers and radius objects

*Status: Native runtime-class/serializer evidence; seven complete fillets verified; kernel continuation extended in 11.99 · Evidence: [C-113]*

The archive class Fillet_c is implemented in installed sldMoPartu.dll, not the model DLL whose class-ID table contains the same name. Exported runtime-class record 0x180604A60 declares name Fillet_c, runtime object size 3144 (0xC48), versionable schema value 0x80000001 and factory 0x180268B40. Primary RTTI selects serializer 0x18026FFE0. These are runtime identities, not file-header widths. `fillet_identity.py` reproduces the runtime record and primary table from the binary with SHA-256 2201edfa07b1a7484acedb6db4600fa7a00e55b16090a03b92fe7d9b14cb2128.

The scoped ordinary zero-condition modern load path starts with the supported body-feature base, then two direct surface-ID keepers at native members 818/820. Each keeper stores i32 surface-ID count and that many direct moEntityId_c bodies, followed by a direct suObArray. Each modern entity-ID body stores its tagged representation pointer as in 11.18. The edge keeper's array is at F0; the face keeper's array is at E0. Constructors prove both concrete array types. The subsequent s_replaceExtObj pass and reference-use updates consume no archive bytes. Readers SHALL retain ID/array entry order and pointer identity without introducing container tags or object slots.

Next are tagged suObList pointers at 840,848,850,858,870; i32-presence nullable direct suObArray at 8D8; direct suObArray at 8F0; and tagged suObList at 878. Archive conditions 1/0x800 and reference-history variants can alter other paths; this reader supports the documented ordinary load path and rejects unsupported inherited data before entering this suffix.

The modern settings follow in this exact order. Offset labels are native members, not file offsets or established semantic names.

| Order | Fields | Representation |
|---|---|---|
| 1 | dimension 828; list 860; byte 800; list 868; byte 814; word 8E0 | tagged display dimension; tagged list; u8; tagged list; u8; i32 |
| 2 | byte 928; point reference 880; bytes 8CC/8D0; word 8D4 | u8; tagged point-reference; u8 × 2; u16 |
| 3 | word 938; byte 92C; word 910; values 918/920 | i32; u8; i32; f64 × 2 |
| 4 | bytes 934/93C/940; feature-reference presence; optional reference 808 | u8 × 3; i32; tagged moFeatureRefWrapper_c when presence is nonzero |

Readers SHALL retain stored word_8E0 even when native loading normalizes values above 2, the full bits of word_8D4 even though native sign-extends its short, and all raw flag bytes/presence values. Earlier field gates precede the scoped versions above 0x27CB.

The cache predicate at 0x1802B4B00 evaluates true in this ordinary modern version scope. Thus an i32 cache-presence word always follows the settings. Zero ends the cache subrecord. Nonzero adds i32 word_72C, tagged component edge 8B0, i32 body count and that many kernel-body archive records. Present kernel bodies SHALL stop the reader unless their stream/model framing is established; suppression markers may end a body record independently.

After those bodies, the present cache stores i32 face-edge map count and pairs of tagged component face/tagged component edge, then direct suObList 9E0. Next is i32 face-index map count; each entry stores a tagged component face, i32 index count and i32 indices. Native loading discards those indices; readers SHALL preserve them, including duplicates and order. Four i32 words AF8,8C0,8C4,8C8 end the present cache. The cache-absent branch omits this entire continuation.

The common late tail stores i32 word_B90, then i32 edge-data count and records of i32/i32/f64/tagged component edge. Direct su_CDWordArray C18 follows, using packed count and u32 values. Then come tagged display dimensions 830/838, u8 byte_944 and i32 words 8E4/8E8. Version >0x2B8E adds u8 byte_C40 and tagged suObList C38. Readers SHALL retain all map entries and stored words independently of native insertion, normalization or sign-extension behavior.

Radius objects within the lists share a base body consisting only of tagged moDisplayDim_c. edgeRadiusObject_c then adds tagged moCompEdge_c and i32 word_20; faceRadiusObject_c adds tagged moCompFace_c; loopRadiusObject_c adds tagged moCompLoop_c then tagged moCompEdge_c. These are tagged objects whose bodies use the stated order. Unsupported new component-loop or other nested classes still stop the enclosing reader.

`--complete-fillets` completes seven fillet objects and 902/906 relation-manager records. Traversal also completes 28 profiles, 15 extrusions, five ordinary revolutions/six revolution cuts, two ribs, three linear patterns and three cosmetic threads. The seven fillets contain 35 edge surface IDs, zero face surface IDs, 28 face-edge map entries, 28 edge-data entries and 42 DWORD values. All seven have cache presence 1 and zero cached bodies. Nested spans are retained without assuming that completed subobjects imply completed parent configurations.

The Edition 0.98 audit retained four present-kernel-body stops: the vessel body at 0x713F and bodies at 0x11109/0x110B3/0x1112B in three base-plate files. Caller traces in 11.99 correct the original cache attribution: the latter three are profile-geometry bodies nested in fillet dependency graphs. Their validated decoding and later stops are documented there. No opaque kernel bytes are skipped or credited as decoded. These results establish the supported fillet path and manager prefixes; full document bit coverage remains incomplete.

### 11.181 General surface references and absolute attachment points

*Status: B / ? — native modern serializer bindings and five completed surface-reference boundaries verified; absolute-point populated native boundaries and raw geometry meanings remain open.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moGeneralSurfRef_w primary table `0x4D1540E8`, complete-object locator `0x4D358290` and serializer slot +28 select `0x4C2866C0`. RTTI establishes moSurfaceRef_w and moRefWrapper_c ancestry. The serializer calls wrapper helper `0x4C2842D0` and then serializes four f64 fields. This is the existing shared surface-reference grammar used by the base of moSingleFaceRef_w; it does not include that derived class's embedded-body presence or later suffix.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | reference wrapper | direct moRefWrapper_c body | always | Existing 11.16 versioned grammar; preserve tagged component/reference identities and optional transforms. | B / ? |
| 2 | raw +A8/+B0/+B8/+C0 | four f64 | always in this scope | Exact member order shown. Historical load/store gate is `v > 0xF22`, below the supported modern minimum. Preserve stored values without imposing ordering or unit rules. | V / ? |

The latest-version null-component, absent-transform reference body is 94 bytes: the 62-byte wrapper followed by 32 bytes of doubles. The four native defaults are -1, 1, 1, -1, but they are defaults rather than omitted wire data in modern files. The reader keeps arbitrary stored doubles and attributes their unresolved meanings as unknown bytes. Only the outer tagged object allocates its normal map slot; its direct inherited body does not allocate another object.

moAbsolutePoint_c primary table `0x4CE7CC88`, locator `0x4D313D68` and serializer slot +28 select `0x4C27A640`, the existing moAttachPoint_c serializer. RTTI confirms moAttachPoint_c ancestry. There is no absolute-point-specific suffix in this dispatch. The scoped alias uses the existing ordinary model-root attachment reader and retains its root/context restrictions:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | points +08/+28 | two direct three-f64 points | always in supported ordinary model-root context | Preserve both stored points independently. | B / ? |
| 2 | raw +20 | i32 | always in scope | Signed word after both points; member offset does not determine wire order. | V / ? |
| 3 | drawing-view reference | tagged moDrView pointer | always in supported branch | Preserve null/cached/new framing. Unsupported contexts still reject rather than infer omitted fields. | B |
| 4 | points +58/+70 | two direct three-f64 points | always in scope | No per-point pointer tags. | B / ? |
| 5 | raw +88 | u8 | always in scope | Preserve every flag bit without normalization. | V / ? |

With a null view, the absolute-point body is 103 bytes. Synthetic tests verify this inherited boundary, signed-word extrema, point order, high flag bits, cached outer identity, truncation and explicit drawing-root rejection. No completed native containing-root span currently includes an absolute point, so this synthetic result receives no native corpus-boundary credit.

The completed-root audit retains **five general surface references**, each nested in a complete moRefAxis_c root and referencing moCompFace_c. All five store the four values -1, 1, 1, -1. A source-stream SHA-256 and containing-root span regression reparse a populated native example. The frontier now records **22,807 complete roots**, **695 complete lists** and **211 explicit stops** across 906 complete modern model contexts. **633 lists reach exact EOF**, spanning **35,216,419 decoded bytes**; **62 complete lists retain four-byte unknown tails**. `resolved-feature-list-frontier-0.191.json` preserves the preceding checkpoint. The six former general-surface/absolute-point note dependencies now reach further nested objects, including generic text; complete display-note roots remain unverified. Full parts, assemblies, drawings and all installed-project bit coverage remains incomplete; no whole ResolvedFeatures coverage credit is assigned.

Evidence: C-209.

### 11.206 Variable fillets and edge/vertex dimension records

*Status: B / ? — native factory/direct-member bindings and eight populated variable fillets with nested dimension records verified; raw option meanings and full document/project coverage remain partial.*

In sldMoPartu.dll, VarFillet_c primary table `0x180478000`, locator `0x1804EE8A0` selects serializer `0x180271BF0`. RTTI establishes moBodyFeature_c ancestry, and the serializer first invokes its imported base serializer. Runtime descriptor `0x180604A90` records allocation size `0x8D0` and creator `0x180268B70`; the creator calls constructor `0x18026AB20`. That constructor creates a su_CDWordArray at +838 and allocates/constructs a moFilletSIdKeeper_c at +868. Both constructor imports are bound in the retained native evidence. Allocation sizes are not wire lengths.

The complete body in ordinary non-DB external-XML archives with version greater than `0x27CB` is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited body feature | direct moBodyFeature_c body | always | Complete inherited body, not an ordinary constant-radius fillet suffix. | B |
| 2 | keeper +868 | direct moFilletSIdKeeper_c body | always in scope | Allocated runtime member serialized through its own virtual table without outer pointer/class framing. Existing signed surface-ID count/direct IDs and direct tagged-reference list grammar applies. Historical gate `v > 0x17F`. | B |
| 3 | lists +870/+878 | two tagged suObList pointers | always | Preserve two pointer occurrences and cached identities. | B |
| 4 | raw +81C/+80C | two u8 | always | Native runtime fields are wider, but wire values are bytes. | B / ? |
| 5 | raw +810 | i32 | always in scope | Historical gate `v > 0xC0`. Native resets values above 2 to zero; offline decoding retains the original word. | B / ? |
| 6 | lists +880/+828 | two tagged suObList pointers | always in scope | Historical gates respectively `v >= 0x50E` and `v >= 0x727`. | B |
| 7 | raw +830 | i32 | always in scope | Historical gate `v >= 0xBF3`. | B / ? |
| 8 | array +838 | direct su_CDWordArray body | always in scope | Packed ReadCount followed by that many u32 words. No tagged outer array object or map slot. Historical gate `v > 0x1084`. | B / ? |
| 9 | raw +808/+814 | u8, i32 | always in scope | Historical gate `v > 0x182A`. Retain distinct wire widths. | B / ? |
| 10 | list +888 | tagged suObList pointer | always in scope | Historical gate `v > 0x1BBD`. | B |
| 11 | raw +818 | u8 | always in scope | Historical gate `v > 0x1FA3`. | B / ? |
| 12 | raw +820 | u8 | `v > 17000` decimal | No byte at version 17000. Native loading normalizes this value to a runtime bool; offline keeps the stored byte. | V / ? |

The minimal complete body is 285 bytes at version 17000 and 286 bytes at/after version 17001 with the tested latest inherited layout, before outer tagged framing. Nonempty keeper bodies, lists and arrays add their own data. The reader does not execute runtime option clamps or post-load geometry updates.

Two nested ordinary dimension-record bodies are:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | moEdgeDim_c edge +10 | tagged moCompEdge_c pointer | always | Primary `0x18047A6F8`, locator `0x1804EE510`, serializer `0x180272B00`; su_CObject ancestry/base import. | B |
| 2 | moEdgeDim_c raw +08/+0C | two i32 | always in scope | Native gate `v > 0x757`; earlier byte-width variant is outside this scope. Complete null/cached-edge body is ten bytes. | B / ? |
| 3 | moVertDim_c dimension +08 | tagged moDisplayDim_c pointer | always | Primary `0x18047A668`, locator `0x1804EE598`, serializer `0x180272C30`; su_CObject ancestry/base import. | B |
| 4 | moVertDim_c vertex +10 | tagged moCompVertex_c pointer | always | Immediately after row 3 within the vertex record. | B |
| 5 | moVertDim_c raw +28 | u8 | always in scope | Historical gate `v >= 0x456`. Preserve noncanonical byte values. | B / ? |
| 6 | moVertDim_c dimensions +18/+20 | two tagged moDisplayDim_c pointers | always in scope | Historical gate `v >= 0x182B`; preserve all three independent dimension occurrences and repeated cached identity. Complete null/cached-pointer vertex body is nine bytes. | B |

`fillets.variable`, `fillets.edge_dimension` and `fillets.vertex_dimension` are installed only in the copied scoped map. Five focused tests cover raw bytes/words and non-normalization, direct DWORD-array ordering/high bits, repeated list/dimension identity, the 17000 transition, complete tagged minima, truncation/context rejection, and source-hash-bound complete native root spans with variable-fillet and nested edge/vertex field equality.

The audit retains **eight variable fillets**, **30 edge-dimension records** and **40 vertex-dimension records** in complete containing roots. All eight variable fillets store +810 = 0 and +830 = -1; six have raw +80C = 1 and two have zero. All other observed raw option bytes/words are zero. Every direct +838 array has six words; seven store `[0,21040,0,23340,22710,22991]` and one stores `[0,21040,1,23340,22710,22991]`. These observations do not establish word meanings, supported domains or general array length restrictions.

Across 906 complete modern contexts the frontier now has **24,036 complete roots**, **745 complete lists** and **161 explicit stops**: 207 additional roots and four additional complete lists relative to Edition 0.211. **675 lists reach exact EOF**, spanning **44,030,463 decoded bytes**; **70 complete lists retain four-byte unknown tails**. Four additional lists reach EOF, adding 1,688,338 exact-EOF bytes. All completed-list endpoint/tail invariants pass. `resolved-feature-list-frontier-0.211.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-234.

