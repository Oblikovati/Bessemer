# Owned feature stream schema snapshot

### 11.10 Resolved-feature stream prefix and archive context

*Status: Partial; prefix and loader context verified, object bodies open · Evidence: [C-25], F.13*

`Contents/Config-N-ResolvedFeatures` is opened separately from the model archive. The loader obtains the storage file version (or its explicit model override). It restores the cached load-array/store-map into the stream archive before reading objects. Readers SHALL preserve this inherited MFC class/object context; a fresh map containing only null is insufficient in the native baseline. The cache wrapper itself writes no framing bytes and its destructor is empty.

| Order | Field | Type | Condition | Meaning | Status |
|---|---|---|---|---|---|
| 1 | `unknown_prefix` | i32 | v > 0x1BC6 | Loaded into a local; role unresolved. | ? |
| 2 | `feature_count` | ReadCount | | Direct `suObList` body count, using 11.8.2's u16/u32 count encoding. | V |
| 3 | `features` | object[feature_count] | | MFC object records using the inherited map; feature-class bodies unresolved. | ? |

The list is serialized directly, without an enclosing `suObList` object tag. The loader can subsequently deserialize MBSM data objects for an eligible part configuration; therefore the list alone is not the entire stream. Readers SHALL NOT discard or count trailing bytes as understood. Staged `resolvedfeatures.read_data` reads remain outside coverage. The guarded complete ordinary part reader in 11.334 accounts for eligible MBSM data and exact EOF before crediting established fields. The modern feature serializer invokes lightweight and resolved virtual stages (11.11). The concept and assembly `SerializeResolvedData` bodies are nested stages, not alternative whole-stream layouts (OI-003).

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

### 11.334 Guarded resolved-feature stream coverage

*Status: Edition 0.354 checkpoint; 839 supported part streams registered; semantic and other-context coverage incomplete · Evidence: C-376*

`resolvedfeatures.read_supported(doc, name)` now composes the verified ordinary part path into a complete stream reader. It accepts only exact names `Contents/Config-N-ResolvedFeatures` with decimal N, derives the corresponding model/header context from the same document, and requires saved CMgr state through 11.327. The existing model reader must complete its supported ordinary part framing before the separate archive inherits its class/object identities. All established local dependency readers through 11.333 are enabled; unsupported classes and malformed records still raise explicit failures.

The reader consumes every declared direct feature-list root. When saved helper state is 1, it verifies the selected configuration is `moPartConfiguration_c` and reads the native-bound MBSM map of 11.330 using that same stream archive. Saved state 0 has no such map. It then requires **exact EOF** and a bounded unknown-byte total. Neither partial roots, class-map failures, invalid counts nor trailing bytes earn any credit. A real otherwise-complete source with appended bytes is rejected and receives zero documented bytes.

`resolvedfeatures.documented_bytes` returns **stream length minus explicitly unknown bytes** only after this complete path succeeds; supported framing does not convert raw fields into known semantics. It returns zero on supported parse/context failures. `swcover.py` registers only the exact `Contents/Config-\d+-ResolvedFeatures` family, leaving other stream names and unsupported part/assembly/drawing contexts uncredited. Staged `read_data` remains separately available and retains its historical dependency scopes.

The inherited CMgr/model/header data are read to establish context, but their source bytes are not added to this stream's credit. The new archive's counters start at zero; already mapped objects consume only their source pointer tags. Existing CMgr credit remains separate and unchanged. Historical CMgr-only regrade scripts explicitly exclude this later resolved-stream decoder when reproducing their snapshots.

All **906 modern part audit outcomes** match the public coverage reader exactly: **839** supported streams contribute **46,233,516 known decoded structure bytes** and **67** failed streams contribute zero. Completed stream payloads total **71,228,187 bytes**, with **24,994,671 unknown bytes** retained. The full installed collector contains **983 part** and **183 assembly** occurrences of this stream family; older/unverified configurations and assembly occurrences receive zero. This is a configuration-stream count, not a count of fully documented files.

The complete stored-byte regrade measures **13,239,143 additional known part bytes**. The part resolved-feature family's unchanged **25,137,466 stored-byte** denominator now contains **11,898,323 unknown bytes**. Total part unknown bytes fall from **68,367,632** to **55,128,489** in the unchanged **227,086,352-byte** document denominator. Part coverage becomes **75.7%**, or **70.5% excluding container and thumbnail bytes**. Assembly coverage remains **83.8% / 81.3%** and drawing coverage **57.3% / 49.9%**. No full-document or installed-project completion is claimed.

`resolved_coverage.py` regrades every installed container document, compares every modern audit decision, records source SHA-256 and decoded/stored lengths, and asserts proportional stored attribution and exact per-kind deltas. `resolved-feature-coverage-0.354.json` retains all **1,166 source stream rows**, the prior totals and new totals. `resolved_coverage_evidence.py` independently rechecks every stored source interval/hash, every ratio, all 839 positives and zero assembly credit; `resolved-feature-coverage-native-source-evidence.json` binds the audit, regrade and generated gap report hashes. `coverage-gaps.md` and Annex G reflect this measured registry.

Tests cover all 906 modern outcomes and their exact known-byte total, exact registry matching, unsupported names, actual appended-source rejection and preservation of unknown-field attribution. Native schema evidence remains in clauses 11.129–11.333; this section composes those bodies without widening unsupported native contexts. Remaining raw semantics, 67 modern feature failures, older formats, assembly/drawing resolved streams, other model streams and all installed project-format semantics remain unfinished.

Validation: all **1,100 regression tests pass** (155.789 seconds); specification lint reports zero findings (558 headings, 96 anchors).

Evidence: C-376.

