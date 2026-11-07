# Ordinary dimension schema reference

Edition 0.341 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.27 Ordinary part dimension handles

*Status: Verified scoped handle layouts; underlying dimension/sketch payloads incomplete · Evidence: [C-42], F.30*

This clause covers moFeatureDimHandle_c, moSkDimHandleRadial_c, moSkDimHandleOffset_c and moSkDimHandleValG2_c in ordinary part archives, v > 0x27CB. Their RTTI base hierarchies include moDimHandle_c and moCompRef_c, and **do not include sgEntHandle**. Accordingly, the common reference wrapper in 11.16.1 reads no extra sketch-object pointer after these handle objects. The sketch pointer described below belongs inside the handle suffix. This distinction is established from inheritance and serializer evidence, not inferred from the word Sketch in a class name.

All four start with the common moCompRef_c body of 11.17.2, followed by the following moDimHandle_c base fields:

| Order | Fields | Type | Condition | Status |
|---|---|---|---|---|
| 1 | has_transform_120 | u16 | | V |
| 2 | transform_120 | direct mgXform_c body | field 1 != 0 | V / ? |
| 3 | drawing_view_tags | moDrViewTag_c object pointers[4] | | V / ?; nonnull payload unimplemented |
| 4 | is_external_feature_dimension | u8 | | V |
| 5 | external_reference | moGFR_c object pointer | | V / ?; nonnull payload unimplemented |
| 6 | reference_dimension | object | Scoped class is Dimension_c or one of the dimension classes in 11.28, or null. | V / ? |
| 7 | transform_108 | direct mgXform_c body | field 6 is a supported nonnull dimension | V / ? |
| 8 | has_string_110 | i32 | same as field 7 | V |
| 9 | unknown_110 | CString | field 8 != 0 | ? |
| 10 | reference_id | i32 | | V |
| 11 | has_transform_128 | u16 | | V |
| 12 | transform_128 | direct mgXform_c body | field 11 != 0 | V / ? |
| 13 | unknown_118 | i32 | | ? |

The two optional-transform presence fields are two bytes; the external-dimension flag is one byte; the optional-string presence field is four bytes. Any nonzero presence value enters its body. These widths SHALL be preserved. The native save keys identify IsExternFeatureDim and RefId. Stored flags and strings are retained, including values later changed by the native runtime.

The native base also has four direct points for drawing owners or an applicable special archive condition. The implemented ordinary part context has neither condition, so those points are absent here. Drawing/special contexts are not implemented by this decoder. Native loading only restores transform_108 and its optional string when a nonnull object is a Dimension_c instance; the scoped parser rejects other nonnull classes explicitly. The underlying nonnull dimension payload itself still requires its class decoder.

#### 11.27.1 Model and sketch suffixes

*Status: Verified scoped suffix framing; pointed-to payloads partial · Evidence: [C-42], F.30*

moFeatureDimHandle_c uses the moModDimHandle_c serializer. After the base, if external_reference is null, it stores one internal Dimension_c-derived object pointer, which SHALL be nonnull. With a nonnull external reference it adds no pointer and the native runtime marks the handle external. The schema preserves the original base flag instead of replacing it.

The three sketch-handle classes use moSkDimHandle_c::Serialize. After the base, a null external_reference is followed by an internal sgSketch pointer, which SHALL be nonnull in the implemented branch. A nonnull external_reference instead introduces an external_sketch_dimension_type u16. Either form is followed by sketch_dimension_id i32. Other archive conditions can use numeric owner handles or alternate virtual serialization; they are outside this ordinary part branch.

The wrapper accepts these four proven handle classes without adding an sgEntHandle-specific pointer. The schema preserves internal object identity and stored dimension IDs; it does not run the native sketch lookup, geometry update or dimension-value migrations. Clause 11.28 implements selected Dimension_c derivatives. moGFR_c, other dimension variants and sgSketch remain open, so this clause does not imply complete native dimension support.

### 11.28 Dimension payloads and typed parameters

*Status: Verified modern ordinary-archive layouts; field meanings and other referenced types partial · Evidence: [C-43], F.31*

This clause covers Dimension_c and the selected geometric classes reached from 11.27, for v > 0x27CB and ordinary part archive conditions. The common Dimension_c body stores:

| Order | Fields | Type | Condition | Status |
|---|---|---|---|---|
| 1 | reference_count | i32 | | V |
| 2 | parameter | object pointer | | V / ?; selected typed parameters below |
| 3 | tolerance | moDimTolerance_c object pointer | | V / ?; modern payload in 11.38 |
| 4 | am_object_id | i32 | | V |
| 5 | is_design_table_dimension | i32 | | V |
| 6 | mask, unknown_5c | i32 × 2 | | ? |
| 7 | attachment_point | moAttachPoint_c object pointer | | V / ?; nonnull payload unimplemented |
| 8 | driven_dimension | u16 | | V / ?; domain open |
| 9 | has_string_78 | u8 | v > 0x42EE | V |
| 10 | unknown_78 | CString | field 9 != 0 | ? |

The parameter field's helper reads exactly one object pointer; it then performs a runtime callback on a nonnull object. Reference count, amObjectID, IsDesignTableDim, Mask, AttachPoint and DrivenDim are named by the native save keys. The loader resets the runtime reference count and may ignore a tolerance depending on parameter state; the schema preserves the stored count and tolerance pointer. The driven field is two bytes and the optional-string presence field one byte. Both SHALL retain their stored width and value. Special archive conditions can add another i32 flag after these fields; that context is not implemented here.

#### 11.28.1 Shared geometric body and angle suffix

*Status: Verified scoped framing; geometric roles and angle word meaning partial · Evidence: [C-43], F.31*

moNumberDim_c, ParallelPlaneDistanceDim_c and ThreeDRadiusDim_c use the same geometric serializer. After the common body they store, in order:

| Order | Fields | Type | Condition | Status |
|---|---|---|---|---|
| 1 | point_80, point_a0, point_c0 | Point × 3 | | ? |
| 2 | has_vectors | u16 | | V |
| 3 | vector_e0, vector_e8 | Vector × 2 | field 2 != 0 | ? |
| 4 | has_transform | u16 | | V |
| 5 | transform_f0 | direct mgXform_c body | field 4 != 0 | V / ? |
| 6 | point_flags | u16 | | V / ? |
| 7 | point_98 | Point | field 6 & 1 != 0 | ? |
| 8 | point_b8 | Point | field 6 & 2 != 0 | ? |

Points/vectors are three f64 values each. No enclosing object tags precede these direct bodies. Any nonzero presence word includes its vectors or transform. The two point bits act independently; other bits are preserved without adding fields. Their meanings remain open, and accounting conservatively treats the whole point_flags word as unknown. Point/vector roles and units are not inferred from their member offsets.

AngleDim_c uses this same geometric body followed by unknown_f8 u16. The native loader replaces zero with 1; the schema preserves the original word. This suffix is not a double-angle value. Numeric-domain and angle-orientation meanings remain open.

#### 11.28.2 Integer, length and angle parameters

*Status: Verified modern bodies; units, flags and value-substitution workflows partial · Evidence: [C-43], F.31*

moLengthParameter_c and moAngleParameter_c share the modern floating-parameter layout of 11.24.2: node lightweight fields, a second moNodeName_c parameter-name pointer, value f64, has_alternate_value u8 and alternate_value f64 when the byte is nonzero. The angle serializer's additional legacy word occurs only before v 0x2DD and is absent from this scope. The primary virtual node stages for all three parameter classes in this clause are node lightweight plus a no-op resolved stage.

moIntegerParameter_c uses the same node/name prefix, then value **i32**, has_alternate_value **u8** and alternate_value **i32** when present. The alternate field is therefore four bytes for integer parameters and eight for floating parameters. Those types SHALL NOT be interchanged. The native runtime substitutes the alternate value when present and may normalize floating signs according to a virtual predicate; the schema retains all stored values and the original presence byte. Full parameter units and node-flag semantics remain open.

All pointers continue to share the inherited model object map. Unknown parameter, tolerance, attachment-point or geometry subclasses fail explicitly. The remaining sgSketch payload dependency is not bypassed or treated as an opaque completed model stream.

<a id="cl-facettess"></a>
### 11.125 Modern camera features and attachment points

*Status: Native serializer/constructor evidence; seven complete camera bodies in five assembly configurations · Evidence: [C-140]*

Scope: ordinary external-XML model archives, version greater than `27CB`, newly constructed `moCameraFeature_c`. Its primary vtable `4CE8C080` has Serialize at slot 5, `4B7B7120`. Both recovered constructors initialize `+5D0` to zero; after the `moModelFeature_c` base, the reader therefore invokes direct attributes `4B7B67F0` at `+378` with arguments `false, true`. The runtime reuse branch selected by a nonzero `+5D0`, older camera layouts and independent attribute serialization are outside this schema. The constructor selector is not an extra wire field.

A camera body consists of the model-feature base (feature body plus raw i32 `+354`) followed by these direct attribute fields. Offsets below are relative to the attributes record, not to the enclosing camera. The direct records reserve no MFC map slots.

| Order | Attribute offsets | Wire fields |
|---|---|---|
| 1 | `10,14,18,1C,20,24` | Six i32 words. |
| 2 | `88,90,98,B8` | Three f64 values, then i32. |
| 3 | `C0,C8` | Tagged attachment pointer, then f64. |
| 4 | `140,A0,160` | i32, point (3 f64), i32. The native copy from `A0` to `148` consumes no additional bytes. |
| 5 | `168,170` | Tagged attachment pointer, then f64. |
| 6 | `D0,D8,E0,E8,EC,F0` | Three f64 values, two i32 words, tagged reference pointer. |
| 7 | `F8` | Direct faceted reference, defined below. |
| 8 | `1D8,1E0,1E8,1F0` | i32, three f64 values. |
| 9 | `1F8` aspect specification | Two f64 values (width/height inputs), then i32 mode. Preserve zero denominators and raw inputs; native recomputation of a ratio consumes no bytes. |
| 10 | `218,220,228` | i32, f64, i32. |
| 11 | `230,238,240` | Tagged attachment pointer, f64, f64. |

The three attachment/value pairs use helper `4B7BBB90`: in this version scope each stores a tagged object and an f64. The loader accepts `moAttachPoint_c` derivatives and discards incompatible runtime types; the schema preserves the wire object when a complete reader exists, without fabricating a replacement. `F0` uses helper `4B7BBC60`, a tagged object without a following f64. Raw scalar meanings remain open and retain unknown-byte accounting.

The direct faceted reference (`4B7B5F90`) begins with a u32. Preserve all bits: low four bits select the payload, bit 4 is the stored enable flag, and bit 5 supplies the direction reversal flag for kind 3. After the selected payload comes one CString in every supported modern version.

| Low four bits | Payload before final CString |
|---|---|
| 0 | None. |
| 1 | i32 point count followed by that many points. |
| 2 | i32 point count followed by that many points, then i32 edge word. |
| 3 | Vector, then point. |
| 4 | Point, then vector. |

Negative or out-of-bounds point counts are errors. Other low-nibble values are unsupported rather than inferred from surrounding bytes. The native writer emits kinds 0–4; the schema retains the original u32 including higher bits. No runtime hit objects, transforms or computed identities created during load add bytes to this direct record. The final CString remains of unresolved application meaning.

A modern `moAttachPoint_c` body (`4C27A640`) stores two points (`+08,+28`), i32 (`+20`), a tagged drawing-view pointer (`+50`), two more points (`+58,+70`), and u8 (`+88`). Its ordinary model context has neither condition 1 nor condition 2 set; special contexts can add a CString and remain unsupported. `moParametricPoint_c` (`4C280A80`) adds two f64 values (`+108,+110`), a tagged reference (`+118`), u16 (`+124`) and i32 (`+130`). For versions greater than `335C`, drawing roots additionally store five points; the present reader requires an explicit part/assembly root and rejects drawing/unknown roots before consuming bytes. Retain object identities and raw words rather than normalizing them to runtime booleans.

`camerafeatures.py` installs through the archive-local environment registry. The model prefix supplies its explicit root class. The native assembly corpus now parses all **174** ordinary modern configurations through the layer-manager pointer: **4,878,787 cumulative prefix bytes**, including **3,394,014 unknown bytes**. It contains **seven** camera bodies totaling **4,570 bytes**, with **3,945 unknown bytes**; one camera has a parametric attachment, and all seven faceted records have kind zero. Populated faceted kinds 1–4, repeated attachment identities, raw flags, invalid counts and truncation have native code plus synthetic tests. Complete resolved assembly suffixes and full-document coverage remain open.

### 11.154 Ordinary modern sheet-metal, bend-allowance and fixed-face bodies

*Status: B — native-derived layered framing with native allowance/fixed-face boundaries; populated native numeric grids, raw meanings and additional sheet-metal classes remain partial.*

moSheetMetal_c is implemented in sldsmmu.dll, SHA-256 `ae20d533b36f105f22de4ed5984fb2245bbf03fb91a84ae473e0941a4b9e3c54`, image base 0x3D630000. Primary table `3DC63F38`, locator `3DC77398`, selects serializer `3DB692E0`. Runtime descriptor `3DDAFC50` selects factory `3DB56E50`, calling constructor `3DB50F00`. That constructor calls moBodyFeature_c and initializes the moSheetMetalCommon_c layer with direct moCommonBendData_c +800. Memory size 0x980 is not wire length.

The grammar below applies to ordinary non-DB external-XML model archives with v > 0x27CB and clear special archive conditions. An additional **archive-helper state at +850** controls several fields. The moArchiveHelperObject_c constructor in sldmfcu.dll (`3CA561B0`) zeros nonvolatile RBP and stores it to +850 at `3CA5658D`, proving the default state is zero. The reader uses this default unless the caller explicitly supplies sheet_metal_helper_850=1; it carries that state across the inherited-map resolved-stream handoff. No file-version heuristic or byte probe selects this state. Other values are rejected. 11.254 binds nondefault setup to configuration-manager member +0x650; decoding that persisted field and broader archive contexts remain incomplete.

After the complete body-feature base, read this common/derived sequence. Lists of offsets denote separate fields in wire order.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common bend +800 | Direct moCommonBendData_c body | Always | Defined below; no tagged object header | B |
| 2 | raw +840, +844 | Two i32 | Always in modern scope | Direct pair helper `3DB6BC90` | B |
| 3 | raw +848 | i32 | Always in modern scope | Original gate v >= 0x10C7 | B |
| 4 | raw +8F0; dimension +8F8; raw +900; dimension +908 | f64; tagged moDisplayDim_c pointer; i32; tagged moDisplayDim_c pointer | Always in modern scope | Preserve raw numeric values and actual dimension classes | B |
| 5 | fixed face +910 | Tagged moFixedFaceInFlat_c pointer | Always in modern scope | Native operator delegates to ReadObject | B |
| 6 | raw +930, +950; raw +958 | Two i32; CString | Always in modern scope | Preserve complete signed/string values | B |
| 7 | raw +960, +964, +968, +96C | Four i32 | Always in modern scope | Not pointer tags or array counts | B |
| 8 | raw +978, +974 | Two i32 | Helper +850 nonzero | Preserve this reversed member-offset order | B |
| 9 | raw +97C | i32 | Helper +850 nonzero and v >= 0x2BC5 | Independent version gate within the helper-controlled tail | B |

With helper state zero, null short pointers, no allowance body and an empty CString, the common/derived payload is **76 bytes**. Constructor-only points, pointer arrays and other runtime members between +850 and +938 are not implicit payload fields. Store preparation `3DB549A0` and final post-load processing do not add load fields. Native versions below 0x1BF reject this feature; early fixed-face/reference and dimension compatibility paths remain outside the supported modern scope.

The direct common bend body is serialized by `3DB5D540`, called from common-layer serializer `3DB68FB0`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | dimension +10; feature +18 | Tagged moDisplayDim_c pointer; tagged moCompFeature_c pointer | Always in modern scope | Two independent references | B |
| 2 | allowance presence | u8 | Always | Any nonzero byte selects the following direct body | B |
| 3 | allowance +08 | Direct moBendAllowData_c body | Presence nonzero | Not a tagged CObject pointer or i32 presence pair | B |
| 4 | raw +00, +04; discarded words 1, 2 | Four i32 | Always in modern scope | Native loads two words into locals; offline decoder preserves all four | B |
| 5 | sheet-feature reference +20 | Tagged moCompSheetmetalFeat_c pointer | Helper +850 nonzero | No bytes for this field with default helper state | B |

Store-side reference filtering and insertion-map updates under condition 0x200000 do not remove field 1 on ordinary load or add extra wire fields. Earlier loads before v 0xBB5 convert a generic feature pointer to a component reference; modern scope uses the component-feature grammar directly.

The optional direct moBendAllowData_c body is serialized by `3DB5BF00`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | arrays +50, +60, +70 | Three direct dynamic arrays | Always | Independently framed as described below | B |
| 2 | raw +38; raw +40; raw +00 | CString; i32; u8 | Always in modern scope | Runtime +00 is wider, but archive field is exactly one byte | B |
| 3 | dimensions +08, +10 | Tagged moDisplayDim_c pointer; tagged moDisplayDistanceDim_c pointer | Always in modern scope | Preserve nulls and shared identity | B |
| 4 | raw +30, +34 | Two i32 | Always in modern scope | Original version compatibility repairs are not applied | B |
| 5 | numeric table +78 | Product of array counts × f64 | Always | Shape is (count +50, count +70, count +60), with +60 varying fastest | B |
| 6 | raw +110, +100, +F8, +108 | Four CString | Always in modern scope | Preserve this ordering | B |
| 7 | discarded material-compatibility word | i32 | 0x2BAF <= v < 0x2BBA | Separate from the following tagged pointer | B |
| 8 | material parameters +128 | Tagged moSMParamsFromMat_c pointer | v > 0x2BAE | Unsupported fresh nested bodies remain explicit stops | B |
| 9 | string arrays +D0, +A8, +80 | Three direct su_CStringArray bodies | Always in modern scope | Independent ReadCount followed by CString items | B |

Each dynamic array begins with element_size u32. Zero terminates that array immediately and yields no count/header payload. A nonzero size SHALL be divisible by eight, matching native load helper `3DB580B0`; it is followed by three u32 metadata words (capacity, growth, count), then exactly element_size × count bytes. Reader wrappers `3DB58190`/`3DB581C0` establish this layout. Store-side growth metadata is limited to 1 or 2; the loader preserves the serialized metadata and bounds payload reads. The offline reader retains raw element bytes rather than assuming every element is a single f64. Array counts, not raw payload byte lengths, determine field 5 dimensions. The table product SHALL fit remaining eight-byte entries before allocation; zero dimensions produce no table bytes. Grid values, dynamic payload bytes, strings and unresolved numeric fields receive no semantic coverage credit.

moFixedFaceInFlat_c primary table `3DC02670`, locator `3DC6E868`, selects serializer `3DB5F8E0`. In modern scope, read either a tagged moOneBend_c pointer at +08 when helper +850 is zero, or an i32 bend identifier at +10 when helper +850 is nonzero. Then read raw +14 i32, tagged moCompFace_c +18, raw +20/+28 f64 and tagged moCompEdge_c +30. The helper-dependent identifier is **not an MFC tag**. The original nondefault-state identifier gate is v >= 0x1454, always satisfied in modern scope. Native store-side bend filtering and runtime ID repair are not applied to the retained wire representation.

Sheet-metal dimensions expose the dependency moScalerDim_c. Its sldmodu.dll primary table `4D241DD8`, locator `4D3736C0`, selects serializer `4C83D2C0`; RTTI includes Dimension_c. Native modern order is the Dimension_c base, three points, optional pair of vectors, optional compact transform and optional points selected by a u16 mask. This is the existing geometric-dimension grammar in 11.28. Scoped dispatch reuses that reader, and dimension-handle validation now accepts this proven class without weakening unknown-class stops.

The frontier contains **19 fresh sheet-metal features**, all under default helper state zero and all with direct allowance bodies whose grid shape is (0,0,0). Eighteen fixed-face pointers are null; one points to a native moFixedFaceInFlat_c helper. No positive native populated grids or nondefault helper state are claimed. Five focused tests cover populated synthetic grids, dynamic-array widths, material-compatibility boundaries, preserved strings/raw bytes, both helper branches, tail gates, pointer-versus-ID fixed-face framing, scalar-dimension dispatch, helper-state handoff, malformed inputs and a hash-bound native feature/fixed-face root end.

Edition 0.165 records **20,752 complete roots**, with **492 list-complete configurations** and **414 explicit stops** across the **906 complete modern model contexts**. Exact-EOF results remain **453 lists / 20,512,540 decoded bytes**, with **39 complete lists retaining four-byte unknown tails**. Eighteen newly reached paths stop at moSMBaseFlange_c; sweeps (17), face/point reference-plane data (16) and blend cuts (15) also remain leading dependencies. Full part, assembly, drawing and all installed project-format bit coverage remains incomplete. Completing these roots does not establish all field semantics or add whole-document coverage credit.

Evidence: C-182.

### 11.169 Reference-dimension handles and entity dimensions

*Status: B — modern native serializers, inheritance and populated corpus boundaries verified; raw fields and engineering validity remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`, excluding archive-condition 1 and drawing-owner tails. moRefDimHandle_c primary table `0x4CDA04B8` and moFeatureDimHandle_c primary `0x4D18C950` select the identical moModDimHandle_c serializer `0x4C83DA80`. Both inherit moModDimHandle_c. The scoped reference-handle reader reuses the existing model-dimension reader (11.27): component-reference base, dimension-handle helper `0x4C833BD0`, then a tagged internal Dimension_c-derived object when the inherited external reference is null. A null internal dimension is rejected. No reference-handle-specific suffix is inferred from a different runtime class name.

The handle helper retains its existing two u16 optional-transform flags, four drawing-view tags, external/reference-dimension pointers, optional reference-dimension transform/string, signed reference ID and raw suffix. Existing pointer identity and version/context restrictions remain in force. moRefDim_c, moRefDim1E_c and moRefDim2E_c have verified Dimension_c ancestry and are admitted to the supported internal/reference-dimension type lists.

moRefDim_c primary table `0x4D166EB8` selects `0x4C83FA20`. It calls the existing Dimension_c serializer, then reads:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | plane transform +80 | direct mgXform_c body | always | Existing transform restore framing; preserve decoded fields and complete serialized bytes. No pointer tag or transform presence prefix. | V / ? |
| 2 | reference-direction presence; vector +88 | u8; three f64 | vector only when byte nonzero | Preserve the original byte and vector. | V / ? |
| 3 | text-direction presence; vector +90 | u8; three f64 | vector only when byte nonzero | Independent of row 2. | V / ? |
| 4 | raw +A4; plane tag +A8; raw +B0 | i32; tagged moDimPlaneEntTag_c pointer; i32 | always | Preserve null/new/cached pointer framing. | V / ? |
| 5 | sketch-dimension handle +B8 | tagged moSkDimHandle_c pointer | always | Null is allowed; newly introduced unsupported derived types stop. | V |
| 6 | raw +A0/+C4/+C8 | three i32 | always | Exact wire order shown. | V / ? |
| 7 | raw +D0 | i32 | version >= 0x46C7 | Older modern versions omit the word completely. | V / ? |

The native store path names DB keys such as Plane and RefDir; ordinary binary load reads the fields above directly. These key names do not introduce CString fields in this scoped layout. Native updates and geometry normalization are not executed offline. The latest-version minimum reference-dimension base is **93 bytes**, including the 29-byte minimum Dimension_c body, 34-byte direct transform, absent vectors, null pointers and +D0. At version `0x27CC` the minimum is 88 bytes because both the Dimension_c post-`0x42EE` string-presence byte and +D0 are absent.

moRefDim1E_c primary table `0x4D1370E8` selects `0x4C83DE10`; moRefDim2E_c primary `0x4D137A48` selects `0x4C83E3A0`. Both call the shared reference-dimension base before their entity suffixes:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | one-entity kind +D8; entity +E0 | u16; tagged object pointer | one-entity class | Retain the kind word and original pointer, including null. | V / ? |
| 2 | one-entity raw +F0; raw +F4 | u16; conditional u16 | +F4 only version >= 0x2F6F | No widening to native in-memory integer fields. | V / ? |
| 3 | first kind +D8; entity +E0; second kind +F0; entity +F8 | u16; pointer; u16; pointer | two-entity class | Both pointers use ordinary MFC identity; repeated objects remain repeated references. | V / ? |
| 4 | two-entity raw +108/+110/+114/+118/+11C | five u16 | two-entity class | Preserve all five occurrences and their raw values. | V / ? |
| 5 | two-entity raw +10C | u16 | two-entity class and version > 0x3747 | This final word follows +11C despite its lower member offset. | V / ? |

The one-entity and two-entity alternatives are separate suffixes; rows 1–2 do not precede rows 3–5. Latest minimum bodies are **101 bytes** and **113 bytes** respectively. All entity words are two bytes, even where native loading expands them into four-byte members. The old `0x141`–`0x159` drawing-view-tag reads and other historical migration branches are excluded. Native loading can replace a null entity's kind with 8 and normalize zero +118 to one; offline reading retains the serialized values. Kind-specific class checks and runtime suppression/conversion behavior do not establish general engineering validity for arbitrary synthetic combinations.

The complete-root audit retains **10 reference-dimension handles**, with **nine moRefDim2E_c dimensions and one moRefDim1E_c**. All ten have null external/reference-dimension pointers, reference ID -1 and zero handle transform flags. Containing-root tests verify stream SHA and exact boundaries for both populated dimension classes. Synthetic tests cover raw entity kinds and repeated pointers, signed words, independently present vectors, direct-transform bytes, version gates, truncation, context rejection and internal/external handle paths.

Edition 0.180 records **22,231 complete roots**, **647 list-complete configurations** and **259 explicit stops** across **906 complete modern contexts**. **593 lists reach exact EOF**, spanning **31,229,297 decoded bytes**; **54 complete lists retain four-byte unknown tails**. Structural-member features, coordinate systems, drafts, profile references, additional dimension/entity types and raw meanings remain open. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; these record boundaries receive no whole-document coverage credit.

Evidence: C-197.

### 11.188 G3 valued-dimension handles

*Status: B / ? — native shared serializer and eight populated containing-root handles verified; external/type domains and runtime dimension migration retain existing limits.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`, with special handle conditions 1, 2 and 4 absent. moSkDimHandleValG3_c primary table `0x4D1BAE08`, locator `0x4D3647A0` and serializer slot +28 select `0x4C840400`, the same serializer selected by the supported moSkDimHandleValG2_c primary table `0x4D1B8E18`. Both inherit moSkDimHandle_c and moCompRef_c. The serializer first calls common dimension-handle helper `0x4C833BD0`; the supported reader reuses `dimensionhandles.sketch_dimension` without adding a G3-specific suffix.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | dimension-handle base | direct common body | always | Existing component reference, optional transforms, four view-tag pointers, external-feature byte, external/reference-dimension pointers, reference ID and raw continuation fields. Retain all inherited version/presence rules. | B / ? |
| 2 | sketch owner | tagged nonnull sgSketch pointer | base external-reference pointer is null | Preserve cached/new identity and reject null or unsupported owner class. | B |
| 3 | external sketch dimension type | u16 | base external-reference pointer is nonnull | Alternative to row 2, not an additional field. Preserve the stored word without applying runtime type migration. | V / ? |
| 4 | sketch dimension ID | i32 | always after selected row 2 or 3 | Native widens this value in runtime state. Retain the original signed word. | V / ? |

The latest-version empty common dimension-handle base is 107 bytes. With a cached internal sketch pointer or a two-byte external type word, the body is 113 bytes before outer tagged framing. Nonempty inherited references/transforms and newly tagged sketch owners add their own bodies; 113 bytes is not a general skip length. Runtime subtype checks, dimension lookup and old ID/value migrations do not justify modifying the stored ID during offline parsing.

Native-proven moCompRef_c ancestry admits this class as a component-reference pointer in `modelreferences.wrapper`. Both the reader and this subtype admission preserve the class's distinct runtime name; the global reader registry is unchanged. A special mode that invokes virtual +778 instead of the ordinary continuation remains outside this scope.

The completed-root audit retains **eight G3 handles**, all with internal sgSketch owners, null external-reference pointers and null reference-dimension pointers. Seven store dimension ID 1 and one stores ID 0. The external type-word branch has synthetic framing coverage but no populated native example in this cohort. Three focused tests cover internal owner identity, external word width, signed ID extrema, wrapper admission, cached outer identity, null-owner/truncation rejection and a source-hash-bound complete native containing-root span with ID equality.

Together, clauses 11.187–11.188 advance the frontier to **23,470 complete roots**, **715 complete lists** and **191 explicit stops** across 906 complete modern contexts. **650 lists reach exact EOF**, spanning **36,850,025 decoded bytes**; **65 complete lists retain four-byte unknown tails**. Every complete list passes the recorded stream-size endpoint/tail invariants. Relative to Edition 0.196 this adds 63 complete roots and ten exact-EOF lists. `resolved-feature-list-frontier-0.196.json` retains the preceding checkpoint. Reference-plane variants, thin extrusions and further identity representations remain leading dependencies. Whole ResolvedFeatures, part, assembly, drawing and all installed-project bit coverage remain incomplete.

Evidence: C-216.

