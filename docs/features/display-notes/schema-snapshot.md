### 11.178 Display-note suffix inventory

*Status: B / ? — native serializer identity and provisional modern wire inventory; nested payloads, flag reconstruction and complete native record boundaries remain open.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. In the installed sldmodu.dll, moDisplayNote_c primary table `0x4D2308C0`, complete-object locator `0x4D3725C0` and serializer slot +28 select `0x4C95CCC0`. Native RTTI establishes moDisplayItem_c ancestry, and the serializer first calls `moDisplayItem_c::Serialize`. The following table begins **after that complete inherited body**. Member offsets identify native state, not stream offsets. This inventory does not make moDisplayNote_c a supported scoped reader.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +644/+648/+64C | three u16 | always in scope | Six wire bytes; native members are spaced four bytes apart. Preserve each complete stored word. | V / ? |
| 2 | vector +628; raw +630 | direct mgVector_c; f64 | always in scope | Retain original vector coordinates. Native load can replace invalid vectors in memory; that replacement is not additional archive data. | B / ? |
| 3 | raw +690/+694 | two i32 | always in scope | Preserve signed words without assigning enum domains. | V / ? |
| 4 | hyperlink +6A0 | tagged suHyperlink_c pointer | always in scope | Preserve null/cached/new identity; a nonnull body is a separate dependency. | B |
| 5 | strings +658/+660/+668 | three CStrings | always in scope | Three separate string occurrences in this order, including empty or equal values. | B / ? |
| 6 | string map +6B8 | tagged su_CMapStringToString pointer | always in scope | Modern load has no preceding map-presence i32. Historical presence framing before `0x98E` is outside this scope. | B |
| 7 | stacked data +590 | tagged moStackedNoteData_c pointer | always in scope | ReadObject/WriteObject in the non-DB branch; not an embedded direct body. Nested data remains a dependency. | B |
| 8 | flags +768 | i32 chunk count, followed by that many u32 words | always in scope | Modern loading uses this framing after `0x1C0A`. Store writes count 2. Native converts each word to a 32-character binary string before reconstructing state; word significance and flag meanings remain open pending helper/instruction validation. Preserve count and ordered raw words; do not replace the wire representation with one u64. | B / ? |
| 9 | chain +5A0; raw +698 | tagged moChainComponent_c pointer; f64 | always in scope | Separate pointer and double after the chunked flags. | B / ? |
| 10 | string +6D8; object +6E0 | CString; tagged su_CObject pointer | always in scope | Runtime destruction/replacement of the existing object consumes no additional bytes. | B / ? |
| 11 | raw +640 | u16 | always in scope | Two serialized bytes, although native loading widens to a four-byte member. | V / ? |
| 12 | BOM +5B0; raw +5B8/+5BC | tagged moBomFeat_c pointer; two i32 | always in scope | Preserve pointer and both signed words. | B / ? |
| 13 | raw +710; raw +5C0 | f64; i32 | always in scope | Historical gates are respectively `v >= 0x1429` and `v >= 0x17E1`, both below the modern minimum. | V / ? |
| 14 | raw +770 | u32 | always in scope | Historical loading begins at `0x1C00`. Preserve all bits. | V / ? |
| 15 | raw +784 | i32 | always in scope | Native load requires `v > 0x200C`, already true here. | V / ? |
| 16 | raw +788 | i32 | `v > 0x2BBB` | Omit on earlier supported modern versions; no placeholder. | V / ? |
| 17 | raw +78C | i32 | `v > 0x2F77` | Independent later gate. | V / ? |
| 18 | cosmetic-thread component +790 | tagged moCompCThread_c pointer | `v > 0x2F9A` | Preserve the normal archive map; not an untagged component body. | B |
| 19 | integer map +6C0 | tagged su_CMapStringToInt pointer | `v > 0x2FB7` | Last serialized field in the recovered main body. Map contents remain a nested dependency. | B |

The version conditions in rows 16–19 come from the **load** branch. The installed current-version writer emits all four. Applying current store output unconditionally to earlier archives would therefore misalign the following record. Older temporary doubles, floats, point coordinates and watermark conversions in this serializer all end before the modern minimum; they are not padding or modern suffix fields. Native display/justification adjustments, dimension embedding and text-line-spacing migrations also operate on runtime state without adding fields to this ordinary modern suffix.

Evidence is retained in `solidworks/research/display-note-native-evidence.json`, with the full module SHA-256, complete-object locator bytes, ancestry, serializer bytes and decompilation. `display_note_evidence.py` reproduces the binding. The chunk conversion helpers, nonnull stacked-note/hyperlink payloads, inherited optional display-item branches and complete populated note boundaries still need verification. The frontier remains the Edition 0.188 result: 22,786 complete roots, 691 complete lists and 215 explicit stops; display notes remain eight leading stops. No additional bytes are credited as decoded or semantically covered. The parts, assemblies, drawings and **all installed SolidWorks project formats** objective remains incomplete.

Evidence: C-206.

### 11.179 Scoped display-note reader and internal note components

*Status: B / ? — scoped suffix framing and null-component branch implemented; complete populated notes remain stopped at a named dependency.*

`displaynotes.read` composes the existing display-object and display-item layers with the suffix in 11.178. It does not append the display-dimension text or dimension-specific fields. The latest-version all-null inherited display item occupies 272 bytes in the synthetic fixture, including the final point/vector collection counts. Older item-tail version gates still apply. The suffix retains all raw flag chunks in stored order without claiming their reconstructed flag meaning. Negative or impossible chunk counts are rejected before allocating or reading the chunks. Nonnull nested objects use ordinary tagged dispatch and stop when their class is unsupported. Registration is limited to the resolved-feature scope; the global object registry is unchanged.

moCompNote_c primary table `0x4CEF5040` selects serializer `0x4C9570A0`, which delegates to `0x4C956B10`. That helper first serializes moCompRef_c, then chooses a detail-item pointer or two external strings using `moCompRef_c::iIsExternal` (`0x4B819670`). A null component at native +58 makes this predicate return zero immediately, independent of filed owner. The scoped reader supports precisely that proven branch:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | component-reference base | direct moCompRef_c body | always | Existing versioned component-reference grammar; preserve its tagged component pointer. | B / ? |
| 2 | detail +E8 | tagged moDetailItem_c pointer | null component only | Can introduce moNoteData_c. Cached and null pointers retain normal identity. | B |
| 3 | detail +F8 | tagged moDetailItem_c pointer | always after row 2 in supported branch | Separate pointer occurrence, including when it repeats +E8. | B |
| 4 | raw +100 | i32 | always in this modern scope | Preserve signed value. Native historical gate is `v >= 0x4BE`. | V / ? |

The latest-version null-component, null-detail body is 90 bytes: 82-byte component-reference base, two two-byte null tags and one i32. A nonnull component stops immediately after the inherited base, before consuming either possible derived representation. The native external branch contains two CStrings at +F0 and the allocated member +08 instead of the +E8 pointer; its predicate's virtual calls and owner comparison have not been implemented. No branch is selected by peeking at apparent tag/string bytes.

The audit now advances all eight former display-note stops through this internal component branch to **moNoteData_c**. It completes no additional root or list: the authoritative counts remain **22,786 complete roots**, **691 complete lists** and **215 explicit stops** across 906 complete modern contexts. `resolved-feature-list-frontier-0.189.json` retains the preceding checkpoint. Four focused tests cover every suffix tail gate, signed/raw extrema, ordered flag chunks, cached object identity, complete tagged composition, truncated/invalid counts, context rejection and explicit nonnull-component rejection. Complete populated note boundaries and meanings remain open; no whole-stream or project bit coverage is added.

Evidence: C-207.

### 11.180 Note data, inherited detail items and attachment context

*Status: B / ? — native modern framing and direct-list dispatch verified; nested geometric attachments and generic text remain dependencies.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moNoteData_c primary table `0x4D22FBC0` selects `0x4C96A590`. Its serializer calls moDetailItem_c::Serialize (`0x4BA5EC30`, primary table `0x4CEF9578`) and then reads one tagged moDimText_c-family pointer at +90. This pointer can introduce moGenericText_c. Native pre-`0x80A` loading serialized the text member directly; that historical branch is outside this scope. Native load can replace a text object with a generic-text runtime object and update language state without consuming additional stored fields. Offline reading retains the original stored object and archive-map identity.

Both detail-item and note-data primary tables select node lightweight helper `0x4C18A1A0` at virtual +4B8 and a no-byte return at +4C0. Thus the inherited moNode_c::Serialize body uses 11.11's lightweight grammar, followed by the detail-item fields below. There is no model-feature, atom or display-item base in this data object.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | node | direct node lightweight body | always | Preserve the node's tagged name, ID, raw flags and versioned string fields. | B / ? |
| 2 | raw +48 | u16 | always | Two wire bytes widened to a four-byte runtime member. | V / ? |
| 3 | owner +40 | tagged CObject pointer | always | Preserve null/cached/new framing; do not infer type from member offset. | B |
| 4 | attachments +50 | direct moAttachPointList_c / suObArray body | always in scope | Packed ReadCount followed by that many tagged attachment pointers. No list class tag or list object-map slot. | B |
| 5 | integer array +60 | i32 count; count i32 values | always in scope | Retain signed values, duplicates and order before any native historical conversion. Reject negative or impossible counts. This count is not ReadCount. | V / ? |
| 6 | raw +78 | i32 | always in scope | Historical load begins after `0xF2F`. | V / ? |
| 7 | attachment +80 | tagged moAttachPoint_c pointer | always in scope | Separate pointer occurrence after the integer array; historical load begins at `0xFB2`. | B |
| 8 | note text +90 | tagged moDimText_c-family pointer | moNoteData_c only | Appended by the note-data serializer, not by the detail-item base. | B |

Constructor `0x4C7FA6E0` delegates to detail-item constructor `0x4BA40150`, which allocates moAttachPointList_c and assigns it to +50. The list primary table `0x4CE7FDA0` selects thunk `0x4CBA123E`; its import resolves to sldmfcu.dll's suObArray::Serialize. This proves the direct array grammar used by the virtual load call. The current store path filters attachments into a temporary list before writing; that filtering does not justify dropping stored entries during offline reading.

The latest-version empty detail-item body is 35 bytes, including the 19-byte empty node body; empty note data adds a two-byte null text pointer, giving 37 bytes. These are synthetic minimum examples, not fixed lengths for nonempty objects. Direct lists allocate no map slots, while newly tagged attachments and text objects do.

The resolved-feature handoff now copies `model_root_class` from the completed model archive along with the existing explicit archive mode. Previously the missing root-class attribute caused supported attachment readers to reject seven note paths. This context comes from the parsed model; it is not guessed from attachment bytes. Existing attachment restrictions remain enforced for unsupported model roots.

The eight former moNoteData_c stops now reach **moGenericText_c (two)**, **moGeneralSurfRef_w (five)** and **moAbsolutePoint_c (one)**. No additional root or list completes: the audit still has 22,786 complete roots, 691 complete lists and 215 explicit stops across 906 complete modern contexts. `resolved-feature-list-frontier-0.190.json` retains the preceding checkpoint. Three focused tests verify direct-list/map accounting, repeated pointers, signed extremes, tagged note composition, exact/truncated boundaries, invalid integer counts and model-root context preservation. Complete populated note boundaries, attachment geometry, generic text and full part/assembly/drawing/all-installed-project bit coverage remain open.

Evidence: C-208.

### 11.181 General surface references and absolute attachment points

*Status: B / ? — native modern serializer bindings and five completed surface-reference boundaries verified; absolute-point populated native boundaries and raw geometry meanings remain open.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moGeneralSurfRef_w primary table `0x4D1540E8`, complete-object locator `0x4D358290` and serializer slot +28 select `0x4C2866C0`. RTTI establishes moSurfaceRef_w and moRefWrapper_c ancestry. The serializer calls wrapper helper `0x4C2842D0` and then serializes four f64 fields. This is the existing shared surface-reference grammar used by the base of moSingleFaceRef_w; it does not include that derived class's embedded-body presence or later suffix.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | reference wrapper | direct moRefWrapper_c body | always | Existing 11.16 versioned grammar; preserve tagged component/reference identities and optional transforms. | B / ? |
| 2 | raw +A8/+B0/+B8/+C0 | four f64 | always in this scope | Exact member order shown. Historical load/store gate is `v > 0xF22`, below the supported modern minimum. Preserve stored values without imposing ordering or unit rules. | V / ? |

The latest-version null-component, absent-transform reference body is 94 bytes: the 62-byte wrapper followed by 32 bytes of doubles. The four native defaults are -1, 1, 1, -1, but they are defaults rather than omitted wire data in modern files. The reader keeps arbitrary stored doubles and attributes their unresolved meanings as unknown bytes. Only the outer tagged object allocates its normal map slot; its direct inherited body does not allocate another object.

moAbsolutePoint_c primary table `0x4CE7CC88`, locator `0x4D313D68` and serializer slot +28 select `0x4C27A640`, the existing moAttachPoint_c serializer. RTTI confirms moAttachPoint_c ancestry. There is no absolute-point-specific suffix in this dispatch. The scoped alias uses the existing ordinary model-root attachment reader and retains its root/context restrictions:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | points +08/+28 | two direct three-f64 points | always in supported ordinary model-root context | Preserve both stored points independently. | B / ? |
| 2 | raw +20 | i32 | always in scope | Signed word after both points; member offset does not determine wire order. | V / ? |
| 3 | drawing-view reference | tagged moDrView pointer | always in supported branch | Preserve null/cached/new framing. Unsupported contexts still reject rather than infer omitted fields. | B |
| 4 | points +58/+70 | two direct three-f64 points | always in scope | No per-point pointer tags. | B / ? |
| 5 | raw +88 | u8 | always in scope | Preserve every flag bit without normalization. | V / ? |

With a null view, the absolute-point body is 103 bytes. Synthetic tests verify this inherited boundary, signed-word extrema, point order, high flag bits, cached outer identity, truncation and explicit drawing-root rejection. No completed native containing-root span currently includes an absolute point, so this synthetic result receives no native corpus-boundary credit.

The completed-root audit retains **five general surface references**, each nested in a complete moRefAxis_c root and referencing moCompFace_c. All five store the four values -1, 1, 1, -1. A source-stream SHA-256 and containing-root span regression reparse a populated native example. The frontier now records **22,807 complete roots**, **695 complete lists** and **211 explicit stops** across 906 complete modern model contexts. **633 lists reach exact EOF**, spanning **35,216,419 decoded bytes**; **62 complete lists retain four-byte unknown tails**. `resolved-feature-list-frontier-0.191.json` preserves the preceding checkpoint. The six former general-surface/absolute-point note dependencies now reach further nested objects, including generic text; complete display-note roots remain unverified. Full parts, assemblies, drawings and all installed-project bit coverage remains incomplete; no whole ResolvedFeatures coverage credit is assigned.

Evidence: C-209.

### 11.182 Generic text, paragraph formats, line heads and characters

*Status: B / ? — native modern field order and cross-module dispatch verified; full populated note roots and text rendering semantics remain open.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moGenericText_c primary table `0x4D095E00` selects serializer `0x4BEE9B80`. Native RTTI establishes moDimText_c ancestry. The serializer first calls shared dimension-text helper `0x4C6F4A60`, already implemented by `sketchprefix.dimension_text`; it then stores the suffix below. This is a text data object, not a display-item body. The tagged +170 reference can point back to a display object already under construction; readers SHALL retain the archive's early map insertion and cached identity.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | dimension-text base | direct moDimText_c body | always | Existing text, character-format pointer, geometry, nested counts and versioned scalar/string-array grammar. No extra class tag for the inherited body. | B / ? |
| 2 | data +158 | tagged moGenTextDataList_c pointer | always | Standard non-DB ReadObject/WriteObject. Nested contents remain separate object bodies. | B |
| 3 | raw +160 | i32 | always | Retain the original word. Native loading ORs bit 1 and can add migration flags in memory; those are not stored extra words. | V / ? |
| 4 | display +170 | tagged moDisplayItem_c pointer | always | Preserve null/cached/new identity, including cyclic references. | B |
| 5 | paragraph +188 | direct utParagraphFormat_c body | always | No class tag, pointer-presence field or paragraph map slot. | B / ? |

The paragraph call resolves through sldmodu IAT `0x4CD335B0` to sldmfcu.dll export `0x3CB266A0`. All of that serializer's load gates precede the supported modern minimum; obsolete temporary fields end before `0x1BF6`. The ordinary modern direct paragraph body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +00 | u16 | always | Runtime member is widened; retain both stored bytes. | V / ? |
| 2 | raw +08/+10/+18 | three f64 | always | Three independent stored values. | V / ? |
| 3 | raw +20/+24 | two u16 | always | No alignment padding between these words and the following string. | V / ? |
| 4 | string +28 | CString | always | Preserve stored text, including empty values. | B / ? |
| 5 | raw +30/+34/+38 | i32; u32; i32 | always | The middle member uses unsigned archive overloads; preserve its full bit pattern. | V / ? |
| 6 | raw +40/+48; raw +50 | two f64; i32 | always | Exact wire order follows row 5. | V / ? |
| 7 | raw +58/+60; raw +68/+6C | two f64; two i32 | always | Retain all stored values without native display normalization. | V / ? |

An empty-string paragraph body is 87 bytes. The latest-version empty dimension-text base is 129 bytes, so an empty generic-text body with null data/display pointers is 224 bytes. These are synthetic minimum examples; nonempty strings, arrays or nested objects change their sizes.

moGenTextDataList_c primary table `0x4D097F08` selects `0x4BEE9A70`. After its zero-byte ordinary su_CObject layer it reads one **tagged suObArray pointer** at +10, one i32 at +128 and four direct f64 values at +18/+20/+28/+30 through helper `0x4BEEC070`. The pointer is not a directly embedded array. Both historical gates (`v >= 0x1815` for the word and `v > 0xA90` for the four doubles) are already satisfied. Its null-array body is 38 bytes. The array can contain line heads and character elements; these are distinct runtime classes and require their own tagged bodies.

moTextDataLineHead_c primary table `0x4D097668` selects `0x4BEEAB70`. Common helper `0x4B680950` serializes two i32 members at +08/+0C after the no-byte ordinary object layer. Helper `0x4BEE9930` serializes a triple of **f32**, not three doubles, in this modern scope. The complete line-head order is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common +08/+0C | two i32 | always | Preserve full signed words. | V / ? |
| 2 | raw +14 | three f32 | always | Direct helper body, 12 bytes. Pre-`0xADA` doubles are outside scope. | V / ? |
| 3 | raw +10 | u16 | always | Native loading converts nonzero to bool. Retain the original two-byte word offline. | V / ? |
| 4 | paragraph +30 | direct paragraph body | always | Same grammar above, no pointer tag. | B / ? |
| 5 | raw +38 | three f32 | always | Second independent direct triple. | V / ? |
| 6 | data +48 | tagged moGenTextDataList_c pointer | always | Recursive list/reference framing uses the same archive map. | B |
| 7 | raw +50/+58/+64 | i32; f64; i32 | always | All load gates precede the modern minimum. | V / ? |

With empty paragraph string and null nested list, a line-head body is 139 bytes. The order intentionally differs from native member-offset order.

moTextDataChar_c primary table `0x4D0961C8` selects `0x4BEEA0D0`, which first calls common character helper `0x4BEEB530`. Its complete modern body is two common i32 words at +08/+0C, three f32 at +20, f32 at +2C, u16 at +30, i32 at +34, f32 at +38, u16 at +10, then the character u16 at +40. Total: **38 bytes**. Both stored short flags are retained before native boolean conversion. The character is retained as a 16-bit unit, including a surrogate; this record alone does not establish Unicode scalar or grapheme boundaries. The old extra coordinate triple after the character applies only below `0xADB` and is absent here.

Five focused tests cover the direct paragraph widths and unsigned word, tagged-versus-direct list framing, cyclic cached identity, float triples, raw short flags, signed extremes, surrogate-unit retention and exact/truncated boundaries. Native verification retains four primary RTTI/COL/type/ancestry bindings, eleven native ranges, the shared text-base call and the paragraph import/export binding. Full module hashes qualify split Pdata function ranges and out-of-line load branches.

The seven prior generic-text stops advance through the nested line-head/character stages; six now stop at moAttachPointData_c. No additional root/list completion is claimed: the frontier remains 22,807 complete roots, 695 complete lists and 211 explicit stops across 906 complete modern contexts. `resolved-feature-list-frontier-0.192.json` preserves the preceding audit. The native text body can finish inside a root that later stops, but that partial root receives no completed-root or whole-stream coverage credit. Remaining text elements, attachment-data bodies, native populated complete note boundaries and all-document/project bit coverage remain open.

Evidence: C-210.

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
