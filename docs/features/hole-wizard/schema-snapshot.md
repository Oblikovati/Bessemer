### 11.89 Atom partition/maps, late fields and body-feature suffix

*Status: Native serializer evidence; nine complete atom/body-base boundaries verified; enclosing extrusion suffix incomplete · Evidence: [C-104]*

In the scoped ordinary zero-condition archive, the continuation after 11.88 begins with a nullable direct array at +0x548, then i32 partition presence. A nonzero presence adds a tagged moPMarkRecord_c pointer and three i32 values at +0x3B8, +0x3C0 and +0x578. The mark record's body is one i32 permanent-mark ID. Runtime pointers and cached kernel objects in that class add no archive fields. Readers SHALL preserve the signed ID and raw presence value.

Next is i32 word_5C8 and three maps in order. Map +0x670 stores i32 count and count pairs of u32 key/u32 value. Map +0x5D0 stores i32 count and count pairs of tagged moCompEdge_c pointer/u32 value. Map +0x3F8 stores i32 count and count pairs of u32 key/u32 value. Readers SHALL preserve duplicates, pointer identity, unsigned high bits and original entry order independently of native hash-map insertion.

The late atom fields are i32 word_488, i32 appearance_count and appearance_count pairs of i32 key/tagged moVisualProperties_c pointer, then i32 direct-data presence at +0x6A0. Nonzero introduces an additional direct serializer that remains unsupported; readers SHALL stop explicitly there. Zero is followed by i32 face-name count and pairs of tagged moCompFace_c pointer/CString, then i32 index-name count and pairs of i32 key/CString, and final i32 word_6B4. All these fields are present in the documented modern scope. Legacy branches and nonzero archive masks may differ.

This completes the atom-feature helper. The ordinary moBodyFeature_c continuation is tagged moAsmFeatData_c pointer, tagged moCompFeature_c pointer, direct embedded suObArray at +0x778 and i32 word_798. The component-feature pointer is omitted under archive condition 1, outside this zero-condition path. These bytes precede the boss/extrusion-specific moEndSpec_c suffix in 11.81. Unsupported nested objects still stop the containing reader.

The dedicated map, late and body-tail audits each reach nine native boundaries. All nine cases store partition records; the three maps contain zero entries. The optional late direct-data body is absent in these cases. Twelve profile objects complete during traversal, while three other enclosing cases still stop on cosmetic-thread objects. These results verify atom and body-feature base framing, not complete boss/extrusion objects or streams. Default complete manager count remains 892; whole-document bit coverage is unchanged.

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

