### 11.89 Atom partition/maps, late fields and body-feature suffix

*Status: Native serializer evidence; nine complete atom/body-base boundaries verified; enclosing extrusion suffix incomplete · Evidence: [C-104]*

In the scoped ordinary zero-condition archive, the continuation after 11.88 begins with a nullable direct array at +0x548, then i32 partition presence. A nonzero presence adds a tagged moPMarkRecord_c pointer and three i32 values at +0x3B8, +0x3C0 and +0x578. The mark record's body is one i32 permanent-mark ID. Runtime pointers and cached kernel objects in that class add no archive fields. Readers SHALL preserve the signed ID and raw presence value.

Next is i32 word_5C8 and three maps in order. Map +0x670 stores i32 count and count pairs of u32 key/u32 value. Map +0x5D0 stores i32 count and count pairs of tagged moCompEdge_c pointer/u32 value. Map +0x3F8 stores i32 count and count pairs of u32 key/u32 value. Readers SHALL preserve duplicates, pointer identity, unsigned high bits and original entry order independently of native hash-map insertion.

The late atom fields are i32 word_488, i32 appearance_count and appearance_count pairs of i32 key/tagged moVisualProperties_c pointer, then i32 direct-data presence at +0x6A0. Nonzero introduces an additional direct serializer that remains unsupported; readers SHALL stop explicitly there. Zero is followed by i32 face-name count and pairs of tagged moCompFace_c pointer/CString, then i32 index-name count and pairs of i32 key/CString, and final i32 word_6B4. All these fields are present in the documented modern scope. Legacy branches and nonzero archive masks may differ.

This completes the atom-feature helper. The ordinary moBodyFeature_c continuation is tagged moAsmFeatData_c pointer, tagged moCompFeature_c pointer, direct embedded suObArray at +0x778 and i32 word_798. The component-feature pointer is omitted under archive condition 1, outside this zero-condition path. These bytes precede the boss/extrusion-specific moEndSpec_c suffix in 11.81. Unsupported nested objects still stop the containing reader.

The dedicated map, late and body-tail audits each reach nine native boundaries. All nine cases store partition records; the three maps contain zero entries. The optional late direct-data body is absent in these cases. Twelve profile objects complete during traversal, while three other enclosing cases still stop on cosmetic-thread objects. These results verify atom and body-feature base framing, not complete boss/extrusion objects or streams. Default complete manager count remains 892; whole-document bit coverage is unchanged.

### 11.90 Extrusion end, direction and starting-end specifications

*Status: Native serializer evidence; scoped complete extrusion audit; whole streams incomplete · Evidence: [C-105]*

The ordinary modern moBoss_c/moExtrusion_c body continues after 11.89 with a tagged moEndSpec_c pointer, f64 value_830 and i32 word_808. The scoped reader supports the ordinary external-XML archive context and versions above 0x27CB. Unsupported inherited or nested records SHALL stop the containing reader. Completion requires every inherited stage, not merely this suffix.

The end specification stores, in order: tagged direction pointer; three i32 words (08,88,8C); tagged face pointer; two i32 side types (0C,10); and two tagged display-dimension pointers (18,20). For each side in order, type 3 adds a tagged point-reference pointer (68 or 70), followed by an unconditional u8 array-presence value and, when nonzero, a direct suObArray (28 or 48). These presence values SHALL be read as single bytes.

Next are f64 value_A0; six i32 words (B8,BC,C0,C4,B0,B4); two tagged display dimensions (C8,D0); four i32 words (A8,AC,D8,DC); two direct suObArrays (E0,100); and three i32 words (128,12C,130). Side type 7 adds its tagged reference-wrapper pointer (78 then 80). A tagged starting-end specification follows, then four i32 words (148,14C,150,154). Version >=0x3B01 adds i32 word_158. Historical branches and alternate native global face-loading modes are outside this scoped layout.

The direction specification stores an i32-presence nullable direct object array, i32 word_10 and u8 primary-vector presence. Only when the primary vector exists does it store that three-double vector, u8 secondary-vector presence with optional vector, and u8 point presence with optional three-double point. Final i32 word_14 is unconditional. Readers SHALL retain the nested presence structure.

The starting-end specification begins with i32 stored type. Types 1/2 add a tagged reference wrapper. Types 3/5 add a tagged display dimension and i32 word_20; type 4 instead adds f64 value_18 and i32 word_20. Other types add no bytes in this modern path. Native normalization of stored types 4/5 to runtime type 3 SHALL NOT replace the stored discriminator in decoded evidence.

The suffix-only audit reaches all nine previously supported body boundaries. The complete-extrusion audit completes 11 extrusion objects and 14 profiles, allowing 894 of 906 relation-manager records to return. Twelve configurations retain explicit revolution, fillet, cosmetic-thread or present-kernel-body context stops. These counts belong to the opt-in audit; they neither register a general extrusion reader nor establish complete configuration streams. Whole-document bit coverage is unchanged.

### 11.153 Ordinary modern extruded reference surfaces and ordered-face arrays

*Status: B — native-derived feature/helper framing; populated helper groups and raw meanings remain partial.*

moExtruRefSurface_c primary table `4CF39A38`, locator `4D32E448`, selects serializer `4BAE03A0` in sldmodu.dll. Runtime descriptor `4DAD25C0` selects factory `4BADFAC0`, which calls body-feature constructor `4B786AF0` and inlines the derived initialization. The constructor establishes direct suObArray +820. Allocation size 0x848 is not serialized length.

For ordinary non-DB external-XML model loading with v > 0x27CB, read the complete body-feature base, then the following suffix. This class does not use the reference-geometry/planar-surface inherited layout of 11.139.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | end specification +800 | Tagged moEndSpec_c pointer | Always in modern scope | Existing end/direction/from bodies apply; required nonnull for native dimension-owner processing | B |
| 2 | raw +810 | i32 | Always in modern scope | Preserve complete signed value | B |
| 3 | ordered faces +808 | Tagged moOrderedFaceArray_c pointer | Always in modern scope | Null, shared reference or new helper object | B |
| 4 | raw +814, +818 | Two i32 | Always in modern scope | Preserve in this order; meanings unresolved | B |
| 5 | array +820 | Direct suObArray body | Always in modern scope | ReadCount followed by tagged object pointers, as in 11.149 | B |

The suffix is at least **18 bytes** when pointers use short reference/null tags and the direct array has a short zero count. Newly serialized pointer bodies add their normal class/object grammar. There is no direct end-specification body, extrusion scalar tail, reference-surface kernel sheet list or i32 array-presence word inserted here. The scoped reader installs the existing end-specification readers and rejects a null required end specification. Unsupported nested bodies remain explicit stops.

After loading +800, native virtual +0x1EB8 selects model-feature post-load `4BB3F600`, whose callbacks +0x1D30 and +0x1EC0 select `4BB3F630` (dimension-owner reset) and `4B79D4F0` (propagated component resolution). These callbacks take no archive parameter and add no fields. Subsequent safeSetFeatureOwner calls similarly repair dimension owners without changing wire layout; offline decoding preserves serialized objects and identity.

Historical loads before v 0x11A8 use an extruded-surface proxy and transfer its reconstructed data, a distinct unsupported layout. The later direct branch retains original gates +810/+808 at v >= 0x116C, +814 at v >= 0x118D, +818 at v >= 0x11BD and +820 at v >= 0x11C1; every gate is satisfied in the supported modern scope. Constructor-only +840 proxy state is not appended wire data.

moOrderedFaceArray_c primary table `4D0CCF78`, locator `4D34D3A0`, selects serializer `4BB2D9E0`. Constructor `4C04EF70` establishes direct object arrays and two unsigned-integer arrays. In modern scope its body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | array +08 | Direct suObArray body | Always | ReadCount and tagged items | B |
| 2 | first group count | i32 | Always in modern scope | Nonnegative count shared by fields 3–4, not ReadCount | B |
| 3 | raw words +48 | count × u32 | Always | Read all words before any field 4 array | B |
| 4 | arrays +28 | count direct suObArray bodies | Always | Each body has its own ReadCount/items; no MFC class headers or map entries for these direct arrays | B |
| 5 | array +68 | Direct suObArray body | Always in modern scope | Independent object-array count | B |
| 6 | second group count | i32 | Always in modern scope | Independent of first group; nonnegative | B |
| 7 | raw words +A8 | count × u32 | Always | Read all words before field 8 arrays | B |
| 8 | arrays +88 | count direct suObArray bodies | Always | One body for each second-group word | B |

The modern helper's empty body is **12 bytes**. Group counts SHALL fit the remaining minimum six bytes per word/empty-array pair. Integer words retain all bits and receive no semantic coverage credit. Words and array bodies are **not interleaved**. Nested arrays preserve item order, nulls and shared references. The historical load gate after field 1 is v > 0x119C; this is always true in modern scope. Standalone tagged helpers still receive normal class/object-map entries even though their internal arrays are direct.

The dependency moExtruRefSurfIdKeeper_c has primary table `4CF3E270`, locator `4D32E998`, selecting import thunk `4CBA2733`. Its full bounded import table resolves to swccu.dll su_CObject::Serialize at `31D3F030`. That routine tests archive mode byte +20 against flag 0x10; when clear, it branches directly to return cleanup. Consequently the keeper has **zero body bytes in the selected ordinary binary archive context**. Its tagged class declaration/reference and object-map insertion still exist. Archive-XML mode flag 0x10 has a metadata path and is not covered by this empty-body rule; external model XML storage does not itself imply that archive mode flag.

The frontier contains **25 fresh extruded reference surfaces**, all with moEndSpec_c pointers, raw words (0,0,0) and empty +820 arrays. Twenty-four ordered-face pointers are null; one points to a fresh helper with empty arrays and zero group counts. Populated groups, unsigned extrema, unequal nested-array sizes, repeated reference identity, raw signed extrema and direct-array count escapes are synthetic tests. A hash-bound native regression verifies a complete root containing both feature and ordered-face helper. Tests also verify new tagged helper/keeper map behavior, required-end rejection, truncation, count bounds and unsupported contexts.

Edition 0.164 records **20,733 complete roots**, **492 list-complete configurations** and **414 explicit stops** across all **906 complete modern model contexts**. **453 lists reach exact EOF**, spanning **20,512,540 decoded bytes**; **39 complete lists retain four-byte unknown tails**. Leading stops are sheet-metal features (21), sweeps (17), face/point reference-plane data (16), blend cuts (15), embedded reference bodies (14) and component reference axes (14). Full part, assembly, drawing and installed-project bit coverage remains incomplete. Framed roots do not establish all field meanings or add whole-document coverage credit.

Evidence: C-181.

