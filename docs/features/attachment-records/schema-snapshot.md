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

### 11.183 Attachment data and completed display-note boundaries

*Status: B / ? — native attachment-data grammar and six populated complete note boundaries verified; raw flag meanings, further nested variants and full coverage remain open.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moAttachPointData_c primary table `0x4CE7C940`, complete-object locator `0x4D313CE0` and serializer slot +28 select `0x4C27A430`. RTTI establishes su_CObject ancestry. Its ordinary binary object base adds no fields. This data record is distinct from moAttachPoint_c and moAbsolutePoint_c: it does not serialize their four-point, integer and byte suffix. Its wire order is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | point +08 | direct three-f64 point | always | Retain all three stored coordinates. | B / ? |
| 2 | point +20 | direct three-f64 point | always in scope | Historical load gate is `v > 0x212`, already true in the supported modern range. | B / ? |
| 3 | view +38 | tagged moDrView pointer | always | Preserve null/cached/new identity. Native store can replace an excluded runtime view with null; offline load retains the pointer actually stored. | B |
| 4 | raw +40/+48 | two f64 | always | Retain both values independently; units, parameter domain and geometric interpretation remain partial. | V / ? |

The body is 66 bytes when its view pointer is null or cached: 48 point bytes, a two-byte pointer tag and 16 scalar bytes. A new view object adds its own class/object framing and body. Direct point records have no tags, presence words or map slots. The scoped reader requires the same explicit ordinary modern archive context as the surrounding note readers and leaves other view payloads independently dispatched.

Two reachable notes also require moFavoriteHandle_c at display-note object +6E0. The resolved-feature scope now installs the existing verified 11.55 favorite-handle reader: i32 +08, i32 +0C, u32 +10. This reuses the previously bound native layout and does not substitute a direct body for the tagged pointer. The global object registry is unchanged.

The audit completes **six native display notes**, together with **six attachment-data records**, **six generic-text objects**, **seven line heads** and **96 character records** inside complete roots. All six attachment-data view pointers are null; the scalar pair is (0,0) in two records, approximately (0.039939,0.618337) in one and (0.660178,0.235213) in three. These observations do not establish scalar domains or units. The notes retain their original string occurrences, raw flags and map identities. Three notes store flag chunks [0,1] and three store [0,0]; the chunk-significance and bit-meaning limitations of 11.178 remain.

These populated boundaries supersede the earlier explicit dependency stops in 11.178–11.182 for this cohort. They do not establish every hyperlink, stacked-note, BOM, external-component, watermark, routing or drawing variant. In particular, complete framing of the stored flag chunks is not complete semantic bit coverage.

The frontier now has **23,208 complete roots**, **697 complete lists** and **209 explicit stops** across 906 complete modern contexts. **635 lists reach exact EOF**, spanning **35,257,972 decoded bytes**; **62 complete lists retain four-byte unknown tails**. Relative to Edition 0.193, this adds 401 complete roots and two exact-EOF lists. `resolved-feature-list-frontier-0.193.json` retains the preceding checkpoint. Three focused tests cover direct point/double widths, cached view identity, tagged object reuse, scoped favorite dispatch, context/truncation rejection and a source-stream-hash-bound complete native note containing populated text and attachment data. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; whole ResolvedFeatures coverage is not credited.

Evidence: C-211.
