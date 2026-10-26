# Ordinary display dimension schema reference

Edition 0.356 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.45 Distance display-dimension suffix

*Status: Native-derived ordinary modern suffix; synthetic tests; complete display base pending · Evidence: [C-60]*

moDisplayDistanceDim_c delegates to the shared moDisplayDim_c serializer before writing the following suffix. This clause starts after that complete base, not at the tagged-object body boundary. The base itself delegates to moDisplayItem_c in the scoped modern versions and directly serializes its embedded moDimText_c; its remaining layout is unresolved.

| Order | Field | Encoding | Presence |
|---|---|---|---|
| 1 | DDMask | i32 | always in scoped v > 0x27CB |
| 2 | BentPts0 parameters | two f64 values | always |
| 3 | bent point at runtime +0xFE8 | three-f64 point | always |
| 4 | SelectedFirst | i32 | always |
| 5 | ParentRadialDim | tagged moDisplayRadialDim_c pointer | ordinary archive condition 2 absent |
| 6 | unknown_1010 | i32 | v > 0x3747 |

Readers SHALL preserve signed integer values and all five doubles. DDMask bit meanings, bent-parameter interpretation and unknown_1010 remain partial. When archive condition 2 is present, the parent field instead uses an eight-byte native reference representation; that alternate context is outside this decoder's scope. Native compatibility updates after the suffix add no direct archive reads. The radial pointer can be shared or cyclic, so complete readers need the same preallocated MFC object map used by the enclosing display object.

The suffix decoder is deliberately separate from the whole-object registry because skipping the unresolved display base cannot establish its starting position. No installed native suffix boundary or increased whole-stream coverage is claimed.

### 11.46 Shared display-object prefix

*Status: Native-derived ordinary modern prefix; 32 installed prefixes verified; remainder partial · Evidence: [C-61]*

In the scoped v > 0x27CB ordinary archive context, moDisplayItem_c first delegates to the display-object serializer. Its initial body is:

| Order | Field | Encoding |
|---|---|---|
| 1 | word_50 | u16 |
| 2 | owner at +0x30 | tagged object pointer |
| 3 | view at +0x40 | tagged moDrView pointer |
| 4 | component reference at +0x48 | tagged moCompRef_c pointer |
| 5 | unknown_54; unknown_11c | two i32 values |
| 6 | visual properties at +8 | direct moEntVisProp_c body (13.6) |

Readers SHALL preserve word_50 and the two signed integers without inferring their domains. Pointers use the enclosing object's MFC map, including references to an already allocated enclosing model. Native loading validates nonnull view and component-reference classes. In archive condition 1, a data-location record precedes this prefix; that alternate context is outside the scoped reader.

Two direct embedded serializers at runtime +0x68 and +0xB0 follow the visual body when condition 1 is absent. Their types/layouts remain unresolved. Subsequent display-object and display-item fields, followed by the display-dimension base and derived suffix, still require decoding. This prefix cannot be registered as a complete display-object reader.

The corpus audit reaches this boundary in all 32 previously reachable sketches: 1,056 display-prefix body bytes (33 per first display item), with cumulative archive positions totaling 395,770 bytes. The other six cases retain their earlier feature-body stops. These positions are partial traversal evidence and add no whole-stream coverage.

### 11.47 Display-object reference lists and continuation

*Status: Native-derived ordinary modern body without optional link data; 32 installed boundaries verified · Evidence: [C-62]*

The two embedded objects following 11.46 are moDispObjRefList_c at runtime +0x68 and +0xB0. Each delegates to standard suObList serialization: ReadCount followed by tagged object pointers, with no enclosing list tag or list map slot. Pointer items continue to allocate or reference slots normally.

moDispObjRef_c stores four i32 values, corresponding to runtime members +8, +0xC, +0x10 and +0x14. Their interpretation remains partial. moCompDispObjRef_c extends that body with one tagged component pointer. Readers SHALL retain all four signed values and the pointer identity. A null component pointer is still present on the wire even though native loading leaves its initialized runtime member unchanged.

After both direct lists, the ordinary modern display-object base continues with:

| Order | Field | Encoding |
|---|---|---|
| 1 | unknown_58 | i32 |
| 2 | linked_objects | tagged moLinkedDispObjArray_c pointer |
| 3 | has_link_data | i32 |
| 4 | optional link-data body | present when has_link_data is nonzero; unresolved |
| 5 | stored_version at runtime +0x118 | i32 |

The scoped decoder stops explicitly on nonzero has_link_data and on unsupported linked-array payloads. Alternate archive conditions 1, 0x8000 and 0x10000000 affect reference/link handling and remain outside this ordinary-context layout. The listed stored-version field completes the display-object base when optional link data is absent; display-item serialization then resumes. This is not the end of a display-dimension object.

All 32 first display objects in the reachable installed sketches have both direct lists empty, linked_objects null and has_link_data zero. Each complete display-object base consumes 51 bytes, totaling 1,632 body bytes; cumulative archive positions total 396,346. Populated lists and references currently have native-code and synthetic evidence only. No complete sketch or whole-stream coverage is claimed.

### 11.48 Display-item initial geometry

*Status: Native-derived ordinary modern prefix; 32 installed boundaries verified; arrow-size collection and later fields unread · Evidence: [C-63]*

After the display-object base of 11.47, moDisplayItem_c stores the following in the scoped ordinary v > 0x27CB context:

| Order | Field | Encoding |
|---|---|---|
| 1 | unknown_390; flags_3e0 | two i32 values |
| 2 | position at +0x290 | three-f64 point |
| 3 | has_array_298; array_298 | i32 presence; when nonzero, ReadCount and tagged object pointers |
| 4 | has_object_288; object_288 | i32 presence; when nonzero, unresolved direct object body |
| 5 | component_feature | tagged moCompFeature_c pointer |
| 6 | vector_400; transform_2a0 | three-f64 vector; direct mgXform_c |
| 7 | indices_2b0 | i32 count; i32 values[count] |
| 8 | has_vector_2c0; vector_2c0 | u8 presence; optional three-f64 vector |
| 9 | unknown_2c8 | f64 |
| 10 | has_transform_2d0; transform_2d0 | i32 presence; optional direct mgXform_c |
| 11 | leaders | i32 count; tagged moDisplayLeader_c pointers[count] |
| 12 | object_2d8; unknown_3b8 | tagged object pointer; f64 |

The optional array at order 3 uses direct suObArray pointer serialization, not a tagged suObArray object. Its presence integer is separate from its count; neither array presence nor direct array creation allocates an MFC map slot. Items do use that map. Readers SHALL retain raw presence values, signed integers, array order and duplicate index values even though native index loading inserts into a set. The scoped decoder rejects negative or impossible counts and stops explicitly when has_object_288 is nonzero. Native loading normalizes some flags and performs owner/leader updates; these runtime operations do not change the stored fields above.

The first 32 reachable display items have null array_298 and absent object_288 and reach order 12; their leaders are empty. The intermediate checkpoint after order 4 totals 397,626 cumulative archive bytes. The checkpoint after order 12 totals 400,550 bytes, including 5,836 bytes from the start of the first display-object bodies. Optional vector/transform populated branches also have synthetic tests. Arrow-size records follow; this prefix is not a complete display item, dimension or sketch.

### 11.49 Display-item arrows and initial line styles

*Status: Native-derived modern fields; 32 installed prefixes verified; populated arrows synthetic only · Evidence: [C-64]*

After 11.48, the scoped ordinary v > 0x27CB display item stores an i32 arrow-map count. Each entry is an i32 index followed by a tagged moArrowSize_c pointer. moArrowSize_c itself stores three f64 values corresponding to runtime +8, +0x10 and +0x18. Their individual meanings remain partial. Readers SHALL retain entry order, duplicate keys, signed indices and pointer identity rather than reconstructing only the runtime map.

The following style fields use runtime offsets as names where semantics remain unresolved:

| Order | Fields | Encoding |
|---|---|---|
| 1 | word_2f0; width_2f8; word_2f4; word_304; width_30c | i32; line width; two i32; line width |
| 2 | word_308; word_300; word_314; word_334; word_338 | five i32 |
| 3 | has_array_4a0; array_4a0 | i32 presence; when nonzero, ReadCount and tagged object pointers |
| 4 | word_318; word_31c; width_320; word_328; word_32c | two i32; line width; two i32 |

Each line width in this modern scope is i16 standard width followed by f32 custom width. Restore type 3 is used for the first two, type 0 for the third; their modern encodings coincide. The optional array uses direct suObArray pointer framing, with no array object tag or map slot. Readers SHALL preserve the raw array presence integer and all style values. Legacy intermediate width layouts are outside scope.

All 32 first display items reach the end of this table; their arrow counts are zero and their array_4a0 item counts are zero. Cumulative archive positions total 402,918 bytes, including 8,204 bytes from first display-object body starts. Later display-item fields remain unread, so this does not establish complete display dimensions or increase whole-stream coverage.

### 11.50 Display-item tail and geometry arrays

*Status: Native-derived ordinary modern tail without populated watermark/spline-leader payloads; 32 installed item boundaries verified · Evidence: [C-65]*

After the style fields in 11.49, the scoped ordinary v > 0x27CB item stores an i32 spline-leader map count, then entries consisting of an i32 index and tagged moSplineLeaderData_c pointer. The pointer body remains unresolved. Readers SHALL retain the wire entry sequence and duplicate indices rather than collapse them into the native map.

An i32 has_watermark follows the map. A nonzero value introduces a direct moWatermarkNoteData_c body, with no enclosing object tag; this body is not yet specified. The current decoder stops explicitly at that boundary. Zero proceeds directly to the version-dependent geometry:

| Version | Fields |
|---|---|
| v <= 0x3371 | no final geometry fields |
| 0x3371 < v < 0x3374 | i32 point count, direct points[count], one direct vector |
| v >= 0x3374 | i32 point count, direct points[count], i32 vector count, direct vectors[count] |

Each point/vector is three f64 values. Native loading rejects point counts greater than four and vector counts greater than two. The scoped decoder additionally rejects negative counts rather than treating them as empty runtime loops. Readers SHALL preserve the original valid counts instead of expanding shorter arrays to the current writer's four points and two vectors.

These fields complete moDisplayItem_c serialization; moDisplayDim_c resumes with its direct embedded moDimText_c and further unresolved fields. All 32 first native items reach that return boundary, containing 128 points and 64 vectors in total, with no spline-leader entries or watermark bodies. Cumulative archive positions total 408,038 bytes; first display-object bodies account for 13,324 bytes. A complete display item is still only a base layer of each dimension, so complete dimension/sketch and whole-stream coverage remain unproven.

### 11.51 Display-dimension text, handles and initial placement

*Status: Native-derived ordinary modern prefix; 32 embedded texts and handle prefixes verified; later base fields unread · Evidence: [C-66]*

After its complete display-item base, moDisplayDim_c directly serializes the embedded moDimText_c at runtime +0x6A0 using the body of 11.32. There is no text-object tag or map allocation. Pointer fields inside that body still use the shared map. All 32 first display dimensions in the reachable native sketches now parse through this embedded body; this adds native use-site evidence beyond the earlier empty sketch-text collection checks.

The following fields occur in order in the scoped ordinary v > 0x27CB context:

| Order | Field | Encoding |
|---|---|---|
| 1 | handles | i32 count; tagged dimension-handle pointers[count] |
| 2 | word_870; word_874; word_878 | three u16 values |
| 3 | text_7f8; text_800; text_818; text_820 | four CString values |
| 4 | vector_880 | three f64 values |
| 5 | unknown_8a8 | f64 |

Native loading validates each handle as a nonnull moDimHandle_c or derived object. The scoped reader resolves supported classes through its existing map/readers and rejects null handles, negative counts and impossible counts. Readers SHALL preserve all words and strings without applying legacy enum remapping. Meanings of the offset-named fields remain partial.

The native loader replaces a non-finite unknown_8a8 with 0.5 and updates runtime state. Readers SHALL retain the original stored double rather than apply that normalization when reconstructing file bytes. The next display-dimension fields remain unread; this is not the distance suffix boundary in 11.45.

The text-only checkpoint totals 412,262 cumulative archive bytes. The extended checkpoint totals 417,889 bytes, including 23,175 bytes from first display-object starts; 32 dimension handles resolve in these prefixes. Six earlier feature-body stops remain. No complete sketch or whole-stream coverage is added.

### 11.52 Display-dimension placement continuation

*Status: Native-derived ordinary modern prefix; 32 installed boundaries verified; optional populated record synthetic only · Evidence: [C-67]*

Following unknown_8a8 in 11.51, the scoped ordinary v > 0x27CB display-dimension base stores:

| Order | Fields | Encoding |
|---|---|---|
| 1 | unknown_8b0; unknown_950; unknown_a78 | three f64 values |
| 2 | points at +0x958, +0x970, +0x988, +0x9A0, +0x9B8 | five three-f64 points |
| 3 | has_data_a50 | i32 presence |
| 4 | data_a50 | when present: three-f64 point, three-f64 vector, f64 value |
| 5 | word_898; word_89c; word_8a0 | three u16 values |
| 6 | word_8c0; word_8e4; word_8ec; word_8f4 | four u16 values |
| 7 | vector_a38 | three f64 values |

The optional record is direct, with no object tag, class declaration or map slot. Native construction initializes two string members in that runtime record, but the serializer reads only its point, vector and double at this location; no strings occur in this wire body. Readers SHALL retain the raw presence integer and seven words. Native loading remaps word_8c0 value 11 to 18 and replaces non-finite unknown_8b0 with zero; byte-faithful readers SHALL preserve the original values. The other geometric meanings remain partial.

All 32 first display dimensions reach the end of this table, adding 160 points. Their optional data_a50 records are absent; populated layout is tested synthetically. Cumulative archive positions total 423,841 bytes, including 29,127 bytes from first display-object starts. The next native branch calls a virtual method at slot +0xA78; its class-specific result must be resolved before consuming the possible following fields. Complete display dimensions and whole-stream coverage remain unproven.

### 11.53 Distance/radial point-reference branch

*Status: Native vtable/load-path evidence; 32 installed prefixes verified; populated references synthetic only · Evidence: [C-68]*

After 11.52, the native serializer tests virtual slots +0xA78, +0x8D8 and +0x8A8. For moDisplayDistanceDim_c these return 1, 0 and 0; for moDisplayRadialDim_c they return 1, 1 and 0. Both classes therefore restore the point-reference branch in the scoped modern v > 0x27CB ordinary context with archive condition 2 absent. The radial path additionally tests v > 0x233, already satisfied here. This dispatch is not established for other display classes.

| Order | Fields | Encoding |
|---|---|---|
| 1 | point references at +0x580/+0x588 | two tagged moPointRef_w pointers |
| 2 | parameters at +0x5A0/+0x5A8 | two f64 values |
| 3 | has_point_590; point_590 | i32 presence; optional three-f64 point |
| 4 | has_point_598; point_598 | i32 presence; optional three-f64 point |
| 5 | word_8e8; word_8f0; flags_8a4 | two u16; u32 |
| 6 | has_array_a58; array_a58 | i32 presence; optional ReadCount and tagged object pointers |
| 7 | has_array_a60; array_a60 | i32 presence; optional ReadCount and tagged object pointers |

The two arrays are direct suObArray pointer encodings; the arrays themselves allocate no MFC map slots. Both are present in this ordinary context with conditions 2 and 4 absent. Readers SHALL preserve raw flags, presence integers, pointer identity and point values. Reference parameter meanings and flag bits remain partial. Unsupported class dispatch is an explicit error in the scoped reader.

All 32 first native display dimensions reach this boundary; both point-reference pointers are null in each. Cumulative archive positions total 425,249 bytes, including 30,535 bytes from first display-object starts. Synthetic tests cover shared nonnull pointers and a populated optional point. Later shared-base fields and derived suffixes remain unread; whole-stream coverage is unchanged.

### 11.54 Display-dimension units and parameter array

*Status: Native-derived ordinary modern fields; 32 installed prefixes with 58 parameters verified · Evidence: [C-69]*

Following 11.53, the scoped ordinary v > 0x27CB display-dimension base stores an alignment CString at runtime +0xA70, tagged moLengthUserUnits_c and moAngleUserUnits_c pointers, an i32 at +0x8F8, an f64 at +0xAD0, the direct parameter array at +0x5B0, and another CString at +0x680. The units use the existing units object layouts and shared MFC map. The meanings of the offset-named integer/double and alignment-string domain remain partial.

The parameter-array encoding begins with u32 element_width. Zero ends the array immediately. A nonzero width is followed by two u32 metadata words, a u32 count, and count × element_width bytes. The native generic helper accepts widths divisible by eight; this typed parameter reader supports width eight and decodes each value as f64. Other nonzero widths are explicit unsupported cases. The two metadata word meanings remain unresolved. Readers SHALL retain them, the stored width, count and values rather than reconstructing allocation defaults.

All 32 first native dimensions reach the end of the +0x680 string, decoding 58 parameter doubles. Cumulative archive positions total 426,993 bytes, including 32,279 bytes from first display-object starts. Attempting the following tagged object at +0x688 encounters moFavoriteHandle_c in all 32 cases; its body remains unread. Six earlier feature-body stops persist. Whole-stream coverage is unchanged.

### 11.55 Favorite handle and subsequent display fields

*Status: Native-derived ordinary modern fields; 32 populated favorite handles and following boundaries verified · Evidence: [C-70]*

After text_680 in 11.54, the shared display-dimension base reads a tagged object at runtime +0x688. In all 32 reachable first display dimensions this object is moFavoriteHandle_c. Its scoped v > 0x27CB body is i32 word_8, i32 word_c and u32 word_10. The third field is gated by v > 0x72C in native loading, already satisfied by the scoped range. Individual field semantics remain partial. Readers SHALL preserve signedness and all stored bits.

The shared base then continues:

| Order | Fields | Encoding |
|---|---|---|
| 1 | unknown_b40; word_b68; word_b6c; word_b60 | f64; two u16; u32 |
| 2 | values_830 | four f64 values at runtime +0x830/+0x838/+0x840/+0x848 |
| 3 | word_af4; unknown_8b8; word_8fc; word_900 | i32; f64; u16; i32 |
| 4 | transform_5c0 | serialized mgXform_c operator body; corrected framing in 11.58 |

All these fields are present in the supported ordinary modern context. Native loading converts word_b6c 0xFFFF to runtime -1; readers SHALL preserve the original u16. Offset-named field domains and geometric meanings remain partial. The favorite pointer uses the existing MFC map and the transform is direct, without an object tag.

All 32 first native dimensions reach the end of this table with nonnull favorite handles. Cumulative archive positions total 431,345 bytes, including 36,631 bytes from first display-object starts. Later shared-base fields and derived distance/radial suffixes remain unread. No whole-stream coverage is added.

### 11.56 Display strings and direct attachment collections

*Status: Native-derived ordinary modern fields; 32 installed empty collection boundaries verified; populated collections synthetic only · Evidence: [C-71]*

After transform_5c0 in 11.55, the scoped ordinary v > 0x27CB display-dimension base stores an f64 at runtime +0x648, followed by four CString values at +0xB70/+0xB78/+0xB80/+0xB88. Their meanings remain partial. Readers SHALL preserve the stored double and string sequence.

Next, with archive condition 0x8000 absent, it directly serializes the moAttachPointList_c instance held at +0xE48. This delegates to suObArray::Serialize: ReadCount and tagged object pointers. There is no presence integer or enclosing array-object tag here. The embedded suObList at +0x908 follows, also as ReadCount and tagged object pointers without a list tag. An i32 at +0xB9C and u32 at +0xE40 complete this stage. Readers SHALL preserve item sequence, pointer sharing and the full integer values.

The direct collections themselves allocate no MFC map slots; their tagged items use the existing map. Constructors identify the runtime collection types, while the import target establishes the attachment array's wire framing. Populated element classes and their constraints remain partial.

All 32 first native dimensions reach this boundary with both collections empty. The preceding string checkpoint totals 431,729 cumulative archive bytes; the attachment checkpoint totals 432,113 bytes, including 37,399 bytes from first display-object starts. Later dimension-jog/reference fields remain unread, so complete display-dimension and whole-stream coverage are unchanged.

### 11.57 Jog/reference sequence and unresolved native alignment

*Status: Historical alignment failure, resolved in 11.58; native sequence and synthetic test · Evidence: [C-72], [C-73]*

The ordinary modern load path after the fields in 11.56 reads two tagged moDimJog_c pointers, one tagged moDisplayDim_c pointer, u32 flags_b64, two tagged moDrViewTag_c pointers, two independently optional points (each i32 presence followed by three f64 values when nonzero), and i32 word_a90. All relevant version gates precede the scoped v > 0x27CB range. Readers SHALL preserve pointer identity, raw presence values and flag bits. Nonnull jog/tag object bodies still require their own supported readers.

This sequence cannot yet be used to claim a verified native continuation. Extending the current prefix decoder fails in every one of the 32 native cases at the second expected view-tag pointer: it reads tag 0x3FF0, interpreted as unmapped object index 16368. A representative trace starts at archive 0x331B, reads expected pointers at 0x331B/0x331D/0x331F, then expected view tags at 0x3325/0x3327, with the latter failing. The byte pattern at the failed boundary is consistent with a floating-point value rather than a valid mapped pointer, suggesting an earlier framing/alignment mismatch. That interpretation is a diagnostic hypothesis, not a recovered layout.

The earlier checkpoint totals remain reproducible parser traversal measurements. This failed continuation means their downstream alignment has not been independently confirmed; zero-valued fields and successful primitive reads alone do not prove the complete native sequence. The native-derived field descriptions remain evidence for the serializer order, while reconciliation of archive context, embedded serializers and prior boundaries is outstanding. Readers SHALL stop at the mismatch and SHALL NOT skip bytes or accept unknown object tags to reach later fields. No additional native byte coverage is credited.

### 11.58 Serialized transform overload correction

*Status: Native instruction evidence; corrected native continuation verified in 32 cases · Evidence: [C-73]*

The transform at runtime +0x5C0 in 11.55 uses operator>>(su_CArchive&, mgXform_c&), not mgXform_c::restore. These entry points have different wire encodings. This operator first reads exactly 30 u32 words (120 bytes), then invokes the perspective restoration helper. Readers SHALL retain all 30 words. The flag word is at index 28; when its high-nibble perspective-kind bits are zero, no perspective payload follows. The scoped decoder rejects nonzero perspective-kind bits explicitly. Detailed internal word semantics remain partial.

The former reader incorrectly used the compact u8/matrix/translation/scale/flags encoding, consuming 34 bytes in these fixtures. This left the parser 86 bytes early per first display dimension. Its zero-valued reads allowed later primitive fields to appear valid until the second expected view-tag pointer failed. This corrects the native boundary interpretation of 11.55–11.57; the historical failure trace remains diagnostic evidence, not an unresolved blocker.

With the corrected operator body, all 32 native first display dimensions reach the end of the jog/reference sequence in 11.57 without byte skipping. Both jog pointers and both view-tag pointers are null in these cases. Corrected cumulative archive totals are 434,097 after 11.55, 434,865 after the four strings, 435,249 after attachment fields and 436,081 after jog/reference fields. These supersede the corresponding earlier totals. The final total includes 41,367 bytes from first display-object starts. Later shared-base fields remain unread; no complete dimension or whole-stream coverage is claimed.

### 11.59 Final modern shared display-dimension fields

*Status: Native-derived ordinary modern shared base complete for the observed empty-option cases; 29 first distance bodies traversed · Evidence: [C-74]*

After word_a90 in 11.57, the supported ordinary shared base stores:

| Order | Fields | Encoding |
|---|---|---|
| 1 | backup at +0xEF8 | tagged object pointer; nonnull payload remains unsupported |
| 2 | word_b4c; word_b50; word_d78; word_d7c; word_f00 | five i32 |
| 3 | unknown_8c8 | f64 |
| 4 | word_8d0; word_f08; word_f0c; word_f10; word_8d8; word_8dc | six i32 |
| 5 | text_808; text_810 | two CString values |
| 6 | word_d80; word_f84; word_f54 | three i32 |
| 7 | text_f60; word_650; word_f80; point_f68 | CString; two i32; three-f64 point |
| 8 | word_f58; word_8e0 | two i32 |
| 9 | text_f88 | CString, only v >= 0x27E0 |

All other listed fields are present throughout the scoped v > 0x27CB range. The native older-version branch derives text_f88 in memory when its field is absent; readers SHALL preserve that distinction rather than invent bytes. Readers SHALL retain signed integer values and string order. Detailed offset-named field domains remain partial.

The subsequent common runtime tail reads six compatibility values only for 0xDD6 < v < 0xDE5, outside the supported range. It adds no modern fields. Thus the derived distance suffix of 11.45 begins directly after this table. Other previously noted optional payloads still require their own readers when present.

All 32 first native display dimensions reach the shared-base end with null backup pointers: cumulative positions 439,985 bytes, including 45,271 bytes from display-object starts. Combining this base with the distance suffix traverses 29 complete first distance-dimension bodies; the three radial cases stop explicitly before their unimplemented suffix. The distance-only cohort totals 372,504 cumulative bytes and 42,543 body bytes. These cohort totals are not comparable to totals over all 32 cases. The audit raises a stop after the first object and does not complete the containing display list, sketch or stream; no whole-stream coverage is credited.

### 11.60 Radial display suffix and scoped display lists

*Status: Native-derived radial suffix; three first radial bodies and 20 complete native display lists verified · Evidence: [C-75]*

After the shared base of 11.59, moDisplayRadialDim_c stores the following in the scoped ordinary v > 0x27CB context:

| Order | Fields | Encoding |
|---|---|---|
| 1 | flags_fd8; bent parameters; end points | i32; six f64; two three-f64 points |
| 2 | callout strings | i32 count; CString values[count] |
| 3 | callout_data; text_1048; list_1068 | tagged moHwCalloutData_c pointer; CString; tagged suObList pointer |
| 4 | has_transform_1060; transform_1060 | i32 presence; optional compact mgXform_c::restore body |
| 5 | mirror_helper; word_fd4 | tagged moMirrorHWHelper_c pointer; i32 |
| 6 | hole_wizard_info | tagged moHoleWizardInfo_c pointer, v > 0x333D |
| 7 | word_10a4 | i32, v > 0x3B21 |

The compact transform here uses the restore entry point, unlike the 120-byte operator body at shared-base +0x5C0. Readers SHALL preserve raw flags, presence integers and the final stored integer even though native loading normalizes word_10a4 to a runtime boolean. Native allocation reserves four callout string slots; the scoped reader supports counts zero through four and rejects other counts. Individual bent-parameter/flag meanings remain partial. Nonnull callout/mirror/hole-wizard objects require supported payload readers.

Scoped whole distance/radial readers compose the documented layers and retain the enclosing MFC map for repeated/cyclic references. They are installed only on the local archive, not in the global reader registry. The first-object audit now traverses all 32 bodies (29 distance, three radial), totaling 441,935 cumulative bytes and 47,221 bytes from display-object starts.

The full-list audit completes 20 native display lists with 78 items, totaling 299,512 cumulative bytes over that 20-case cohort. Twelve other reachable sketch lists stop explicitly: nine on moDisplayAngularDim_c, two on moDisplayRevolveDim_c and one on moHwCalloutData_c. The six earlier boss/revolution feature dependencies remain. These lists are still nested in partial sketches; no complete sketch or whole-stream coverage is added.

### 11.61 Angular/revolved display dimensions and post-list groups

*Status: Native-derived ordinary modern bodies; 31 display lists/143 items and following empty groups verified · Evidence: [C-76]*

moDisplayAngularDim_c uses the shared base through 11.59, followed by two u16 words at +0x100C/+0x1010, a u8 at +0x1008, two three-f64 vectors at +0xFB8/+0xFD0, four f64 parameters at +0xFE8 through +0x1000, and i32 word_1018. Readers SHALL preserve the words, byte and geometric values without boolean normalization. Parameter meanings remain partial.

In the scoped v > 0x27CB range, moDisplayRevolveDim_c first serializes the complete radial body of 11.60, then adds:

| Order | Fields | Encoding |
|---|---|---|
| 1 | IsDiameter | i32 |
| 2 | TextDirForRevolveSketch; TextDirForRevolveFeature | two three-f64 vectors |
| 3 | TextLocForRevolveSketch; TextLocForRevolveFeature; WitnessLenForRevolveSketch; WitnessLenForRevolveFeature | four f64 values |
| 4 | DimPlane; IsDimPlaneSet | direct compact mgXform_c::restore body; i32 |

The earlier v <= 0x4BA revolved inheritance path differs and is outside scope. Raw integer values remain preserved. Primary vtable evidence establishes that angular classes take the same shared point-reference branch as distance classes, and revolved classes take the same branch as radial classes; these are explicit additions to the dispatch set in 11.53.

With locally composed readers for all four classes, 31 native display lists complete with 143 items. The remaining reachable list stops on populated moHwCalloutData_c; six earlier feature-body dependencies persist. Cumulative positions across the 31 completed lists total 594,474 bytes. Reading the following u16-count equal-distance point, equal-distance constraint and logical-group collections (11.43–11.44) succeeds in all 31 cases; all three counts are zero, advancing the cumulative total to 594,660 bytes. The helper then returns to the main sketch serializer, whose remaining tail is unread. No complete sketch or whole-stream coverage is claimed.

### 11.62 Hole-wizard callout data

*Status: Native-derived ordinary modern body; one populated native callout and 32 display lists verified · Evidence: [C-77]*

The moHwCalloutData_c body in the scoped v > 0x27CB ordinary context is:

| Order | Fields | Encoding |
|---|---|---|
| 1 | hole_wizard | tagged moCompHoleWzd_c pointer |
| 2 | items_10 | direct suObArray: ReadCount, tagged object pointers[count] |
| 3 | word_34; word_38; word_50; word_54 | four i32 values |
| 4 | text_40; text_48 | two CString values |
| 5 | items_58 | direct suObArray: ReadCount, tagged object pointers[count] |
| 6 | word_3c | i32 |

Both arrays are constructed embedded members, so their serializers introduce no array presence integers, object tags or array map slots. Tagged pointer entries and the hole-wizard pointer share the enclosing archive map. Readers SHALL preserve all integer values, string order, array entries and pointer identity. Offset-named field semantics and populated element-class constraints remain partial. Nonnull unsupported hole-wizard/array element payloads still stop explicitly.

Adding this scoped class reader completes all 32 reachable native display lists: 146 items, including one populated callout object. Their cumulative end positions total 610,304 bytes. The following equal-distance point, constraint and logical-group collections are empty in all 32 cases, bringing cumulative positions to 610,496 bytes and returning from helper 0x4C5AB5A0. Five boss and one revolution feature-body dependencies still prevent the other six sketch cases from reaching this stage. The main sketch tail remains unread; whole-stream coverage is unchanged.

### 11.97 Base display dimension and component-feature references

*Status: Native vtable/serializer evidence; three complete linear patterns and cosmetic threads verified in scoped audit · Evidence: [C-112]*

Primary moDisplayDim_c RTTI selects serializer 0x4C835790, the shared dimension serializer already used by the supported derived display dimensions. Its primary vtable +0xA78 resolves to the zero-return function 0x4B29C5A0. The base class therefore omits the two point-reference tags, two f64 reference parameters and two i32-presence optional points at native offsets 590/598. Readers SHALL NOT substitute the derived distance/radial reference block for this class. The related virtual slots 8D8,8A8,8B0 also return zero; the last affects only historical runtime repair in the scoped path.

After the dimension-placement fields, the base-class reference continuation starts directly with u16 word_8E8, u16 word_8F0 and u32 flags_8A4, followed by the same two i32-presence nullable direct object arrays A58/A60 documented for the shared display base. The remaining units, favorites, strings, attachments, jogs and late fields are the common dimension body. There is no derived suffix after that body for moDisplayDim_c. The reader is installed in the archive's scoped registry, preserving the global registry.

Primary RTTI for moCompFeature_c and moCompBodyFeature_c selects exactly the same serializer 0x4BBB9A60. In the ordinary modern zero-condition archive, it reads the component-reference base (11.17) then direct moFRData_c at member F0: i32 feature ID and the modern u32 feature-time representation. There is no tagged FRData object, presence word or derived body-feature suffix on this path. The FRData serializer is 0x4BB48620, reused from the constructor-verified reference-plane path in 11.86. Archive conditions 1 and related masks can introduce presence/alternate reference-data branches outside this reader. Readers SHALL preserve signed ID, unsigned time and component-reference identity. Wrapper validation admits both proven component-feature classes.

With these dependencies implemented, `--complete-patterns` returns 901 relation-manager records in 906 attempts. It completes 27 profiles, 14 extrusions, five ordinary revolutions plus six revolution cuts, two ribs, three linear patterns and three cosmetic threads. The report's cumulative revolution counter is 11 and includes the six cuts. Each complete pattern has base moDisplayDim_c references at 1250/1260, distance dimensions at 1258/1268, and null 1310/1318 dimensions. These are six base-class reference use sites across the three pattern objects, not a count of distinct dimension allocations.

The three complete cosmetic-thread records each contain moCThreadData_c with eight cone values, zero instance/cache counts and absent optional point_120. These native paths now exercise the formerly synthetic-only thread-data framing in 11.93. Populated instance/cache class bodies remain separate dependencies; the existing synthetic tests cover grouped pointer framing but do not establish those class layouts. Exact pattern/thread source streams and nested body spans are retained in `complete-pattern-corpus.json`.

Five configurations still stop explicitly: four Fillet_c records and one present kernel body requiring stream/model-context framing. Completing these relation-manager prefixes does not establish complete configuration streams, assemblies, drawings or installed project formats. Whole-document bit coverage remains unchanged and below the requested target.

