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
