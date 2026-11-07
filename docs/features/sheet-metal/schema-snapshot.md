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


### 11.254 Configuration-manager prefix and template sheet-metal context

*Status: B / ? — native configuration-manager prefix, template serializer alias and helper-state source verified; configuration bodies and automatic persisted-state selection remain incomplete.*

`Contents/CMgr` is an independent MFC archive rooted at `moConfigurationMgr_c`. Primary vtable `0x4CEC7AF8`, locator `0x4D31D0E0`, selects serializer `0x4C22C140` in hash-bound installed `sldmodu.dll`. The supported modern path calls `moFeature_c::Serialize`, then reads tagged configuration pointers associated with runtime members `+0x358,+0x360`. Configuration bodies must be decoded before later manager fields, including the sheet-metal archive state described below. The prefix reader uses a fresh null-at-zero object map for CMgr; it does not import a model configuration's cached map or read a model lightweight-section length before the feature prefix.

Descriptor `0x4DAEB4B8` binds factory `0x4C21C1A0` to default constructor `0x4B896990`. The constructor calls `moFeature_c(...,0)`. Revalidated feature constructor `0x4BB4BE10` forwards that zero argument to concept constructor `0x4BDB67C0`, which initializes child-list pointer `+0x58` to null and allocates a list only for a nonzero argument. CMgr's lightweight/resolved virtual slots select concept serializers `0x4C189B60` and `0x4C18A920`. The resolved concept serializer emits a direct child list only when that pointer exists. Therefore CMgr uses the established feature grammar **without the direct feature-list body/count**. Treating it as an ordinary folder inserts an incorrect two-byte count and misaligns the following fields. Constructor allocation 0x8C8 is runtime size, not stream length.

Three installed CMgr streams verify this distinction: Simulation `frame.sldprt` (**21,829 bytes**), API `SMGussetAPI.SLDPRT` (**2,758 bytes**) and API `fpf.sldprt` (**3,031 bytes**). Each begins with a 26-byte root class/object declaration, followed by a **139-byte inherited feature prefix**, ending at stream offset **165**. Each first configuration pointer declares `moPartConfiguration_c` at **0xA5** and stops before its body, with diagnostic cursor **192** after the class declaration. `configuration-manager-prefix-corpus.json` retains full stream and prefix SHA-256 values. The missing configuration body is an explicit dependency; no complete manager object or stream is claimed.

The manager serializer reads i32 runtime member **`+0x650`** for versions **above `0x1416`**, followed by a u32 map count and its entries. It is not a constant stream-tail word and cannot safely be obtained by indexing backwards from EOF. Native model functions **`0x4C249C50`** and **`0x4C250750`** obtain the model's configuration manager and copy this member into **archive-helper `+0x850`** when preparing model/configuration archives. This binds the conditional sheet-metal layouts in 11.154 and later clauses to persisted configuration-manager state. A freshly constructed archive helper's default zero alone does not prove the saved configuration's state.

`moTemplateSheetMetal_c` has primary vtable `0x3DC665D0`, locator `0x3DC77498` and serializer entry **`0x3DB6B1D0`** in hash-bound installed `sldsmmu.dll`. Its exact five-byte leaf **`E9 0B E1 FF FF`** jumps to established `moSheetMetal_c` serializer **`0x3DB692E0`**. The leaf has no unwind-table entry; the target is resolved from its instruction bytes rather than inferred from a nearby function range. It adds no derived wire payload. All inherited common-bend, dimension, fixed-face and helper-state conditions remain those of the target serializer.

Applying default helper state 0 to the two template cases misaligns the first derived display-dimension tag. An explicitly selected **helper-state-1 experiment** completes template roots of **8,291 bytes** in `frame.sldprt` and **8,764 bytes** in `SMGussetAPI.SLDPRT`; their raw stored `+0x8F0` binary64 values are approximately **0.003**. Both experiments advance to `moCompSheetmetalFeat_c`. This supports the state dependency and native alias, but does **not** prove that the persisted CMgr member has been decoded. `template-sheet-metal-state-probe.json` labels its assumed helper state explicitly. These experimental roots are excluded from the ordinary corpus completion totals and receive no whole-document coverage credit.

The production template reader now requires explicitly verified sheet-metal context before consuming its body. It does not infer state 1 from the class name, a plausible double, a successful object tag or a file-version threshold. The inherited-map resolved-stream handoff carries both the chosen state and the explicit context-verification marker. Since the CMgr prefix still stops before its configurations, the ordinary audit retains both template cases as **persisted configuration-manager context required** stops, before any template body bytes are accepted. The scalar state and its complete CMgr continuation remain open work.

The frozen 0.273/0.274 audit preserves every complete root record across 906 contexts. Ordinary totals remain **25,521 complete roots**, **828 complete lists** and **78 explicit stops**, including the two now-explicit configuration-manager context requirements. Exact-EOF totals remain **750 lists / 62,819,665 decoded bytes**, plus **78 complete lists with four-byte unknown tails**. Neither the three CMgr prefixes nor the mode-1 experiments are counted as completed whole streams.

Three focused tests verify the three source-bound CMgr prefixes and exact configuration-body stops, rejection before template body consumption without explicit context, and both mode-1 experimental root hashes with their assumption retained. The established sheet-metal tests continue to cover both conditional payload grammars. The historical split regression asserts the new explicit context requirement and its exact diagnostic cursor at the old template stop. Full part/assembly/drawing/all-installed-project semantic bit coverage remains incomplete; this increment provides native state provenance and a new bounded stream prefix, not complete manager or template-document coverage.

Validation: all **924 regression tests pass** (86.196 seconds); specification lint reports zero findings (478 headings, 96 anchors).

Evidence: C-296.


### 11.327 Saved configuration-manager context for template sheet metal

*Status: Verified opt-in context propagation; two template bodies complete · Evidence: C-369*

The completed CMgr reader now supplies persisted sheet-metal helper state to model and separate resolved-feature readers. Native model functions 0x4C249C50 and 0x4C250750 copy manager memory member +0x650 into archive-helper +0x850, as established in 11.254/11.261. The state is not a constant EOF word or a default inferred from a fresh helper. Native CMgr Serialize 0x4C22C140, both state-copy functions and their decompilation identities are retained and revalidated in `template-sheet-metal-persisted-context-native-evidence.json`.

`configurationmanager.sheet_metal_context(doc)` first requires supported manager framing, the postload graph guard and exact EOF. It then obtains the saved +0x650 value and its recorded source position, verifies the original signed i32 at that position, and accepts only helper states 0 or 1. Versions <=0x27CB or manager layouts without the verified state-position field reject. The result records the value, field offset, CMgr source SHA-256, version and stream name. This establishes the state used for parsing; its broader application semantics remain partial and no additional semantic byte credit is assigned.

`modelprefix.read(..., configuration_context=True)` derives this context from the same document before reading the model root. It installs +0x850 and its verification marker on that archive instance. This mode currently requires the supported part-manager path; assembly context is not inferred. `resolvedfeatures.read_data` copies the state, verification marker and source provenance into the separate stream archive while preserving the model's inherited object identities. It copies the provenance dictionary so mutations do not change the model archive's record. The default model-prefix mode remains unchanged for historical replays.

Both existing template probes store state **1**. The frame and API-gusset sources now complete their `moTemplateSheetMetal_c` objects using that saved value, reproducing the earlier explicit-mode experiment without assuming a state. They reach cursors **324,619** and **27,885**, respectively, and stop explicitly on fresh `moCompSheetmetalFeat_c` objects. Neither resolved list is complete, so this work adds no whole-stream or stored-byte credit. All 919 part CMgr framing completions and current document coverage totals remain unchanged.

`template_persisted_context.py` reproduces the two probes in `template-sheet-metal-persisted-context-corpus.json`, retaining configuration names, resolved-stream hashes, root intervals/hashes, completed template records and CMgr provenance. `template_persisted_context_evidence.py` checks the native state propagation and source-derived endpoints. Tests verify the exact saved i32/hash, both template bodies, unchanged unverified historical context and rejection of unsupported state values, missing positions and mismatched source bytes. Other resolved-feature dependencies, complete lists and semantic/document/project coverage remain open.

Validation: all **1,085 regression tests pass** (122.231 seconds); specification lint reports zero findings (551 headings, 96 anchors).

Evidence: C-369.


### 11.328 Component sheet-metal feature reference

*Status: Native-bound scoped body; three additional resolved roots · Evidence: C-370*

`moCompSheetmetalFeat_c` is implemented in `sldsmmu.dll` (SHA-256 ae20d533b36f105f22de4ed5984fb2245bbf03fb91a84ae473e0941a4b9e3c54). Primary RTTI table **0x3DC1F3B8**, locator **0x3DC71908**, binds Serialize +0x28 to **0x3D8617D0**. That entry is an exact five-byte E9 jump to **0x3D8617E0**; the implementation ends at **0x3D861843**, occupying **99 bytes**. Its complete bounded disassembly and raw import bindings are retained; no missing decompilation is treated as a recovered body.

The serializer first imports `moCompRef_c::Serialize`, then constructs an archive-helper view and checks storing/loading. Its store branch writes signed i32 from object member **+0xE8**; its input branch reads a signed i32 into that member. There is no version gate or second suffix word in this implementation. The wire grammar is therefore the ordinary modern component-reference body of 11.17 followed by **one raw i32**. Its application meaning remains unknown. It differs from `moCompFeature_c`, which appends a feature ID and archive time. Reusing that two-word suffix would consume four bytes from the following record.

`modelcomponents.component_sheet_metal` implements this grammar without global registration. `resolvedfeatures.read_data(..., sheet_metal_references=True)` installs it only in the new separate-stream archive. The model context's reader table is copied and remains unchanged. Existing reads retain their original dependency stops. A synthetic all-zero modern component prefix followed by raw value -17 consumes **86 bytes** and leaves a sentinel intact; a truncated suffix rejects.

Using the verified saved context from 11.327, the frame source now completes **69 roots**, adding two beyond its prior 67, and reaches cursor **329,984**. It stops on fresh `moTemplateFlatPattern_c` at tag **0x508E3**. The API-gusset source completes **21 roots**, adding one beyond its prior 20, and reaches cursor **32,524**. It stops on fresh `moSMGussetEndCornerFilletSurfIdRep_c` at **0x7EE2**. Endpoints include consumed class declarations at failure. These are later explicit dependencies, not completed lists.

`component_sheetmetal_evidence.py` checks module identity, primary table, exact thunk target, complete implementation bytes, the imported inherited serializer and signed archive input, and both source-derived later endpoints. `component-sheetmetal-native-evidence.json` retains the native instructions/imports and source-root intervals/hashes; `component-sheetmetal-corpus.json` retains the two extended probes. Tests check the one-word boundary/truncation, scoped registration, exact added root totals and both later stops. Neither list is complete, so no whole-stream or stored coverage is added. Semantic meanings, complete resolved streams and full part/assembly/drawing/project coverage remain unfinished.

Validation: all **1,087 regression tests pass** (121.919 seconds); specification lint reports zero findings (552 headings, 96 anchors).

Evidence: C-370.

