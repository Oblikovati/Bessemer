# Sensor and chamfer schema snapshot

### 11.138 Modern sensor features and inline sensor data

*Status: B — ordinary modern framing, including populated point/reference vectors; raw values and enum meanings remain partial.*

moSensor_c primary table `4D2CDFD0` selects serializer `4CAC38C0`. The constructor calls moModelFeature_c, constructs inline moSensorData_c at +0x360 and initializes three pointer vectors at +0x558, +0x570 and +0x588. In ordinary non-DB external-XML loading with v > 0x27CB, the wire order is the common model-feature body, the **direct** sensor-data body, then the three vectors. Sensor data has no MFC object tag or map slot. Each vector has a fixed u32 count followed by that many tagged pointers: moRefWrapper_c for the first two and moDispObjRef_c for the third. These counts are not ReadCount. Readers SHALL reject counts exceeding the available minimum pointer-record space and preserve order, nulls, duplicates and shared-map references.

Native code reads all three vectors. When the loaded data member +0x08 equals 2, it disposes of the second vector after reading it; that runtime action removes no wire bytes. The reader retains the serialized references. No physical sensor type is inferred from this numeric value.

The inline data serializer is `4CAC74C0`. The following table gives file order using offsets relative to the **sensor-data** object, not the outer feature. Unless stated otherwise, fields are present throughout the supported modern scope. Every `raw` value remains semantically unresolved, including values that control later presence. Separate names SHALL retain both serialized occurrences of member +0x88.

| # | Native member offsets, hex | Wire type | Presence or rule | St. |
|---|---|---|---|---|
| 1 | 08, 0C, 10, 14, 18, 1C, 20, 24, 34, 38, 3C, 40 | i32[12] | Preserve values; 0C selects field 25 | B |
| 2 | 48, 50 | f64[2] | Always | B |
| 3 | 78, 8C | i32[2] | Always | B |
| 4 | 90 | f32 | Always | B |
| 5 | 98 | CString | Always | B |
| 6 | A0, A4, A8, AC | i32[4] | Always | B |
| 7 | B0, B8 | f32[2] or f64[2] | f32 below v 0x3765; f64 from that version | B |
| 8 | C0 | CString | Always | B |
| 9 | 60 vector | u32 count; direct sensor points | Fixed count, then count times 64 bytes; no object tags | B |
| 10 | F4, F8, FC, 100 | i32[4] | Always | B |
| 11 | D0, D8, E0, E8 | CString[4] | Always | B |
| 12 | F0, 104, 108 | i32[3] | Always | B |
| 13 | 110 | f64 | Always | B |
| 14 | 28, 2C, 30 | i32[3] | Always | B |
| 15 | 80 | CString | Always | B |
| 16 | 88 | i32 | First serialized occurrence | B |
| 17 | 15C, 160, 164, 168 | i32[4] | Always | B |
| 18 | 88 | i32 | Second serialized occurrence; retain independently | B |
| 19 | 180, C8, 1B0 | CString[3] | Always | B |
| 20 | 190 | f64 | Always | B |
| 21 | 158 | i32 | Always | B |
| 22 | 198; 1A0; 1A8 | f64; i32; f64 | Always | B |
| 23 | 188 | CString | Always | B |
| 24 | 1B8, 58, 1BC, 1C0 | i32[4] | Always | B |
| 25 | 1C8; 1D0; 1D8 vector | CString; i32 count; CString items | Only when raw member 0C equals 0x47; positive count gives that many items, nonpositive count gives none | B |
| 26 | 1F0 | i32 | v > 0x27E3 | B |
| 27 | 7C | i32 | v >= 0x3B16 | B |

The modern direct moSensorPoint_c body (`4CAC8360`) is exactly 64 bytes: i32 members **00, 04, 08**; six f32 members **0C, 10, 14, 18, 1C, 20**; then seven i32 members **24, 28, 2C, 34, 30, 38, 3C**. The +34/+30 order is deliberate. These values do not receive coordinate, unit or identifier semantics from their widths alone. Every historical point-field gate is satisfied in this modern scope.

For data selector 0x47, the load path reads the string-group header even when its count is nonpositive. The current store path instead tests that its existing count is positive before writing the group. This asymmetry is retained as a native limitation; readers follow the load grammar, and no writer round-trip guarantee is made for such states. Data +1F0 and +7C have distinct version gates, and the B0/B8 precision change does not change the six f32 fields inside a sensor point.

The installed audit retains **101 freshly serialized sensors** inside completed roots. Seven contain one direct point each; 94 contain none. Second-vector counts include 1, 2, 4 and 6; all observed first and third vectors are empty. Data +08 is 0 or 1; +0C values are 1, 2, 3, 15, 62 and 9999, so no populated native 0x47 string-group fixture is claimed. Synthetic tests cover that branch with positive, zero and negative counts, duplicate inherited references, a populated third vector, both copies of +88, point-member order, precision/version boundaries and truncation/count rejection.

The edition 0.149 frontier has **18,522 complete roots**, **181 list-complete configurations** and **725 explicit stops**, with all **906 modern model contexts** complete. Of those lists, **178 reach exact EOF**, spanning **5,327,298 decoded bytes**; three still retain four unknown tail bytes. No sensor-class stop remains. The leading next dependencies are moPlanarSurface_c (54), moICE_c (44), Chamfer_c (43), moAttribute_c (41) and moContextChainComponent_c (40). These layout results are not full-document semantic bit coverage and do not close the outstanding part, assembly, drawing or installed-project requirements.

Evidence: C-166.

### 11.142 Modern chamfer feature suffix

*Status: B — ordinary modern load framing with populated native keepers/arrays; byte/word meanings remain partial.*

Chamfer_c is implemented in sldMoPartu.dll. Primary RTTI table `180464348`, locator `1804ED798`, selects serializer `18023EB00`. Its +0x4B8/+0x4C0 hooks import the common moConcept_c lightweight/resolved serializers from sldmodu.dll. Runtime descriptor `180604170` selects factory `18023E230`, whose constructor call selects `18023E280`; the descriptor's 0x8F0 memory size is not wire size. The constructor calls moBodyFeature_c, allocates a moFilletSIdKeeper_c at +0x878 and constructs a su_CDWordArray at +0x8C0.

This clause applies to ordinary non-DB external-XML loading with condition 1 clear and v > 0x27CB. Every historical field gate in the following suffix is satisfied in that scope. Native enum/byte values are retained without guessing a chamfer type, unit or boolean meaning.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | body-feature base | Existing complete inherited body | Always | Model-feature header, direct arrays, optional ID keeper, atom stages and body suffix; excludes extrusion/revolution/rib suffixes | B |
| 2 | surface-ID keeper +878 | Direct moFilletSIdKeeper_c body | Always in scope | Existing 11.98 keeper grammar, no class tag or wrapper slot | B |
| 3 | lists +880, +888, +890 | Three tagged suObList pointers | Always on load | Shared map, including nulls and repeated references | B |
| 4 | dimensions +808, +810, +818 | Tagged moDisplayDim_c, moDisplayAngularDim_c, moDisplayDistanceDim_c pointers | Always in scope | Precisely this order | B |
| 5 | raw +858, +898, +870 | u8[3] | Always in scope | Preserve bytes, including noncanonical values | B |
| 6 | references +8A0, +8A8, +8B0 | Tagged edge, face, face pointers | Always in scope | moCompEdge_c, moCompFace_c, moCompFace_c | B |
| 7 | raw +8B8 | i32 | Always in scope | Meaning unresolved | B |
| 8 | array +8C0 | Direct su_CDWordArray | Always in scope | ReadCount then u32[count]; values remain raw | B |
| 9 | dimensions +820, +828, +830 | Three tagged moDisplayDistanceDim_c pointers | Always in scope | Shared map | B |
| 10 | references +838, +840, +848 | Three tagged moCompEdge_c pointers | Always in scope | Shared map | B |
| 11 | reference +850 | Tagged moCompVertex_c pointer | Always in scope | Existing component-boundary reference grammar | B |
| 12 | raw +874 | u8 | Always in scope | Meaning unresolved | B |
| 13 | reference presence +800 | i32 | Always in scope | Any nonzero value introduces field 14 | B |
| 14 | reference +800 | Tagged moRefWrapper_c pointer | Field 13 nonzero | No additional pointer-presence encoding | B |
| 15 | raw +8E8 | u8 | Always in scope | Last serialized field; original gate v > 0x2396 | B |

The direct keeper uses primary moFilletSIdKeeper_c table `4D1FB650` and serializer `4BBBD030`: fixed i32 surface-ID count, that many direct moEntityId_c bodies, then a direct suObArray reference body. Each entity ID is its existing tagged representation pointer. The reference array uses ReadCount and pointers; neither the keeper nor this array adds an outer map slot. The DWORD array at field 8 likewise has no tag or presence word. All counts SHALL fit their remaining minimum record sizes. Readers preserve duplicate inherited references and raw high-bit DWORD values.

The chamfer load path reads all three field-3 pointers unconditionally. Store-side calls to moRefHistHelperFeatData_c::iSerializeEdges/iSerializeFaces can omit lists in other histories/contexts; they do not authorize omitting bytes from this ordinary load grammar. The direct keeper is also conditional in special condition-1 contexts, which are outside scope. After field 15, virtual +0x1EB8 selects post-load hook `180248820`; it invokes model-feature post-load handling and repairs the angular dimension's owner. This is runtime state handling, not another serialized field.

The scoped reader composes the complete body-feature base with this suffix. Explicit research probes remain preserved by default, and unsupported nested classes retain stops. Known field boundaries do not supply meanings for the five raw bytes, raw +8B8 or array +8C0 values; those values are not credited as semantically decoded bits.

The installed audit retains **57 freshly serialized chamfers** in completed roots. Their keeper surface-ID counts range from 2 through 16, with populated reference arrays from 1 through 15 entries. Fifty-five chamfers have six DWORD values and two have empty arrays. All observed +800 presence words are zero, and the raw byte tuple (+858,+898,+870,+874,+8E8) is (0,10,1,0,0). Tests include a hash-bound native chamfer with populated keeper/array, plus a synthetic payload distinguishing extended ReadCount from tagged pointers, retaining duplicate identity, noncanonical byte values, high-bit DWORDs and a negative nonzero presence word. Zero-length direct collections, omitted optional reference, truncation and context rejection are also checked. No native positive +800-presence fixture is claimed.

The edition 0.153 frontier records **18,981 complete roots** and **292 list-complete configurations**, with **614 explicit stops** and all **906 modern model contexts** complete. **286 lists reach exact EOF**, spanning **9,112,488 decoded bytes**; six still have four-byte unknown tails. No chamfer stop remains at this frontier. Leading dependencies are context-chain components (41), mirror patterns (39), moPLine_c (36), moOneContour_c (33), shells (29) and reference axes (29). Full part, assembly, drawing and installed-project bit coverage remains incomplete.

Evidence: C-170.

