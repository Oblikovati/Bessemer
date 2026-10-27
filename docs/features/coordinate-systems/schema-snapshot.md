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


### 11.100 Reference-plane features and face-derived plane data

*Status: Native serializer/dispatch evidence; three complete reference-plane bodies; all 906 supported modern manager prefixes complete · Evidence: [C-115]*

The scoped ordinary external-XML reader supports moRefPlane_c in versions greater than 0x27CB. Its primary RTTI table is 0x4D170958; slot 5 selects serializer 0x4C283C60. Reference-geometry helper 0x4C283A00 calls atom-feature serializer 0x4BB1EC00. The inherited body is the model-feature header, direct arrays 360/380, optional ID keeper, atom references, choosers, maps and late fields described above. It does not contain the derived body-feature suffix (assembly-feature data, component feature and array 778). Readers SHALL preserve this inheritance distinction.

The reference-geometry suffix is mgPoint_c root point (+6B8), u32 flags (+6D4), i32 word (+6D8), tagged moFeatureRefWrapper_c pointer (+6E0), and, for v > 0x2F85, i32 word (+6E8). The modern minimum exceeds the native gates for the first scalar and feature reference. Retain the stored flag word: native loading strips bit 2 while mapping the remaining state, then conditionally reapplies that bit using an environment flag. Runtime normalization does not replace the serialized value. The feature then reads one tagged moRefPlnData_c-compatible pointer (+6F0). Subsequent owner assignment and feature-update callbacks consume no archive bytes.

Base moRefPlnData_c serializer 0x4C283D30 has no su_CObject body bytes and dispatches through vtable offset 0x150. Both base table 0x4D175710 and face-derived table 0x4D139400 select serializePlaneData at 0x4C2880A0. The modern shared body is:

| Order | Stored type | Native member / purpose |
|---|---|---|
| 1 | mgPoint_c, mgVector_c | Plane root point and normal |
| 2 | mgXform_c | Plane transform, using the existing variable transform grammar |
| 3 | f64[4] | Border extents; preserve all four raw doubles |
| 4 | tagged pointer | moCompEdge_c at +48 |
| 5 | i32[3], u8 | Words +50/+54/+58, then a discarded legacy byte |
| 6 | i32[2], u16 | Words +5C/+60 and raw flag +64 |
| 7 | tagged pointer, if v > 0x2B83 | moCompFaceMeshFacetFin_c at +68 |
| 8 | two tagged pointers, if v > 0x2BA5 | moRefAxis_c at +70, moCompSketchEntHandle_c at +78 |

The five unnamed i32 words, legacy u8 and raw u16 flag remain 23 unknown bytes per shared data body. Native code normalizes the u16 to a boolean; the reader retains all 16 stored bits. Native default-plane subclasses insert a display root point after the legacy byte; they are not aliased to this base grammar. Other derived classes require their own serializer evidence. Older layouts and database-stream archives remain outside this scoped reader.

moFaceRefPlnData_c serializer 0x4C27DF40 emits shared plane data, then: direct moPlaneRef_w body (+90), tagged moDisplayDim_c pointer (+98), i32 words +A0/+A4, i32 presence for an optional direct moPlaneDataRef_w body (+A8), and i32 word +B4. All scalar gates in this suffix precede the modern minimum. Direct bodies SHALL NOT read an MFC tag or allocate an object-map entry. Nonzero optional presence is retained as stored, including negative values.

The +90 direct body's constructor 0x4C270F20 selects moPlaneRef_w. Serializer 0x4C281720 emits the existing reference wrapper and four surface doubles, then mgPoint_c root point, mgVector_c normal, mgXform_c transform and mgPoint_c display root point. Optional +A8 selects moPlaneDataRef_w: serializer 0x4C63D5A0 calls 0x4C27B490, emitting reference wrapper, two mgXform_c values (+A8/+130), then mgPoint_c (+1B8) and two mgVector_c values (+1D0/+1E8). Helpers 0x4C2866C0 and 0x4C2842D0 establish shared surface/reference framing. Unsupported nested classes still stop.

The `--reference-planes` audit completes 906/906 supported modern part relation-manager prefixes without errors or partial boundaries. Three face-derived feature bodies are each 850 bytes, with 356 unknown bytes including inherited fields. Exact spans are [72474,73324) in Sustainability/base plate, [72387,73237) in advdrawings/base plate and [72507,73357) in assemblyvisualize/base plate, all Contents/Config-0. All three have a null display dimension and zero optional plane-data-reference presence; the populated optional body has native and synthetic evidence only. The report retains full stream hashes and source paths.

The extended audit completes 31 profiles, 18 extrusions, five ordinary revolutions, six revolution cuts, two ribs, three linear patterns, three cosmetic threads, 13 fillets and three reference planes, retaining four decoded kernel frames. These results establish manager prefixes; they do not establish remaining configuration bytes, legacy configurations, assembly/drawing archives or installed project payloads. Whole-document bit coverage remains incomplete and its aggregate metric is unchanged while these readers remain audit-scoped.


### 11.29 Sketch point prefix and supporting references

*Status: Native-derived modern nonminimal-owner layout; installed point collections remain incomplete · Evidence: [C-44], F.32*

This clause applies to ordinary external-XML part archives with v > 0x27CB, a nonminimal sketch owner and neither archive condition 2 nor 4. It specifies a prefix, not the complete sgSketch body. Readers SHALL preserve the inherited object map and SHALL stop on an unsupported reference body, minimal-owner context or nonzero jet presence. Direct points do not introduce an MFC class tag or object-map entry.

The sketch begins with an i32 point count. Each entry is a u16 category followed by the direct point body below. Native category 1 selects an external-point prototype, 2 a frame-point prototype and other values the ordinary prototype; the same archive routine restores each. The next geometry collection is outside this clause.

| Order | Direct entity fields | Encoding |
|---|---|---|
| 1 | permanent mark; unknown_0c | i32; i32 |
| 2 | visual properties | direct moEntVisProp_c (11.12), nonminimal owner only |
| 3 | reference count; stored minimal flag | i32; u16 (0 in this scope) |
| 4 | color; constraint status; line type; width | three u16; direct utLineWidth_c restore type 2 |
| 5 | external entity | sgExtEnt_c pointer |
| 6 | flags_54; flags_56; unknown_50 | u16; u16; i32 |
| 7 | group; description presence; optional description | moGroup pointer; u8; CString when nonzero |
| 8 | unknown_68; unknown_74; unknown_78 | f64; u32; u32 |

The point appends a u16 mask, position, u16 point type and u16 geometric-reference count. Position contains two f64 values when mask bit 0x10 is set, otherwise three. Mask bit 0x20 denotes the unsupported minimal-owner branch. In the supported branch two sgEntHandle pointers follow, then u16 center-reference count, i32 unknown_108 and two further positions using the same coordinate width. A u16 jet-presence word follows; zero is followed by the u16 bidirectional external-reference count. Nonzero jet bodies remain unspecified. Other mask meanings are partial and all stored bits SHALL be retained.

The modern sgEntHandle body shared by sgLineHandle and sgArcHandle is u16 entity index, an i32 replacement index when the u16 equals 0x777F, then i32 reference ID and i32 DimOnCM. sgExtEnt_c contains four pointers (moRefWrapper_c, moDrView, moSkIntersectInfo_c, moSkIsoCurveInfo_c), then reference-axis CString in this scope.

moSketchExtRef_w contains the 11.16 wrapper followed by an i16 mode (-2, -1 or 0) and moBackedUpData_c pointer. moTempAxisRef_w contains that wrapper, two f64 values, a direct three-f64 point, three-f64 vector and u8 unknown_f0. For v > 0x371D it appends two three-f64 points and i32 unknown_128. These wrappers retain the existing component-reference restrictions; further component/feature classes are not implied supported.

