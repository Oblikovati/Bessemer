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

### 11.345 Display-weld suffix and unresolved symbol dependencies

*Status: Native-bound research layouts; explicit source stops; no added coverage · Evidence: C-387*

`moDisplayWeld_c` is defined in installed **slddetailsu.dll**, SHA-256 `8994ccdfc11441bbdb48a52569c7c97ad5c01a4ece2455ae938e01e659a054ca`. Its primary vtable **0x180085A00** selects Serialize **0x180047730..0x180047ADD**; RTTI proves `moDisplayItem_c` ancestry. The function imports the exact inherited display-item serializer. It does not inherit the display-dimension-specific text/unit/reference suffix.

After the existing ordinary modern display-object/item body, the derived weld layout is:

| Order | Native member | Ordinary wire field | Load gate |
|---:|---|---|---|
| 1 | Vector pointed to by +5A0 | Three f64 values, directly serialized | Version at least 0x20A |
| 2 | +5A8 | Raw f64 | Same gate |
| 3 | +5D8 | Raw i32 | Version at least 0x11D2 |
| 4 | +5B8 | Archive CString | Version at least 0x918 |
| 5 | +5C0 | Generic tagged object pointer (`su_CObject` load operator) | Same gate |

Every listed gate holds in the existing modern ordinary context. Legacy owner/view pointer reads below 0x216/0x183, and earlier runtime witness-direction repair, are outside that context. The native favorite-model checks and callback at **0x180047B00** receive runtime state rather than an archive argument; they do not justify inserting additional wire fields. Complete runtime repair remains unverified.

At edition 0.365, `displayweld.py` was a **research-only** composition of the existing display-item primitives and this suffix. Numeric values and the CString remain unknown for coverage purposes. A synthetic suffix with a three-byte ANSI string and a short mapped object reference occupies **42 bytes**, including **40 unknown bytes** and the two-byte pointer tag. This is boundary evidence, not an observed complete native weld object. Global/public supported registration was unchanged at that checkpoint; clause 11.347 adds the guarded public path.

The first dependency is `moCompDetailItem_c` in the C-20 model DLL: primary vtable **0x4CEF4C00**, serializer **0x4C956B10**. Its `moCompRef_c` prefix is followed, in the proven internal branch, by tagged `moDetailItem_c` +E8, tagged detail +F8 and raw i32 +100. The latter pair is present from version **0x4BE**. This matches the existing restricted `displaynotes.component` helper. The native external branch instead reads strings and depends on `moCompRef_c::iIsExternal`; it is not inferred merely from the serialized class name.

The predicate at **0x4B819670** returns zero when runtime component +58 is null. For a nonnull component it calls virtual slots **+168** and **+270**, considers runtime member +10 and, with a supplied owner, compares the filed-owner identity. The new source replays preserve the existing null-component restriction. `moUnitComponent_c` has not yet been proved internal for these particular references; its presence is neither proof of externality nor authorization to skip the predicate.

Two further primary bindings establish the next native symbol layouts:

| Class | Primary vtable | Serialize | Native parent |
|---|---|---|---|
| `moWeldSymbol_c` | 0x4D23FFA0 | 0x4C96D760 | `moDetailItem_c` |
| `moCosmeticWeldBeadSymbol_c` | 0x4D2404F8 | 0x4C9577E0 | `moWeldSymbol_c` |

The modern weld-symbol body calls the existing detail-item serializer, then reads **18 CStrings** at +88 through +110, raw f64 +130, a directly serialized vector pointed to by +138, u16 +128, four i32 values +118/+11C/+120/+124, **five CStrings** +148 through +168, u16 +140/+170 and two tagged `moSFSymbol_c` pointers +178/+180. It then invokes directly owned virtual serializers at **+188**, **+2E0** and **+438**, gated respectively by version greater than **0x7D2**, **0x1433** and **0x1438**. These gates all hold in the supported modern context. At the edition 0.365 checkpoint, the concrete types and full bodies of those three embedded objects still required constructor/dispatch evidence. Clause 11.346 identifies the owned text objects and implements their research composition; no collection grammar or outer object tag was inferred at this earlier checkpoint.

The cosmetic-weld serializer directly calls the weld-symbol serializer, then reads two i32 pairs at +590/+598, two f64 values at +5A0 and a final i32 pair at +5B0 when version is greater than **0x1207**. Its modern derived suffix is **40 bytes**. This native inventory does not establish a complete containing object until the inherited embedded bodies are resolved.

`research/display_weld.py` replays the three edition 0.364 weld first stops with the provisional display reader and the restricted null-component helper. `tutorial/api/insert_weld.sldprt` advances to **`moCosmeticWeldBeadSymbol_c` at 0x59B**. `tutorial/api/weldment_box3.sldprt` and `tutorial/weldments/weldment_box2.sldprt` stop at the unverified nonnull-component predicate for `moUnitComponent_c`. No complete display-weld object or EOF stream is established. Diagnostic cursors and partial roots earn no coverage credit.

`display_weld_evidence.py` verifies both installed module hashes, all four primary/ancestry bindings, **five complete native function bodies** (display weld, component detail, weld symbol, cosmetic symbol and externality predicate), inherited imports, direct delegation and all three source stops. `display-weld-native-evidence.json` binds those results to the research decoder and retained decompilation hashes. `display-weld-corpus.json` retains source SHA-256, completed preceding root spans/hashes and explicit dependency failures. The details module was imported only for read-only bounded decompilation; no SolidWorks process or source document was modified.

Validation: **two focused tests pass** for suffix widths/order, following sentinel, mapped-object identity, conservative unknown total, truncation, context rejection, all three source stops and absence of global registration. The preceding **1,126-test** regression result remains the supported production baseline; this edition adds no public reader. Specification lint reports zero findings (569 headings, 96 anchors). Annex G remains at edition 0.364: 870 completed resolved streams and measured parts **76.1% / 70.9%**. Complete documents and all installed project formats remain unfinished.

Evidence: C-387.

### 11.348 Current-part weld component predicate and end-cap research continuation

*Status: Native-bound research readers; two incomplete source streams; no added coverage · Evidence: C-390*

At the Edition 0.368 checkpoint, the public restriction in 11.347 remained in force. This clause records that research continuation for ordinary modern part archives; neither `weldinternal.component` nor the end-cap reader was installed publicly at that checkpoint. Clause 11.349 adds the guarded supported composition. A recovered field boundary alone does not make its enclosing stream complete.

`moCompDetailItem_c::Serialize` at **0x4C956B10** compares its external/internal predicate with and without the archive's current owner. Its internal branch follows the existing `moCompRef_c` prefix with tagged `moDetailItem_c` pointers at runtime +E8 and +F8 and a raw i32 at +100. The nonnull case requires owner/model identity evidence. The following installed native chain establishes the current-part case:

- `setFiledOwner` at **0x4B84DCE0** stores the owner's registered model index. The owner getter at **0x4B845220** resolves that index through the global model manager.
- Ordinary `moUnitComponent_c::Serialize` at **0x4C2392A0**, with the stored external-model byte zero and no external owner wrapper, calls its +320 model setter with the archive's current owner. The setter at **0x4B84DD00** constructs or updates a model handle.
- Model-handle construction/setter and the pointer-to-index lookup preserve the same registered model identity. Lookup **0x4BEB4C90** validates a cached index against the actual object pointer before returning it; otherwise it searches the registry for that pointer. The model getter at **0x4B845C10**, through **0x4BE7B090**, uses the same global manager and index lookup as the owner getter.
- Unit virtual +168 at **0x4B84B5C0** tests for a nonnull model. Virtual +270 at **0x4B84A130** returns internal when its registered owner lookup equals that model. This is the identity branch; the part's virtual fallback is not substituted for it. In this build the relevant part fallback slots return true and false respectively, so a part class-name test alone would not prove the branch.

The research guard requires a complete ordinary current-part framing context, the original parsed `moPart_c` root present by identity in the inherited object map, an exact `moUnitComponent_c` class, stored external-model byte zero, and `component.owner is current_root`. A separately allocated equal dictionary or same-named part is rejected. The context marker records framing provenance; it does not certify unknown field meanings or emulate native runtime registration. The unit component need not equal a particular canonical unit-component instance: both installed weldment sources contain another valid instance owned by the same current part. External models, foreign owners, unresolved owners, missing provenance and other component classes remain unsupported.

The end-cap surface-ID representation has primary vtable **0x4CF73E98**, shared serializer **0x4BBB79B0**, and concrete virtual +2A0 target **0x4BBBF3F0**. Its inherited ID body and two signed i32 suffix fields match `modelboundary.end_face`; the research replay binds that reader locally as `moEndCapChamferSurfIdRep_c`.

`moEndCap_c` is bound in the installed **sldMoPartu.dll**, primary vtable **0x1804DF888**, serializer **0x180389070**. The corresponding model-DLL inventory has no primary binding for this class. In the supported modern version scope, its wire order is:

| Order | Body |
|---|---|
| 1 | Existing `moBodyFeature_c` body |
| 2 | Tagged `suObList` at runtime +800 |
| 3 | Four raw i32 fields: +808, +80C, +828, +82C |
| 4 | Four tagged `moDisplayDistanceDim_c` references: +840, +838, +848, +850, in that order |
| 5 | Raw f64 +820; raw i32 +830 |
| 6 | Tagged `moDisplayArcLengthDim_c` at +858 |

The native +828/+82C and +850 branch begins at version **0x1BCF**; +848 begins at **0x962**; +830 begins at **0xF7F**; +858 begins at **0x1FAF**. All gates are satisfied by the research reader's ordinary modern scope above 0x27CB. Earlier migrations and the final runtime virtual call consume no additional modern input. All five raw i32 fields and the f64 remain semantically unknown: **28 bytes** are explicitly excluded from value credit. Tagged reference identities and order are retained, including nulls and shared objects.

`weld_component_probe.py` preserves the initial owner-identity observations. `weld_internal_component.py` advances both weldment sources through their detail cabinets and end-cap ID representations to explicit `moEndCap_c` stops. `weld_end_cap_replay.py` then consumes their end-cap feature bodies and stops at **`moGusset_c`**, at source offsets **0x25A0D** and **0x2AF02** respectively. The original `insert_weld.sldprt` still reaches exact EOF at 29,940 bytes. Each replay retains source hashes, completed root boundaries, unknown totals and explicit stopping errors; no tail is silently accepted.

Four focused tests verify root identity/provenance rejection, scoped context propagation, ordered shared end-cap dimension references, conservative unknown accounting, every truncated payload boundary and the exact source replay outcomes. `weld_internal_evidence.py` binds the installed native code, vtable slots, source replays and decoder hashes in `weld-internal-native-evidence.json`. At Edition 0.368, public audit totals remained **871 complete / 35 explicit stops**, and stored coverage remained the Edition 0.367 report. All **1,139 regression tests passed** (158.644 seconds); lint reported zero findings. The frozen `research/frozen-decoder-0.368/resolvedfeatures.py` preserves that packet's original decoder hash. Clause 11.349 resolves the gusset and following weld-bead dependencies.

Evidence: C-390.

### 11.349 Gusset, weld-bead feature and guarded weldment completion

*Status: Native-bound ordinary modern part graph; two additional exact-EOF streams; meanings partial · Evidence: C-391*

This clause uses the current-part component predicate and end-cap layouts established in 11.348. It applies only to ordinary modern non-DB external-XML part loading above version **0x27CB**, with helper +834 clear and the completed saved model/CMgr context required by `resolvedfeatures`. Other helper states, external owners and incomplete enclosing streams remain excluded.

The installed model DLL binds `moGusset_c`, primary vtable **0x4CF650F8**, to serializer **0x4BB26AE0**. Its inherited function **0x4BB25180** is the existing `moExtrusion_c` serializer: the complete body-feature body, tagged `moEndSpec_c` +810, f64 +830 and i32 +808. The native extrusion diagnostic name and imported base call confirm the layer; it is not replaced with an unqualified body-feature prefix. The gusset continuation is:

| Order | Body |
|---|---|
| 1 | Tagged `suObList` pointers +858, +860 |
| 2 | Raw i32 +868, +86C, +870; tagged distance dimension +8D8 |
| 3 | Raw f64 +878; raw i32 +880; raw u32 combining runtime +890 and +88C |
| 4 | Tagged distance dimensions +898, +8A0, +8A8; tagged angular dimension +8B0 |
| 5 | Raw u32 combining runtime +884 and +888; tagged distance dimension +8B8 |
| 6 | Raw i32 +8E0; tagged distance dimensions +8C0, +8C8; tagged angular dimension +8D0 |
| 7 | Raw i32 +8E4, +8F8, +8FC, +900; tagged `moRefWrapper_c` +908 |

The native packed words unpack bits 0 and 1 into separate runtime fields. The decoder preserves the entire original u32, including upper bits, and keeps it semantically unknown. The second packed word begins at **0x8FC**, +8B8 above **0x8FE**, order 6 above **0xF7D**, +8E4 above **0xF8D**, and the remaining order-7 fields above **0x1BBB**. These gates all resolve true in this clause's scope. The gusset suffix has **52 unknown raw bytes** independently of variable nested objects. Runtime feature-owner updates consume no input.

The installed part DLL binds `moWeldBeadFeat_c`, primary vtable **0x1804E2128**, to **0x1803961B0**. After its complete `moBodyFeature_c` body, the ordinary helper-+834-clear order is:

| Order | Body |
|---|---|
| 1 | Raw CString +858; tagged `moDisplayItem_c` +850 |
| 2 | Tagged `moDetailCabinet_c` +860 |
| 3 | Existing `mgXform_c` serialization +868; raw u32 combining runtime +840/+844 |
| 4 | Tagged `weldBeadSideData_c` pointers +828, +830; tagged `suObList` +838 |
| 5 | Five tagged distance dimensions +800, +808, +810, +818, +820 |
| 6 | Raw i32 +948, +94C, present above version **0x12B8** |

A nonzero helper +834 omits order 2 and obtains the cabinet from the runtime owner. That is a different archive context and is not selected by guessing whether the next bytes form a pointer. The ordinary context uses the serialized cabinet. The leading CString and packed/tail words remain unknown for semantic credit; existing transform field accounting is retained.

`weldBeadSideData_c`, primary vtable **0x1804E2098**, serializer **0x180396490**, contains three tagged `suObList` pointers at +30/+38/+40, raw i32 +50, then raw i32 +74 and f64 +78 above version **0xDEA**. There is no additional inherited serialized body. Its **16 raw bytes** remain unknown. List references retain their archive-map identity; repeated list pointers do not serialize duplicate bodies.

`weldsupport.install(..., weldment_dependencies=True)` adds these three tagged classes and `moEndCap_c`/its surface-ID class, and replaces only the local component-detail reader with the identity guard from 11.348. `resolvedfeatures.read_data` keeps its staged default **false** for the new option. Public `read_supported` and `documented_bytes` enable it with `weld_dependencies=True`; set **`weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False`** to replay the Edition 0.367 public scope. Earlier profile regrade commands explicitly pass false for the new option. Global registration is unchanged.

The retained research snapshots record successive explicit end-cap, gusset, bead-feature and side-data stops. `weld_bead_side_replay.py` finally reaches exact EOF for both weldment streams:

| Source | Decoded bytes | Complete roots | Unknown bytes | Known structure bytes |
|---|---:|---:|---:|---:|
| API `weldment_box3.sldprt` | 351,010 | 43 | 126,181 | 224,829 |
| Weldments `weldment_box2.sldprt` | 371,730 | 52 | 145,912 | 225,818 |

Their combined **450,647 known decoded bytes** are credited only after complete framing and exact EOF. `insert_weld.sldprt` preserves its preceding endpoint and unknown total. The full **906-configuration** audit `resolved-weldment-audit-0.369.json` reports **873 complete / 33 explicit stops / zero context stops**, with **26,376 complete roots**. All **871** preceding positives preserve endpoints, root counts and unknown totals. Complete streams occupy **76,902,890 decoded bytes**, including **27,121,671 unknown** and **49,781,219 known structure bytes**.

The Edition 0.367 audit/coverage reports and its frozen gap report are retained. Add `--weldment-dependencies` to the full Edition 0.367 audit and regrade commands to produce the Edition 0.369 reports. `resolved_coverage_evidence.py --weldment-dependencies` independently verifies all **1,166 source rows**, hashes, decoded sizes, stored spans and integer proportions. Added stored part credit is **58,287 bytes**; cumulative resolved-family known bytes become **14,067,475**, leaving **11,069,991 unknown** of the unchanged 25,137,466 stored bytes. Total part unknown bytes become **54,300,157** of 227,086,352. Rounded coverage remains **76.1% / 70.9%** for parts, **83.8% / 81.3%** for assemblies and **57.3% / 49.9%** for drawings. No whole-document or installed-project-format completion is claimed.

Focused tests cover packed-word preservation, mapped list/dimension order, every truncated gusset/side payload boundary, native root provenance, exact public EOF, disabled-profile zero credit and rejection of appended bytes. `weldment_evidence.py` binds the installed serializers and owner/model chain to these sources and verifies preservation of the prior positives; the C-390 packet reproduces byte-for-byte using its frozen decoder source.

Validation: all **1,142 regression tests pass** (157.904 seconds); specification lint reports zero findings (573 headings, 96 anchors).

Evidence: C-391.

