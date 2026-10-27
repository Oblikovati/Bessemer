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


### 11.172 Structural contour indices, orientation and segments

*Status: B — native modern layouts, constructor member types and populated containing-root boundaries verified; raw engineering meanings and runtime contour behavior remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. These classes complete the nested contour/index paths introduced in 11.171. Tagged instances use ordinary MFC class/object-map entries; the explicitly direct arrays below do not allocate additional container slots. Their common ordinary binary su_CObject layer consumes zero bytes.

moWeldSegSkIndex_c primary table `0x4CF5B9E0` selects `0x4BB44880`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +08/+0C/+10 | three i32 | always | Preserve all signed words. No curve pointer, string or count is added by this leaf. | V / ? |

The body is **12 bytes**. The enclosing moWeldSegSkIndices_c container remains the packed-count suObArray-derived object of 11.171; its plural name does not imply that the leaf's three words use an additional count.

moContourOrientation_c primary table `0x4D211F98` selects `0x4BB42E10`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | curve +08; point +10; direction +18 | tagged moCurveRef_w; tagged moPointRef_w; tagged moDirectionSpec_c | always | Preserve null/new/cached identity and original occurrence order. | V |
| 2 | raw +20/+24; raw +28 | two i32; f64 | always | Preserve complete words and double. | V / ? |
| 3 | point +108 | tagged moPointRef_w | always | Independent pointer occurrence, which can repeat +10. | V |
| 4 | raw +114/+118/+11C; raw +120 | three i32; f64 | always | Exact stored order; no direct vector implied by neighboring doubles. | V / ? |
| 5 | raw +30/+110 | two i32 | always in modern scope | Gates after 0x180B and at 0x1834 precede this scope. Lower-offset +30 follows +120 on wire. | V / ? |

The null-pointer orientation body is **52 bytes**, including four two-byte tags and 44 scalar bytes. Its scoped dependency uses the existing moDirectionSpec_c reader; no different direction body is inferred from this container.

moWeldSingleContour_c primary table `0x4D117260` selects `0x4BB44DB0`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | orientation +20 | tagged moContourOrientation_c pointer | always | Preserve archive identity. | V |
| 2 | array +28 | direct suObArray body | always | Packed ReadCount and tagged items, without an outer tag/map slot. | V |
| 3 | raw +50/+58 | two f64 | always in modern scope | Both gates after 0xF5D precede scope. | V / ? |
| 4 | raw +60/+64/+6C/+68 | four i32 | always in modern scope | +6C precedes +68; historical gates after 0xF8C/0xFBB and at 0x12D3 precede scope. | V / ? |

The empty/null single-contour body is **36 bytes**. Runtime descriptor `0x4DAD4538` selects factory `0x4BB41D00`, allocating `0x70` bytes and calling constructor `0x4C161410`. The constructor installs suObArray's vtable at +28, establishing the direct virtual-call type. The runtime class inherits suPtrArray, but its serializer calls su_CObject::Serialize followed by the explicit fields above; it does not prepend serialization of that inherited pointer-array container. Runtime inheritance alone is insufficient to add wire fields.

moWeldSegment_c primary table `0x4D1171D0` selects `0x4BB44A40`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | curve +08; raw +10/+14/+18 | tagged moCurveRef_w; three i32 | always | Complete signed words after the pointer. | V / ? |
| 2 | raw +20/+28/+30/+38 | four f64 | always in modern scope | The latter three begin after 0xF5D; preserve all four. | V / ? |
| 3 | array +50 | direct suObArray body | always | Packed ReadCount and tagged items, no outer tag/map slot. | V |
| 4 | raw +40/+44/+48/+CC/+D0 | five i32 | always in modern scope | First three begin after 0xFBA; final two at 0x12D3, all before scope. | V / ? |

The empty/null segment body is **68 bytes**. Descriptor `0x4DAD4508` selects factory `0x4BB41CD0`, which calls constructor `0x4C160F80`. That constructor installs suObArray's vtable at +50. Old temporary array reads before `0xFBB` and the two discarded words between `0xF8D` and `0xFAA` are excluded; there is no such temporary payload in the modern body.

The complete-root audit now verifies **11 structural-member records**: five with raw +9C8 mode 0 and six with mode 1. Their completed roots contain **23 segment-index leaves**, **six orientation objects**, **six single contours** and **11 weld segments**. All 11 segment +50 arrays are empty in these native samples. Populated-array synthetic tests verify framing and identity without asserting native geometric validity for arbitrary references. Hash-bound native tests verify complete structural-member root spans containing both index and contour families. Raw numeric meanings, allowed value combinations and runtime contour construction remain open.

Edition 0.183 records **22,335 complete roots**, **669 list-complete configurations** and **237 explicit stops** across **906 complete modern contexts**. **614 lists reach exact EOF**, spanning **32,698,965 decoded bytes**; **55 complete lists retain four-byte unknown tails**. Atom-feature direct data, drafts, surfaces, cut lists, profile/chooser variants and other dependencies remain unresolved. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these record boundaries.

Evidence: C-200.

