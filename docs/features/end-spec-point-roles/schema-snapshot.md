# Approved schema snapshot

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

### 11.164 Endpoint references and spline tangency controls

*Status: B — native modern layouts, direct-handle types and populated corpus boundaries verified; raw scalar meanings and special contexts remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`, with sketch-control archive conditions 1, 2 and 4 clear. `moEndPointRef_w` primary table `0x4D17D008` selects `0x4C27D8F0`; the serializer calls point-reference helper `0x4C281C60`, which calls common reference wrapper `0x4C2842D0`. Thus the endpoint inherits the complete wrapper from 11.16 and its three-f64 point at +A8 before the following suffix. The runtime class and cached pointer identities remain distinct from the base point-reference class.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited point reference | reference wrapper; point | always | Existing common wrapper, including conditional transforms and versioned raw words, followed by three f64 point coordinates. No additional class tag for the inherited layers. | V / ? |
| 2 | raw +E8 | f64 | always in this scope | Preserve the stored value. Historical load before `0x1FB` reads an i32 discriminator instead; that conversion is excluded. | V / ? |
| 3 | raw +F0 | i32 | always in this scope | Preserve complete signed word. Historical load omits it through `0x726`. | V / ? |

The latest-version null-component, no-transform endpoint body is 98 bytes before an outer object/class tag: 62-byte wrapper, 24-byte point and 12-byte raw suffix. Wrapper version gates still apply; this fixed example is not a replacement for their grammar. The native base resets a runtime +E0 pointer without consuming an archive field. The suffix values are not assigned parameter-range or boolean semantics solely from the runtime class name.

`moSplineTangencyControl_c` primary table `0x4D1CF2A0` selects serializer `0x4C5C2690`. It calls derivative-control helper `0x4C5B2260`, which calls sketch-control helper `0x4C5C4690`. In the stated ordinary mode the common sketch-control helper reads exactly one tagged sketch pointer. The derivative helper then serializes two embedded handles directly, before the tangency point/vector suffix.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | sketch +08 | tagged sgSketch pointer | always in this scope | Preserve null or actual cached/new object identity. Special conditions can source the owner from runtime context instead and are excluded. | V |
| 2 | spline handle +40 | direct sgSplineHandle body | always | Existing entity-handle grammar: unsigned u16 index, signed i32 replacement only for sentinel `0x777F`, then i32 reference ID and i32 DimOnCM. No class tag or object-map entry. | V / ? |
| 3 | point handle +18 | direct sgPointHandle body | always after row 2 | Same handle grammar, read as a separate record even when values match the spline handle. Native member-offset order does not determine wire order. | V / ? |
| 4 | point +128 | three f64 | always | Preserve all coordinates; geometric role beyond the native point type remains partial. | V / ? |
| 5 | vector +140 | three f64 | always | Preserve all coordinates without normalization. | V / ? |

The null-sketch, short-index tangency body is 70 bytes. Each escaped handle index adds four bytes, so a two-escaped-handle body is 78 bytes. Neither direct handle allocates a map slot; only an outer tagged control and its tagged sketch pointer can do so. Native runtime-class descriptor `0x4DAF6970` selects factory `0x4C5C2130`, which allocates `0x160` bytes and calls derivative constructor `0x4C5B2020`. That constructor establishes sgPointHandle at +18 and sgSplineHandle at +40. Both handle primary serializer slots select `0x4C591ED0`, establishing the shared direct-body grammar. The factory's other points/defaults, the sketch watcher and runtime validity state add no wire fields to this serializer.

Native load assigns the sketch to the embedded handles, checks runtime entity resolution and creates watcher state. Offline reading preserves the two stored handle records without resolving sketch entities or executing watcher updates. Historical tangency coordinate transformation before `0x96C` and nonordinary sketch-owner modes are excluded. Complete pointer framing does not establish validity of arbitrary synthetic index/reference combinations.

The complete-root audit retains **14 endpoint references** and **40 spline tangency controls**. All endpoints reference moCompEdge_c; +E8 is 0.5 in 13 cases and 0.0 in one, while +F0 is zero in all 14. All 40 controls have sgSketch owners. Per-root summaries retain both direct handle records and point/vector coordinates; synthetic tests exercise escaped indices, signed extrema, equal-but-separate direct records, map accounting and exact/truncated boundaries.

Edition 0.175 records **22,083 complete roots**, **618 list-complete configurations** and **288 explicit stops** across **906 complete modern model contexts**. **564 lists reach exact EOF**, spanning **29,438,247 decoded bytes**; **54 complete lists retain four-byte unknown tails**. Remaining dependencies include flat patterns, connection points, weldment features, coordinate systems and draft features. Part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these record boundaries.

Evidence: C-192.

### 11.204 Loft synchronization references

*Status: B / ? — native ancestry/base call and 36 populated references in complete containing roots verified; raw option/parameter meanings and full document coverage remain partial.*

moLoftSynchRef_c primary table `0x4D2D51C0`, complete-object locator `0x4D37FE58` and serializer slot +28 select `0x4CAEEB60`. RTTI establishes moEndPointRef_w ancestry, and the serializer first calls moEndPointRef_w::Serialize at `0x4C27D8F0`. The complete ordinary non-DB external-XML body for versions greater than `0x27CB` is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | endpoint reference | direct moEndPointRef_w body | always | Existing point-reference/wrapper body plus raw +E8 double and +F0 word. No extra tagged base object or map slot. | B |
| 2 | raw +F8 | i32 | always | Preserve signed word. | B / ? |
| 3 | raw +FC/+100 | two u32 | always | Separate complete words, including high bits. | B / ? |
| 4 | raw +108 | f64 | always in scope | Historical load gate `v > 0xA1E`. | B / ? |
| 5 | reference +1A0 | tagged moRefWrapper_c pointer | always in scope | Historical gate `v > 0xBA8`; observed targets include moGeneralCurveRef_w. Retain standard null/cached/new identities. | B |
| 6 | raw +19C/+1A8 | two i32 | always in scope | Historical gate `v > 0xC71`. These runtime offsets do not determine the wire sequence. | B / ? |
| 7 | point reference +1B0 | tagged moPointRef_w pointer | always in scope | Follows both row 6 words. | B |
| 8 | raw +1B8/+1C0 | two f64 | always in scope | Historical gates respectively `v > 0xC71` and `v > 0xC79`; both precede this scope. | B / ? |
| 9 | raw +110/+118 | two f64 | always in scope | Historical gate `v >= 0x11B7`; follows +1C0 on wire despite lower runtime offsets. | B / ? |
| 10 | points +120/+138 | two × three f64 | always in scope | Separate native point archive operations under the same historical gate as row 9. | B / ? |
| 11 | curve references +1C8/+1D0 | two tagged moCurveRef_w pointers | always in scope | Historical gate `v >= 0x1213`; preserve repeated cached pointers and null occurrences independently. | B |

The complete suffix after the inherited endpoint occupies 116 bytes with null/cached pointer occurrences. An empty latest inherited endpoint occupies 98 bytes, making the latest empty complete body **214 bytes** before outer tagged framing. Earlier supported wrapper gates yield empty bodies of 202, 206 and 210 bytes at the tested versions `0x27CC`, `0x2B73` and `0x2FB0`. The historical pre-`0x11B7` point/default initialization is runtime behavior and adds no modern placeholder fields.

`splinecontrols.loft_synch` is installed only in the copied feature-context reader map. Three focused tests cover raw signed/high-bit words, all suffix offsets in wire order, both points, independent point/reference pointers, duplicate cached curve pointers, exact following-byte alignment, tagged identity/map accounting, wrapper gates, truncation/context rejection, and a source-hash-bound complete native containing root with stored raw/point/target-class equality.

The audit retains **36 complete loft synchronization references** in completed roots. In 32, +1A0 targets moGeneralCurveRef_w; four have a null +1A0 pointer. All observed +1B0/+1C8/+1D0 pointers are null. Raw +F8 values are 0 (14 records), 7 (10), 4 (6), 6 (3) and 5 (3); no option meaning or closed domain is assigned from these observations.

Across 906 complete modern contexts, the frontier now has **23,679 complete roots**, **736 complete lists** and **170 explicit stops**: 65 additional roots and four additional complete lists relative to Edition 0.209. **667 lists reach exact EOF**, spanning **38,435,564 decoded bytes**; **69 complete lists retain four-byte unknown tails**. One additional list reaches EOF, adding 272,233 exact-EOF bytes. Completed-list endpoints and tail lengths pass the recorded stream-size invariants. `resolved-feature-list-frontier-0.209.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-232.
