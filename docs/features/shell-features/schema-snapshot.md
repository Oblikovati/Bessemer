### 11.89 Atom partition/maps, late fields and body-feature suffix

*Status: Native serializer evidence; nine complete atom/body-base boundaries verified; enclosing extrusion suffix incomplete · Evidence: [C-104]*

In the scoped ordinary zero-condition archive, the continuation after 11.88 begins with a nullable direct array at +0x548, then i32 partition presence. A nonzero presence adds a tagged moPMarkRecord_c pointer and three i32 values at +0x3B8, +0x3C0 and +0x578. The mark record's body is one i32 permanent-mark ID. Runtime pointers and cached kernel objects in that class add no archive fields. Readers SHALL preserve the signed ID and raw presence value.

Next is i32 word_5C8 and three maps in order. Map +0x670 stores i32 count and count pairs of u32 key/u32 value. Map +0x5D0 stores i32 count and count pairs of tagged moCompEdge_c pointer/u32 value. Map +0x3F8 stores i32 count and count pairs of u32 key/u32 value. Readers SHALL preserve duplicates, pointer identity, unsigned high bits and original entry order independently of native hash-map insertion.

The late atom fields are i32 word_488, i32 appearance_count and appearance_count pairs of i32 key/tagged moVisualProperties_c pointer, then i32 direct-data presence at +0x6A0. Nonzero introduces an additional direct serializer that remains unsupported; readers SHALL stop explicitly there. Zero is followed by i32 face-name count and pairs of tagged moCompFace_c pointer/CString, then i32 index-name count and pairs of i32 key/CString, and final i32 word_6B4. All these fields are present in the documented modern scope. Legacy branches and nonzero archive masks may differ.

This completes the atom-feature helper. The ordinary moBodyFeature_c continuation is tagged moAsmFeatData_c pointer, tagged moCompFeature_c pointer, direct embedded suObArray at +0x778 and i32 word_798. The component-feature pointer is omitted under archive condition 1, outside this zero-condition path. These bytes precede the boss/extrusion-specific moEndSpec_c suffix in 11.81. Unsupported nested objects still stop the containing reader.

The dedicated map, late and body-tail audits each reach nine native boundaries. All nine cases store partition records; the three maps contain zero entries. The optional late direct-data body is absent in these cases. Twelve profile objects complete during traversal, while three other enclosing cases still stop on cosmetic-thread objects. These results verify atom and body-feature base framing, not complete boss/extrusion objects or streams. Default complete manager count remains 892; whole-document bit coverage is unchanged.

### 11.92 Rib feature and silhouette-edge dependencies

*Status: Native serializer evidence; two complete ribs in one native configuration; whole streams incomplete · Evidence: [C-107]*

In the ordinary modern archive scope above 0x27CB, moRib_c uses the inherited body-feature stages in 11.81 and 11.88–11.89. Its suffix stores i32 word_834, u32 word_828, tagged display-distance dimension (838), tagged display-angular dimension (848), tagged display-radial dimension (850), tagged display-distance dimension (858), and tagged moCompSolidBody_c pointer (860). Readers SHALL read all five pointer tags, independent of word_834. The native writer uses bits 0x80, 0x600 and 0x08 to select populated versus null dimension pointers; it still emits null tags for unselected pointers. The final solid-body pointer exists from version 0x709, below this scope.

The component silhouette edge moCompSilhEdge_c first serializes the complete component-edge base from 11.17, then i32 transform presence and, when nonzero, a compact mgXform_c. Readers SHALL preserve signed presence and read its full four bytes. This tail exists from version 0x1C0A, below the modern scope. A drawing owner adds four i32 persistent entity IDs from version 0x3352; that owner-dependent branch is outside this ordinary part reader. The supported wrapper-reference whitelist now admits this component-edge subclass.

The boundary silhouette edge moSilhEdgeRef_c starts with the boundary-reference base from 11.18. It then stores three tagged moFaceRef_c pointers at offsets 188,190,198, u32 flags_1A0, and f64 values_1B0/1B8. Readers SHALL retain all raw flag bits. Native interpretation tests bits 1,2,4 in sequence to assign its runtime mode, and bit 8 for a separate Boolean; preserving only the resulting mode would lose archive information. These fields all exist in this modern scope. Legacy versions before 0x671 instead store the mode directly; values_1B0/1B8 begin at 0xDD4.

The moCompVertex_c primary RTTI serializer is exactly the existing moCompBEntRef_c serializer. Its modern body is therefore the component boundary-reference base (component reference, byte alias presence/optional CString, tagged boundary pointer, tagged external pointer), with no additional vertex suffix. This class is supported both by archive dispatch and the wrapper-reference whitelist. Its boundary pointer can itself identify a vertex-reference object with the separately documented point suffix.

The opt-in `--complete-ribs` audit also enables supported revolution and extrusion readers. Across 906 configurations, it returns 898 relation-manager records and completes 21 profiles, 11 extrusions, five revolutions and two ribs. Both ribs occur in the Sprinkler Base configuration; their raw word pairs are (258,3) and (274,2). The report retains exact source/stream spans, including nested object spans that overlap their parents. Eight configurations retain explicit fillet, cosmetic-thread or present-kernel-body framing stops. These results establish supported object boundaries and manager prefixes; whole-document bit coverage remains unchanged.

### 11.147 Ordinary modern shell-feature suffix

*Status: B — native-derived suffix framing with populated lists and edge references; raw byte and remaining nested meanings stay partial.*

moShell_c is implemented in sldMoPartu.dll. Primary table `180480E48`, locator `1804EFB00`, selects serializer `1802DB200`. Its +0x4B8/+0x4C0 hooks import the common concept serializers from sldmodu.dll. Runtime descriptor `1806052A0` selects factory `1802D97D0`, whose constructor call selects `1802DA070`. The constructor calls moBodyFeature_c, installs the shell tables and initializes its list/dimension/reference members. Memory allocation size 0x850 is not wire size.

In ordinary non-DB external-XML loading with v > 0x27CB, the wire order is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | body-feature base | Existing complete inherited body | Always | Includes atom stages and body suffix; excludes extrusion/revolution/rib suffixes | B |
| 2 | list +820 | Tagged suObList pointer | Always on load | Retain nulls, item order and shared references | B |
| 3 | dimension +810 | Tagged moDisplayDim_c pointer | Always | Actual class can be a supported derived dimension | B |
| 4 | raw +818 | u8 | Always | Preserve the full byte; no inferred boolean/type meaning | B |
| 5 | lists +828, +830 | Two tagged suObList pointers | Always in modern scope | Original gate v >= 0x163; pointers may be independently null | B |
| 6 | edge +840 | Tagged moCompEdge_c pointer | Always in modern scope | Original gate v > 0x468 | B |
| 7 | solid body +848 | Tagged moCompSolidBody_c pointer | Always in modern scope | Original gate v > 0x7DE | B |

The suffix is six tagged pointers and one byte, **13 bytes when every pointer is a short null/reference tag**. Newly serialized pointed-to objects add their normal MFC headers/bodies/map slots. No field above is a direct list body or an i32 presence/list pair. In particular, the raw byte leaves subsequent pointer tags unaligned. The constructor's internal suPtrArray at +800 and state at +808/+838 are not appended wire fields in the selected serializer.

The load path reads field 2 unconditionally. Store-side moRefHistHelperFeatData_c::iSerializeFaces filtering can omit it in other history contexts; that branch does not change this ordinary load grammar. The scoped reader consumes all six pointer positions and retains unsupported nested classes as explicit stops rather than guessing their extent.

After the suffix, virtual +0x1EB8 selects post-load callback `1802E6520`. It calls model-feature post-load handling and removes trailing items from list +828 while that list is longer than list +830, treating a null list as count zero. This is a runtime normalization, not a count/word stored after field 7. The offline reader preserves both serialized lists, their original item order and shared archive identity. It does not trim away wire evidence.

The installed frontier retains **37 freshly serialized shell features** in completed roots. All have a moDisplayDistanceDim_c pointer, raw +818 equal to 0 and populated suObList +820 bodies containing 1 through 5 items. All +828 lists are present and empty. The +830 pointer is null in 25 cases and points to an empty list in 12. Seventeen shells have moCompEdge_c +840 references; all +848 solid-body pointers are null. A native regression verifies the stream SHA-256 and exact root end for a shell with at least two list items. Synthetic tests cover six nonnull inherited pointers, noncanonical raw bytes, unequal lists preserved without native trimming, a complete feature with a newly tagged list and repeated list identity, null pointers, truncation and context/version rejection. Positive solid-body pointers are synthetic inherited references, not populated native shell-body fixtures.

The edition 0.158 frontier records **19,964 complete roots**, **389 list-complete configurations** and **517 explicit stops**, with all **906 modern model contexts** complete. **383 lists reach exact EOF**, spanning **15,556,297 decoded bytes**; six retain four-byte unknown tails. No shell-class stop remains at this frontier. Leading dependencies are circular patterns (40), moOneContour_c (33), hole-wizard features (27), scale features (23), base bodies (22), sheet-metal features (21) and extruded reference surfaces (21). Full part, assembly, drawing and installed-project bit coverage remains incomplete.

Evidence: C-175.

