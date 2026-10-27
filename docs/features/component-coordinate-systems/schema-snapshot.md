### 11.163 Reference points and point-data variants

*Status: B — native modern feature/subtype framing and populated corpus boundaries verified; raw scalar meanings and special contexts remain partial.*

Scope is ordinary non-DB external-XML model archives with version greater than `0x27CB`. `moRefPoint_c` primary table `0x4D176A18` selects serializer `0x4C308960`, which calls the common reference-geometry serializer `0x4C283A00` before the following three tagged pointer occurrences. The inherited root point and reference geometry retain their existing layout. Offsets below identify native members rather than stream positions.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited reference geometry | common record | always | Existing body/atom reference-geometry composition from 11.100, including its versioned fields; no nested class tag for this inherited layer. | V / ? |
| 2 | point data +6F8 | tagged moRefPointData_c-derived pointer | always | Scoped reader requires non-null data. Preserve the actual runtime subclass and map identity. | V |
| 3 | profile +6F0 | tagged moProfileFeature_c pointer | always | Null/cached/new object framing; no separate presence byte. | V |
| 4 | surface reference +700 | tagged moSurfaceRef_w pointer | always | Native load reads a generic object pointer; native store uses the surface-reference overload. Preserve null and actual runtime identity. | V |

The modern point-data base uses serializer `0x4C308800`. After the zero-byte ordinary binary su_CObject layer, it reads no further fields at versions at least `0x3C2`; native store also emits none. A tagged base instance still consumes its normal MFC class/object tags and map entries. Readers SHALL NOT infer a point/vector body from its class name or omit the object-map entry because its payload is empty. The selected derived classes call this base directly and then append the following bodies, in the displayed wire order.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | moFaceCenterRefPointData_c | tagged single-face reference +10 | always | One pointer occurrence; all-null payload is 2 bytes. Serializer `0x4C308270`. | V |
| 2 | moCenterEdgeRefPointData_c | tagged curve reference +10 | always | One pointer occurrence; all-null payload is 2 bytes. Serializer `0x4C307FD0`. | V |
| 3 | moFaceEdgePointData_c | tagged curve reference +18; i32 +20 | always in this scope | Six-byte null-reference body. Preserve the raw word. Serializer `0x4C3082F0`. | V / ? |
| 4 | moFaceVertexProjectionPointData_c | tagged single-face +18; tagged point reference +20; tagged plane reference +28 | always in this scope | Three ordered pointer occurrences, including nulls and repeats; all-null payload is 6 bytes. Serializer `0x4C3083F0`. | V |
| 5 | moTwoEdgeRefPointData_c | tagged curve references +10/+18; i32 +20/+24 | always in this scope | Preserve both pointers and raw words; all-null payload is 12 bytes. Serializer `0x4C308D30`. | V / ? |
| 6 | moAlongCurveRefPointData_c | tagged curve +10; f64 +18/+28; i32 +20/+30; tagged moDisplayDim_c +38; i32 +34/+40 | always in this scope | Exact order differs from member-offset order. All-null payload is 36 bytes, including 32 raw scalar bytes. Serializer `0x4C307CD0`. | V / ? |

The gates for the selected modern derived fields precede this scope: the projection's final plane pointer begins at `0x969`, the two-edge final word after `0x97F`, and the along-curve final word at `0x1104`; other along-curve additions also precede the supported minimum. Historical base point/vector records, pre-`0x3C2` extra derived references and pre-`0x9A2` feature-level dimension/discarded-word conversions are excluded. Native data-owner updates, feature refresh and ordinary DB-only node work do not add binary fields here. Unsupported nested reference/kernel forms remain explicit stops.

`moCompRefPoint_c` primary `0x4CD90198` selects shared serializer `0x4BBBA440`, already used by the component-axis reference in 11.160. Its ordinary body is the complete component-reference base, i32 feature ID, u32 feature time and a tagged moRefGeom_c-derived geometry pointer. It has no plane-only +100 word. The scoped wrapper admits this proven moCompRef_c-derived class without an entity-handle sketch-pointer extension. Latest-version null-base composition totals 92 bytes before the outer class tag. The offline reader preserves stored pointers rather than resolving runtime geometric ownership.

The complete-root audit retains **36 reference-point features**: 11 face-center, ten along-curve, eight face/vertex projection, four center-edge and three two-edge data objects. All 36 feature-level profile and surface pointers are null. It also retains **15 component-point references**, all with null stored geometry pointers. Modern base-only and face/edge data bodies are bound to native code and exercised synthetically; they are not claimed as populated corpus variants. Per-root summaries preserve raw subtype scalars and pointer classes independently of feature completion.

Edition 0.174 records **21,942 complete roots**, **607 list-complete configurations** and **299 explicit stops** across **906 complete modern model contexts**. **553 lists reach exact EOF**, spanning **28,542,701 decoded bytes**; **54 complete lists retain four-byte unknown tails**. Leading dependencies include flat patterns, connection points, spline tangency controls, endpoint references and weldment features. Part, assembly, drawing and all installed-project bit coverage remains incomplete; these boundaries receive no whole-document coverage credit.

Evidence: C-191.


### 11.170 Coordinate systems and axis data

*Status: B — native selectors, direct member types and populated corpus boundaries verified; raw field meanings and runtime geometry remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moCoordSys_c primary table `0x4CED5DB8` selects serializer `0x4C27C160`, which first calls shared reference geometry `0x4C283A00` (11.100). It does not inherit the derived extrusion/revolution body-feature suffix. After the reference-geometry body:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | origin +6F0 | tagged moPointRef_w pointer | always | Preserve null/new/cached identity, including supported derived endpoint references. | V |
| 2 | selectors +71C/+720 | two u16 | always | Full raw selector values, widened in native memory but not on wire. | V / ? |
| 3 | axis 1 +6F8 | tagged moCSysAxisData_c-family pointer | either selector equals 1 | One occurrence when active, even if both selectors equal 1. | V |
| 4 | axis 2 +700 | tagged moCSysAxisData_c-family pointer | either selector equals 2 | Follows active axis 1, independent of selector order. | V |
| 5 | axis 3 +708 | tagged moCSysAxisData_c-family pointer | either selector equals 3 | Follows earlier active axes. | V |
| 6 | raw +710/+714/+718 | three u8 | always | Do not widen to four-byte runtime members or normalize nonzero values. | V / ? |
| 7 | point +728 | three f64 | always | Preserve stored coordinates without recomputing from origin/axes. | V / ? |
| 8 | raw +744/+748 | two i32 | version > 0x371A | Both words absent in older modern versions. | V / ? |
| 9 | raw +750/+758/+760 | three f64 | version > 0x371A | Follow row 8; no extra presence value. | V / ? |

The exact 26-byte leaf at `0x4B93D350` implements isAxisActive as equality with either selector; it is not a bitmask test. The writer emits axes in numeric order 1, 2, 3 and rejects records unless exactly two axes are active. The load path uses the selector predicate without this final writer check. Offline reading preserves duplicates/out-of-range selectors and consumes only the pointers selected by that predicate; such synthetic load-framing cases do not assert native writer validity. With two active null axes and a null origin, the latest suffix is **69 bytes**; through version `0x371A` it is **37 bytes**. A load-only selector combination selecting no axes consumes four fewer bytes.

moCSysAxisData_c primary table `0x4CEE8550`, moLineCSysAxisData_c primary `0x4CEE8748` and moPntCSysAxisData_c primary `0x4CEE85F8` all select the same serializer `0x4C27ABB0`. The two derived classes have verified moCSysAxisData_c ancestry and append no separate wire suffix. After the zero-byte ordinary binary su_CObject layer:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | references +08 | direct suObArray body | always | Packed ReadCount followed by tagged object pointers. No outer class/pointer tag or object-map slot. | V |
| 2 | vector +28 | three f64 | always | Preserve raw vector components; runtime unit-vector construction does not authorize offline normalization. | V / ? |

Axis runtime descriptor `0x4DAEDFD0` selects factory `0x4C270050`, which allocates `0x40` bytes and invokes constructor `0x4C270DB0`. That constructor installs the suObArray vtable at member +08, proving the direct virtual-call type, and constructs a unit vector at +28. The minimum axis-data body is **26 bytes**. Tagged enclosing axis-data objects still allocate ordinary MFC class/object-map entries; their direct arrays do not. Array references and repeated axis pointers retain original archive identity.

The coordinate-system serializer ends by calling moNode_c::SerializeDBData `0x4C04C120`, also used by previously documented ordinary feature families. Its work is guarded by moArchiveHelper_c::isOKForDBSerialize (`sldmfcu.dll` export `0x3CA59350`) and operates on the separate database/deferred store; it adds no fields to this ordinary stream layout. Database contents and special archive contexts remain separate work. Complete module hashes bind the retained out-of-line native bodies and tail-call bytes where Pdata describes only fragments.

The audit verifies **10 coordinate systems**. Selectors are `[2,1]` in seven and `[1,2]` in three, while axis pointers always occur in numeric order. Among 20 selected axis pointers, 14 are null, five introduce moLineCSysAxisData_c and one introduces moPntCSysAxisData_c. Each populated axis array contains one reference. Origin pointers are five moPointRef_w, three moEndPointRef_w and two null. Native tests verify hash-bound complete root spans for null, line and point axis forms. Synthetic tests cover all selector/presence paths, duplicate/out-of-range selector retention, signed values, version gates, truncation, direct array framing and tagged derived/cached identity.

Edition 0.181 records **22,248 complete roots**, **655 list-complete configurations** and **251 explicit stops** across **906 complete modern contexts**. **600 lists reach exact EOF**, spanning **31,484,744 decoded bytes**; **55 complete lists retain four-byte unknown tails**. Structural-member features, drafts, profile references, cut lists and additional feature/entity payloads remain unresolved. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these boundaries.

Evidence: C-198.


### 11.219 Component coordinate-system references

*Status: B / ? — native ancestry/helper dispatch and one populated reference inside a complete native root verified; transform/word semantics and whole-document/project coverage remain partial.*

In the ordinary non-DB external-XML context (version >0x27CB), moCompCoordSys_c primary 0x4CD91040 / COL 0x4D2ECF00 selects serializer 0x4BBB9060. Native RTTI establishes moCompFeature_c and moCompRef_c ancestry. The serializer calls helper 0x4BBBA440, which calls the existing component-feature serializer 0x4BBB9A60 and then restores a tagged reference geometry. The helper's obsolete version <0x2D1 branch predates scope. After that helper the derived serializer adds a fixed transform only above version 0x3B27:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited component feature | direct existing component-feature body | always | Component-reference fields, signed feature ID and u32 feature time; no tagged base-object insertion. | B |
| 2 | reference geometry +F8 | tagged moRefGeom_c | always in scope | Null or normal tagged target with shared object-map identity. Native load checks subtype and fixes runtime ownership. | B |
| 3 | transform +100 | fixed 30 u32 | version >0x3B27 | Exact sldmgu archive mgXform operator; preserve all words. Special kinds selected by word 28 &0xF0 remain explicit stops. | B / ? |

The derived native read call binds IAT 0x4CD3C4B0 to sldmgu.dll `operator>>(su_CArchive&,mgXform_c&)`, as for the fixed archive-transform grammar documented earlier. This is not the variable mgXform_c::restore grammar. The store path has the matching fixed operator. Geometry ownership fixups and archive condition 0x200000's nullable store choice add no separate wire field: row 2 is always a tagged occurrence in this modern branch.

The complete empty body with null/cached pointers is 76 bytes at 0x27CC, 88 at 0x3B27, 208 at 0x3B28, and 212 from 0x3EF6, reflecting inherited component-reference gates. New targets contribute normal tagged bodies/map slots. The class is installed only in the copied modern feature reader map and admitted to the wrapper's component-reference context from native ancestry.

Three focused tests cover every inherited component-reference gate and the derived transform boundary, signed ID/u32 time limits, cached geometry and wrapper identity, arbitrary retained transform words, special-kind rejection, every truncated latest empty-body prefix, scoped-context rejection, exact following-byte/self-map alignment, and source-hash-bound native containing-root span/ID/time/geometry/transform equality.

The native corpus adds **one component coordinate-system reference**, with feature ID 86, feature time 1584719846 and null reference geometry; its fixed-transform words are retained without reinterpretation. This completes the third move/copy feature/payload and its containing feature list in samples/Simulation Examples/Verification/nafems_3dnlg1.sldprt. Across 906 complete modern contexts, the frontier has **24,306 complete roots**, **766 complete lists** and **140 explicit stops**. **693 lists reach exact EOF**, spanning **46,191,350 decoded bytes**; **73 complete lists retain four-byte unknown tails**. Relative to Edition 0.224 this adds one root and one exact-EOF list / 31,696 bytes. Completed-list endpoint/tail invariants pass. `resolved-feature-list-frontier-0.224.json` preserves the preceding checkpoint. These diagnostic counts add no whole-document coverage credit; part, assembly, drawing and all-installed-project bit coverage remain unfinished.

Evidence: C-247.

