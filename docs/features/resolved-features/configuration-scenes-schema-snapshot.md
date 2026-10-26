# Ordinary configuration scene schema snapshot

### 11.275 Runtime scene factories and child-list allocation

*Status: Partial · two exported runtime-class records and exact factory bodies · Evidence: C-317*

The installed exported CRuntimeClass records bind scene names seen in MFC headers to their actual load factories:

| Runtime class | Descriptor address | Factory | Runtime object size | Stored runtime schema |
| --- | --- | --- | --- | --- |
| swCubicalScene_c | 0x4DAF95D0 | 0x4C6E5A00 | 0x2B8 | 0x80000001 |
| swSphericalScene_c | 0x4DAF9600 | 0x4C6E5A50 | 0x2B8 | 0x80000001 |

The native export names are `?classswCubicalScene_c@swCubicalScene_c@@2UCRuntimeClass@@A` and `?classswSphericalScene_c@swSphericalScene_c@@2UCRuntimeClass@@A`. Each record's class-name pointer, size, schema and create-function pointer are read directly from the hash-bound module. The versionable-schema flag in 0x80000001 is runtime metadata; the observed fresh wire headers retain schema 1. Runtime size 0x2B8 is an allocation size and SHALL NOT be used as a serialized body length.

Both factories allocate **0x2B8 bytes**, call the default scene constructor **0x4C6E6230 directly**, then install the respective cubical or spherical vtable. They do not call the exported default cubical constructor at 0x4C6E5D30. That exported constructor is an independently verified route to the same default scene constructor, but is not the factory's intermediate call.

The default scene constructor calls the concept constructor **0x4BDB6850**, which allocates and stores the direct node list at +0x58 as documented in 11.271. This closes the factory-binding gap for both derived scene types: under successful native allocation, freshly loaded cubical and spherical scenes have the concept child-list pointer required by the resolved serializer. It does not establish an enclosing helper +0x834 value, nor remove the existing ordinary-context restriction on the research parser.

`configuration_scene_factory_evidence.py` independently resolves both exported records, validates their class names, size/schema/factory fields, binds the exact factory code and checks each direct call to 0x4C6E6230. Results and decompilation hashes are retained in `configuration-scene-factory-native-evidence.json`. Lossless production owned-map integration and its ordinary helper context remain the next verification steps. Coverage is unchanged; the 101 research cursor EOF outcomes of 11.274 remain excluded from supported-manager totals.

Evidence: C-317.

### 11.276 Scene-map condition mask and initial helper state

*Status: Partial · native helper initialization and two condition operations · Evidence: C-318*

Archive condition **0x200** and helper state **+0x834** are distinct native fields. The owned scene-map loader adds condition 0x200 through `moArchiveHelper_c::addCondition`. Its installed sldmfcu.dll implementation at **0x3CA57C00** loads the shared helper-object pointer and, when both pointer and condition are nonzero, performs `condition_mask_at_+0x10 |= condition`. The matching `removeCondition` at **0x3CA59410** performs `condition_mask_at_+0x10 &= ~condition`. Both routines return without accessing +0x834. Adding condition 0x200 neither sets nor clears that state.

The exported `moArchiveHelperObject_c` constructor at **0x3CA561B0** initializes the shared helper object. It zeros EBP at **0x3CA561CD** and stores that zero to **[R14+0x834] at 0x3CA5656D**. It initializes the +0x10 condition mask separately from its incoming argument. Consequently a fresh helper begins with +0x834 zero, and the scene loader's condition operation preserves that value. A reused helper can carry a different +0x834 state; neither a scene name nor condition 0x200 proves otherwise.

The scene grammar of 11.269–11.275 remains scoped to ordinary non-DB external-XML loading with archive binary flag 0x10 clear and helper +0x834 zero. This matches the existing ordinary model-reader context. Nonzero reused-helper state remains excluded because the concept resolved stage can omit the direct feature list. Context selection must be explicit in a reader; successful parsing of a candidate byte sequence is not a replacement for that scope restriction.

`configuration_scene_context_evidence.py` binds the installed sldmfcu.dll hash, independently resolves the constructor and condition-operation exports, retains the complete exception-table constructor range and exact condition-operation bytes, and verifies the zero initialization and separate +0x10 updates. Results are in `configuration-scene-context-native-evidence.json`. This supplies native support for the initial helper-state assumption in the research scene replays; it does not establish all reused runtime paths. Lossless production scene-map integration and corpus regrading remain outstanding. No byte coverage changes in this edition.

Evidence: C-318.

### 11.277 Bounded owned scene-map framing reader

*Status: Partial · opt-in map framing; fresh scene bodies remain explicit dependencies · Evidence: C-319*

`configurationscenemaps.owned_map` implements the two direct owned scene collections from 11.269 without the research probe's count substitution. It requires the existing ordinary external-XML context and version above 0x46C1, rejecting unsupported scope before consuming bytes. The result retains start/end cursors, both signed declared counts, ordered object entries and ordered association entries.

The first collection reads each tagged scene pointer followed by its raw i32 value. The second reads each raw i32 association key followed by its tagged scene pointer. Neither collection allocates an MFC map slot for its entry wrapper. Duplicate keys and multiple references to the same scene remain distinct entries with shared object identity. Null scene pointers remain null. Negative signed counts are retained and skip their loops. Positive counts are bounded against a minimum of six remaining bytes per entry (two-byte pointer plus four-byte integer), and primitive/tag readers reject truncation at the actual field boundary. The serialized integer values and keys remain unknown bytes until their meanings are established.

The CMgr prefix reader exposes this framing through **`scene_map_records=True`**. When enabled, the +0x6E0 stage calls the bounded reader and preserves the real counts and entries in `data_6e0`. The +0x6D8 decal map is unaffected. Default prefix and supported-manager behavior remain unchanged. This option does not install scene body readers: a fresh scene class still produces an explicit dependency stop unless a verified reader is installed in the archive's local reader table. It therefore cannot turn the earlier research cursor EOF outcomes into production coverage by itself.

`test_configurationscenemaps.py` verifies signed extrema, duplicate association keys, null/shared mapped pointers, no entry-wrapper object slots, exact cursor and raw-byte accounting, negative counts, impossible counts, truncation, legacy/context rejection before input, and fresh scene rejection without an explicit body reader. The three focused tests and the three existing owned-manager tests pass. The full regression suite passes all **967 tests** (94.992 seconds); specification lint reports zero findings (501 headings, 96 anchors). Complete lossless scene-body integration, native supported-manager replay/regrading and full document/project semantic coverage remain outstanding. No new file-byte credit is assigned.

Evidence: C-319.

### 11.278 Native replay of bounded scene maps with actual entries

*Status: Partial · 117 real map results; local research scene-body readers · Evidence: C-320*

`configuration_scene_map_reader_replay.py` validates the bounded reader of 11.277 against the native map spans retained in 11.274. It enables the actual CMgr +0x6E0 map path with `scene_map_records=True` and installs the existing scene-body routines only in a local research archive reader table. It does **not** substitute zero for either map count. The returned manager object therefore contains the actual map counts and ordered entries, including all raw first-map values and association keys.

For every one of the **117** installed source files, the replay checks the decompressed manager SHA-256, both declared counts, entry boundaries, scene classes, raw map integers, association keys, complete map boundaries and map-span SHA-256. Those counts and intervals exactly match the earlier independently retained manual map replay. It additionally records the MFC object-map slots for every scene pointer and association pointer, retaining object identity instead of comparing only class names.

The results contain **297 distinct scene objects** and **626 associations**. Every association points to a scene in its containing first map. There are **329 repeated association references**, computed as association count minus distinct associated scene slots per map. These are repeated pointers to shared objects; they do not introduce 329 additional scene bodies or object-map slots.

With actual map entries retained, **101** candidate manager cursors still reach exact stream EOF and **16** retain the unsupported postload-graph stop. This confirms that the earlier cursor results were not caused by deleting scene payload bytes or consuming incorrect map lengths. It does not promote those candidates into the default supported coverage audit: scene-body registration is still local to research, and lossless/count-bounded scene-body integration remains outstanding. Candidate EOF remains distinct from full semantic byte coverage.

The native replay is now a regression in `test_configurationscenemaps.py`; all four focused map tests pass. The full suite passes **968 tests** (91.382 seconds); specification lint reports zero findings (502 headings, 96 anchors). `configuration-scene-map-reader-corpus.json` retains the map hashes, counts, pointer slots and outcomes. The test skips explicitly if the installed source corpus is unavailable. Default manager parsing and measured coverage remain unchanged. Remaining work includes scene-body integration, supported corpus regrading, graph variants and full part/assembly/drawing/all-installed-project semantic bit coverage.

Evidence: C-320.

### 11.279 Scoped scene bodies with retained counts and wire bytes

*Status: Partial · 297 exact native scene-body intervals; explicit local installation · Evidence: C-321*

`configurationscenes.py` implements the modern ordinary condition-200 scene grammar of 11.270–11.276 without globally registering scene classes. Its `install(archive)` function validates ordinary external-XML context and version above 0x46C1, then installs cubical/spherical scene and nested scene-data readers in that archive's local reader table. Unsupported context and legacy versions are rejected before consuming body bytes. The supported-manager default does not install these readers yet.

The scene reader retains the merged-list presence, each direct node list's declared packed count and raw tail, the signed wall count, ordered wall keys/types and all decoded suffix fields. Direct node-list pointers, the concept visual pointer and the floor-selection pointer accept null or mapped references while rejecting fresh bodies in those still-unverified helper positions. Reader tables are restored on both success and failure. Wall visual properties continue through their existing verified nested scalar/pointer reader.

The nested scene-data reader preflights its three CString fields and four i32 selectors to reach the signed wall-info count. Before invoking the existing versioned scalar decoder, it bounds positive counts by a **22-byte minimum per modern direct entry**: i32 key, two minimal narrow CStrings and four i32 fields. It retains negative counts unchanged, with empty native loops. The parent scene's positive wall count is bounded by at least eight bytes per key/type pair, followed by type-specific parsing. Node-list counts are bounded by the minimum tagged-pointer width; unknown wall types remain explicit errors.

Both the scene and nested scene-data results retain **exact `wire_hex` slices**, start/end cursors and the original decoded values. These slices preserve noncanonical count/string encodings, floating bit patterns and values changed by native load-time normalization. They are source-byte retention alongside the documented grammar, not a claim that all retained fields have established semantic meanings or deserve byte credit. No raw region is counted twice merely because it appears in a containing scene slice and a nested data slice.

The native replay of 11.278 now accepts `scoped_bodies=True`. In that mode it uses the new reader and compares every scene's interval, source hash and wall-type sequence with the frozen manual corpus, as well as all map counts/entries/pointer slots. It reproduces **297 scene bodies totaling 1,335,679 bytes** in the same **117 maps**, retaining 626 associations and the same 101 candidate EOF / sixteen postload-graph outcomes. This comparison checks boundaries and identities independently of the new reader's result structure.

`test_configurationscenes.py` verifies all native intervals/maps, scope rejection before input, helper-pointer rejection with reader-table restoration, oversized wall-info count rejection before actual input, negative count retention and truncation. All three focused tests pass. The full suite passes **971 tests** (95.955 seconds); specification lint reports zero findings (503 headings, 96 anchors). Default coverage remains unchanged pending explicit supported-reader integration, unknown-byte accounting review and corpus regrading. Full document and all-installed-project semantic bit coverage remain incomplete.

Evidence: C-321.

### 11.280 Scoped scene unknown-byte accounting

*Status: Partial · 297 source-bound scene accounting results; no file credit · Evidence: C-322*

The scoped reader now separately records unknown-byte accounting for each nested scene-data body and its containing scene. Existing raw node fields, direct node-list tail words, wall geometry/style fields and nested material/visual unknowns remain charged by their established readers. This edition additionally charges **four bytes for every scene-data wall-info key**, **four bytes for every scene wall key**, and the scene's **four-byte flag word**. The framing and runtime offsets of these fields are established, but their value domains or bit meanings remain unresolved. Counts and type discriminators retain their documented structural roles.

Nested unknown charges are included once in the containing scene's archive delta. The retained scene-data wire slice and containing scene wire slice do not themselves add unknown-byte charges: they preserve exact source bytes alongside decoded values. Likewise, nested raw data is not credited a second time because it appears in both slices. Each scene result requires its unknown delta to lie between zero and its body length.

`configuration_scene_accounting.py` replays the actual bounded maps with scoped scene bodies, preserving the native interval and pointer-identity checks of 11.278–11.279. It retains start/end intervals, source SHA-256, body lengths, known/unknown byte totals, signed wall counts and nested scene-data unknown totals in `configuration-scene-accounting.json`. Across **297 scene bodies**, the unchanged **1,335,679-byte** body denominator splits into **695,955 documented structure bytes** and **639,724 unknown bytes**. These totals include bodies inside candidate managers and the sixteen unsupported postload graphs; they are not eligible whole-stream credit by themselves.

The native accounting regression in `test_configurationscenes.py` reproduces the complete accounting report and checks that each containing scene includes at least its nested scene-data unknowns, 72 raw rectangular-geometry bytes plus four raw key bytes per native wall, and the raw four-byte scene flag word. All four focused scene tests pass. The full suite passes **972 tests** (97.246 seconds); specification lint reports zero findings (504 headings, 96 anchors). Supported-manager integration and a fresh corpus-wide stored-byte regrade remain required before these body-level figures can affect reported part coverage. Full document/project semantic bit coverage remains incomplete.

Evidence: C-322.

### 11.281 Supported owned scene maps and stored-byte regrade

*Status: Partial · 101 additional complete managers; 123,756 additional stored part bytes · Evidence: C-323*

The supported ordinary singleton part-manager reader now enables the scoped scene bodies and bounded +0x6E0 maps from 11.277–11.280. `read_supported(doc)` defaults to **`scenes=True`**; **`scenes=False`** preserves the edition 0.288 scene-map frontier. Prefix callers opt in with **`scene_map_records=True`**. In this integrated path that prefix option also installs `configurationscenes` in the archive's local reader table for versions above 0x46C1, superseding the preliminary framing-only behavior described in 11.277. Unsupported legacy manager layouts keep their explicit stops.

The integrated reader retains actual signed collection counts, ordered entries, raw map values/association keys, declared nested counts, scene pointer identities and exact scene/data wire slices. It uses the reviewed unknown-byte charges of 11.280. It does not use the research count-substitution hook. Fresh helper-list/visual/floor-selection bodies outside the verified scope remain dependency stops. Default graph validation still requires the existing guarded singleton configuration identity and postload predicates; scene parsing does not relax those guards.

`configuration_scene_supported.py` replays the same **919 installed part manager occurrences** and retains `configuration-manager-supported-0.301.json`. It records **717 complete supported managers / 202 explicit stops**, preserving all **616** previous positive summaries exactly. The **101** additional complete streams total **1,031,720 decoded bytes**: **535,620 known structure bytes** and **496,100 unknown bytes**. Cumulative complete streams total **3,127,073 decoded bytes**, comprising **1,578,420 known structure bytes** and **1,548,653 unknown bytes**.

The sixteen scene-bearing managers with unsupported postload graphs remain uncredited. Together with other graph variants, the current audit retains 86 postload-graph stops. Other remaining groups include 77 unsupported component-version/context cases, ten legacy owned-manager layouts, toolbox specification arrays, decal maps and further dependencies. Exact EOF for the 717 guarded roots establishes only the supported stream grammar; their raw fields and other document streams still prevent full semantic document coverage.

The versioned coverage runner regrades only completed supported CMgr streams, credits their known decoded proportion of stored bytes using the existing floor rule, and leaves file denominators unchanged. It adds **123,756 stored part bytes** beyond edition 0.288, bringing cumulative CMgr credit to **518,079 bytes**. Remaining stored CMgr unknown bytes are **1,348,033** of the unchanged **1,866,112-byte** denominator. Total part unknown file bytes become **68,726,650**. Part coverage is **69.7% overall / 63.2% excluding container and thumbnail**, compared with the prior 63.1% excluded-overhead figure. Assembly coverage remains 83.8% / 81.3%; drawing coverage remains 57.3% / 49.9%. Installed project formats remain outside this container-corpus denominator.

`configuration_scene_supported_evidence.py` revalidates the installed native bytes behind the base, factory, helper-context, scene-data and wall evidence, preserves the old positive summaries, and binds the new audit and coverage hashes in `configuration-scene-supported-native-evidence.json`. The corpus-wide coverage regression now requires 717 positives and 1,578,420 known decoded bytes. Historical replay tests and scripts explicitly pass `scenes=False` where they reproduce earlier frontiers. No whole document is declared complete; remaining manager variants, raw meanings and full part/assembly/drawing/all-installed-project semantic bit coverage remain open.

Validation: all **972 regression tests pass** (131.471 seconds); specification lint reports zero findings (505 headings, 96 anchors).

Evidence: C-323.

