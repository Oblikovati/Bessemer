# Chooser, browser and configuration graph schema snapshot

### 11.129 Separate resolved-feature lists and browser folders

*Status: B — inherited-map prefix framing; later feature classes and field meanings remain partial.*

`Contents/Config-N-ResolvedFeatures` uses the cached model archive context established in 11.10. The modern ordinary part reader first completes the corresponding configuration stream, then copies its class/object slots and archive-local reader modes to a new stream archive. Copying preserves object identities; it does not reserve additional slots or emit bytes. An arbitrary fresh map is not equivalent. `resolvedfeatures.read_data(data, context, max_objects=3)` requires a completed modern ordinary model archive and returns the first up-to-three direct list roots. Unsupported classes, impossible counts and truncated fields raise errors rather than permitting resynchronization.

The direct list begins with one raw i32 for v > 0x1BC6, followed by ReadCount and that many tagged object pointers. ReadCount is the u16 / FFFF-plus-u32 form in 11.2. The raw prefix is retained and remains semantically unknown. In all 889 successful audited prefixes its value equals the inherited map length; this observation alone does not establish its purpose. There is no enclosing list-object tag and no trailing moNodeList word: the native loader calls suObList::Serialize, not moNodeList_c::Serialize. The same map is retained for all roots and nested records.

The following modern folder bodies extend the common `moFolder_c` body of 11.15. Each array below is a **direct** suObArray body: ReadCount followed by tagged object pointers, without the i32 nonnull flag used by suObArray::serializePtr. The arrays add map slots only for their contained tagged objects. Stored order, duplicates, null pointers and inherited object references are preserved.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | favorite folder base | moFolder body | moFavoriteFolder_c | Common feature and two raw folder words | B |
| 2 | `favorite_items` | Direct object array | v > 0x140D | Native embedded collection +0x3C0 | B |
| 3 | `subfolders` | Direct object array | v > 0x1C2F | Native embedded collection +0x3E0 | B |
| 4 | `unknown_400` | i32 | v > 0x1C2F on load | Preserve raw; no semantic credit | B |
| 5 | history folder base | moFolder body | moHistoryFolder_c | Common folder body | B |
| 6 | `history_items` | Direct object array | v > 0x17E5 | Native embedded collection +0x3C0 | B |
| 7 | selection-set folder base | moFolder body | moSelectionSetFolder_c | Common folder body | B |
| 8 | `selection_sets` | Direct object array | v > 0x1BCB | Native embedded collection +0x3B8 | B |
| 9 | `unknown_3d8` | i32 | v > 0x1BCF on load | Preserve raw; no semantic credit | B |

All listed gates are satisfied by the implemented v > 0x27BC scope. Favorite/history constructors identify the embedded arrays; their vtable +0x28 serializers call the common folder serializer before the listed fields. The selection-set serializer subsequently calls moNode_c::SerializeDBData, which operates on the external database/deferred store and adds no reads from this ordinary archive stream. Database content semantics remain separate work.

`moConfigCommentsFolder_c` uses the common folder serializer directly and adds no suffix. Modern `moFavoriteFeatItemData_c` and `moHistoryFeatItemData_c` call moNode_c::Serialize: their runtime tables select node lightweight at +0x4B8 and an empty RET 0 resolved stage at +0x4C0. The base item body therefore consists of the node lightweight layout of 11.11, including its existing unknown-byte attribution. The final derived feature-item serializers additionally read a tagged moCompFeature_c pointer at +0x40, after that node body. Edition 0.140 omitted this pointer by selecting a temporary base-class table; 11.130 corrects the binding. Runtime identities remain distinct.

The installed audit examines **906 modern part configurations**. With the scoped integrations in 11.136, all 906 required model contexts complete, and **905** complete the selected three-root prefix. These prefixes total **629,751 bytes** in **83,628,137 stream bytes**. Their six observed root sequences are retained individually with stream/prefix SHA-256, spans, declared counts and inherited-map sizes. One case stops in the separate list at a nested moPlaneSection_c body inside moPlaneSectionFolder_c. No model-context failure remains in this modern corpus. No audited list is complete at three roots. The blank native part advances through comments, favorite and history folders at offsets 203, 410 and 606; clauses 11.131–11.136 extend its supported continuation.

The edition 0.140 report is retained as `resolved-feature-three-roots-0.140.json`. Its former invalid-count case was a reader alignment error: the favorite feature item's omitted pointer began with a new-class declaration, which was misread as an extended array count. The corrected native-bound reader frames that case through offset 772, with one favorite feature item, its moCompFeature_c pointer and zero subfolders. Earlier sequence variants that incorrectly treated feature pointers as top-level roots are superseded.

This staged reader remains outside the whole-stream coverage registry. Later roots, native post-list stages, older archives, assembly/drawing resolved-feature contexts and unresolved field meanings prevent a full-stream or full-document coverage claim.

Evidence: C-157.

### 11.130 Final item tables, document folders and selection-set data

*Status: B — native-bound modern load framing; field meanings and nested classes remain partial.*

A constructor may assign a base-class vtable and then overwrite the same primary pointer with its derived table. Readers and evidence collectors SHALL use the final assignment when selecting the complete runtime serializer. Favorite feature items finish at vtable `4CE35188`, serializer `4B65AC60`; history feature items finish at `4CE42D08`, serializer `4B6622F0`. Their earlier temporary tables select the base item serializers `4B65AEC0` and `4B662410` only. Both derived serializers call their base, then serialize the moCompFeature_c pointer at +0x40 for the modern scope. This pointer follows ordinary null/new/reference archive rules and may reference an inherited or already-read object. It is not an array count or a new list root.

The sensor-folder constructor binds vtable `4D06AA60` to serializer `4BEC2E00`. On load it calls only the common folder serializer; the sensor database export call is store-only. `moDocsFolder_c` binds `4D05DE08` directly to the common folder serializer `4BEC2AF0`, without a suffix. `moLiveSectionFolder_c` binds `4D066FF8` to `4BEC2BE0`: common folder body, then one i32 for v > 0xF59. Native code compares that word with 1 when assigning its runtime byte at +0x370. Readers preserve the original four bytes; its field meaning remains unresolved and receives no semantic credit.

Selection-set data likewise has two primary assignments: temporary moItemData_c table `4CE3C048`, then final moSelectionSetData_c table `4CE3CA68`. The final serializer `4B65E130` calls the base node-stage serializer `4B65E000`, then reads this suffix:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `raw_40` | i32 | v > 0x1BCB | Native member +0x40; preserve raw, no semantic credit | B |
| 2 | `raw_48` | CString | v > 0x1BCB | Native member +0x48; preserve text, no semantic credit | B |
| 3 | `comment` | Tagged pointer | v > 0x3720 | moFeatComment_c pointer; shared map, no reset | B |
| 4 | `items` | Direct suObList | v > 0x1BCB | ReadCount then tagged item pointers, in stored order | B |

The constructor identifies +0x50 as a direct suObList, not an array pointer. The modern base node path uses the node lightweight layout and empty resolved stage, as verified by both temporary and final vtable slots. Nonempty selection item and comment bodies still require their own class readers. An unsupported nested class causes an explicit stop.

At the edition 0.141 checkpoint, `resolved_feature_list_frontier.py` probed all declared roots with the same inherited context, recording only the last fully completed root before an error. Across **906** modern configurations, **14** stop constructing model context; **892** reach the separate list, framing **4,608 complete roots** before explicit unsupported-class stops. None reaches a complete list. The leading next dependencies are moDetailCabinet_c (821 cases), moSensor_c (50), moSurfaceBodyFolder_c (15) and moSelectionItemData_c (4). Context failures and list failures remain separate. Failed partial objects receive no credit, and the diagnostic remains outside the coverage registry.

Evidence: C-158.

### 11.135 Complete body readers and per-body chooser data

*Status: B — ordinary modern composition and native-bound chooser framing; raw semantics and nested dependencies remain partial.*

The existing complete inherited body stages now have archive-local entry points for the following concrete runtime classes. Their primary tables and common concept stages are independently verified in the installed model module. Each complete entry point executes every inherited reference, chooser, map, late-atom and body stage before the selected suffix; a partial prefix is never returned as a complete object.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | moExtrusion_c / moBoss_c | Complete extrusion body | Ordinary modern scope | Both +0x28 serializer slots select 4BB25180 | B |
| 2 | moCut_c | Complete extrusion body | Ordinary modern scope | Serializer 4BB227C0 delegates to 4BB25180; remaining load work changes runtime flags only | B |
| 3 | moRevolution_c / moRevCut_c | Complete revolution body | Ordinary modern scope | Both +0x28 slots select 4BB322F0 | B |
| 4 | moRib_c | Complete rib body | Ordinary modern scope | +0x28 selects 4C3B7E70 | B |
| 5 | moPerBodyChooserData_c | Chooser record below | Ordinary modern scope | +0x28 selects 4C281080 | B |

The scope remains v > 0x27CB and the ordinary external-XML context of the existing body clauses. The common model-feature/profile/atom/body layouts and suffix version gates remain those already specified; this composition adds no wrapper bytes or archive-map resets. Native cut loading may adjust a node flag based on creation version; the offline reader preserves serialized flags rather than emulating that runtime normalization.

`bodyfeatures.install_scoped` copies the archive's reader table. It preserves explicit caller-supplied probes by default, while the separate ResolvedFeatures reader requests the complete supported entry points explicitly. Complete part-resolved traversal installs missing body/profile readers and, when absent, the scoped reference-plane reader. Earlier partial-stage selections are unaffected. No complete reader is inserted into the global whole-object registry. Unsupported nested bodies and special archive branches still raise explicit errors.

The per-body chooser constructor is reached through the native runtime-class creator: runtime record 4DAECB60, creator 4C26E030, constructor 4B775800. Its in-memory allocation is 0x98 bytes, which is not a serialized size. The constructor identifies three embedded suObArray members and a separate embedded su_CDWordArray. The modern body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `array_08` | Direct suObArray | Always | ReadCount and tagged object pointers | B |
| 2 | `array_28` | Direct suObArray | Always | Independent ReadCount and tagged object pointers | B |
| 3 | `array_48` | Direct suObArray | v > 0x518 | Independent ReadCount and tagged object pointers | B |
| 4 | `raw_68` | i32 | v > 0x754 | Preserve stored value; no semantic credit | B |
| 5 | `raw_6c` | i32 | v > 0x754 | Preserve stored value; no semantic credit | B |
| 6 | `dwords_78` | Direct su_CDWordArray | v > 0x84E | ReadCount then u32 values; all values remain semantically unknown | B |
| 7 | `raw_70` | i32 | v > 0x995 | Preserve stored value; no semantic credit | B |

All these gates are satisfied by the implemented modern scope. The fourth collection contains four-byte words, **not object pointers**. None of the embedded collections has an i32 pointer-presence flag or enclosing object tag. Counts SHALL fit their respective two-byte pointer or four-byte word minimum. Stored order, duplicates, null pointers and inherited identities are retained. Load callbacks 4B77E6D0 and 4B77E670 perform reference-use bookkeeping on the first two object collections; the schema reader does not mutate runtime reference counts or add wire fields for those actions.

At the edition 0.146 checkpoint, the selected three-root prefix across 906 modern part configurations succeeded in **898** cases. Seven contexts remain unsupported before opening the separate list and one fails in its first three roots. The full-list frontier records **15,990 complete root occurrences**, **74 complete lists**, **825 explicit list stops** and **seven model-context stops**. All 74 complete lists reach exact EOF and span **1,697,087 decoded bytes**. Completed body-root occurrences include 199 extrusions, 114 bosses, 112 cuts, 65 revolutions, five revolution cuts and seven ribs, with **1,175,078 wire bytes** in their root spans. These are root occurrences, including MFC references, rather than counts of newly serialized object instances.

The leading next dependencies are moBBoxCenterData_c (256 cases), detail folders (55), sensors (53), Fillet_c (47) and planar surfaces (41). Counts identify only each case's first failure. The previous edition 0.145 frontier remains retained. Native regressions verify all six body classes at non-reference body boundaries; synthetic chooser cases distinguish object and DWORD arrays, retain noncanonical words and reject malformed/truncated records. Successful framing does not complete raw field meanings, semantic coverage accounting, other document streams or installed project formats. The full-document bit-coverage goal remains open.

Evidence: C-163.

### 11.187 Per-body chooser data with stored filename

*Status: B / ? — native inheritance/suffix and 16 populated containing-root records verified; stored string and raw option semantics remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moPerBodyChooserDataWithFileName_c primary table `0x4CE87928`, complete-object locator `0x4D3160E0` and serializer slot +28 select `0x4C280F20`. Native RTTI establishes moPerBodyChooserData_c ancestry. The derived serializer first calls existing base helper `0x4C281080`, then appends the three fields below. All selected base and suffix version gates precede the supported minimum.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | chooser base | direct moPerBodyChooserData_c body | always | Three separate direct object arrays at +08/+28/+48, two i32 at +68/+6C, direct u32 array at +78, then i32 +70. Each array has its own packed ReadCount. Preserve duplicate/order/map identity. | B / ? |
| 2 | stored filename +98 | CString | always | No separate filename-presence word. Preserve the stored string and encoding; its filesystem meaning is not established by the runtime class name. | B / ? |
| 3 | reference +A0 | tagged moRefWrapper_c-family pointer | always in scope | Historical load begins after `0x777`. Standard non-DB ReadObject/WriteObject; no untagged wrapper substitution. | B |
| 4 | raw +A8 | i32 | always in scope | Historical load begins at `0x77C`. Preserve the complete signed word. | V / ? |

With all four arrays empty, the inherited base occupies 20 bytes. An empty CString, null reference and the raw word add seven bytes, giving a minimum **27-byte** body before outer tagged framing. Embedded base/array bodies do not allocate class/object map slots of their own. Newly tagged array items and the reference pointer retain normal MFC map behavior.

The completed-root audit retains **16 filename chooser records**. Every observed base has object-array counts 2/0/1, an empty u32 array, null +A0 reference and raw +A8 equal to zero. Stored strings resemble identifiers; readers do not validate them as UUIDs, normalize their case or use them to open filesystem paths. Three focused tests cover populated direct arrays, repeated cached pointers, duplicate/high-bit u32 values, UTF-16 CString decoding, signed suffix extrema, minimum tagged composition, truncation and a source-hash-bound complete containing-root span with stored string/word equality. Nonnull native reference variants remain unexercised by this cohort.

Evidence: C-215.

### 11.282 Remaining postload configuration-graph shapes

*Status: Partial · 86 source-bound stopped graphs; guard remains unchanged · Evidence: C-324*

`configuration_postload_graphs.py` replays exactly the **86** edition 0.301 cases stopped by the supported manager's postload-graph guard. It validates every decompressed source SHA-256, catches the actual guard stop and retains both root pointer identities, the ordered direct +0x368 list, and all configuration nodes reachable through those roots, list entries and serialized parent/child fields. Identity indices are local deterministic inventory labels, not MFC slot numbers or native addresses.

All 86 cursors are at the end of their serialized manager streams. The guard reasons are **67 non-singleton direct-list identity cases** and **21 selected-root child-list cases**; these counts overlap. There are **1,591 distinct reachable configuration nodes**, ranging from two to 377 per file. Every retained node is `moPartConfiguration_c`, has raw +0x77C zero and has no optional feature +0x1E0 payload. The selected root pointers match in these cases; neither differing selected-root identity nor those two branch fields explains these stops.

The serialized relationship observations require care. All **1,591 parent +0x698 pointers are null**, while child +0x6A0 lists contain **22 links**. These fields therefore do not form a reciprocal serialized parent/child relation. For example, `samples/introsw/hinge_1.sldprt` has one directly listed selected root pointing to a second configuration through its child list, while both nodes' serialized parent pointers are null. This observation does not establish corruption or authorize a reader to invent reverse pointers. Runtime traversal/link fixups must be checked independently.

In **65 files**, the direct list contains every inventoried node. In **21**, some nodes are reachable through child links but absent from that direct list. All direct lists retain unique identities; no duplicate list entry is inferred from repeated names or shared graph links. The existing native manager postload code calls `getConfigurationsList` with its runtime traversal options before dispatching configuration hooks, so direct-list length alone is not sufficient to prove which nodes receive those calls.

The inventory is retained in `configuration-postload-graphs.json`, with source hashes, exact stop positions, node fields and identity links. Its reproducer asserts the 86/1,591/22 totals, all-null serialized parents, zero branch flags and exact field-end cursors. No production guard is relaxed and no byte credit is added. The next proof obligation is native configuration traversal and link-fixup behavior for these graph shapes, followed by per-node postload dispatch verification. Full part/assembly/drawing/all-installed-project semantic bit coverage remains incomplete.

Evidence: C-324.

### 11.283 Native forward traversal of postload configurations

*Status: Partial · bound tree walker and list-generator callbacks; 86 acyclic source graphs · Evidence: C-325*

The exported `moConfigurationMgr_c::getConfigurationsList` at **0x4B8A9DA0** builds a `moConfigurationListGenerator_c`, runs its tree walker, then appends the generated runtime list to the caller's list. The postload call of 11.282 passes include-type bits **3** and final argument **1**. These arguments have distinct effects; the final argument is not itself the child-recursion switch.

The generator's primary vtable **0x4CEC9800**, locator **0x4D31D2C0**, has four relevant slots:

| Offset | Target | Role on this path |
| --- | --- | --- |
| +0x00 | 0x4B8B6BA0 | startWalk |
| +0x08 | 0x4B8A64B0 | add visited configuration to generated list; return zero |
| +0x10 | 0x4B29C5A0 | completion callback, exact `xor eax,eax; ret` |
| +0x18 | 0x4B8AD600 | recursive iWalkR |

`startWalk` selects the manager's direct +0x368 list because generator state +0x28 is zero. For each nonnull configuration it invokes the recursive slot. Include-type bits 3 bypass both native type-filter predicates. The recursive method invokes the +0x08 callback, and its zero return permits descent through the configuration's +0x6A0 list (whose runtime head pointer is at +0x6A8). It does not use the serialized parent +0x698 pointer to find children. The all-null parent pointers observed in 11.282 therefore do not suppress child traversal.

The final `getConfigurationsList` argument 1 is stored at generator +0x80 and causes every visited configuration to be added, independently of configuration runtime +0xA60. Generator +0x30 remains zero on this construction path: insertion uses ordinary AddTail rather than addUnique, and additional-representation expansion guarded by that state is disabled. Consequently repeated reachable children could produce repeated runtime list entries; an implementation must not silently deduplicate serialized identities to mimic this traversal. The observed direct lists have no duplicate identities, but that alone would not prove uniqueness of all graph walks.

The walker and generator manipulate runtime lists/counters and accept no archive cursor. On the selected argument path their relevant calls add no serialized fields. The existing bound manager postload method then invokes virtual +0x11C0 for each generated configuration. Prior part-configuration/base/feature dispatch evidence is revalidated against the installed bytes; each node's zero raw +0x77C and absent optional feature branch remain required when extending that no-inline-byte proof beyond a singleton.

`configuration_graph_traversal_evidence.py` binds the four primary slots, four new native function ranges, the exact three-byte completion callback, and all prior postload-hook bytes. It also independently checks child-edge acyclicity for all **86** retained source graphs; all terminate. Null parent pointers alone were not used as proof of termination. Results and decompilation hashes are retained in `configuration-graph-traversal-native-evidence.json`.

The production singleton guard remains unchanged in this edition. The next step is an explicit per-node graph guard and supported-corpus replay, preserving identities and rejecting cycles or unverified node/branch variants. No new stored-byte credit or full document/project completion is claimed.

Evidence: C-325.

### 11.284 Guarded multi-configuration postload completion

*Status: Partial · 86 additional complete managers; 190,883 additional stored part bytes · Evidence: C-326*

The supported reader extends its no-inline-byte postload proof from a singleton to explicitly validated configuration graphs. `read_supported(doc)` defaults to **`graphs=True`**; **`graphs=False`** retains the former singleton guard. Prefix callers opt in with **`configuration_graph_records=True`**. The new guard follows the native forward-child traversal established in 11.283 without inventing parent links or altering serialized list entries.

Acceptance requires matching nonnull selected root pointers, a nonempty direct list with unique nonnull configuration identities, and a selected root reachable from that list. Every reachable node must be `moPartConfiguration_c`, retain a null serialized parent +0x698 pointer, have raw +0x77C zero and have no optional feature +0x1E0 payload. Its child field must be a list of nonnull configuration objects. An explicit iterative active/done traversal rejects child cycles and validates each distinct node's branch predicates. It does not recursively call Python per graph level, so deep valid graphs do not depend on the interpreter recursion limit.

Shared-child identities remain shared in the parsed graph. The validation's distinct-node set proves the same zero-inline-byte branch for each reachable object; it does not rewrite or deduplicate the native walk's list entries. Repeated visits to a verified node do not create archive reads. The reader records the number of distinct verified nodes and zero postload inline bytes while preserving all existing configuration pointers and child lists. Nonnull serialized parents, unknown node classes, branch flags, duplicate direct roots, unreachable selected roots and cycles remain explicit unsupported paths.

`configuration_graph_supported.py` replays the same **919 installed part managers**, retaining `configuration-manager-supported-0.304.json`. The audit now records **803 complete supported managers / 116 explicit stops**, preserving all **717** previous positive summaries exactly. All **86** previous graph-stop cases now complete under the per-node proof. Their streams total **4,759,277 decoded bytes**, comprising **1,765,025 known structure bytes** and **2,994,252 unknown bytes**. Cumulative supported streams total **7,886,350 bytes**: **3,343,445 known** and **4,542,905 unknown**. Exact completion of these stream grammars does not resolve the raw-field meanings or other document streams.

Stored-byte regrading adds **190,883 part bytes** beyond edition 0.301. Cumulative CMgr credit becomes **708,962 bytes**, leaving **1,157,150 stored CMgr bytes** unknown out of the unchanged **1,866,112-byte** denominator. Total part unknown file bytes become **68,535,767**. Part coverage is now **69.8% overall / 63.3% excluding container and thumbnail**; assembly coverage remains 83.8% / 81.3%, and drawing coverage remains 57.3% / 49.9%. Project formats remain outside this container-corpus denominator.

`test_configurationgraphs.py` replays all 86 native graphs with the new option and confirms that disabling it retains the old stop. Synthetic tests preserve shared-child identity, accept a 1,501-node chain, and reject cycles and unverified root/node/branch variants. `configuration_graph_supported_evidence.py` revalidates the native traversal and prior postload-hook bytes, the unchanged previous positives, and the new audit/coverage hashes in `configuration-graph-supported-native-evidence.json`. Historical replay scripts explicitly disable the graph option when reproducing older frontiers. The corpus-wide coverage regression now requires 803 positives and 3,343,445 known decoded bytes. Remaining class/version/layout dependencies, raw meanings and full part/assembly/drawing/all-installed-project semantic bit coverage remain open; no whole document is declared complete.

Validation: all **975 regression tests pass** (106.749 seconds); specification lint reports zero findings (508 headings, 96 anchors).

Evidence: C-326.

