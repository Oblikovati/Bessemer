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


### 11.171 Structural members and profile and plane dependencies

*Status: B — native field order, inheritance, direct members and populated structural-member boundaries verified; raw meanings and additional contexts remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moWeldMemberFeat_c primary table `0x4D20F860` selects `0x4BB43DB0`, which calls body-feature serializer `0x4BB20FA0` before this suffix. Counts below are signed i32, not packed ReadCount, except the explicitly direct suObArray. Reject negative or out-of-buffer counts. Native member offsets identify fields, not stream positions.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | profile +820; plane +828 | tagged moProfileRef_w; tagged moPlaneRef_w | always | Ordinary null/new/cached pointer framing. | V |
| 2 | raw +910/+914; curve collection +890 | two i32; i32 count and tagged moCurveRef_w pointers | always | Preserve order and pointer identity. | V / ? |
| 3 | point collection +830 | i32 count; repeated pointer and two i32 | always | Each record is tagged moPointRef_w, raw +8F0 and raw +8D0. Both metadata values occupy four bytes on wire despite native storage in word arrays. | V / ? |
| 4 | object +918; raw +920; string +928 | tagged object; f64; CString | always | No inferred type for the generic object. | V / ? |
| 5 | point +950; custom properties +990 | tagged moPointRef_w; tagged moCustomPropertyList | always | The property list is tagged here, unlike the direct list in 11.168. | V |
| 6 | raw +938/+93C/+940/+944/+948; curves +8B0 | five i32; i32 count and tagged curve pointers | always | Preserve complete signed words and repeated curve references. | V / ? |
| 7 | raw +930; raw +9C8; contours +9C0 | f64; i32; conditional tagged moWeldmentContours_c | pointer only when +9C8 > 0 | Negative/zero raw values omit the pointer; do not replace the positive comparison with nonzero. | V / ? |
| 8 | array +9F0 | direct suObArray body | always | Packed ReadCount and tagged items; no outer class/pointer tag or map slot. | V |
| 9 | strings +960/+968; raw +958; string +970 | two CStrings; i32; CString | always | Exact non-offset wire order. | V / ? |
| 10 | raw +A14/+A18/+978; string +980; raw +988 | three i32; CString; i32 | always | Preserve signed values and original strings. | V / ? |
| 11 | advanced properties +9A8 | tagged moAdvCusPropList_c | always | No extra presence word. | V |

All selected load gates precede the modern minimum, including the advanced-property pointer after `0x27B2`. The empty/null suffix is **99 bytes**, or **350 bytes** including the latest empty body-feature base. Native store suppresses the point collection for its +9C8 mode 1; native load still reads the earlier explicit point count. Offline reading preserves the serialized count and both full i32 point metadata values before native truncation to u16. It does not infer earlier lengths from later modes or execute runtime cleanup.

Runtime descriptor `0x4DAD4568` selects factory `0x4BB41D30`, which allocates `0xA20` bytes and invokes constructor `0x4C75E0E0`. That constructor installs suObArray's vtable at +9F0, proving the direct virtual dispatch type. moWeldSegSkIndices_c primary `0x4D117140` selects `0x4BB44920`, which delegates to suObArray without another suffix. moAdvCusPropList_c primary `0x4CDD3448` selects `0x4B2C0160`, which delegates to suObList without additional ordinary payload fields. Both enclosing classes remain tagged when reached through pointers, while their delegated array/list bodies use packed counts. New unsupported element types stop explicitly.

moWeldmentContours_c primary `0x4D1172F0` selects `0x4BB45250`: i32 contour count, that many tagged moWeldSingleContour_c pointers, then f64 +F0 and f64 +F8. Both doubles begin after `0xF5D`, before this modern scope. The minimum body is **20 bytes**. The container does not establish the single-contour payload, which remains a dependency in native structural-member records.

moProfileRef_w primary `0x4D17C858` selects `0x4C2821F0`: common moRefWrapper_c helper `0x4C2842D0`, then one raw u32 at +A8. Its historical gate after `0x854` is satisfied throughout this scope. The latest null/optional-empty wrapper plus suffix occupies **66 bytes**. No extra profile pointer, presence flag, string or outer array is appended. Complete-root summaries retain **10 native profile references**, all wrapping moCompProfile_c, without assigning a semantic meaning to +A8.

The structural-member plane reference reaches moLinePtRefPlnData_c and moPerLinePtRefPlnData_c. Both inherit **moLinePtBaseData_c** and moRefPlnData_c; the perpendicular class does not inherit moLinePtRefPlnData_c. Their +150 virtual slots select common plane-data helper `0x4C2880A0`, establishing the existing non-default data layout of 11.100, without the default-plane display-point insertion. Their wire layouts are:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common plane data | direct existing plane-data body | both classes | Includes version-gated facet/axis/sketch references from 11.100. No new class tag for this helper. | V |
| 2 | point +88; curve +90 | tagged moPointRef_w; tagged moCurveRef_w | both classes | Modern native load rejects a null curve; offline reading does likewise. Point may be null. | V |
| 3 | raw +98/+9C | two i32 | both classes | Original signed values; gates after 0x10D4 precede the modern minimum. | V / ? |
| 4 | raw +A0; raw +A4 | u16; i32 | perpendicular class only | Exact two-byte/four-byte widths; gates 0x13B and 0x982 precede scope. | V / ? |

moLinePtRefPlnData_c primary `0x4D139B90` selects shared helper `0x4C27F230`. moPerLinePtRefPlnData_c primary `0x4CD9D1F0` selects `0x4C281290`, which calls that helper before its six-byte suffix. Both preserve stored pointers and scalar values without historical conversion or geometry reconstruction. Their latest minimum bodies are **157 bytes** and **163 bytes**, including a nonnull cached curve pointer; ordinary earlier modern versions omit the known six version-gated common-data pointer bytes. Complete-root summaries retain **six line/point** and **five perpendicular line/point** plane-data objects.

Edition 0.182 recorded **22,324 complete roots**, **661 list-complete configurations** and **245 explicit stops** across **906 complete modern contexts**. **606 lists reached exact EOF**, spanning **32,353,872 decoded bytes**; **55 complete lists retained four-byte unknown tails**. At that edition, no complete native moWeldMemberFeat_c root was credited because contour/index payloads still stopped. Edition 0.183 resolves the layouts in 11.172 and verifies 11 complete native structural-member records. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; these record boundaries receive no whole-document coverage credit.

Evidence: C-199.


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

