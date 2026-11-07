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

### 11.331 Selection-item data nodes

*Status: Native-bound modern node/pointer body; two additional exact-EOF streams; later failures explicit · Evidence: C-373*

`moSelectionItemData_c` primary RTTI table **0x4CE3C558**, locator **0x4D30BE50**, binds Serialize +0x28 to **0x4B65E090**. This serializer first calls inherited helper **0x4B65E000**, which uses ordinary `su_CObject::Serialize` below version 0x1BCC and `moNode_c::Serialize` at/above that gate. The derived input reads a tagged **moRefWrapper_c pointer at +0x40** only above 0x1BCB. Both conditions are already satisfied by the supported ordinary component context (>0x27CB); older selection-item layouts are not enabled.

Modern node Serialize **0x4C1890A0** dispatches its lightweight and resolved stages. The concrete selection-item table binds virtual **+0x4B8** to node lightweight input **0x4C18A1A0** and **+0x4C0** to no-op **0x4B29DAE0**, whose exact bytes are C2 00 00. This establishes the complete inherited wire body without importing feature or folder fields. The modern grammar is:

| Order | Field |
| --- | --- |
| 1 | Tagged node-name pointer (+0x18). |
| 2 | Raw i32 +0x0C, raw u32 flags +0x28, signed node ID. |
| 3 | Raw u32 +0x2C when version >0x143A. |
| 4 | Raw CString +0x20 when version >0x23CD. |
| 5 | Tagged reference-wrapper pointer (+0x40). |

Node-name and wrapper objects use the same inherited archive map. Runtime node flag edits do not replace stored values. Unknown node fields retain their existing attribution; only structural tags and established ID/name framing receive credit. A minimal modern body with null name/wrapper and empty raw CString consumes **21 bytes**, including **13 unknown bytes**. Tests retain a following sentinel, preserve signed/raw values, reject truncation and reject unsupported legacy context before consumption.

`resolvedfeatures.read_data(..., selection_items=True)` installs `selectionitems.read` only in that separate archive's local table. It does not globally register the class. Current four-file outcomes, using saved configuration context and verified post-list MBSM framing, are:

| File | Complete selection objects | Cursor | Outcome |
| --- | --- | --- | --- |
| Plastics Examples/handle.sldprt | 4 | 17,637 | Exact EOF; 24 feature roots. |
| Plastics Examples/mounting_plate.sldprt | 2 | 31,830 | Exact EOF; 28 roots, followed by an empty MBSM map. |
| Plastics Examples/screw_cap.sldprt | 1 | 11,222 | Later object reference names a class slot. |
| Plastics Examples/wrench.sldprt | 8 | 13,771 | Later sketch valued-G2 count fails bounds validation. |

The two later failures are retained as failures; class-map indices are not guessed and invalid sketch counts are not skipped. Completing the selection-item nodes does not validate the rest of these failed graphs. **15 selection objects** complete across the four probes. No whole-document or stored-stream coverage credit is added by this staged extension.

`selection_items.py` reproduces the source-bound probes in `selection-items-corpus.json`, retaining node fields, reference archive slots and root/end observations. `selection_items_evidence.py` verifies primary and stage bindings, the derived/base/node native bodies and no-op, and exact probe outcomes. `selection-items-native-evidence.json` retains those native bytes and decompilation identities. Tests reproduce both EOFs and both explicit later failures, in addition to body boundary/context checks. Semantic fields and complete part/assembly/drawing/project coverage remain unfinished.

`resolved_persisted_context_audit.py --mbsm --selection-items` reports **832 exact-EOF streams / 74 explicit stops / zero context stops**, with **25,611 completed roots** across the same 906 configurations. `resolved-selection-audit-0.351.json` retains every outcome. All **830** previously complete streams preserve exact endpoints, root counts and unknown-byte totals. Completed streams now contain **68,230,395 decoded bytes**, including **24,037,174 unknown bytes**. This diagnostic remains outside the stored-coverage registry; failed graphs and unresolved semantics receive no added credit.

Validation: all **1,093 regression tests pass** (129.860 seconds); specification lint reports zero findings (555 headings, 96 anchors).

Evidence: C-373.
