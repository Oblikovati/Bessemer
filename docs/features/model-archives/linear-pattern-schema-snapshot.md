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

### 11.214 Mirrored solids, matrix-index helpers and unsigned arrays

*Status: B / ? — native ancestry/constructor/helper bindings and four mirrored-solid roots with six matrix helpers verified; raw option meanings and full document/project coverage remain partial.*

moMirrorSolid_c primary table `0x4CFF0048`, locator `0x4D33CFA0` selects serializer `0x4BB2C2D0` and has moBodyFeature_c ancestry. Its direct base call and inherited node hooks are bound to the supported ordinary body-feature layout. Runtime descriptor `0x4DAD3D50` records allocation size `0x9F8`, creator `0x4BB170E0` and constructor `0x4BCF6540`. The constructor establishes direct suObArray members +820/+840 and suObList +880. Other constructed runtime containers are not assumed to be serialized here.

In ordinary non-DB external-XML archives with version greater than `0x27CB`, and archive reuse state +834 clear, the complete body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited body feature | complete direct moBodyFeature_c body | always | Existing inherited framing, without another tagged base object. | B |
| 2 | face +800 | tagged moCompFace_c pointer | always | Modern native saving writes null here; offline loading preserves any stored supported pointer. | B |
| 3 | raw +808 | i32 | always in scope | Historical gate `v >= 0x253`. | B / ? |
| 4 | first plane +810 | i32 presence; optional point and normal | always in scope | Historical gate `v > 0x53B`. Any nonzero presence adds two three-f64 records: root point then normal vector. Preserve the full presence word and stored geometry. | B / ? |
| 5 | arrays +820/+840 | two direct suObArray bodies | always in scope | Packed ReadCount and tagged objects, separately framed. The +820 body is omitted in native reuse mode; that mode is excluded here. | B |
| 6 | raw +9B8 | i32 | always in scope | Follows +840; historical gate for both is `v >= 0x70B`. | B / ? |
| 7 | surface +9A0; helper +9C8 | tagged moSurfaceRef_w, tagged moMatrixIndexHelper_c | always in scope | Historical gate `v >= 0x7F7`. Native pre-gate wrapper construction is runtime behavior outside this scope. | B |
| 8 | raw +9B4/+9BC/+9F0 | u8, i32, u8 | always in scope | Keep independent widths and the stored values before runtime updates. | B / ? |
| 9 | raw +9C0/+9C4 | two u8 | always in scope | Historical gates `v > 0xB71` and `v > 0xBA7`. | B / ? |
| 10 | list +880 | direct suObList | `v > 0x2F86` | Packed count and tagged members; no outer container map slot. | V |
| 11 | words +8C8 | u32 count, count × i32 | `v > 0x2F93` | Helper `0x4BB17E10` reads fixed-width words, not packed ReadCount. Preserve duplicates/order and signed entry bits. | V / ? |
| 12 | raw +9B0; surface +9A8 | u8, tagged moSurfaceRef_w | `v > 0x3745` | Two distinct occurrences under equivalent native gates. | V / ? |
| 13 | second plane +818 | i32 presence; optional point and normal | `v > 0x3760` | Same framing as row 4, independently retained. Native final cleanup consumes no further archive values. | V / ? |

The latest empty complete body occupies **294 bytes** before outer tagged framing. The suffix alone occupies 30 bytes at `0x27CC`, 32 after `0x2F86`, 36 after `0x2F93`, 39 after `0x3745`, and 43 after `0x3760`, using null pointers/empty containers/absent planes. Each populated plane adds 48 bytes. These are tested minima, not populated-feature fixed lengths. Native runtime filtering/removal of array entries does not alter offline stored-order retention.

moMatrixIndexHelper_c primary `0x4CE87EF8`, locator `0x4D316460` selects serializer `0x4C27F890`. RTTI includes moBaseBodyChooser_c, but this serializer calls su_CObject's ordinary empty base rather than serializing a chooser prefix. Descriptor `0x4DAEDE20`, creator `0x4C26FDC0` and constructor `0x4B775250` prove two direct suObArray members:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | array +10 | direct suObArray | always | Packed count and tagged objects. | B |
| 2 | array +30 | direct suObArray | always | Separate array; preserve duplicate/cached identities. | B |
| 3 | raw +08 | i32 | always | Follows both arrays. Minimum helper body is eight bytes. | B / ? |

Observed nested suUIntArray objects are defined in sldmfcu.dll, SHA-256 `9ab6733de9a90a1564f671f83b9298c3ed15f205a359849d6b2fd68c90e95290`. Primary `0x3CF2EE20`, locator `0x3CF82F60` selects serializer `0x3CB7A630`. Its body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | element count | i32 | always | Fixed-width signed count. Reject negative or out-of-bounds extents before reading values. | B |
| 2 | growth value | i32 | always | Native SetSize's growth argument; retained even for an empty array. | B / ? |
| 3 | elements | count × u32 | always | Native bulk-reads four bytes per element. Preserve high bits, duplicates and order. No packed-count escape or typed-object tag per element. | B / ? |

The unsigned-array header is eight bytes, including for an empty array. Its outer tagged object participates in ordinary identity; the direct arrays inside the helper do not add container map slots. All three readers are installed only in copied scoped maps.

Six focused tests cover both plane bodies and full-width presence words, raw byte/word retention, duplicate array entries, all suffix gates, bounded fixed-width vector/array counts, latest tagged minimum/self identity, unsigned high-bit values/growth metadata, truncation/context rejection, and source-hash-bound complete native root spans with raw/plane/helper/nested-unsigned-array field equality.

The audit retains **four mirrored solids** and **six matrix helpers** in complete roots. All four mirrored solids include the first plane and omit the second, with direct array counts 0/1. Four helpers store raw +08 = 2 and array counts 1/1; the other observed helpers store 12 with 1/1 and 7 with 2/2. These observations do not define option domains or general array-length constraints.

`mirror_solid_evidence.py` retains two model primary/ancestry bindings, two factory descriptors, seven model native bodies and the separately hash-bound unsigned-array serializer. Across 906 complete modern contexts the frontier now has **24,282 complete roots**, **765 complete lists** and **141 explicit stops**: 31 additional roots and two additional complete lists relative to Edition 0.219. **692 lists reach exact EOF**, spanning **46,159,654 decoded bytes**; **73 complete lists retain four-byte unknown tails**. Two additional lists reach EOF, adding 43,478 exact-EOF bytes. Completed-list endpoint/tail invariants pass. `resolved-feature-list-frontier-0.219.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-242.

