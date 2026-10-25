# Drawing VBLists reference snapshot

Edition 0.206 · 2026-10-25 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 13.9 Drawing view-bucket stream prefix

*Status: Native storage dispatch and serializer, 20 complete bucket-list prefixes; 19 nonempty shaded bodies remain opaque · Evidence: [C-154]*

Definition: stream `Contents/VBLists` of a drawing. The scoped reader supports modern model archive versions `v > 0x27CB`; all 20 installed drawing samples use model version **19000**. The native storage helper initializes the archive with the document's model version, so the decoder obtains it from `_MO_VERSION_N`, rather than substituting the `_DL_VERSION_N` display-list version or a default.

`SolidDrawingStorageMFC_c::iRestoreDrawingsDisplayLists`, in `drwcplu.dll` at `0x180005A30`, opens the stream named by literal `VBLists` and invokes `uiDrawingHLRShadedObject_c::Serialize` in `slddrwu.dll` at `0x3E127280`. The serializer writes/reads a direct signed 32-bit bucket count, that many `uiViewBucket_c` object pointers, and then a direct `uiDrawingShadedData_c` body. The leading value is a **pointer count**, not a byte length. The earlier classification of this stream as a length-prefixed MFC archive is corrected by this clause.

*Table 13-2 — Modern drawing VBLists prefix*

| Order | Field | Encoding | Meaning and handling |
|---|---|---|---|
| 1 | `view_bucket_count` | i32 | Number of following top-level object pointers; MUST be nonnegative and fit the available stream bytes |
| 2 | `view_buckets` | `view_bucket_count` MFC object pointers | `uiViewBucket_c` bodies as in 13.4.6; null pointers and references use the shared archive map |
| 3 | `shaded_data_present` | i32 | First field of the direct shaded-data body; zero ends that body, any nonzero value selects its continuation |
| 4 | shaded-data continuation | Remaining bytes when field 3 is nonzero | Retained opaque by this prefix decoder; no length prefix or complete internal layout is inferred |

The bucket count does not count newly allocated objects or MFC class declarations. One map is retained across the entire top-level pointer list and nested bucket bodies. Readers MUST NOT reset it per entry or expand an existing-object reference into additional wire bytes. The native writer emits the list count as a four-byte `long` and then visits its linked-list entries; it does not use the variable-width MFC `ReadCount` encoding for this outer count.

The shared bucket reader retains existing unknown-field accounting from 13.4.6, including fields whose meanings remain open. It uses drawing context; this stream has no `Contents/DisplayLists` format word, user environment or view-transform preamble. Those structures MUST NOT be prepended to this layout. New MFC classes and objects begin with the ordinary clause 11.5 map state, rather than an inferred map imported from the model configuration stream.

`uiDrawingShadedData_c::Serialize` at `0x3E127420` reads its presence word for the modern versions in scope and returns immediately when it is zero. For nonzero presence it continues with per-view arrays, string-array objects and shaded-model records. Those continuations are not decoded here. `solidworks/drawingvblists.py` stops the returned archive at `prefix_end`, preserves its map, returns the rest as `opaque_shaded_data`, and counts every remaining byte as unknown. This is a prefix reader, not validation of a nonempty shaded body. A zero presence word followed by extra bytes is rejected because the verified body has already ended.

Across the **20** installed drawing streams, **176** top-level pointers precede the shaded data. The streams contain **8,215,597 decoded bytes**; the verified prefixes occupy **3,281,626 bytes**. Of those prefix bytes, **411,416** retain unresolved meanings under the shared readers. The remaining **4,933,971** bytes are opaque shaded continuations. The resulting credited decoded bytes are **2,870,210**, with **5,345,387** decoded bytes remaining unknown. These quantities do not imply complete stream coverage.

The `samples/tutorial/AutoCAD/7550-021.slddrw` stream is **230 bytes**, has one top-level bucket and a zero shaded-data flag, and reaches exact EOF after the prefix. Its shared bucket fields still contain 12 unknown bytes. The other 19 streams store nonzero flag value 1 and retain opaque tails. The corpus report binds each stream and prefix to its SHA-256 and records its exact boundary, root count, class counts and unknown attribution.

At the edition 0.137 checkpoint, the coverage registry credited only these prefix fields, tags and existing documented bucket contents. Proportional attribution of their compressed stream bytes reduced unknown drawing file bytes by **792,707**, from 4,301,943 to **3,509,236**. Drawing file-byte coverage rose from **18.0% to 33.1%** under Annex G's measure; container and thumbnail exclusions gave **21.5%** content coverage. Clause 13.10 extends the prefix inside nonempty shaded bodies. The remaining shaded records, drawing `Contents/DisplayLists`, older drawing layouts and complete document bit coverage remain open.

### 13.10 Drawing shaded-data per-view prefix

*Status: Native array/string-list serializers and 19 extended prefixes; model/tessellation records and later fields remain opaque · Evidence: [C-155]*

When the shaded-data presence word in 13.9 is nonzero, `uiDrawingShadedData_c::Serialize` continues with arrays whose lengths are the outer `view_bucket_count`. In the modern scope `v > 0x27CB`, all version gates in Table 13-3 are enabled. The arrays do not repeat that outer count on the wire.

*Table 13-3 — Modern drawing shaded-data per-view prefix*

| Order | Field | Encoding | Native member or condition | Meaning and coverage |
|---|---|---|---|---|
| 1 | `shaded_model_indices` | `view_bucket_count` u32 words | +0xD8 collection | Copied into the drawing's named shaded-model-index array |
| 2 | `raw_words_20` | `view_bucket_count` u32 words | +0x20; v > 0x762 | Values retained as unknown |
| 3 | `string_lists_28` | `view_bucket_count` direct string-list bodies | +0x28; v > 0x813 | Each body is an i32 string count, followed by that many CStrings; text meaning remains unknown |
| 4 | `string_lists_68` | `view_bucket_count` direct string-list bodies | +0x68; v > 0x985 | Same direct body encoding; separate collection |
| 5 | `model_count` | i32 | +0xD0 | Number of following direct shaded-model records |
| 6 | shaded-model records and later fields | Remaining bytes | Model-record serializer begins at `0x3E128320` | Opaque to this reader |

The first array is stored in a native `su_CUIntArray` at shaded-data +0xD8. The drawing storage reader copies that collection through the `getShadedModelIndexArray`/`setShadedModelIndexArray` API pair. Its file words are retained unsigned; the observed values include `FFFFFFFF` and small model indices. The second array is distinct even though every observed word is `FFFFFFFF`. Its interpretation is not inferred from that repetition and all its bytes remain unknown.

Both string-list groups use fresh, directly constructed wrapper objects with vtable `0x3E41C0E0`. Their serializer, `0x3E127FC0`, reads a **fixed four-byte signed count** and then the ordinary clause 11.2 CString representation for each entry. It does not use the two-/six-byte MFC `ReadCount` encoding. The wrappers are not introduced with MFC object/class tags, do not insert slots in the enclosing object map, and are serialized separately for each view in each group. A reader MUST NOT combine the two groups, add object tags or reset the map between them.

Every observed string list is empty, but the bounded reader also accepts nonempty lists according to the native count/CString loop. It retains each string's physical bytes and span, alongside its decoded value, and counts the complete CString as unknown because the field's meaning is unresolved. Thus a successful read does not grant semantic credit for its text. The list counts themselves have established framing meaning. Negative or impossible list counts and invalid/truncated CString data are rejected.

`drawingvblists.read(..., through_shaded_prefix=True)` advances through this prefix and stops immediately after `model_count`; the default mode continues to expose the 13.9 bucket-only boundary. At the edition 0.138 checkpoint the coverage registry used the extended mode; it now uses the complete mode in 13.11. It preserves the original archive and map for the model records. The reader checks nonnegative model count against a conservative seven-byte minimum per modern record, derived from the initial i32, CString and u16 fields of `0x3E128320`; this is only a bound, not validation of those records or their tessellation bodies. All remaining bytes retain zero semantic credit.

The 19 nonempty bodies contain arrays for **175 view entries**, **350 empty direct string-list bodies**, and **33 shaded-model records**. This stage advances **2,876 bytes** beyond the 13.9 boundaries. It credits **2,176** of those decoded bytes: 700 shaded-model-index bytes, 1,400 list-count bytes and 76 model-count bytes. The 700 bytes of `raw_words_20` remain unknown. Synthetic fixtures distinguish fixed i32 list counts from MFC counts, preserve both narrow and UTF-16 CString wire forms, retain opaque model bytes, and reject malformed counts and truncated prefixes.

At the edition 0.138 checkpoint, the cumulative verified prefixes occupied **3,284,502** of the **8,215,597** decoded VBLists bytes. Credited decoded bytes total **2,872,386**; **5,343,211** remain unknown, including **4,931,095** opaque model/later bytes. Both bucket-only and extended prefix boundaries and hashes are retained in the corpus report. Proportional stored-byte attribution leaves **3,508,706** unknown drawing file bytes; rounded drawing coverage remains **33.1%**, or **21.5%** excluding container and thumbnails. Complete model/tessellation decoding and full-document bit coverage remain open.

### 13.11 Drawing shaded models and trailing collections

*Status: B — complete framing for the installed version-19000 drawing corpus; unresolved field meanings remain unknown.*

The complete mode, `drawingvblists.read(..., through_models=True)`, continues from 13.10 using the same archive and object map. Native model serializer `0x3E128320` writes the following direct record for each declared model. Scope remains the modern drawing archives of 13.9; earlier layouts are unsupported.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `is_part_word` | i32 | Always | 1 selects part tessellation; 0 selects assembly tessellation; other values rejected | B |
| 2 | `model_reference` | CString | Always | Preserve physical wire bytes; resolver and path rewrite rules remain open; no semantic credit | B |
| 3 | `raw_30` | u16 | v > 0x929 | Native member +0x30; preserve raw, no semantic credit | B |
| 4 | `old_lights` | Tagged pointer | Always | Shared old-light array serializer; ordinary archive-map rules apply | B |
| 5 | `visual_properties` | Tagged pointer | Always | `moVisualProperties_c`, shared serializer | B |
| 6 | `tessellation` | Direct shared body | Always | `uoTempPartTessData` for part; `uoTempAssemblySHDDataArray` for assembly; clause 13 shared layouts | B |

Native face serializer `0x49FC3F60` retains the ordinary i32 appearance-map count and key/value loop. The tessellation handle restore at `0x31BFA2A0` calls `gcFacetContainer_c::restore` without adding a wire field. The alignment difference is inside facet restore `0x3CAD9380`: for v > 0x2BD0 the edge-point block is present only when the archive backend is absent, its virtual +0x90 returns false, or v < 0x2BE5. At v >= 0x2BE5 a true selector omits that block. The selector is external archive context, not a serialized flag. Its precise backend class and semantic name remain unidentified. The scoped drawing reader supplies `facet_archive_mode_90=True`; standalone facet readers retain false. This matches the native branch and all 33 installed model layouts. No bytes are skipped heuristically, and no appearance-map representation is changed.

After all models, the shaded serializer writes three groups of direct lists. Each group contains `view_bucket_count` lists, in view order: collection +0xF8 for v > 0x90E, +0x118 for v > 0x9A5, then +0x48 for v > 0x11AF. Their respective wrapper vtables `0x3E41C200`, `0x3E41C290` and `0x3E41C170` share serializer `0x3E127DA0`. Every list is i32 count followed by that many four-byte words. Negative or impossible counts are invalid. These direct wrappers add no object tags or map slots. Word meanings remain unresolved; all populated words receive zero semantic credit. The native corpus contains only empty lists, totaling 2,100 count bytes. Synthetic populated lists verify count framing, ordering and unknown attribution.

All **20** installed VBLists streams now end exactly at EOF, including **33** shaded models (**23 part, 10 assembly**) in 19 streams and one absent-shaded stream. The reader frames **8,215,597 decoded bytes**, credits **7,567,905**, and retains **647,692 unknown bytes** for unresolved shared fields, reference strings and raw words. Earlier bucket and shaded-prefix boundaries and hashes remain in `drawing-vblists-corpus.json`. Full framing does not establish complete geometry semantics, editable-file round trips, coverage of older drawings, or full-document bit coverage. Proportional stored-byte attribution leaves **2,240,327 unknown drawing file bytes**, giving **57.3%** file-byte coverage or **49.9%** excluding container and thumbnails. Other drawing streams and installed project formats remain separate open work.

Evidence: C-156.
