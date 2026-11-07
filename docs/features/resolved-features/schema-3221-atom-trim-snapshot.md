### 11.89 Atom partition/maps, late fields and body-feature suffix

*Status: Native ordinary atom/body-base framing; direct-map continuation specified in11.173; raw meanings remain partial · Evidence: [C-104], [C-201]*

In the scoped ordinary zero-condition archive, the continuation after 11.88 begins with a nullable direct array at +0x548, then i32 partition presence. A nonzero presence adds a tagged moPMarkRecord_c pointer and three i32 values at +0x3B8, +0x3C0 and +0x578. The mark record's body is one i32 permanent-mark ID. Runtime pointers and cached kernel objects in that class add no archive fields. Readers SHALL preserve the signed ID and raw presence value.

Next is i32 word_5C8 and three maps in order. Map +0x670 stores i32 count and count pairs of u32 key/u32 value. Map +0x5D0 stores i32 count and count pairs of tagged moCompEdge_c pointer/u32 value. Map +0x3F8 stores i32 count and count pairs of u32 key/u32 value. Readers SHALL preserve duplicates, pointer identity, unsigned high bits and original entry order independently of native hash-map insertion.

The late atom fields are i32 word_488, i32 appearance_count and appearance_count pairs of i32 key/tagged moVisualProperties_c pointer, then i32 direct-data presence at +0x6A0. Nonzero introduces the dedicated direct moFeatureMultiBodySurfIdMap_c body specified in11.173, rather than a tagged object or a primary-Serialize dispatch. After that selected body, or immediately when presence is zero, the continuation is i32 face-name count and pairs of tagged moCompFace_c pointer/CString, then i32 index-name count and pairs of i32 key/CString, and final i32 word_6B4. All these fields are present in the documented modern scope. Legacy branches and nonzero archive masks may differ.

This completes the atom-feature helper. The ordinary moBodyFeature_c continuation is tagged moAsmFeatData_c pointer, tagged moCompFeature_c pointer, direct embedded suObArray at +0x778 and i32 word_798. The component-feature pointer is omitted under archive condition 1, outside this zero-condition path. These bytes precede the boss/extrusion-specific moEndSpec_c suffix in 11.81. Unsupported nested objects still stop the containing reader.

At the historical C-104 checkpoint, the dedicated map, late and body-tail audits each reached nine native boundaries. All nine cases stored partition records; the three maps contained zero entries, and the optional late direct-data body was absent. Twelve profile objects completed during traversal, while three other enclosing cases stopped on cosmetic-thread objects. Those results verified atom and body-feature base framing, not complete boss/extrusion objects or streams; the reported default complete manager count was892. The later C-201 direct-map contract and its separately qualified containing-root evidence are specified in11.173. These retained historical counts are not a current frontier measurement or whole-document coverage claim.

### 11.173 Direct atom-feature surface identifier maps

*Status: B — native load/store framing, direct construction and populated containing-root boundaries verified; key/value meanings and runtime map semantics remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. The late atom-feature stage `0x4BB1EC00` reads the existing i32 presence value for native member +6A0. When nonzero, it allocates `0xB8` bytes, invokes constructor `0x4BCE39B0` and directly calls load helper `0x4BCE7600`. Native store uses helper `0x4BCE7A60`. These are dedicated load/store functions, not a tagged archive object or primary Serialize virtual call. The presence gate after version `0x104C` precedes this scope.

Constructor `0x4BCE39B0` installs moFeatureMultiBodySurfIdMap_c's primary table `0x4CFEAE88` after its moBaseSurfIdMap base. RTTI/COL evidence verifies that ancestry and su_CObject inheritance. Runtime owner references, timestamps and map/update state created by the constructor are not additional serialized fields. The primary Serialize slot is a different path from the dedicated load/store helpers; no general tagged-map reader is registered from this direct layout.

After a nonzero +6A0 presence value, the direct body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | entry count | i32 | always | Signed count, not packed ReadCount. Reject negative or out-of-buffer counts. Zero consumes no entries. | V |
| 2 | entry key | u32 | each entry | Preserve all 32 stored bits and original entry order. | V / ? |
| 3 | entry value | i32 | each entry | Preserve signed value. Its sign selects the following optional branch. | V / ? |
| 4 | child-list presence | i32 | entry value < 0 | Preserve full stored presence word; any nonzero value selects row 5. This field is absent for nonnegative entry values. | V |
| 5 | child count | i32 | negative value and child-list presence nonzero | Reject negative or impossible counts. Zero is permitted. | V |
| 6 | child integers | count × i32 | row 5 present | Preserve every value, including negative integers and duplicates that native loading may filter. | V / ? |

The negative-value child-list branch begins after `0x1059`, before this modern minimum. There is no class tag, object-map slot, key CString, pointer-sized integer or per-entry alignment. The empty direct body occupies **four bytes**, excluding the preceding presence word. An entry with a nonnegative value occupies **eight bytes**; a negative-valued entry adds a four-byte presence word, and a present list additionally adds its four-byte count and four bytes per child.

The native loader populates internal pointer-keyed maps from these integer values. It drops some negative-valued entries when no usable children remain, ignores negative child integers and deduplicates child keys. The offline reader preserves the original ordered wire entries and lists without implementing those lossy runtime operations. Duplicate outer keys remain distinct records. The enclosing atom-feature reader resumes immediately after the direct body with its existing face-name/index-name collections and raw +6B4 word. Arbitrary synthetic values establish framing, not native writer validity or complete surface-ID engineering meaning.

The complete-root audit retains **13 direct maps** with **53 entries**. There are 42 nonnegative-valued entries and 11 negative-valued entries, all 11 with stored child-list presence one. Child counts are two in seven entries, three in three entries and four in one entry. Direct-map counts range from one to eight entries. Evidence is collected only from fresh objects within completed containing roots; incomplete objects at a later stop receive no completed-record credit.

Four focused tests verify duplicate outer keys, signed/unsigned extremes, noncanonical presence words, negative/duplicate child retention, empty bodies, malformed counts and truncation, context rejection, atom-stage continuation and unchanged object-map slots. The native test verifies the stream SHA, exact containing-root span and retained direct map. Key/value semantics, runtime surface-ID updates, historical/special archive contexts and additional feature payloads remain open.

Edition 0.184 records **22,418 complete roots**, **676 list-complete configurations** and **230 explicit stops** across **906 complete modern contexts**. **615 lists reach exact EOF**, spanning **32,762,939 decoded bytes**; **61 complete lists retain four-byte unknown tails**. The prior atom-feature direct-data stop is resolved in this scope. Drafts, surfaces, cut lists, sweep cuts, annotations and other feature dependencies remain unresolved. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these boundaries.

Evidence: C-201.

### 11.222 Surface-cut and trim-reference-surface features

*Status: B / ? — five surface cuts and one trim verified inside complete native roots; raw meanings, unobserved pointer combinations and full-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature context, version >0x27CB. sldMoPartu.dll primary RTTI binds moSurfCut_c (0x1804BBD88 / COL 0x1804F2548) to 0x180357290, and moTrimRefSurface_c (0x1804C54B8 / COL 0x1804F29D0) to 0x180360030. Both have moBodyFeature_c ancestry, call its imported serializer and use the ordinary inherited moConcept_c lightweight/resolved hooks. Surface-cut follows the complete body-feature prefix with:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | atom feature +808 | tagged moAtomFeature_c | always in scope | Preserve pointer identity. | B |
| 2 | raw +818,+81C | two i32 | always | Separate signed words. | B / ? |
| 3 | face +820 | tagged moFaceRef_c | always in scope | Nullable tagged occurrence. | B |
| 4 | surface +810 | tagged moConstSurfRef_w | always in scope | Existing constant-surface grammar. | B |
| 5 | plane +900 | tagged moPlaneRef_w | always in scope | Existing plane-reference grammar. | B |
| 6 | arrays +8C0,+8E0 | two direct suObArray | always in scope | Each ReadCount and tagged items, in order. | B |
| 7 | chooser +838 | direct moBodyChooser_c | always in scope | Direct object list then tagged moAtomFeature_c owner; no extra tagged chooser object. | B |

Exact moSurfCut_c::getSurface names current member 0x808 as a surface-feature pointer of declared type moAtomFeature_c*. getSheetIndex returns fullsigned32 member 0x818. Named setSurface sequentially writes its first formal pointer from RDX to 0x808, then its second formal integer from R8D to 0x818, on the original receiver. No normalization, index bounds, zero/one-based convention, pointer/index consistency check or atomic-pair update is established.

The canonical serializer writes 0x808 through its typed moAtomFeature_c pointer operator, followed by 0x818 through signed32 codecs. Selected LOAD uses a latched signed version comparison >=0xD6 for the typed pointer branch, then reads 0x818; older obsolete-pointer/list handling remains excluded. Archive identity and later provider effects are not a guarantee that the current pair remains coherent or that a surface cut succeeds. Raw 0x81C and runtime 0x828 KeepPieceIndex receive no new saved meaning.

All positive native claims in C-3529 use the original retained packets. Its two incident companions preserve one excluded duplicate 719-byte static capture and record zero qualifying missing-body source extensions. The duplicate is not new positive evidence. Raw file values, readers and accounting remain unchanged. Evidence: C-3529.

Native load gates >=0xD6, >=0x3AB, >0x4CA, >0x7F9 and >=0x70F all precede scope. Constructor 0x180356F70 binds the two suObArray members and embedded body chooser. With null/cached pointers and empty arrays/chooser, the complete body is **275 bytes**. New targets contribute normal tagged bodies and map slots.

Trim constructor 0x18035F590 allocates a moTrimSurfData_c at runtime +800 through constructor 0x18035F840 and constructs the direct body chooser at +808. The serializer does not write a tagged trim-data object: it dispatches selected members of that allocation directly. Runtime offsets prefixed data below refer to that allocated data object. After the complete body-feature prefix the modern trim sequence is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | data AllTools +20, Surfaces +48 | two direct suObArray | always in scope | Named embedded collections, serialized in this order. | B |
| 2 | data profile +18 | tagged moProfileFeature_c | always in scope | Normal archive map identity. | B |
| 3 | data direction +40 | tagged moDirectionSpec_c | always in scope | Nullable target. | B |
| 4 | data raw +10 | u32 | always in scope | Native gate >0x4DF predates scope. | B / ? |
| 5 | feature chooser +808 | direct moBodyChooser_c | always in scope | Direct object list then tagged owner. | B |
| 6 | data solid +68 | tagged moSolidRef_w | always in scope | Existing solid-reference grammar. | B |
| 7 | data keeper +78 | direct allocated moEntRefKeeper_c body | always in scope | suObArray serialization; no pointer tag or separate count prefix beyond that array grammar. | B |
| 8 | data array +A0 | direct suObArray | always in scope | Separate direct array. | B |
| 9 | data raw +08,+0C | two i32 | always in scope | Preserve signed words. | B / ? |

**CombineIndices and collection mutation requests (C-3666).** Exact own moTrimSurfData_c APIs bind the existing saved direct array +0xA0 to CombineIndicesArray. The name and typed provider requests do not establish scalar integer elements, ID meanings, closed item classes or relationships to another list; its existing tagged-object array grammar remains unchanged.

getCombineIndicesArray(out) first requests SetSize(out,0,-1,1), then Append(out,original data+0xA0), and returns the current output DWORD count. That is a total after the requests, not a count of successful new appends. The caller output is mutated and has no local null guard. setCombineIndicesArray(input) first requests deleteAll on original data+0xA0, then tail-requests typed mo_ComponentBodiesIndex_c::copyComponentIndicesArray(original input,original data+0xA0), with a void return. Neither provider name proves copying, successful deletion, final contents or element semantics.

The own setAllTools method requests deleteAll on +0x20, then tail-requests mo_CrvSrfRefArray_Copy(original input,+0x20,0) and returns its integer result. setSurfaces requests deleteAll on +0x48, then mo_SurfRefArray_Copy(original input,+0x48,0), followed by mo_ConstSurfRefArray_Copy(the same original input,+0x48,0). It returns the native 32-bit sum of the second result and preserved first result. These integers are not established as successful insertions, element counts or Boolean success. The second provider can observe effects of the first on aliased or current state.

All original reference addresses remain subject to valid storage/lifetimes and provider effects. Reset-before-source-use can change aliased input. Null pointers, invalid counts/backing/items, callbacks, ABI compliance, exceptions and partial mutation remain qualified. No snapshot, independent clone, deep ownership, unique insertion, rollback, atomic replacement or geometric result follows. The previously documented getAllTools/getSurfaces/setOriginalTools policies remain distinct.

The complete 1,030-byte parent serializer and 350-byte data constructor authenticate +0xA0 and the existing +0x20/+0x48 arrays through the allocated data at feature +0x800. Four new API roots contain 250 code bytes; the preparation helper contains 14 reachable bytes under a 48-byte ceiling, excluding 34 padding or adjacent bytes. The typed copy/delete/append providers were not followed. Existing context/version rules, raw keys, object-array framing and wire order remain unchanged, with no parser/accounting, semantic byte credit or phase completion added. Evidence: C-3666.

Exact own-data getAllTools and getSurfaces return addresses of the embedded collections at data 0x20 and 0x48 respectively. They are aliases to current collections, not snapshots. Named setOriginalTools addresses the same data 0x20 collection, invokes typed su_CObArray::SetSize with arguments (0,-1,1), then tail-calls Append with the supplied source collection reference. This proves operation order and the named association; it does not prove deep-copy semantics, closed item classes, final contents, alias safety or rollback on failure. The existing direct suObArray archive grammar is unchanged. The serializer reloads current feature 0x800 separately before dispatching each collection, so same-instance lifetime across callbacks is not promised. Evidence: C-3533.

The following exact moTrimSurfData_c property roles refer to the allocated data reached through feature 0x800. They do not add a separately tagged TrimData object or split packed u32 data 0x10 into additional wire fields.

| Saved component | Native property / writer | Qualified policy |
|---|---|---|
| data 0x10 mask 0x01 | MutualTrim | Getter projects bit0; setter zero clears it, nonzero sets it, preserving other bits. |
| data 0x10 mask 0x08 | RemovePicked | Getter projects bit3; setter zero clears it, nonzero sets it, preserving other bits. |
| data 0x10 mask 0x10, gated by 0x01 | SpacePartition | Getter is zero unless MutualTrim is set. Nonzero setter ORs 0x11, forcing both components; zero setter clears only 0x10. |
| data 0x10 mask 0x04 | setConsuming | Setter-only association; zero clears and nonzero sets 0x04, preserving other bits. |
| data 0x10 mask 0x40 | UseRollback | Getter returns (current word >> 6) & 1; setter zero clears 0x40, nonzero sets it, preserving the other 31 current bits. |
| data 0x10 high 16 bits | CompleteOpts saved fragment | Current provider synchronization below; not a complete 32-bit saved property. |
| data 0x08 | CreateSolidFlag | Raw full signed32 native getter/setter and archive word. |
| data 0x0C | CreateSolidOption | Raw full signed32 native getter/setter and archive word. |

Exact own-data moTrimSurfData_c::setUseRollback(int) reads original data 0x10 and writes old&0xFFFFFFBF for zero input or old|0x40 for nonzero input. The complementary exact useRollback query reads that word, shifts right by six and masks with one, returning only the current request bit. The same whole unsigned32 archive codec stores this component in the allocated TrimData object reached through feature 0x800. These local query/setter checkpoints do not restrict the whole file word to Boolean values, prove a successful rollback effect or guarantee a universal final state. They are separate from feature runtime UseRollback at 0x8B0. Mask 0x2 and other unassigned components remain open; the existing CompleteOpts high-fragment and version/proxy policies are unchanged. No wire field is added. Evidence: C-3531, C-3533.

CompleteOpts has distinct saved-fragment and runtime-provider policies. Its setter first writes the full 32-bit input to current 0x70->0x98 when 0x70 is nonnull, then copies only that input's low 16 bits into data 0x10's high 16 bits, preserving the low 16 bits. With null 0x70, its getter returns the current packed high 16 bits fragment. Otherwise the getter returns the full 32-bit value at current 0x70->0x98 and conditionally synchronizes that value's low 16 bits into the packed high 16 bits, again preserving the low 16 bits. The getter can therefore mutate the packed field, and its provider-backed result is not restricted to 16 bits. Provider type, numeric option meanings, aliases and later state remain separate. Offline decoding reads the stored u32 and does not call these methods.

The selected direct modern LOAD path requires its current query >=0xDF7. Separate current unsigned queries admit data 0x10 from 0x4E0 in both directions and data 0x08 from 0x1FE5 in both directions. Selected STORE writes data 0x0C without its LOAD gate; LOAD admits it from 0x2000 and has no direct reset at the skipped branch. Primitive codecs are unsigned32 for 0x10 and signed32 for 0x08/0x0C. Proxy transfers, replacement data and separate provider queries retain their original context; no single immutable query value or current-object identity is promised. Other low bits and upper-fragment numeric domains remain unassigned. No file Boolean/enum closure or successful trim, consuming or solid-creation effect follows. Evidence: C-3525.

The trim proxy/transfer path below version 0xDF7 is excluded. Later native gates >0xDF8, >0x1FE4 and >=0x2000 also precede scope, so all rows are present throughout the supported context. Constructor evidence identifies direct arrays +20,+48,+80,+A0 and an allocated entity-reference keeper +78. Array +80 is constructed but is not separately dispatched by this serializer; it must not be inserted into the wire sequence. The keeper's primary serializer thunk 0x1803AD148 resolves to sldmfcu.dll suObArray::Serialize. The body chooser independently binds through sldmodu.dll primary 0x4CE879D0 to 0x4C27AAA0: ordinary zero-byte su_CObject base, direct object list and tagged owner. Runtime allocation sizes 0xC0 for trim data and 0x8B8/0x908 for feature objects are not serialized lengths. The complete empty trim body is **281 bytes**.

Both readers are installed only in the copied modern feature-context map. Four focused tests cover full pointer/collection ordering, duplicate and cached identities, signed/u32 extremes, populated arrays/keeper/chooser, direct-data map accounting, tagged minima at 0x27CC and version 19000, cached self/following-byte alignment, truncation/context rejection and source-hash-bound complete native containing-root spans/raw fields/counts/target classes for both features.

Five native surface cuts have raw +818 = -1; raw +81C is zero in three and one in two. Both arrays and chooser lists are empty. Each has a plane-reference pointer and null atom/face/constant-surface pointers. The complete native trim has array counts (1,1,0,0), one chooser item, profile moProfileFeature_c, null direction/solid, u32 raw +10 = 458760 and raw +08/+0C zero. These observations do not define valid domains or restrict other pointer combinations.

Across 906 complete modern contexts there are **24,399 complete roots**, **774 complete lists** and **132 explicit stops**: 22 additional roots and three additional lists relative to Edition 0.227. **701 lists reach exact EOF**, spanning **46,743,994 decoded bytes**, an increase of 211,155 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. Later affected parsing reaches moOffsetRefSurface_c or trim-edge entity representations rather than skipping them. `resolved-feature-list-frontier-0.227.json` preserves the preceding checkpoint. Whole-document coverage credit is unchanged; complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-250.

### 11.1988 UI producer requests for the saved Trim CompleteOpts fragment

*Status: four selected native CompleteOpts input requests verified; source-word meanings, numeric domains and final saved state remain open - Evidence: C-3857.*

Three anonymous functions in the current UI DLL contain four actual indirect CALLs through ordinary IAT slot0x3EBED4D8. That slot names the exact own moTrimSurfData_c::setCompleteOpts(int) declaration at0x1803645F0 in the Part DLL. The calls establish intended native receiver/input interfaces, not validated runtime class or receiver storage. No EAT declaration names any of these three UI roots, and no UI owner class follows from addresses or proximity.

| Anonymous root / reached call | Observed data receiver | Raw full32 input producer |
|---|---|---|
|0x3E8E5170 /0x3E8E5249|OriginalRCX is captured inRBX. A current virtual+0x108 response is captured inRSI; its current QWORD+0x800 is captured inRDI before later opaque requests. RDI suppliesRCX at the selected call.|Late CURRENT DWORD[originalreceiver+0x18C] suppliesEDX at0x3E8E5240.|
|0x3EADAD20 /0x3EADAE22|OriginalRCX is captured inRBX. Reload CURRENT pointer[originalreceiver+0x4B48], then load its CURRENT QWORD+0x800 intoRCX.|CURRENT DWORD[originalreceiver+0x50E0] suppliesEDX.|
|0x3EADB010 /0x3EADB21C|OriginalRCX is captured inRDI. Load CURRENT pointer[originalreceiver+0x4B48], then its CURRENT QWORD+0x800 intoRCX.|A separate CURRENT pointer[originalreceiver+0x4B58] supplies its CURRENT DWORD+0x18C toEDX.|
|0x3EADB010 /0x3EADB23D|After the preceding opaque CompleteOpts request returns normally, reload CURRENT pointer[originalreceiver+0x4B48] and its CURRENT QWORD+0x800 forRCX.|Reload CURRENT pointer[originalreceiver+0x4B58]; its CURRENT DWORD+0x158 suppliesEDX.|

Each selected source is transported as the complete native DWORD to the declared int input, without a local mask, clamp or Boolean conversion at that load/call checkpoint. These raw source offsets receive no engineering names, option labels, enum interpretation or valid-domain closure. In the last root the two requests are ordered on normal continuation, but both source and native-data pointers are reacquired. They need not identify the same objects or values after the first request. No last-write-wins, shared identity, atomic pair, coherent snapshot or final-state theorem follows.

The first root skips its selected local work when the initially captured data pointer is NULL. It dereferences the virtual response to obtain that pointer without a local response NULL guard. Other reached paths and earlier opaque requests have their own branch conditions. A previous check or a declared interface does not validate later reloaded pointers, capacity, ownership, aliases or lifetimes. Providers can mutate source and destination storage, throw or fail to return; these selected calls are request policies, not proof of successful trimming or saved/current identity.

**Existing saved-fragment contract.** The already published Trim policy in11.222 binds feature+0x800 to the owned Trim data, whose packed+0x10 word is serialized as unsigned32. The exact CompleteOpts setter first writes the full32 input to current data+0x70 provider member+0x98 when that provider is nonnull. It then places the input low16 into data+0x10 high16 while preserving the locally observed low16. This is a saved fragment of a wider current provider-backed property, not an additional wire field or a new fully saved32 option. The unchanged historical current-query, archive, proxy, helper and path restrictions still apply. A mandatory authentication companion pins that existing published role/wire contract; no setter or canonical codec proof is rerun or newly decoded.

Thus these four concrete caller producers join the existing saved CompleteOpts fragment conditionally through the declared setter. They do not identify numeric option meanings, bit2 of the packed word, allowed file patterns, defaults, class population, provider success, geometry, final persistence or parser aliases. Raw keys, wire order, widths, presence, reader admission and stored-value preservation remain unchanged.

The exact three complete chained roots contain340+737+2757=3834 entry-CFG bytes, with zero excluded bytes and zero unresolved indirect jumps. The complete global all-shape gate classified1463 metadata range echoes and no native retained coverage. One authorized missing-only capture persisted the raw journal before CFG or semantic assertions. All four reference sites are independently authenticated as actual CALLs to the exact IAT; cached reference metadata alone was not treated as a CALL. No additional helper, constructor, provider, geometry/math, caller, vtable, registry, string or readonly contents are followed. There is no target execution, corpus/parser, accounting/coverage or authoritative-document change. Edition0.3220 is context only, not an immutable source pin. Evidence: C-3857.
