### 11.94 Linear-pattern header and derived suffix

*Status: Native serializer evidence; three shared-header boundaries verified; enclosing patterns incomplete · Evidence: [C-109]*

Primary moLPattern_c serializer 0x4BB28E00 first calls shared pattern serializer 0x4BB2E550, which calls the body-feature base from 11.81 and 11.88–11.89. The scoped reader returns that supported base independently of any derived feature suffix. This base completion is not a complete pattern object. The following layout applies to ordinary zero-condition external-XML archives above version 0x27CB.

The shared pattern header stores tagged seed suObList (980), tagged suObList (9D0), i32 word_9D8, tagged suObArray (818), u8 byte_9F4, i32 word_9F8 and tagged suObList (8D0). Before version 0x42CF, the next field is u8 byte_940. From 0x42CF, it is instead tagged suObList (8D8) followed by i32 word_940. Both variants then store a tagged moMatrixIndexHelper_c pointer (948). Readers SHALL preserve the raw byte/word and pointer identity. Native normalization of byte_9F4 from 2 to 1 under owner-dependent conditions does not change its stored width. Archive condition 0x2000 can substitute direct feature-reference data for the seed list; that branch is outside this zero-condition reader.

Immediately after this header, the shared serializer invokes an embedded collection at +0x838, followed by a packed-count object-pointer map, later embedded collections/helpers, variation records and further versioned fields. The initial probe stops before +0x838; constructor-verified maps, the following object array and shape data are documented in 11.95. Readers SHALL stop at the first unsupported stage. The header reader never skips the base to reach the derived suffix.

After the complete shared base, the moLPattern_c-specific suffix starts with four tagged display dimensions (1250,1258,1260,1268). This branch is proven by its primary vtable +0x2680 resolving to the zero-return function 0x4B29C5A0. Other derived pattern classes may omit these four tags and are not admitted by this scoped suffix reader. Next are u8 bytes 1290/1294, two generic tagged object pointers (1270,1278), two tagged line references (1280,1288), and u8 byte_129C.

The suffix continues with i32 words 12D8/12DC, tagged reference wrappers 12E0/12E8, i32 words 12F0/12F4, tagged reference wrappers 12F8/1300, i32 words 1308/130C, tagged display dimensions 1310/1318, and i32 words 1320/1324. Version >=0x3F00 adds u8 byte_1298. Readers SHALL retain raw byte values and signed words. Earlier line-reference and advanced-field gates all precede the scoped modern versions. Runtime ownership repair and flag updates add no suffix bytes.

`--pattern-header` reaches the explicit collection boundary in all three native hex-nut-beater configurations at offsets 72811,72815,72815. Each has one seed, empty 9D0/8D0 lists, null array_818/matrix helper, byte_9F4=1, word_9D8=1 and word_9F8/word_940=0. These traversals complete three revolution cuts and 24 profiles; the cumulative revolution counter is eight, including those three cuts and five ordinary revolutions. Manager completion remains 898/906, with three intentional pattern stops plus four fillet and one kernel-context stop. No pattern or cosmetic thread is returned complete. The standalone derived-suffix reader has native and synthetic evidence, but is not yet reached by those native paths. Whole-document bit coverage is unchanged.

### 11.95 Pattern maps, embedded array and shape data

*Status: Constructor/native serializer evidence; three collection and shape boundaries verified; enclosing pattern incomplete · Evidence: [C-110]*

The moLPattern_c default constructor 0x4BFCC320 calls shared pattern constructor 0x4C052390. That constructor proves +0x838 is su_CMapObToInt and +0x868 is su_CMapObToPtr, while +0xF48 is an embedded suObArray. These are direct bodies rather than new tagged container objects. Other proven members include embedded suObList +0x8E0 and direct moShapePatternData_c +0xF98; their runtime vtables do not introduce file tags.

After the header in 11.94, map_838 stores packed ReadCount and that many pairs of tagged object key/i32 value. The SWCCU serializer 0x31D492F0 proves both the packed count and signed integer overload. Map_868 follows with packed ReadCount and pairs of tagged object key/tagged object value, as explicitly read by the pattern serializer. Next are u8 byte_9E8 and the direct suObArray body at F48 (packed count and tagged object items). Readers SHALL retain entry order, duplicate keys, signed integer values and object identity. Inserting these entries into a native-style hash map would discard wire evidence. No outer container class/object slot is allocated for either map or the embedded array.

The direct shape-data body follows, beginning with i32 stored type at member offset 30. Type 0 ends that body immediately. For nonzero types, dimension tags occur as follows; all offsets in this clause are relative to the shape-data member.

| Stored type | Initial dimension pointers, in wire order |
|---|---|
| 1 | tagged radial display dimension at 48 |
| 2 | tagged distance display dimension at 50 |
| 3 | tagged distance display dimensions at 58 then 50 |
| 4 | tagged radial display dimensions at 40 then 48 |
| Other nonzero | no initial dimension tags |

Every nonzero type then stores i32 point_mode_98. Zero adds a tagged moRefWrapper_c pointer at 68; any nonzero value instead adds a three-double point associated with 90. A three-double vector at 18 follows. Type 1 adds neither a count word nor angular dimension. Types 2/3 add a tagged angular display dimension at 60. Other nonzero types add i32 word_80 then the angular display dimension. Finally all nonzero types store i32 word_9C. Readers SHALL preserve raw type and point-mode values, including unrecognized nonzero types, using the native branch structure. These fields are present throughout the documented version scope above 0x27CB; older helper branches before 0xBEE are outside it.

Following the shape body, the parent pattern stores u8 byte_1048 and u8 byte_104C. The next native decision calls isVaryInstanceCircularEnabled and may serialize direct dimension-increment/original-dimension records before the remaining shared-base tail. The initial shape-only stage stops there; the enabled variation branch and following shared tail are documented in 11.96. Readers SHALL select that branch from the native version predicate, rather than inferring absence from empty shape data or zero flag bytes.

The separate `--pattern-collections` audit reaches three boundaries at 72818/72822/72822, seven bytes after their headers. Both maps and the F48 array are empty; byte_9E8 is 1. `--pattern-shape` reaches 72824/72828/72828: shape type is 0 and the two following flag bytes are zero. The manager total remains 898/906. These audits retain the same three completed revolution cuts, 24 profiles and explicit later stops without returning a partial pattern or cosmetic thread as complete. Nonzero shape branches and populated maps have native-code and synthetic evidence; whole-document bit coverage is unchanged.

### 11.96 Pattern variation records and shared-base tail

*Status: Native serializer evidence; three complete shared pattern-base boundaries verified · Evidence: [C-111]*

The variation switch at 0x4C087430 returns whether moVersionManager_c::getCurrentFileVerion() is greater than 0x1437. It does not test the preceding pattern flag bytes. The following reader implements the enabled branch in the documented modern scope above 0x27CB. It does not generalize that branch selection to arbitrary legacy runtime/version-manager states. Primary moLPattern_c vtable +0x870 returns zero, so the modern advanced dimension-pattern index-array branch is absent for this class.

The parent variation body stores i32 increment count and that many direct moDimIncrementId_c records, then u8 byte_9FC and u8 byte_A00. Next are i32 original-dimension counts and direct records for arrays A08 and A28, in that order. Two more direct moOrigDimData_c bodies follow unconditionally at A48 and CB0, even when both counts are zero. Readers SHALL NOT insert object/class tags or map slots for these direct records. Unsupported nested tagged objects still stop the parent.

A modern direct increment record has the following order: u8 handle-missing flag, tagged object pointer, u8 name presence and optional CString, i32 words 18/1C/20, u8 display-dimension-missing flag, tagged object pointer, u8 byte_24, f64 value_28, i32 word_50, u8 byte_30 and f64 value_38. Both pointer tags exist regardless of their missing flags. When a missing flag is nonzero, native loading discards the corresponding pointer; readers SHALL preserve that tagged archive object anyway. Zero flags identify a dimension handle and display dimension respectively. All these increment fields exist throughout this modern scope; the final byte/value pair begins at version 0x1FE5, below it.

A modern direct original-dimension record stores these fields in order:

| Order | Fields | Representation |
|---|---|---|
| 1 | handle-missing flag; object; name presence/name | u8; tagged object; u8 and optional CString |
| 2 | byte 18; value 20; byte B4; values B8/C0 | u8; f64; u8; f64 × 2 |
| 3 | words 260,C8,CC,D0,D4,1D8,1D0,1D4,218 | i32 × 9, in that order |
| 4 | increment presence; nested increment | u8; direct increment body when nonzero |
| 5 | component features | i32 count and count tagged moCompFeature_c pointers |
| 6 | word 1C0; embedded array 1B0; word 25C | i32; direct suObArray body; i32 |

The original-dimension handle pointer is also unconditional, even when native loading discards it. Native loading deduplicates nonnull component features and may discard other stored words; readers SHALL retain duplicates, null pointers, signed values and the full stored sequence. Constructor 0x4C0510E0 proves that member 1B0 is suObArray. Its direct body allocates no container object slot. The runtime cached value at +200 copies value_20 and adds no file bytes.

The remaining shared pattern tail is a tagged suObList (merged by native loading into embedded list 988), then i32 word_1120. Version >0x2F86 adds direct suObList body 8E0. Version >0x2F93 adds u32 count and count i32 indices at 928. Version >0x3745 adds u8 byte_1124. Version >=0x42DC adds u8 byte_1128; native loading interprets it as true only when exactly 1, so readers SHALL retain the raw byte. Older versions derive that runtime value from seeds without additional file bytes. This completes the supported shared base; the linear suffix from 11.94 follows immediately.

`--pattern-variation` reaches three boundaries at 73000/73004/73004. Increment/A08/A28 counts and the two parent flag bytes are zero; the two unconditional original records still occupy the wire. `--pattern-tail` reaches 73016/73020/73020, with zero tail indices, byte_1124=0 and byte_1128=1. The Edition 0.96 `--complete-patterns` probe entered the derived suffix and stopped on new moDisplayDim_c objects at 0x11D38/0x11D3C/0x11D3C; it returned no complete linear patterns or cosmetic threads and retained 898/906 manager completions. The supported base display dimension and superseding complete-object results are documented in 11.97. Synthetic tests cover populated and nested direct records, pointer tags ignored by native flags, duplicate features/indices, tail version transitions and truncation. Whole-document bit coverage is unchanged.

### 11.144 Mirror patterns and direct plane-reference suffixes

*Status: B — ordinary modern pattern composition and plane-reference framing; raw shared-pattern meanings remain partial.*

moMirrorPattern_c primary table `4CFED530`, locator `4D33CDC8`, selects serializer `4BB2C150`. It calls shared moPattern_c serializer `4BB2E550` before its own plane suffix. The mirror class's +0x4B8/+0x4C0 hooks select the common concept serializers, and virtual +0x870 returns **zero**, as in the supported ordinary linear-pattern branch. Runtime descriptor `4DAD3B10` selects factory `4BB16DD0` and constructor `4BCED3F0`; the 0x11B0 memory allocation is not wire size. That constructor calls shared pattern constructor 4C052390 before installing the final mirror tables.

The supported scope is ordinary non-DB external-XML loading with v > 0x27CB, the enabled variation path from 11.96 and zero virtual +0x870. The native isVaryInstanceCircularEnabled helper `4C087430` tests whether the version manager's current file version exceeds 0x1437. Other variation/index-array branches, archive masks and older layouts remain outside this composition.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common pattern body | Complete shared pattern grammar | Always in scope | Body-feature base, pattern header, maps/array, shape data, 1048/104C bytes, variation records and shared tail from 11.94–11.96 | B |
| 2 | plane +11A8 | Tagged moPlaneRef_w pointer | v > 0x3745 | Optional by version; pointer can be null | B |
| 3 | plane +1138 | Direct moPlaneRef_w body | Always in scope | No pointer tag, class wrapper, presence word or object-map slot | B |

Field 1 ends at the shared pattern tail, before any derived linear suffix. A mirror does **not** receive the linear-pattern dimensions/directions from 11.94. The common composition is now a shared reader called by both complete linear and mirror readers; partial linear-prefix stages retain their explicit stops. Class-specific dispatch still determines which derived suffix follows.

Before reading the mirror suffix, the native load path allocates and constructs the +1138 plane-reference object, updates runtime state at +730, and may normalize a referenced state value. Those actions consume no plane tag or count. It reads the +11A8 tagged pointer only above 0x3745, then calls the +1138 object's serializer directly. Allocated size 0x1F8 and constructor-initialized map/point members do not imply extra bytes after field 3. The maps at mirror offsets +1140/+1170 are not appended to this serializer's wire path.

moPlaneRef_w primary table `4CF530E8` selects serializer `4C281720`, whose modern body is the existing surface-reference wrapper, a direct point, a direct vector, a direct compact mgXform and a direct display-root point (11.100). The surface-reference wrapper is the ordinary moRefWrapper_c body plus four raw f64 fields from 11.16. This body is identical whether reached through a tagged moPlaneRef_w object or directly in field 3. Only the tagged form adds its MFC class/object map slots; its nested reference pointers and the direct form share the inherited map. Historical coefficient and early point/transform gates precede the supported modern range.

The scoped installer supplies moPlaneRef_w dispatch locally alongside the mirror reader. Unsupported nested reference classes retain explicit stops. The ordinary wrapper's raw fields, variation flags and other shared pattern values retain their existing semantic limitations; completing this composition does not grant full-stream bit-coverage credit.

The installed frontier retains **48 freshly serialized mirror patterns**. All include a null field-2 tag and a populated direct field-3 reference: 47 use moCompRefPlane_c and one uses moCompFace_c. A native regression replays the hash-bound face-based case through its exact enclosing pattern-root end. Synthetic tests exercise both sides of the 0x3745 gate, null/inherited/new tagged planes, direct-plane point/vector/transform placement, unchanged direct-object map slots, duplicate nested identity, truncation and context rejection. No nonnull native +11A8 fixture is claimed. Existing linear-pattern tests continue to check their own suffix and partial-prefix behavior after the shared-body extraction.

The edition 0.155 frontier records **19,445 complete roots**, **328 list-complete configurations** and **578 explicit stops**, with all **906 modern model contexts** complete. **322 lists reach exact EOF**, spanning **12,172,850 decoded bytes**; six retain four-byte unknown tails. No mirror-pattern stop remains at this frontier. Leading dependencies are moPLine_c (40), reference axes (37), shells (36), moOneContour_c (33) and hole-wizard features (27). Full bit coverage remains incomplete across parts, assemblies, drawings and installed project formats.

Evidence: C-172.

### 11.148 Ordinary modern circular-pattern suffix

*Status: B — native-derived framing with native dimension alternatives; raw fields and populated integer collections remain partial.*

moCirPattern_c primary table `4D023470`, locator `4D340CD8`, selects serializer `4BB219E0` in sldmodu.dll. Runtime descriptor `4DAD3720` selects factory `4BB168A0` and constructor `4BDA52B0`. The constructor calls the shared pattern constructor. Allocation size 0x1210 is not wire size. Common concept hooks select `4C189B60` and `4C18A920`; virtual +0x870 selects `4B29C5A0`, whose bytes return zero, establishing the shared variation layout used here.

The following grammar applies to ordinary non-DB external-XML loading with v > 0x27CB and archive condition 0x800 clear. It follows the complete shared pattern base (11.94–11.96, 11.144), including its variation and versioned tail. No linear- or mirror-pattern derived suffix intervenes.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | dimensions +1130, +1138, +1140, +1148 | Four tagged moDisplayDim_c pointers | Always in scope | Load uses generic dimension declarations; store declares +1138/+1148 angular. Preserve actual derived classes and shared identity | B |
| 2 | object +1150; line +1158 | Tagged CObject pointer; tagged moLineRef_w pointer | Always | Condition 0x800 would insert an owner i32 before +1150 and is outside this scope | B |
| 3 | raw +1160 | u8 | Always | Preserve all bits; meaning unknown | B |
| 4 | dimension +1198; raw +11A8 | Tagged moDisplayAngularDim_c pointer; u8 | Always | Raw byte has no inferred boolean meaning | B |
| 5 | dimension +11A0 | Tagged moDisplayAngularDim_c pointer | Always | Preserve null/reference/new-object distinction | B |
| 6 | raw +11AC, +11B0, +11B4, +11C0, +11C4 | Five u8 | Always | Preserve bytes in this order | B |
| 7 | map +11D0 | i32 count, repeated i32 key and integer vector | Always after dimension validation | Each vector is i32 count followed by that many i32 values; counts SHALL be nonnegative and fit available bytes | B |
| 8 | vector +1200 | i32 count, repeated i32 values | Always | Independent count; same bounds rule | B |
| 9 | point +1168; vector +1180 | Three f64; three f64 | v > 0x3EFA | Direct geometry bodies without object tags | B |

The native post-load callback `4BDB1620` repairs dimension owners without synthesizing missing dimensions. At least one of dimension +1138 or +1198 SHALL be nonnull: both null lead to AfxThrowFileException, not a shorter valid suffix. The scoped reader rejects that state before reading the integer collections.

Map and vector counts are fixed signed i32, not ReadCount. Despite a misleading pointer type in decompilation, the map key is serialized as a 32-bit integer, not an MFC pointer tag. Native map insertion replaces duplicate keys; the offline reader preserves every serialized entry and its order. Integer keys/values and the seven raw bytes receive no semantic coverage credit. Native post-tail adjustment of the second instance count when raw +11C4 is zero consumes no additional archive bytes.

The frontier contains **43 freshly serialized circular patterns** in completed roots. Twenty-six have dimension +1138 and seventeen instead have +1198; all have +1130 and no +1140/+1148/+11A0 dimension. All have empty map/vector collections and the final geometry pair. Their raw-byte tuples are (0,0,0,1,1,0,0) in 26 cases and (0,1,0,1,1,0,0) in 17. Populated collections, signed extrema, duplicate keys and noncanonical bytes are covered synthetically, not claimed as populated native fixtures. Tests also cover the 0x3EFA/0x3EFB gate, inherited identity, malformed/truncated inputs, missing dimensions, unsupported contexts and a hash-bound native root boundary.

The edition 0.159 frontier records **20,199 complete roots**, **406 list-complete configurations** and **500 explicit stops**, with all **906 modern model contexts** complete. **400 lists reach exact EOF**, spanning **16,919,494 decoded bytes**; six list-complete streams retain four-byte unknown tails. Leading dependencies are hole-wizard features (35), moOneContour_c (33), scale features (23), base bodies (22), sheet-metal features (21) and extruded reference surfaces (21). This traversal adds no whole-document semantic coverage credit. Full part, assembly, drawing and all installed project-format bit coverage remains incomplete.

Evidence: C-176.

