# Folder and face-color schema snapshot

### 11.132 Late browser folders and cut-list sort options

*Status: B — primary-table-proven modern layouts; raw flags, nested classes and complete stream semantics remain partial.*

The primary MSVC RTTI tables in the installed model module establish these serializer bindings. Secondary tables at object offset +0x88 are not used to select the primary archive body. The primary table's locator has object offset zero, and its type descriptor carries the exact runtime class name.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | moInkMarkupFolder_c body | Common moFolder body | Modern ordinary scope | Primary table 4D073CA0 +0x28 selects 4BEC2AF0; no suffix | B |
| 2 | moSolidBodyFolder_c body | Surface-body-folder layout | Modern ordinary scope | Primary table 4D052E58 +0x28 selects 4C224970, identical to 11.131's serializer | B |
| 3 | moEqnFolder_c body | Common moFolder body | Modern ordinary scope | Primary table 4D0400A8 +0x28 selects 4BB24BB0; following migration helper adds no bytes in scope | B |
| 4 | moCutListSortOptions_c `raw_08` | i32 | Always in scoped body | Preserve native member +0x08 as raw; no semantic credit | B |
| 5 | moCutListSortOptions_c `items` | Direct suObArray | After field 4 | ReadCount then tagged object pointers; no i32 pointer-presence word | B |
| 6 | moMaterialFolder_c base | Common moFolder body | Modern ordinary scope | Primary table 4D04D5D0 +0x28 selects 4BEC2CA0 | B |
| 7 | moMaterialFolder_c `raw_370` | CString | v > 0x109E | Native member +0x370; preserve stored text, no semantic credit | B |
| 8 | moMaterialFolder_c `raw_378_37c` | u32 | v >= 0x1423 | Combined native state word; retain all bits, no semantic credit | B |

The scope is v > 0x27BC in the ordinary external-XML model context. Runtime class identities remain distinct even where serializers are shared. Equation-folder serializer 4BB24BB0 calls the common folder body, then helper 4BB3ED10. The helper reads migration data only for load versions **0x393 through 0x3AE inclusive**, outside this scope. Readers MUST NOT insert those legacy arrays or the migration keeper into the modern body.

Cut-list sort options use primary table 4D04F308 and serializer 4C230C30. The constructor identifies native +0x10 as an embedded suObArray; the serializer reads its raw i32 before directly invoking that array's serializer. The array does not serialize its address or add a wrapper tag. Counts use the u16 / FFFF-plus-u32 ReadCount encoding, and MUST fit the available two-byte pointer-record minimum. Inherited references, duplicate entries, nulls and stored ordering are preserved. Populated option-element classes remain subject to their own reader support.

Material-folder native loading interprets a negative signed state word as two low-bit states: +0x378 gets bit 0 and +0x37C gets bit 1. For a nonnegative word, +0x378 becomes word != 0 and +0x37C becomes zero. The native writer emits bit 31 plus those two boolean states. These transformations do not replace the stored word in this schema: readers retain the original u32, including noncanonical combinations and unused bits. The runtime meanings of the two states and the CString remain unresolved.

The stream registry now identifies ResolvedFeatures by its cached-context feature-list grammar, with explicit prefix-only clause references. This classification does not establish full payload coverage.

At the edition 0.143 checkpoint, the full-list frontier framed **10,587 complete roots** in 892 separate lists, while 14 cases stop constructing their required model context. Completed root occurrences include **774 ink-markup folders**, **768 solid-body folders**, **766 equation folders** and **766 material folders**; duplicates or references are counted as occurrences. Complete cabinet roots increase to **775**, totaling **513,136 bytes**. No list reaches its declared end. The leading next dependency is moRefPlane_c (767 cases), followed by detail folders (54), sensors (52) and cut-list folders (7). The edition 0.142 report remains retained separately. Counts describe the first stop per case, not an exhaustive list of missing classes.

At that checkpoint, a separate diagnostic ran the existing 11.100 scoped reference-plane readers on these archives without changing the production reader. It reaches moDefaultRefPlnData_c in 766 cases and moRefDimHandle_c in six. That trial accepted no additional complete root and added no coverage credit. Its baseline is retained in resolved-feature-list-reference-plane-trial-0.143.json; 11.133 supplies the subsequent native evidence. Complete resolved-feature lists, post-list stages and full-document bit coverage remain open.

Evidence: C-160.

### 11.265 Face-color configuration records

*Status: Partial · Complete ordinary record body; 142 native bodies across 23 parts · Evidence: C-307*

`moFaceColorConfig_c` has primary vtable **0x4D02A840**, locator **0x4D3414F8**, and serializer **0x4C230E80** in hash-bound `sldmodu.dll`. Its RTTI hierarchy includes `su_CObject`; that inherited ordinary Serialize adds no wire fields. The record's body is:

| Order | Runtime member | Wire encoding | Presence |
| --- | --- | --- | --- |
| 1 | +0x10 | tagged `moCompFace_c` pointer | always |
| 2 | +0x18 | tagged `moVisualProperties_c` pointer | always |
| 3 | +0x08 | signed i32 | input version > 0x7EC; current output always writes it |

The enabled decoder requires the existing modern ordinary archive context, beyond that historical gate. Minimum body length is **eight bytes** with null or already-mapped pointer values. Fresh referenced objects include their ordinary tagged object/class headers and complete nested bodies; back-references reuse existing slots. The record has no direct geometry body or direct visual-property restore before its pointers. Its +0x18 value therefore uses tagged `moVisualProperties_c` framing, unlike the direct visual values in other configuration maps. The i32's meaning remains raw; all **142 native values are zero**, which does not establish a boolean or enum domain.

**Condition scope and dependent bodies.** Native input/output adds archive condition **0x200** around the pointer pair and i32, then removes it. `moCompFace_c` binds its Serialize slot to the shared **`moCompBEntRef_c` serializer at 0x4BBB7E80**; its base `moCompRef_c` serializer is **0x4BBBA690**. Their complete current native ranges and prior decompilations are retained. The boundary serializer's **0x2006607** permission mask includes 0x200, allowing this pointer body in the independent configuration archive context. The current condition-200 path uses the established modern component-reference, alias, boundary-reference and external-reference fields of 11.17–11.18; no extra condition-200-only wire field is introduced by these functions.

The visual-property class is separately bound in hash-verified **sldmfcu.dll**: primary vtable **0x3CF40A48**, locator **0x3CF87B40**, Serialize **0x3CBCE370**. Its versioned ordinary body composes with the existing visual/material-property readers. The new face-color reader remains local to the CMgr archive. Boundary-reference reader registration is initialized before the archive copies its reader map, so a fresh first document has the same dependency support as later documents.

**Native cases.** The edition 0.284 audit contained **23 part occurrences** stopped at `moFaceColorConfig_c`. Instrumented replay retains **142 complete record bodies totaling 157,371 decoded bytes**, with exact source hashes, interval bytes, record object slots, face/visual reference slots, color values and raw i32. All references are nonnull native `moCompFace_c` / `moVisualProperties_c` values. Per-file retained intervals are disjoint, and the first body begins exactly at the prior unsupported-class cursor. These body totals include nested referenced-object bytes and are not added again to complete-stream coverage.

All 23 cases advance past at least one complete face-color record. **Eight** now complete their guarded singleton CMgr roots; **eight** stop at nonempty condition-200 part body states, **five** at unverified postload graph variants, **one** at a legacy part-config-object layout and **one** at a positive direct visual-map body. The last fifteen cases receive no partial CMgr coverage credit.

The expanded full installed-part audit contains **501 complete supported managers / 418 explicit stops** among the unchanged **919 occurrences**. Every previous 493 positive summary remains unchanged. Complete manager streams total **1,555,091 decoded bytes**: **773,226 known structure bytes** and **781,865 raw unknown bytes**. Newly complete streams add **51,669 decoded bytes**, comprising **27,258 known structure bytes** and **24,411 raw unknown bytes**. Raw selector values and unknown nested reference fields remain excluded from credit.

**Coverage checkpoint.** Regrading the full container corpus adds **7,579 documented stored part bytes** beyond edition 0.284. Cumulative CMgr stored credit is **297,181 bytes**; part CMgr unknown bytes are **1,568,931** out of the unchanged **1,866,112-byte** stored denominator. All part unknown file bytes are **68,947,548**. Rounded coverage remains **69.6% of part file bytes / 63.1% excluding container and previews**. Assembly and drawing measurements are unchanged. The previous gap report and versioned coverage snapshots are retained.

Three focused tests replay every targeted source and its 142 body/slot snapshots, verify the eight completions and later explicit stops, exercise null pointers and existing face/visual references, preserve object identity and signed extremes, and reject truncated or context-free input. At the edition 0.285 checkpoint, the corpus-wide coverage test required **501 positive results and 773,226 known decoded bytes**. No whole document is completed by this extension. Selector meaning, condition-dependent body-state/scene records, remaining manager paths, other document streams and all-installed-project semantic coverage remain open.

Validation: all **955 regression tests pass** (117.734 seconds); specification lint reports zero findings (489 headings, 96 anchors).

Evidence: C-307.

