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

### 11.346 Direct weld text bodies, cosmetic attachments and surface-finish symbols

*Status: Constructor-bound research readers; one completed containing cabinet root; stream coverage unchanged · Evidence: C-388*

The native weld-symbol constructor **0x4C827600..0x4C82783B** resolves the three embedded serializers left open in 11.345. It directly constructs **`moDimText_c` at +188**, then invokes the vector-constructor iterator at +2E0 with **stride 0x158**, **count 2** and callback **0x4B29CDA0**. The callback is the exported default-constructor closure of `moDimText_c` and calls its typed constructor **0x4C85B610**. Thus the two array members are at +2E0 and +438. These are three directly owned text objects, not three tagged object pointers or an archive object-array body.

`moDimText_c` primary vtable **0x4D2481A8** selects the existing serializer **0x4C6F4A60**. Constructor assignments and primary RTTI bind the owned instances to the same existing `sketchprefix.dimension_text` grammar. No outer class header, pointer tag, archive map slot or extra serialized array count is added. Runtime initialization of text styles at virtual +A8 is not another wire field.

`weldsymbols.read` now composes the common detail-item body, the previously inventoried modern weld-symbol fields and the three direct text bodies. `weldsymbols.cosmetic` adds the native 40-byte derived suffix. These were research readers at edition 0.366; clause 11.347 adds guarded supported integration. All newly read raw numeric/string meanings remain conservatively uncredited. A synthetic empty modern common weld-symbol body occupies **503 bytes**, including **three 129-byte direct text bodies**; the cosmetic extension takes it to **543 bytes**. These figures describe that fixture, not a fixed extent for populated text or nested references.

The next two native dependencies are:

| Class | Primary vtable | Serialize | Inherited body and derived fields |
|---|---|---|---|
| `moCosmeticWeldBeadAttachPoint_c` | 0x4D033528 | 0x4BE1BDE0 | Existing `moAttachPoint_c`; tagged `moCompFeature_c` +108, raw f64 +110 |
| `moSFSymbol_c` | 0x4D23F040 | 0x4C96AE70 | Existing `moDetailItem_c`; modern surface-finish fields and direct dimension text |

The cosmetic attachment has no derived version gate. `weldsymbols.attachment` retains the existing attachment reader's ordinary part/assembly model-root restriction; it does not enable drawing-specific attachment contexts. A synthetic short-reference case consumes **113 bytes**, including its inherited 103-byte attachment, two-byte feature pointer and eight-byte raw value.

The modern surface-finish symbol body follows the detail-item body with raw i32 +8C/+90; **ten CStrings** +98 through +E0; raw i32 +104; one direct **`moDimText_c` +108**; and raw i32 +100. The constructor candidate **0x4C81DE20** directly calls the same typed text constructor at +108, proving the embedded type and absence of an outer tag. Its later suffix is:

| Version condition | Additional fields |
|---|---|
| Greater than 0x42DE | Raw i32 +F0, raw f64 +F8 |
| Greater than 0x46BF | CStrings +260/+268, raw i32 +270/+274 |

Earlier gates for the tenth-string group, +104, direct text and +100 all resolve in the supported modern context. The native loader's `<PrefixStr =...>` extraction works on the already serialized text body; it does not introduce a separate string field. The reader preserves stored text rather than emulating runtime normalization. The empty synthetic surface-finish body is 190 bytes before the two later suffixes, 202 after the first, and 212 after both. Focused tests exercise each boundary comparison without widening legacy support.

`research/weld_symbol_owned.py` extends the preceding research replay locally with the weld, cosmetic symbol, attachment and surface-finish readers. In `samples/tutorial/api/insert_weld.sldprt`, the **`moDetailCabinet_c` root at bytes 1,014–4,651** now completes. Its five newly decoded objects are one display weld, one cosmetic weld symbol, one cosmetic attachment and two surface-finish symbols. All three direct weld text strings and both surface-finish direct text strings are empty in this source. This proves containing-root framing and the observed empty-text branch; it does not establish every populated-text behavior.

The source advances to **seven completed roots** and then stops explicitly at **`moCosmeticWeldCutList_c` at 0x12EF** (diagnostic cursor 4,876). `weldment_box3.sldprt` and `weldment_box2.sldprt` still stop at the unverified nonnull unit-component externality predicate. All three streams remain explicit failures. At that checkpoint the public supported reader rejected the display-weld class and granted **zero** credit to the source stream (`weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` reproduces it); completing one nested object or containing root does not earn prefix-only coverage.

The edition 0.365 replay and source-stop packet remain unchanged, including their retained reader hashes. `weld_symbol_owned_evidence.py` verifies the installed module identity, three additional primary/ancestry bindings, **seven complete native functions**, exact constructor/array callback bytes, the default-closure export, constructor type reuse and the completed containing root. `weld-symbol-owned-native-evidence.json` binds those observations to `weldsymbols.py`, retained decompilations, the preceding native packet and `weld-symbol-owned-corpus.json`. The two unit-component virtual targets discovered for the pending predicate are **0x4B84B5C0** (+168) and **0x4B84A130** (+270); their full branch analysis and owner-identity check remain required.

Validation: **three focused tests pass** for direct text boundaries and map-slot absence, cosmetic suffix widths and unknown accounting, surface-finish version comparisons, attachment identity/extent/truncation, all source stops, containing-root objects and continued zero public coverage. The preceding two display-weld research tests are preserved. No production decoder registration changed. Annex G remains at edition 0.364, and full document/all installed-project coverage remains unfinished. Specification lint reports zero findings (570 headings, 96 anchors).

Evidence: C-388.

### 11.347 Cosmetic weld tree, bead data and guarded integration

*Status: Native-bound ordinary modern dependency graph; one additional EOF stream; meanings partial · Evidence: C-389*

The following primary bindings in the installed C-20 model DLL complete the native wire inventory needed by the first weld sample. All readers retain the existing modern ordinary context restriction. Native offsets are member addresses, not wire order.

| Class | Primary vtable | Serialize | Parent/body |
|---|---|---|---|
| `moCosmeticWeldCutList_c` | 0x4CE25158 | 0x4BB437B0 | `moReferenceCurve_c`, then five raw i32 values |
| `moCosmeticWeldSubFolder_c` | 0x4D031850 | 0x4BB43AC0 | `moFeature_c`, tagged custom-property list and raw word |
| `moCosmeticWeldBead_c` | 0x4D02F738 | 0x4BB43610 | `moFeature_c`, tagged bead data/display item and two raw words |
| `moCosmeticWeldBeadData_c` | 0x4D02F6A8 | 0x4BB43050 | Ordinary object prefix, direct arrays/endpoints, tagged parameters/symbol, kernel body and suffix |
| `moCWeldBeadFromToParams_c` | 0x4D02F588 | 0x4BB42D50 | Raw i32 +08, f64 +10/+18: **20 bytes** |
| `moIntermittentCWeldBeadParams_c` | 0x4D02F618 | 0x4BB43B90 | Raw i32 +08/+0C, f64 +10/+18: **24 bytes** |
| `moCosmeticWeldEndPtRef_c` | 0x4CF5BCB0 | 0x4BB439A0 | Raw f64 +08/+10, i32 +18/+1C, f64 +20: **32 modern bytes** |
| `moWeldWireEdgeToFaceMap_c` | 0x4CF5BC20 | 0x4BB44FF0 | Raw u32 +08 and two signed-count i32 lists |

The cut-list parent is **`moReferenceCurve_c`**, implemented by the existing `helixes.reference_curve` body; it is distinct from `moRefCurve_c` in 11.338. The derived word order is **+888, +780, +778, +784, +788**, not member-address order. Their native load gates are respectively version greater than **0x11A8**, **0x11DF**, **0x1417**, **0x1BEE**, and version at least **0x1BFA**. All hold in the supported modern context. The subfolder reads tagged `moCustomPropertyList` +350 and raw i32 +358, the latter from version greater than 0x11DF. The bead reads tagged `moCosmeticWeldBeadData_c` +6B8, tagged `moDisplayItem_c` +6C0 (greater than 0x11D1), raw i32 +6C8 (greater than 0x11DF) and raw i32 +6CC (at least 0x1201).

The bead-data default constructor **0x4BE1A200** proves its directly owned types. The arrays use the existing `suObArray` helper **0x4B3B9830**; endpoint members are constructed by **0x4BE1A800**, whose assigned vtable **0x4CF5BCB0** identifies `moCosmeticWeldEndPtRef_c`. In particular, +E8/+110 are two endpoint objects with stride **0x28**, not object arrays. The modern serialized sequence is:

| Order | Body |
|---:|---|
| 1 | Direct object arrays +48 and +68 |
| 2 | Direct endpoint-reference bodies +E8 and +110, without outer tags |
| 3 | Raw i32 +1B8; tagged from/to parameters +1C0, intermittent parameters +1C8 and cosmetic symbol +1D0 |
| 4 | Raw f64 +1E0; existing strict kernel archive body +1E8, **without an added attribute list** |
| 5 | Direct object array +270; raw f64 +290 |
| 6 | Direct object arrays +08 and +28 |
| 7 | Direct object array +138 |
| 8 | Raw i32 +1BC |
| 9 | Direct object array +198 |
| 10 | Direct object arrays +158 and +178 |

The native gates for the modern suffixes are greater than 0x11A8 for order 5, greater than 0x11C4 for order 6, greater than 0x1205 for order 7, greater than 0x1BED for order 8, greater than 0x1C03 for order 9 and at least 0x1C13 for order 10. The cosmetic-symbol pointer branch replaces the older weld-symbol pointer from version **0x11E0**. The endpoint's final f64 appears above **0x11FA**. All these comparisons resolve in the supported modern scope. Earlier temporary-array conversions and runtime constructor fields are not extra modern wire bodies; unlisted constructed members are not serialized merely because they exist in memory.

Each direct array uses the existing packed count and tagged entries, without an outer object header or map slot. An all-empty synthetic bead-data record with null parameter/symbol pointers and a suppressed kernel body occupies **113 bytes**, of which **88** are conservatively unknown raw values. The fixed parameter records and endpoint values remain fully unknown for semantic credit. The from/to runtime copy into +80 consumes no additional input.

The wire-edge/face map reads one u32, then two **signed i32** counts, each followed by `max(count, 0)` raw i32 values. Negative stored counts consume no entries, matching the native loop rather than inventing padding or unsigned allocations. Positive counts are bounded against remaining bytes. Raw list values and the leading word remain uncredited; list count framing retains its structural meaning. Native dynamic-array allocation and index bookkeeping add no wire fields.

`weldsupport.install` installs **13** class readers locally: the six display/component/symbol/attachment readers established in 11.345–11.346 and the seven tagged classes added here. Endpoint references are consumed directly within bead data. `resolvedfeatures.read_data(..., weld_dependencies=True)` selects the installer; the staged default remains false. Public `read_supported` and `documented_bytes` enable it with complete saved CMgr/model context, exact stream names, declared root count, eligible MBSM, exact EOF and bounded unknown totals. `weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` replays the preceding supported scope. Global registration remains unchanged.

The nonnull unit-component predicate remains unsupported in `displaynotes.component`. The new installer retains that exact restriction: neither a class-name test nor a guessed owner relationship enables the native external/internal branch. Consequently `weldment_box3.sldprt` and `weldment_box2.sldprt` still fail explicitly and earn zero credit.

`insert_weld.sldprt` now reaches exact EOF at **29,940 bytes / 24 roots**, with **11,861 unknown** and **18,079 known structure bytes**. Its completed graph includes **11** newly bound objects: the five cabinet objects from 11.346, one cut-list, one subfolder, one bead, one bead-data object and two wire maps. The bead has non-suppressed kernel data. Its nine direct-array counts, in the wire order +48/+68/+270/+08/+28/+138/+198/+158/+178, are **1/2/2/1/2/0/0/0/0**. Both typed parameter pointers and its symbol pointer are null in this source; positive fixed parameter bodies are native-bound and tested synthetically. Each observed wire map has one entry in each list. No enum or geometric meaning is inferred from these counts.

The full **906-configuration** `resolved-weld-audit-0.367.json` reaches **871 exact-EOF streams / 35 explicit stops / zero context stops**, with **26,291 completed roots**. All **870** preceding positives preserve endpoints, root counts and unknown totals. Completed streams contain **76,180,150 decoded bytes**, comprising **26,849,578 unknown** and **49,330,572 known structure bytes**. Exact source EOF and a supported dependency graph are required before the additional stream earns credit.

`weld_cut_list_evidence.py` verifies eight primary/ancestry bindings, **13 complete native functions**, owned-array/endpoint constructor identity, all three targeted source outcomes and preservation of every preceding positive. `weld-cut-list-native-evidence.json` binds the current decoder/installer, retained decompilations, prior native packets and `weld-cut-list-corpus.json`. The two preceding research packets reproduce **byte-for-byte unchanged**: their verifier scripts use `research/frozen-decoder-0.366/` copies of the two older decoder sources under their original SHA-256 values, while retaining the same explicit source stops. This preserves the earlier evidence when public integration changes module descriptions.

`resolved_coverage.py --surface-dependencies --remaining-planes --mid-surfaces --reference-curves --reference-surfaces --geometric-dependencies --backup-dependencies --sketch-dimension-dependencies --isocurve-dependencies --weld-dependencies` writes `resolved-feature-coverage-0.367.json`. `resolved_coverage_evidence.py --weld-dependencies` independently rechecks all **1,166 source rows**, hashes, decoded sizes, stored spans, integer proportions and audit outcomes in `weld-feature-coverage-native-source-evidence.json`. Added stored credit is **4,936 part bytes**, taking cumulative resolved-family known bytes to **14,009,188** and leaving **11,128,278 unknown** of the unchanged 25,137,466 stored bytes. Total part unknown bytes become **54,358,444** of 227,086,352. Rounded parts remain **76.1% / 70.9%**, assemblies **83.8% / 81.3%**, drawings **57.3% / 49.9%**. The frozen edition 0.364 gap report and historical verifier still pass. No whole document or installed project-format completion is claimed.

Validation: four focused tests pass for the direct endpoint/array boundary, suppressed-body framing, mapped parameter identity, fixed record widths, signed list counts and bounds, unknown accounting, local registration/staged defaults, source EOF, remaining explicit stops and zero credit after appending bytes. All **1,135 regression tests pass** (159.616 seconds); specification lint reports zero findings (571 headings, 96 anchors).

Evidence: C-389.

