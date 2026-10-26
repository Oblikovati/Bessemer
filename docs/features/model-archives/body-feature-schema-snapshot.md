# Body feature and reference-plane schema reference

Edition 0.356 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.81 Inherited boss/extrusion feature framing

*Status: Native dispatch/base-chain evidence; partial probe advances to nested profile features; whole feature bodies unsupported · Evidence: [C-96]*

Primary vtables for moExtrusion_c and moBoss_c select the same serializer. It first calls moBodyFeature_c::Serialize. Its derived suffix is a tagged moEndSpec_c pointer, f64 at runtime +0x830 for v > 0x997 and i32 at +0x808 for v > 0xDBE, all present in this scope. These suffix bytes SHALL NOT be read until the inherited body completes. The native entry also reports a diagnostic when archive condition 0x2000 is absent; this observation does not establish a replacement condition for ordinary-file readers.

The body-feature base calls an atom-feature helper, which calls moModelFeature_c::Serialize, which calls the common feature body from 11.15. For v > 0x27CB, model-feature serialization adds i32 word_354 after a helper whose archive reads apply only to legacy versions 0x393 through 0x3AE. Thus that intermediate helper consumes no bytes in the documented scope. The atom-feature prefix next directly serializes two embedded object arrays at +0x360 and +0x380, then reads u16 ID-keeper presence and, when nonzero, a tagged moIdKeeper_c pointer. A present null pointer is invalid. The array bodies use packed ReadCount and tagged items without outer array tags.

The atom-feature remainder is not yet implemented. Once it completes, the modern ordinary body-feature continuation reads a tagged moAsmFeatData_c pointer, a tagged moCompFeature_c pointer when archive condition 1 is absent, an embedded object array at +0x778 and i32 word_798. Native version gates are v > 0x6E, v > 0xF4C, v > 0x10A3 and v > 0x11E7 respectively. Nested payloads and alternate contexts remain explicit dependencies; these recovered call orders are not a complete feature decoder.

The dedicated partial probe installs temporary boss/extrusion readers and always raises a partial-prefix exception if it reaches its declared end. In the 906-case native corpus, all 12 prior boss/extrusion stops instead advance into moProfileFeature_c while reading the inherited feature body. No inherited-prefix end is claimed for these cases. The other stops remain the revolution body and the unspecified kernel context. Successful manager totals stay at 892; whole-document coverage is unchanged.

### 11.88 Atom-feature references and direct body choosers

*Status: Native serializer/constructor evidence; nine reference/chooser boundaries verified, including 18 present choosers · Evidence: [C-103]*

After the inherited prefix in 11.81, the modern ordinary atom-feature body serializes nullable arrays at +0x520 and +0x528, then a tagged moAtom_c pointer. Each nullable array is i32 presence followed, only when nonzero, by a direct suObArray body: packed ReadCount and tagged items. Readers SHALL NOT treat the presence value as an object tag or allocate an outer array map slot. Noncanonical nonzero values are retained.

With ordinary condition 1 absent and the special condition-0x2000 path inactive, the next fields are i32 index_count, index_count i32 indices, i32 word_538, another nullable direct array at +0x540 and i32 word_53C. All native version gates predate this scope. Native loading normalizes word_538 values above three; readers SHALL preserve the stored integer independently of that runtime change. Indices and duplicates retain wire order.

The following chooser stage stores u16 presence for the direct chooser at +0x4A0, its body when present, u16 word_498, u16 presence/body for +0x4A8, i32 word_4B0, u16 presence/body for +0x4B8, u16 presence/body for +0x4C0 and i32 word_570. Every chooser body is a direct embedded suObArray followed by a tagged moAtomFeature_c owner pointer. No chooser class tag or outer object slot is stored in these direct calls. Readers SHALL preserve owner back-references and distinguish the chooser's u16 presence from nullable-array i32 presence.

The dedicated audit verifies nine reference boundaries with nine stored indices and 11 present nullable arrays, then nine chooser boundaries containing 18 present direct choosers. Twelve supported profiles still complete during these traversals. Three other enclosing cases retain cosmetic-thread dependencies before this stage; revolution and kernel-context stops also remain. The atom-feature remainder and enclosing body/extrusion suffix are still unread. Default complete manager count remains 892; whole-document bit coverage is unchanged.

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

### 11.91 Revolution feature, end specification and axis reference

*Status: Native serializer evidence; three complete scoped revolution objects; whole streams incomplete · Evidence: [C-106]*

The modern ordinary moRevolution_c body uses the inherited feature/atom/body stages in 11.81 and 11.88–11.89. Its suffix is a tagged display dimension at +0x800, u8 byte_81C, tagged display dimension at +0x808, i32 word_818, tagged suObList at +0x810, two i32 words (820,868), tagged moLineRef_w axis, and tagged moRevEndSpec_c end specification. Version >=0x3F05 adds u8 byte_8E0. Readers SHALL preserve both raw bytes. The native loader converts byte_8E0 to a runtime Boolean; the archive representation remains one byte. The list is present on this ordinary zero-condition path; archive condition 1 can omit it in other contexts. Legacy gates all precede the scoped versions above 0x27CB.

The revolution end specification starts with five i32 words in order (08,138,13C,0C,10), two tagged moRefWrapper_c pointers (118,120), two tagged moPointRef_w pointers (128,130), two f64 values (38,40), two i32 words (140,144), and four tagged display dimensions (18,20,28,30). These fields are unconditional in the modern scope, independent of the two side type words. Readers SHALL retain pointer identity and raw discriminator values. Older files before 0x11C3 substitute doubles for dimensions and are outside this reader's scope.

The axis moLineRef_w serializes the wrapper base from 11.16, two f64 curve parameters (A8,B0), a three-double point (C0), a three-double vector (D8), and u8 byte_F0. It does not use the additional embedded-body/pointer suffix of moGeneralCurveRef_w. The shared curve helper writes its parameters before the line geometry for versions above 0xFA3; historical relocation of those parameters is outside this modern layout.

The internal moRevolvedRadiusDim_c body uses the geometric dimension base from 11.28, then a tagged moSkDimHandleValG2_c pointer and i32 word_128. This is a model dimension, distinct from the display-dimension serializers. Its native serializer calls the same geometric helper used by ThreeDRadiusDim_c and ThreeDdiameterDim_c. The supported model-dimension handle whitelist includes this derived class.

The opt-in `--complete-revolutions` audit also enables the supported extrusion readers. It completes three revolutions, 11 extrusions and 17 profiles, returning 897 relation-manager records in 906 attempts. `complete-revolution-corpus.json` retains each revolution's source path, stream, start/end offsets and suffix flag bytes. Nine configurations still stop on rib, fillet, cosmetic-thread or present-kernel-body framing dependencies. These are object and manager-prefix results; no complete configuration stream or whole-document bit-coverage claim follows.

### 11.92 Rib feature and silhouette-edge dependencies

*Status: Native serializer evidence; two complete ribs in one native configuration; whole streams incomplete · Evidence: [C-107]*

In the ordinary modern archive scope above 0x27CB, moRib_c uses the inherited body-feature stages in 11.81 and 11.88–11.89. Its suffix stores i32 word_834, u32 word_828, tagged display-distance dimension (838), tagged display-angular dimension (848), tagged display-radial dimension (850), tagged display-distance dimension (858), and tagged moCompSolidBody_c pointer (860). Readers SHALL read all five pointer tags, independent of word_834. The native writer uses bits 0x80, 0x600 and 0x08 to select populated versus null dimension pointers; it still emits null tags for unselected pointers. The final solid-body pointer exists from version 0x709, below this scope.

The component silhouette edge moCompSilhEdge_c first serializes the complete component-edge base from 11.17, then i32 transform presence and, when nonzero, a compact mgXform_c. Readers SHALL preserve signed presence and read its full four bytes. This tail exists from version 0x1C0A, below the modern scope. A drawing owner adds four i32 persistent entity IDs from version 0x3352; that owner-dependent branch is outside this ordinary part reader. The supported wrapper-reference whitelist now admits this component-edge subclass.

The boundary silhouette edge moSilhEdgeRef_c starts with the boundary-reference base from 11.18. It then stores three tagged moFaceRef_c pointers at offsets 188,190,198, u32 flags_1A0, and f64 values_1B0/1B8. Readers SHALL retain all raw flag bits. Native interpretation tests bits 1,2,4 in sequence to assign its runtime mode, and bit 8 for a separate Boolean; preserving only the resulting mode would lose archive information. These fields all exist in this modern scope. Legacy versions before 0x671 instead store the mode directly; values_1B0/1B8 begin at 0xDD4.

The moCompVertex_c primary RTTI serializer is exactly the existing moCompBEntRef_c serializer. Its modern body is therefore the component boundary-reference base (component reference, byte alias presence/optional CString, tagged boundary pointer, tagged external pointer), with no additional vertex suffix. This class is supported both by archive dispatch and the wrapper-reference whitelist. Its boundary pointer can itself identify a vertex-reference object with the separately documented point suffix.

The opt-in `--complete-ribs` audit also enables supported revolution and extrusion readers. Across 906 configurations, it returns 898 relation-manager records and completes 21 profiles, 11 extrusions, five revolutions and two ribs. Both ribs occur in the Sprinkler Base configuration; their raw word pairs are (258,3) and (274,2). The report retains exact source/stream spans, including nested object spans that overlap their parents. Eight configurations retain explicit fillet, cosmetic-thread or present-kernel-body framing stops. These results establish supported object boundaries and manager prefixes; whole-document bit coverage remains unchanged.

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

### 11.133 Default reference-plane data and origin profiles

*Status: B — native-bound ordinary modern bodies; complete-document semantics and post-list data remain partial.*

The scoped plane reader of 11.100 now supports the concrete moDefaultRefPlnData_c runtime class. Its primary table `4D13A690` selects serializer `4C283D30`, which calls su_CObject::Serialize and dispatches virtual +0x150. Both the default class and base moRefPlnData_c table `4D175710` bind that slot to `4C2880A0`, moRefPlnData_c::serializePlaneData. That shared serializer performs an explicit IsKindOf test for moDefaultRefPlnData_c; no serialized flag selects the following insertion.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common plane-data prefix | Fields of 11.100 | Always in scope | Root point, normal, transform, border, edge pointer, three i32 words and discarded u8 | B |
| 2 | `display_root_point` | Point, 3 × f64 | Default runtime class, v > 0x81D | Native mgPoint restore and setDisplayRootPt call; exactly 24 wire bytes | B |
| 3 | common plane-data continuation | Fields of 11.100 | After field 2 | +0x5C and +0x60 words, raw u16 flag and versioned facet/axis/sketch pointers | B |

The inserted point is distinct from the initial plane root point. It is read immediately after the legacy discarded byte and before the +0x5C word. There is no object tag or new map slot for this direct point. The implemented plane context is v > 0x27CB and ordinary external-XML mode, which satisfies the point's older version gate. Base plane-data bodies do not contain the insertion. Other derived classes are not silently treated as defaults. The shared unknown fields retain their prior 23-byte accounting; the display-root point has an established native role.

`resolvedfeatures.read_data` installs these scoped reference-plane and display-dimension readers on its copied archive-local reader table. It does not mutate the completed model context's table or reset inherited object slots. The archive remains in the same explicit ordinary mode; unsupported nested classes still stop the read.

The origin-profile primary table `4D0F0B88` selects serializer `4BB2E2F0`, which first calls the common profile serializer `4BB2F910`. Its remaining load actions change runtime update/editability state and add no wire fields. The resolved-feature archive therefore maps moOriginProfileFeature_c to the existing complete ordinary profile body, including its non-reused sketch/geometry dependencies or the reused branch in 11.161. The original runtime class remains moOriginProfileFeature_c. Unsupported nested profile bodies and special archive branches retain explicit errors; no profile body is bypassed to reach a later root.

The next configuration-table folder is independently bound by primary RTTI table `4D06C7D8` to common folder serializer `4BEC2AF0`. Its concept lightweight/resolved slots also match the established common concept stages `4C189B60` and `4C18A920`. It adds no derived suffix, but its nested native/design-table feature classes remain unsupported; no completed configuration-table folder root is claimed by this audit.

At the edition 0.144 checkpoint, the installed frontier recorded **2,304 complete plane roots**, spanning **1,272,574 bytes**: **2,298** use moDefaultRefPlnData_c and six use base moRefPlnData_c. It also records **765 complete origin-profile roots**, spanning **713,116 bytes**. Overall there are **13,657 complete roots**, 892 explicit separate-list stops and 14 model-context stops. No installed-corpus list is complete. The leading stops are moNativeConfigurationTableFeature_c (656 cases), moProfileFeature_c (57), moDetailFolder_c (54), moSensor_c (52) and moDesignTableFeature_c (45). These are first-stop counts, not exhaustive dependency inventories.

The separate native blank-part fixture does complete all **17 declared list roots**: its three plane spans are [2944,3483), [3483,4049), [4049,4619), followed by the origin profile [4619,5554). The stream is 5,558 bytes, so **four post-list bytes remain opaque** and retain unknown attribution. A complete list is not an exact-EOF or whole-stream claim. Tests distinguish the initial and display-root points, verify following fields and versioned pointer alignment, preserve inherited context, and reject truncated planes/origin payloads. The previous frontier and reference-plane trial reports remain retained as edition 0.143 baselines. Complete document bit coverage remains open.

Evidence: C-161.

### 11.151 Ordinary modern hole-wizard feature, slot-axis data and fixed plane data

*Status: B — native-derived complete scoped suffix framing and populated native parameter pairs; raw meanings and positive native slot branches remain partial.*

moHoleWzd_c primary table `4D0A5030`, locator `4D34B468`, selects serializer `4BB27330` in sldmodu.dll. Runtime descriptor `4DAD3EA0` selects factory `4BB17260`, which calls constructor `4BF80EF0`. The constructor calls base-hole constructor `4B3E7A10`, derived from moSketchedFeature_c. Memory allocation size 0xC28 is not wire size. Serializer `4B3E7CA0` establishes the complete body-feature inherited layout and one compatibility exception: **0x2F46 <= v < 0x2F68 reads an additional i32 immediately after the inherited body**, before the following suffix. Preserve this word as unknown; do not apply it outside that interval.

The following load grammar is scoped to ordinary non-DB external-XML archives with v > 0x27CB. A list of offsets in a row denotes separate fields in the listed wire order; offsets are native member identifiers, not stream positions. Each raw value is retained without native normalization. CString means the length-prefixed archive string grammar in clause 11.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +80C; object +810; raw +818; raw +820 | i32; tagged CObject pointer; f64; i32 | Always in scope | Pointer may refer to a supported reference body; preserve identity | B |
| 2 | raw +828, +830 | Two CString | Always in scope | Serialized radius/depth label storage; retain complete strings | B |
| 3 | raw +844, +848, +84C | Three i32 | Always in scope | Preserve signed values, including unknown type codes | B |
| 4 | raw +850, +858, +860 | Three CString | Always in scope | No fixed-size string buffers on wire | B |
| 5 | raw +870, +878, +880, +888, +890 | Five f64 | Always in scope | Preserve wire values | B |
| 6 | raw +868; raw +898, +8C8 | CString; two i32 | Always in scope | Ordering differs from member ordering | B |
| 7 | raw +8A0, +8B0; raw +910, +914; raw +8B8 | Two CString; two i32; CString | Always in scope | Retain values before runtime compatibility repair | B |
| 8 | raw +840 | i32 | Always in scope | Native may normalize this value after reading; offline reader preserves it | B |
| 9 | pairs +958 | i32 count; repeated (i32, f64) | Always in scope | Fixed signed count, not ReadCount; nonnegative and bounded by remaining 12-byte pairs | B |
| 10 | raw +8CC, +8D0, +8D4, +8D8, +8DC, +8E0, +8E8, +8EC | Eight i32 | Always in scope | Preserve this non-contiguous order | B |
| 11 | raw +8F0; raw +8F8, +8FC, +8E4; raw +8C0 | f64; three i32; CString | Always in scope | Raw meanings remain unresolved | B |
| 12 | raw +980, +900, +904, +908, +90C, +918, +A2C, +A28 | Eight i32 | Always in scope | Separate fixed-width words | B |
| 13 | raw +A30, +A38; raw +A40; raw +A48; raw +A58, +A5C, +A60 | Two CString; i32; f64; three i32 | Always in scope | Nonzero +A40 enables the slot-specific block below | B |
| 14 | raw +998 | i32 | v > 0x2F47 | Independent of slot enablement | B |
| 15 | raw +BB0 | i32 | v > 0x332E | Independent of slot enablement | B |
| 16 | dimension +A68 | Tagged moDisplayDistanceDim_c pointer | +A40 nonzero | Derived dimension body uses its normal tagged grammar | B |
| 17 | slot axes +BD8 | Direct moSlotCenterAxisData_c bodies | +A40 nonzero | Before v 0x2B5E, body count equals field 9 count; otherwise a separate nonnegative i32 count precedes the bodies | B |
| 18 | raw +A70, +A74, +A78; dimension +A80 | Three i32; tagged moDisplayDim_c pointer | +A40 nonzero | These follow the slot-axis bodies | B |
| 19 | raw +83C | i32 | +A40 nonzero and v < 0x42EA | Older placement; do not also read field 24 | B |
| 20 | vector +AA8 | Three f64 | +A40 nonzero | Direct vector, following field 19 when present | B |
| 21 | raw +BA8 | f64 | v > 0x3355 | Outside the slot-specific block | B |
| 22 | raw +BF8; raw +BFC, +C00 | u8; two i32 | v > 0x3F02 | Preserve the original byte even though native converts it to boolean | B |
| 23 | array +C08 | Direct suObArray body | v > 0x3F02 | ReadCount and tagged items as in 11.149; constructor proves the embedded type | B |
| 24 | raw +83C | i32 | v >= 0x42EA | New unconditional placement, including when +A40 is zero | B |

The empty fixed suffix through field 13 is 214 bytes when its pointer uses a short null tag, all strings are empty and the pair count is zero. Later gates and the enabled slot block extend that boundary. Slot counts SHALL fit the remaining minimum 72-byte direct bodies. Negative or impossible counts fail rather than being used as opaque lengths. The pre-0x2B5E branch has **no independent slot count on wire**. Direct slot bodies do not insert MFC class/object-map entries; a separately tagged moSlotCenterAxisData_c object uses the same body with its normal map insertion.

moSlotCenterAxisData_c primary table `4CF53050` selects serializer `4BB335F0`. Its body is exactly point +08, point +20 and vector +38, each three f64: **72 bytes**. No object tags occur between these direct geometric fields. The hole-wizard constructor initializes the +BD8 runtime pointer array, while each body is allocated and serialized directly by the hole-wizard load path. The +C08 suObArray is a distinct collection with independent framing.

Runtime hole-profile/sketch owner updates and final post-load dispatch consume no extra archive fields. Native compatibility handling can overwrite +910 when +848 equals 5, normalize +840, change legacy selection/type values and convert +BF8 to boolean. The offline reader preserves their serialized values. Earlier historical branches, including reference conversion before v 0x30F and old string/type repairs, are not folded into the modern reader. Raw numeric values, strings and pair elements receive no semantic coverage credit until their meanings are established.

The dependency moFixedRefPlnData_c has primary table `4D13AA60`, locator `4D3575C0`, selecting serializer `4C283D30`. It calls su_CObject::Serialize and virtual +0x150, bound to common plane-data serializer `4C2880A0`. RTTI confirms it derives from moRefPlnData_c and is not a moDefaultRefPlnData_c. Therefore its modern body is precisely the common plane-data body in 11.100, without the default plane's inserted display-root point and without an additional derived suffix. Fixed-plane border/numeric values are retained before native clamping and normalization.

The frontier contains **48 freshly serialized hole-wizard features** in completed roots. Pair counts are 1 in 35 cases, 2 in eight, 3 in three, 4 in one and 0 in one. All have slot enablement zero and empty +C08 arrays. Native +848 values include 0, 1, 2, 3 and 5. Populated slot-axis bodies, noncanonical enablement/bytes, independent dimensions and array identity are supported by synthetic tests, not claimed as positive native slot fixtures. Tests exercise each relevant gate boundary, the compatibility interval, the +83C relocation, malformed counts, truncation, fixed-plane dispatch and a hash-bound native containing-root end with multiple pairs.

Edition 0.162 records **20,584 complete roots**, **463 list-complete configurations** and **443 explicit stops** across all **906 complete modern model contexts**. **429 lists reach exact EOF**, spanning **19,975,017 decoded bytes**; **34 list-complete streams retain four-byte unknown tails**. Leading stops are base bodies (22), sheet-metal features (21), extruded reference surfaces (21), sweeps (17), blend cuts (15) and component reference axes (14). Full part, assembly, drawing and all installed project-format semantic bit coverage remains incomplete; these framed roots add no whole-document coverage credit.

Evidence: C-179.

### 11.157 Ordinary modern face/point reference-plane data

*Status: B — native-derived common/derived layout with native point and plane/face references; raw word meanings and other contexts remain partial.*

moFacePtRefPlnData_c primary table `4D175C80`, locator `4D35BD98`, selects serializer `4C27DCC0` in sldmodu.dll. After su_CObject::Serialize, virtual +0x150 selects common plane-data serializer `4C2880A0`. RTTI includes moRefPlnData_c and excludes moDefaultRefPlnData_c, so the common body is exactly 11.100 without the default-plane-only display-root-point insertion. The direct plane reference described below has its own display point, a distinct field.

In ordinary non-DB external-XML loading with v > 0x27CB, the complete body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common plane data | Existing direct moRefPlnData_c body | Always | Common geometry, border, raw fields and versioned pointers from 11.100; no extra class tag | B |
| 2 | point reference +88 | Tagged moPointRef_w pointer | Always in modern scope | Normal MFC null/reference/new-object grammar; existing point-wrapper body | B |
| 3 | plane reference +90 | Direct moPlaneRef_w body | Always | Allocated internally but serialized inline; no pointer tag, presence word or map insertion | B |
| 4 | raw +98, +9C | Two i32 | Always in modern scope | Original gate v > 0x10D4; preserve signed wire values | B |

On load, field 3 is created by allocating 0x1F8 bytes and calling constructor `4C270F20`, which installs moPlaneRef_w primary table `4CF530E8`. Its +0x28 slot selects serializer `4C281720`, establishing the exact direct body rather than inferring its type from allocation size. Its modern wire fields are the surface-reference wrapper, root point, normal vector, compact transform and display-root point, as in 11.144. Constructor-initialized extra point storage is not an appended wire array; 0x1F8 is an in-memory size.

With short pointer tags, absent optional transforms and empty/null common fields, the current-version body is **355 bytes**: 145 common bytes, a two-byte point tag, 200 direct plane-reference bytes and eight final raw bytes. Newly serialized pointed-to objects and optional transforms increase this boundary. Earlier supported versions retain the common/wrapper version gates and can have shorter bodies. The direct plane dictionary is separate from a tagged point object and receives no class/object-map entry of its own. Repeated component pointers within both references retain shared identity.

Native historical behavior before v 0x51 reads a generic object and converts a component vertex into a point wrapper. Versions through 0x149 also invoke the common plane-data virtual serializer again at the end. Those paths are outside this modern scoped reader and are not modeled by adding unconditional duplicate common fields. Common-plane native border clamping/runtime reconstruction does not replace the serialized raw evidence.

The frontier contains **10 fresh native face/point plane-data bodies**, all with moPointRef_w +88 references. Eight direct plane bodies refer to moCompRefPlane_c components and two to moCompFace_c components. Raw (+98,+9C) is (0,1) in eight cases and (1,0) in two. These observations do not establish a boolean or enum meaning for either word. Three focused tests cover shared inherited references, newly tagged data/point map slots versus the direct plane's absence of a slot, geometry/raw values, common/wrapper version boundaries, truncation and unsupported contexts, plus a stream-hash-bound native containing-root end.

Edition 0.168 records **20,975 complete roots**, **518 list-complete configurations** and **388 explicit stops** across all **906 complete modern model contexts**. **479 lists reach exact EOF**, spanning **22,638,852 decoded bytes**; **39 complete lists retain four-byte unknown tails**. Leading dependencies are blend cuts (15), domes/sketch regions/component axes (14 each), 3D profiles (13), reference points/flat patterns (12 each) and blends (11). Full part, assembly, drawing and installed-project bit coverage remains incomplete. Unknown word meanings and post-list tails remain uncredited; these object boundaries do not establish full document semantics.

Evidence: C-185.

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

### 11.173 Direct atom-feature surface identifier maps

*Status: B — native load/store framing, direct construction and populated containing-root boundaries verified; key/value meanings and runtime map semantics remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. The late atom-feature stage `0x4BB1EC00` reads the existing i32 presence value for native member +6A0. When nonzero, it allocates `0xB8` bytes, invokes constructor `0x4BCE39B0` and directly calls load helper `0x4BCE7600`. Native store uses helper `0x4BCE7A60`. These are dedicated load/store functions, not a tagged archive object or primary Serialize virtual call. The presence gate after version `0x104C` precedes this scope.

Constructor `0x4BCE39B0` installs moFeatureMultiBodySurfIdMap_c's primary table `0x4CFEAE88` after its moBaseSurfIdMap base. RTTI/COL evidence verifies that ancestry and su_CObject inheritance. Runtime owner references, timestamps and map/update state created by the constructor are not additional serialized fields. The primary Serialize slot is a different path from the dedicated load/store helpers; no general tagged-map reader is registered from this direct layout.

After a nonzero +6A0 presence value, the direct body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | entry count | i32 | always | Signed count, not packed ReadCount. Reject negative or out-of-buffer counts. Zero consumes no entries. | V |
| 2 | entry key | u32 | each entry | Preserve all 32 stored bits and original entry order. | V / ? |
| 3 | entry value | i32 | each entry | Preserve signed value. Its sign selects the following optional branch. | V / ? |
| 4 | child-list presence | i32 | entry value < 0 | Preserve full stored presence word; any nonzero value selects row 5. This field is absent for nonnegative entry values. | V |
| 5 | child count | i32 | negative value and child-list presence nonzero | Reject negative or impossible counts. Zero is permitted. | V |
| 6 | child integers | count × i32 | row 5 present | Preserve every value, including negative integers and duplicates that native loading may filter. | V / ? |

The negative-value child-list branch begins after `0x1059`, before this modern minimum. There is no class tag, object-map slot, key CString, pointer-sized integer or per-entry alignment. The empty direct body occupies **four bytes**, excluding the preceding presence word. An entry with a nonnegative value occupies **eight bytes**; a negative-valued entry adds a four-byte presence word, and a present list additionally adds its four-byte count and four bytes per child.

The native loader populates internal pointer-keyed maps from these integer values. It drops some negative-valued entries when no usable children remain, ignores negative child integers and deduplicates child keys. The offline reader preserves the original ordered wire entries and lists without implementing those lossy runtime operations. Duplicate outer keys remain distinct records. The enclosing atom-feature reader resumes immediately after the direct body with its existing face-name/index-name collections and raw +6B4 word. Arbitrary synthetic values establish framing, not native writer validity or complete surface-ID engineering meaning.

The complete-root audit retains **13 direct maps** with **53 entries**. There are 42 nonnegative-valued entries and 11 negative-valued entries, all 11 with stored child-list presence one. Child counts are two in seven entries, three in three entries and four in one entry. Direct-map counts range from one to eight entries. Evidence is collected only from fresh objects within completed containing roots; incomplete objects at a later stop receive no completed-record credit.

Four focused tests verify duplicate outer keys, signed/unsigned extremes, noncanonical presence words, negative/duplicate child retention, empty bodies, malformed counts and truncation, context rejection, atom-stage continuation and unchanged object-map slots. The native test verifies the stream SHA, exact containing-root span and retained direct map. Key/value semantics, runtime surface-ID updates, historical/special archive contexts and additional feature payloads remain open.

Edition 0.184 records **22,418 complete roots**, **676 list-complete configurations** and **230 explicit stops** across **906 complete modern contexts**. **615 lists reach exact EOF**, spanning **32,762,939 decoded bytes**; **61 complete lists retain four-byte unknown tails**. The prior atom-feature direct-data stop is resolved in this scope. Drafts, surfaces, cut lists, sweep cuts, annotations and other feature dependencies remain unresolved. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these boundaries.

Evidence: C-201.

### 11.189 Reference-plane data at an angle

*Status: B / ? — native common dispatch, direct plane-reference type and six populated containing-root records verified; raw selectors and geometric construction semantics remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moRefAtAngleData_c primary table `0x4D1395D0`, complete-object locator `0x4D356F70` and serializer slot +28 select `0x4C2823A0`. RTTI establishes moRefPlnData_c ancestry without moDefaultRefPlnData_c ancestry. Virtual +150 selects shared plane-data helper `0x4C2880A0`, so the common body is the ordinary nondefault plane-data grammar of 11.100. The ordinary su_CObject layer adds no fields. The suffix is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common plane data | direct shared body | always | Preserve existing point/vector/transform, border, edge, raw word/byte and versioned reference fields. This is not the default-plane variant's extra display point. | B / ? |
| 2 | plane reference +90 | direct moPlaneRef_w body | always | Wrapper, four surface doubles, root point, normal, direct transform and display point. No pointer tag, presence word, class header or object-map slot. | B / ? |
| 3 | line +98 | tagged moLineRef_w pointer | always in scope | Preserve null/cached/new framing; historical pre-`0x169` direct-line conversion is excluded. | B |
| 4 | angle dimension +A0 | tagged moDisplayAngularDim_c pointer | always | Independent pointer after the line reference. Native owner assignment consumes no additional archive fields. | B |
| 5 | raw +A8/+AC/+B0/+B4 | four i32 | always in scope | Native load gates for +AC and +B0/+B4 are after `0x10A6` and `0x10D4`, below the supported minimum. Retain each complete signed word. | V / ? |

The native load path allocates a plane-reference object through constructor `0x4C270F20` and invokes its serializer directly. That constructor installs primary table `0x4CF530E8`, whose serializer is `0x4C281720`, matching the existing direct plane-reference grammar. Its 0x1F8 runtime allocation is not a serialized length. The latest-version empty common plane body is 145 bytes; the empty direct plane reference is 200 bytes. With null line/dimension pointers and four raw words, the complete synthetic minimum angle-data body is **365 bytes** before outer tagged framing.

The completed-root audit retains **six angle-plane data records**. Their direct plane-reference components are moCompRefPlane_c in four cases and moCompFace_c in two. All six have moLineRef_w line references and moDisplayAngularDim_c dimensions. Raw +A8 is zero in four and one in two; +AC/+B0 are zero and +B4 is one in all six. These observed values do not establish boolean/enum domains or orientation rules. Scoped readers preserve stored references and raw values without constructing or validating the resulting plane geometry.

Evidence: C-217.

### 11.190 Reference-plane data through three points

*Status: B / ? — native common dispatch and six populated containing-root records verified; point geometry and historical conversion modes retain existing limits.*

Scope is the same ordinary modern archive range as 11.189. moRefThreePtsData_c primary table `0x4D1399C0`, locator `0x4D357098` and serializer slot +28 select `0x4C283D60`. RTTI establishes moRefPlnData_c ancestry without the default-plane subclass. Its virtual +150 also selects `0x4C2880A0`. After the no-byte ordinary object base and common nondefault plane data, it stores:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | first point +88 | tagged moPointRef_w pointer | always in scope | Standard non-DB ReadObject/WriteObject, not an embedded three-coordinate point. | B |
| 2 | second point +90 | tagged moPointRef_w pointer | always | Separate pointer occurrence; preserve repeated cached identity. | B |
| 3 | third point +98 | tagged moPointRef_w pointer | always | No additional count, presence word or plane-reference body. | B |

With a 145-byte empty common plane-data body and three null/cached two-byte tags, the latest-version synthetic body is **151 bytes** before outer class/object framing. New point references add their own class/object framing and bodies. The three pointer occurrences must be retained even if they resolve to the same cached object; this framing does not assert geometric validity of coincident or collinear points. The historical pre-`0x51` vertex-to-point conversion and pre-`0x14A` repeated common-body branch are outside the supported scope.

The completed-root audit retains **six three-point plane-data records**, all containing three nonnull moPointRef_w pointers. No distinctness, coplanarity or coordinate-system semantics are inferred from their class names. Four focused tests for 11.189–11.190 cover direct-versus-tagged framing, raw i32 extrema, repeated point identity, outer object caching, minimum bodies, truncation/context rejection and source-stream-hash-bound complete native containing-root spans with stored-word or ordered point-class equality. Native evidence verifies three primary RTTI/COL/type/ancestry bindings, both shared virtual slots, five native bodies and the direct angle-plane constructor/vtable dispatch.

The frontier now has **23,545 complete roots**, **719 complete lists** and **187 explicit stops** across 906 complete modern contexts. **654 lists reach exact EOF**, spanning **37,532,217 decoded bytes**; **65 complete lists retain four-byte unknown tails**. Every complete list passes the recorded stream-size endpoint/tail invariants. Relative to Edition 0.197 this adds 75 complete roots and four exact-EOF lists. `resolved-feature-list-frontier-0.197.json` preserves the preceding checkpoint. Thin extrusions, further identity representations and optional loft data remain leading dependencies. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; whole ResolvedFeatures coverage is not credited.

Evidence: C-218.

