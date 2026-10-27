### 11.15 Modern feature and folder records

*Status: Partial meanings; verified ordinary archive layout; conditional archive modes open · Evidence: [C-26], [C-29], F.17*

This clause covers v > 0x27BC in the ordinary model-reading context with external XML extraction, no clipboard/import pointer pair, no archive-noise byte and no suppressed resolved-node-list stage. These conditions come from archive/runtime state, not a per-record flag. Other modes SHALL NOT be guessed from nearby byte values. The implementation requires an explicit `ordinary_external_xml` context and grants no whole-stream coverage.

#### 11.15.1 Common feature body

The archive-condition predicate is documented by C-33: with no helper it returns zero. For mask zero it tests whether the stored flags are zero. For any nonzero mask it returns flags & mask, so a truth test means **any masked bit**, not all bits. Composite masks in the serializers SHALL be interpreted accordingly.

*Status: Verified scoped layout; most member meanings open · Evidence: [C-26], [C-29], F.17*

The common feature body begins with the node lightweight body (11.11) and the concept lightweight continuation (11.13). When the native class has a child-feature-list container, a direct resolved feature node list follows (ReadCount, objects, final i32). Classes whose constructors leave that container null omit the whole direct list, including its count and final word; attributes and attribute definitions are proven examples (11.140). This is class/runtime state, not a presence byte inferred from the stream. The following fields are then stored in order:

| Order | Fields | Types | Condition | Status |
|---|---|---|---|---|
| 1 | `unknown_290`, `unknown_284`, `unknown_294` | i32, i32, u32 | | ? |
| 2 | `has_unknown_1e0` | u8 | | ? |
| 3 | `unknown_1e0` | CString | has_unknown_1e0 != 0 | ? |
| 4 | `unknown_2a0`, `unknown_2b0`, `unknown_2b4`, `unknown_2a8` | i32 × 4 | | ? |
| 5 | `unknown_1d8`, `unknown_2ac` | f64, i32 | | ? |
| 6 | `unknown_250` | object | | ? |
| 7 | `unknown_1f0`, `unknown_1ec` | CString, u8 | | ? |
| 8 | entity visual properties | direct body | moEntVisProp_c, shared with 13.6: two u16, u8, u32, layer u16, line-width body. | V / ? |
| 9 | `unknown_2e4`, `unknown_a0`, `unknown_2e8` | i32, i32, object | | ? |
| 10 | `unknown_200`, `unknown_2f8`, `unknown_2f9` | u32, u8, u8 | | ? |
| 11 | `unknown_310`, `unknown_hidden`, `unknown_304` | i32 × 3 | | ? |
| 12 | `unknown_29c`, `unknown_298` | u32 × 2 | Stored high/low FILETIME words passed to setFeatTimeStampForConfigs; domain/uses remain open. | ? |
| 13 | `unknown_90`, `unknown_2a4`, `unknown_33c` | u16, i32, i32 | | ? |

The final XML stage can read a CString in certain inline-XML archive modes. With external extraction it exchanges tags, names and descriptions with the XML extractor instead of consuming that CString from this record. The scoped reader does not infer an inline value or an optional noise byte. Unknown object fields preserve graph references; their bytes are counted once as unknown, including any nested unknown bytes.

#### 11.15.2 Folder suffixes

*Status: Verified scoped layouts; member meanings open · Evidence: [C-29], F.17*

`moFolder_c` and `moCommentsFolder_c` append `unknown_358` and `unknown_368`, both i32, after 11.15.1. `moFtrFolder_c` and `moMateReferenceGroupFolder_c` then append `unknown_380` object, `unknown_374` i32, `unknown_38c` i32 and `unknown_394` i32. Their verified IsMateFolder virtual function returns zero, so the conditional member +0x370 is absent. Do not apply this suffix to every derived folder without checking that predicate.

#### 11.15.3 Position-group folder

*Status: Verified scoped layout; reference bodies and meanings partial · Evidence: [C-29], F.17*

`moPosGroupFolder_c` inherits the common feature body through `moModelFeature_c`, not through the folder suffix in 11.15.2. The model-feature stage appends `unknown_354` i32; its older migration helper consumes no bytes in this version range. The position-group suffix is:

| Order | Fields | Types | Condition | Status |
|---|---|---|---|---|
| 1 | `unknown_380` | CString | | ? |
| 2 | `direction_count` | u16 | | ? |
| 3 | `unknown_388`, `unknown_3a0` | u16[direction_count] × 2 | Two consecutive arrays, not interleaved pairs. | ? |
| 4 | references | ReadCount, object[count] | Direct suObArray body, no final node-list word. | V / ? |
| 5 | `unknown_3a8`, `unknown_3ac` | i32 × 2 | | ? |
| 6 | `unknown_3b0` … `unknown_3b3` | u8 × 4 | v >= 0x3EEB | ? |

The native loader has three slots per direction array; it reads min(direction_count, 3) words for each and fills unused runtime slots with zero. The implemented schema accepts counts through 3 and explicitly rejects larger values; noncanonical larger-count writer/loader behavior remains open. The raw flag bytes are preserved even though the native loader normalizes them to boolean values. The reference-array elements include the unresolved geometric-reference classes listed in F.17.


### 11.166 Flat patterns, bounding boxes and fold folders

*Status: B — native modern field order, member types and absent-body corpus boundaries verified; selected present compressed raw bodies are verified in 11.244; other raw forms remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moFlatPattern_c primary table `0x3DC2B130` selects serializer `0x3DB5FBC0` in sldsmmu.dll. It first calls bend-group helper `0x3DB68EC0`: the body-feature base followed by the common bend-group layer of 11.155. Member offsets identify native fields, not stream offsets. After that layer, the suffix is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | array +948 | direct suObArray body | always | ReadCount followed by tagged pointers; no outer class tag or map entry. | V |
| 2 | references +8F8/+900/+908 | three tagged moCompBEntRef_c pointers | always | Preserve null, new and cached identity. | V |
| 3 | raw +9B0; raw +9B4 | i32; conditional i32 | +9B4 only version > 0x372B | Preserve full signed words. | V / ? |
| 4 | array +968 | direct suObArray body | always | Same framing as row 1. | V |
| 5 | raw +9BC/+9C0/+9F0/+9C4; raw +9C8 | four i32; f64 | always | Exact order shown, including +9F0 before +9C4. | V / ? |
| 6 | raw +9E0; raw +9D8; raw +9E4/+9D0/+9EC/+AA8 | i32; f64; four i32 | always | Preserve raw values without assigning enum meanings. | V / ? |
| 7 | feature +9F8; list +A38 | tagged moCompFeature_c; tagged suObList | always | These are pointers, unlike the direct arrays. | V |
| 8 | body presence +A08; optional raw body | i32; native raw kernel archive | body only when presence nonzero | Calls mo_loadBodyFromArchiveRaw directly, without suppression u8. The selected strict compressed form is decoded in 11.244; other forms stop explicitly. | B |
| 9 | raw +988; array +990 | i32; direct suObArray body | always | Continue only after an absent or independently verified complete raw body. | V / ? |
| 10 | face +920 | tagged moCompFace_c pointer | version > 0x3364 | Older modern versions omit this pointer. | V |
| 11 | raw +928; string +930 | i32; CString | word only version > 0x3761; string only when word nonzero | Preserve the complete signed word and string. | V / ? |
| 12 | reference +910; bounding box +918 | tagged moRefWrapper_c; direct moFlatPatBoundBox_c body | always | Box uses a direct virtual call without a pointer tag or map slot. | V |
| 13 | raw +A28/+A2C/+A30/+A34 | four i32 | always | Preserve all four occurrences. | V / ? |
| 14 | raw +A80 | i32 | explicit archive helper +850 nonzero | Same proven helper state as 11.154; unsupported states are rejected. | V / ? |
| 15 | pair count; ordered pairs | i32; count × two i32 | always | Reject negative/impossible counts. Preserve duplicate first words and order; native C++ map insertion can collapse keys. | V / ? |

Runtime descriptor `0x3DDAF0B0` selects factory `0x3DB55CC0`, which allocates `0xB38` bytes and calls constructor `0x3D936870`. That constructor installs suObArray vtables at +948, +968 and +990. Native loading removes some suppressed moUiBend_c nodes from +948 after reading; offline reading retains serialized entries without executing this filtering.

moFlatPatBoundBox_c primary table `0x3DC1C4C8` selects `0x3DB5FB40`. After its zero-byte ordinary binary su_CObject layer it directly serializes the object referenced by member +10. Constructor `0x3D8484C0` proves that member is an allocated moBoundBoxData_c. Another embedded data object at +20 and runtime points are not additional serialized bodies in this function. moBoundBoxData_c primary `0x3DC1C430` selects `0x3DB5CD30`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | vectors +50/+68 | two sets of three f64 | always | x, y, z order in each set; engineering interpretation remains open. | V / ? |
| 2 | raw +28/+30/+38/+40/+80/+88 | six f64 | always | Preserve native wire order. | V / ? |
| 3 | raw +90; raw +48 | i32; f64 | always in modern scope | Final double begins at version 0x1FDE, before this scope. | V / ? |

The data body is 108 bytes with no class tag through either direct bounding-box call. Tagged instances still use ordinary MFC class/object-map framing. The latest-version empty flat-pattern suffix, with absent body, null pointers, empty arrays, zero +928, helper +850 zero and no pairs, is 220 bytes, excluding inherited layers. Versions `0x27CC` through `0x3364` omit ten bytes; the +920, +9B4 and +928 gates then add two, four and four bytes respectively.

moFoldUpDownSketchFolder_c primary table `0x3DC0A810` selects `0x3DB60D70`: existing moFolder_c serializer (11.15), then tagged moFoldUpDownSketchXformData_c at +568 and moFoldUpDownFeatXformData_c at +570. The second pointer begins after version `0xCA0`, so both occur throughout this modern scope. Their transform payloads are not established by this serializer.

The edition 0.177 audit reached **moFoldUpDownSketchXformData_c in all 12 prior flat-pattern stops**, leaving **22,129 roots**, **631 complete lists**, **275 explicit stops**, **577 exact-EOF lists** and **54 complete lists with four-byte unknown tails** across **906 modern model contexts**. Edition 0.178 resolves the fold-transform layouts (11.167) and verifies 12 completed native flat-pattern boundaries with absent raw bodies. Selected present compressed raw-body framing is verified in 11.244. Other raw forms, raw meanings, historical/special contexts and full part/assembly/drawing/all-installed-project bit coverage remain open. No whole-document coverage credit is assigned.

Evidence: C-194.


### 11.167 Fold and unfold transform data

*Status: B — modern native containers and empty corpus boundaries verified; populated wire order tested synthetically; engineering semantics remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moFoldUpDownSketchXformData_c primary table `0x3DC0A780` selects `0x3DB60EC0`. Runtime descriptor `0x3DDAF1A0` selects factory `0x3DB55E10`, which allocates `0x188` bytes and proves direct suObList members at +10/+58/+A0 and a direct su_CByteArray at +108. Native wire order differs from member-offset order:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | lists +58/+A0 | two direct suObList bodies | always | Each is ReadCount followed by tagged object pointers; no outer tag or map slot. | V |
| 2 | transform count; transforms +E8 | i32; count × direct mgXform_c restore body | always | Reject negative counts or counts exceeding the remaining minimum-size bodies. Each transform is at least 34 bytes. Preserve original serialized bytes and decoded fields. | V / ? |
| 3 | property count; associations +158 | i32; count × two tagged pointers | always | Each pair is moCompFeature_c followed by moFoldUpDownSketchXformPropertyData_c. Preserve ordered pairs, repeated keys and pointer identity. | V |
| 4 | byte array +108 | ReadCount; count raw u8 | always | No per-byte tag or size stride. Retain all raw values as unknown meaning. | V / ? |
| 5 | list +10 | direct suObList body | always | Follows the byte array without alignment or padding. | V |

The obsolete property map at +128 occurs only for versions greater than `0xC85` and less than `0xC87`; it is entirely absent in this modern scope. The +158 map follows in every modern version. Native loading uses setIfNotSet, which can discard duplicate keys in memory; the offline reader retains every serialized association. The property object's own payload remains an explicit dependency when a new instance occurs; this container does not imply its layout.

moFoldUpDownFeatXformData_c primary table `0x3DC09E38` selects `0x3DB60C40`. Runtime descriptor `0x3DDAF200` selects factory `0x3DB55F10`, which allocates `0xC0` bytes and proves direct suObList members at +10/+58. Its complete wire order is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | list +58 | direct suObList body | always | ReadCount and tagged items. | V |
| 2 | transform count; transforms +A0 | i32; count × direct mgXform_c restore body | always | Same geometry framing and conservative unknown accounting as the sketch-transform collection. | V / ? |
| 3 | list +10 | direct suObList body | always | No property count, byte array or extra suffix. | V |

Both serializers import mgXform_c::restore through IAT `0x3DBDAD68`, bound to sldmgu.dll export `0x321031D0` and the existing geometry reader. Its body is u8 matrix presence, optional nine-f64 matrix, three-f64 translation, f64 scale and u8 flags, followed by the existing perspective/matrix extension selected by the high flag nibble. The offline reader preserves the complete serialized transform in addition to decoded fields, including values discarded by shared geometry helpers; it counts all transform bytes as unknown semantics in these containers. No runtime transform is executed. Direct transforms allocate no archive object slots.

su_CByteArray primary table `0x31D50E78` selects serializer `0x31D3BA70` in swccu.dll. Exact native load instructions call ReadCount `0x31D36AD0`, resize the array and tail-call raw Read `0x31D368C0` with that element count. Thus an empty byte array occupies a two-byte packed count, rather than an i32 count. The empty sketch-transform body is **16 bytes**; the empty feature-transform body is **8 bytes**. Tagged enclosing instances use ordinary MFC class/object-map framing.

The complete-root audit now retains **12 flat patterns**, **12 sketch-transform records** and **12 feature-transform records**. All transform lists/counts, property maps and byte arrays are empty in these native samples. All flat-pattern arrays/pair maps and raw-body presence are zero, bounding-box data is zero, and raw +988 is one in 11 records and zero in one. These observations establish native boundaries for these forms, not populated transform/property validity or raw engineering meanings.

Edition 0.178 records **22,141 complete roots**, **643 list-complete configurations** and **263 explicit stops** across **906 complete modern contexts**. **589 lists reach exact EOF**, spanning **31,023,044 decoded bytes**; **54 complete lists retain four-byte unknown tails**. Weldments, coordinate systems, drafts, reference-dimension handles and other feature dependencies remain unresolved. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these record boundaries.

Evidence: C-195.


### 11.253 Fold/unfold sketch-transform property data

*Status: B / ? — native class/serializer binding, two populated records and complete containing Flat Pattern root verified; property scalar domains remain unresolved.*

`moFoldUpDownSketchXformPropertyData_c` selects serializer `0x3DB61310` through primary vtable `0x3DC0C540` and locator `0x3DC6F7F0` in hash-bound installed `sldsmmu.dll`. The native body constructs an archive-helper wrapper, then directly reads or writes these six fields in order:

1. i32 associated with runtime `+0x08`.
2. i32 associated with runtime `+0x0C`.
3. i32 associated with runtime `+0x10`.
4. **u32** associated with runtime `+0x14`.
5. CString associated with runtime `+0x18`.
6. CString associated with runtime `+0x20`.

The native instructions use the signed `long` archive overload for the first three words and the unsigned `ulong` overload for the fourth. The decoder preserves this distinction, including unsigned values above 0x7FFFFFFF. The wrapper adds no bytes. This serializer calls no inherited payload serializer and has no version/condition-dependent fields in its native body. In the supported ordinary modern context, its minimum body is **18 bytes**: sixteen scalar bytes and two one-byte empty CString lengths. CString encoding and length escapes follow 11.4, including UTF-16 strings. Tagged object/class headers and fresh containing objects are additional bytes.

The existing sketch-transform record stores a signed i32 property-entry count followed by pairs of tagged component-feature and tagged property-data pointers. Each fresh property object is entered in the shared archive map before its six-field body. Repeated pointers retain their object identity; the property body itself adds no further pointer/map slots. This implementation adds the missing data reader to that established composition rather than changing the parent count or pointer framing.

In `samples/tutorial/costing/sheet_metal_part.sldprt`, the completed Flat Pattern root spans decoded offsets **399,411..439,754**, totaling **40,343 bytes**. It contains **two fresh property records**. Both store raw signed words **3,4,1** and unsigned word **8,421,504**. Their first strings are **`Sketch20-1`** and **`Sketch10-2`**; both second strings are empty. The raw word's numeric appearance alone does not establish a color or flag interpretation. `fold-sketch-property-targeted-corpus.json` retains source-stream/root SHA-256 values and all six fields for both objects.

The costing sample's declared feature list now ends exactly at its **439,754-byte** decoded stream boundary. This completes the previously blocked Flat Pattern root after the embedded forming-tool work in 11.252. It is a feature-list framing result, not proof that all native file bits or property meanings are understood. Semantic meanings of the scalar words and string roles, further archive contexts and complete part/assembly/drawing/all-installed-project bit coverage remain open. No whole-document semantic credit is assigned.

The frozen 0.272/0.273 audit preserves every prior complete root record across 906 contexts and adds this one completed root. Totals are **25,521 complete roots**, **828 complete lists** and **78 explicit stops**. The additional exact-EOF list adds **439,754 decoded bytes**, giving **750 exact-EOF lists / 62,819,665 decoded bytes**, plus **78 complete lists with four-byte unknown tails**. These are framing measurements and add no whole-document semantic coverage credit.

Two focused tests verify signed/unsigned extrema, Unicode and narrow CStrings, empty minimum framing, truncation/context rejection, both source-bound native property values and the complete containing root/list endpoint. The embedded-part regression now asserts the completed Flat Pattern at its former property stop. Historical embedded-part research remains frozen to checkpoint 0.272.

Validation: all **921 regression tests pass** (100.365 seconds); specification lint reports zero findings (477 headings, 96 anchors).

Evidence: C-295.

