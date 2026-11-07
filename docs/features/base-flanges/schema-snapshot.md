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


### 11.155 Ordinary modern sheet-metal base-flange feature

*Status: B — native-derived inheritance, selected-direction and sketch-handle framing; additional native modes/populated collections and raw meanings remain partial.*

moSMBaseFlange_c primary table `3DC14A10`, locator `3DC70C60`, selects serializer `3DB66B30` in sldsmmu.dll. Runtime descriptor `3DDAF380` selects factory `3DB561A0`, whose call at `3DB561BC` selects constructor `3D822650`. RTTI and constructor `3D822D50` establish inheritance through moSheetMetalBendGroup_c and moSheetMetalCommon_c to moBodyFeature_c. Allocation size 0xA20 is not wire length.

In ordinary non-DB external-XML loading with v > 0x27CB, begin with the complete body-feature base, then the common bend data and three raw words +840/+844/+848 from 11.154. The archive-helper +850 state remains explicit and has the same default-zero behavior. This feature uses the common bend layer, **not the derived moSheetMetal_c suffix**.

The bend-group continuation, serializer `3DB68EC0`, is direct suObArray +858 (ReadCount/tagged objects), followed by raw +878 u32. Native code filters null and suppressed entries from that array after reading it; the offline reader retains the serialized array unchanged. This runtime filtering neither consumes extra fields nor changes the subsequent word's position.

After the bend-group layer, the base-flange suffix is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | profile +888 | Tagged moProfileFeature_c pointer | Always | Existing profile/sketch grammar and shared identity | B |
| 2 | raw +898, +8C8 | Two i32 | Always | Preserve original signed values | B |
| 3 | mode +8F8 | i32 | Always in modern scope | Selects direction slots as specified below; preserve raw value | B |
| 4 | reference byte | u8 | Always in modern scope | Full byte retained; controls type 4/5 reference presence | B |
| 5 | raw +880 | i32 | Always in modern scope | Original gate v >= 0x690 | B |
| 6 | direction slot 0 | Conditional direct fields | Mode == 0 or mode == 1 | No enclosing object tag or byte count | B |
| 7 | direction slot 1 | Conditional direct fields | Mode == 0 or mode == 2 | Follows slot 0 when both are selected | B |
| 8 | handle arrays +958, +978, +9B8, +9D8 | Four counted direct collections | Always | Fixed i32 counts and one sketch pointer per nonempty collection | B |
| 9 | raw +8CC | i32 | v > 0x3B0C | Native stores a boolean interpretation; retain original i32 | B |
| 10 | from-end specification +A18 | Tagged moFromEndSpec_c pointer | v > 0x46B9 | Existing from-end grammar, not a direct body | B |

The exact load predicate for direction index i in {0,1} is mode == 0 or i == mode - 1. Other mode values select neither slot; the reader preserves them without inventing a third direction. Store/load decisions derive from this word, not an array count or helper +850 state. Each selected slot has the following order:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | type +8D0 + 4i | i32 | Selected slot | Preserve raw type | B |
| 2a | raw value +8B8 + 8i | f64 | Type 0 or 6 | No pointer tag in this branch | B |
| 2b | reference +8E8 + 8i | Tagged CObject pointer | Type 3 | Read a pointer even when reference byte is zero | B |
| 2c | reference +8D8 + 8i | Tagged moRefWrapper_c pointer | Type 4 or 5 and reference byte nonzero | Zero byte omits the entire pointer tag | B |
| 3 | dimension +A08 + 8i | Tagged moDisplayDim_c pointer | Selected slot | Always follows the chosen/no-payload type branch | B |
| 4 | raw extra +8A8 + 8i | f64 | Selected slot | Present independently of type | B |

For types outside the listed payload branches, proceed directly to the dimension. Runtime sketch-closure checks in those branches do not add fields. Native profile editability/closure updates and derived +8FC state are not serialized substitutes for the retained mode/type data. The historical pre-0x670 branch converts a single older end-condition representation; it remains outside the modern reader. All earlier dimension/value gates are satisfied in modern scope.

Each of the four handle collections is serialized by `3DB6F600`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | handle count | i32 | Always | Nonnegative fixed count, not ReadCount; SHALL fit remaining minimum pointer slots | B |
| 2 | sketch | Tagged sgSketch pointer | Count > 0 | Exactly once, before the first handle | B |
| 3 | handles | count tagged sgEntHandle pointers | Count > 0 | Preserve nulls, order and repeated references | B |

Count zero ends a collection after its four-byte count; there is no sketch tag in an empty collection. A direct collection adds no class/object-map entry of its own. New sketch/handle objects use normal MFC map insertion, and inherited/shared pointers retain identity. Native sets each nonnull handle's runtime sketch owner after loading; offline decoding retains the serialized sketch separately without mutating shared cached objects.

Raw call-site bytes at `3DB671B1` establish third arguments +958, +978, +9B8 and +9D8 for the four serializer calls. Ghidra omitted the last call's third argument; it is **+9D8**, not an unspecified continuation. The constructor also initializes pointer array +998, but this serializer does not append it. This distinction is verified from the lea r8 instructions, not inferred from member ordering.

Synthetic tests exercise both selected slots, single-slot modes, unrecognized mode/type values, types 3/4/5 with zero and noncanonical reference bytes, populated handle collections sharing sketches/handles, preserved inherited-array nulls, new tagged from-end specifications, count/truncation/context rejection and both late-field gate boundaries. A stream-hash-bound native regression verifies a complete base-flange containing-root end.

The frontier contains **18 fresh native base flanges**. Every one has mode 1, reference byte 1 and a single type-0 direction, with empty +858 and four handle collections. All include both version-gated late field positions. Other mode/type branches and populated collections have synthetic evidence only. Edition 0.166 records **20,783 complete roots**, with **492 list-complete configurations / 414 explicit stops** across all **906 complete modern model contexts**. Exact-EOF results remain **453 lists / 20,512,540 decoded bytes**, with **39 four-byte tails**. Leading stops include sweeps (17), face/point reference-plane data (16), blend cuts (15), embedded reference bodies/component axes (14 each) and flat patterns (12). Full part, assembly, drawing and installed-project bit coverage remains incomplete; unresolved raw meanings and post-list tails receive no coverage credit.

Evidence: C-183.


### 11.327 Saved configuration-manager context for template sheet metal

*Status: Verified opt-in context propagation; two template bodies complete · Evidence: C-369*

The completed CMgr reader now supplies persisted sheet-metal helper state to model and separate resolved-feature readers. Native model functions 0x4C249C50 and 0x4C250750 copy manager memory member +0x650 into archive-helper +0x850, as established in 11.254/11.261. The state is not a constant EOF word or a default inferred from a fresh helper. Native CMgr Serialize 0x4C22C140, both state-copy functions and their decompilation identities are retained and revalidated in `template-sheet-metal-persisted-context-native-evidence.json`.

`configurationmanager.sheet_metal_context(doc)` first requires supported manager framing, the postload graph guard and exact EOF. It then obtains the saved +0x650 value and its recorded source position, verifies the original signed i32 at that position, and accepts only helper states 0 or 1. Versions <=0x27CB or manager layouts without the verified state-position field reject. The result records the value, field offset, CMgr source SHA-256, version and stream name. This establishes the state used for parsing; its broader application semantics remain partial and no additional semantic byte credit is assigned.

`modelprefix.read(..., configuration_context=True)` derives this context from the same document before reading the model root. It installs +0x850 and its verification marker on that archive instance. This mode currently requires the supported part-manager path; assembly context is not inferred. `resolvedfeatures.read_data` copies the state, verification marker and source provenance into the separate stream archive while preserving the model's inherited object identities. It copies the provenance dictionary so mutations do not change the model archive's record. The default model-prefix mode remains unchanged for historical replays.

Both existing template probes store state **1**. The frame and API-gusset sources now complete their `moTemplateSheetMetal_c` objects using that saved value, reproducing the earlier explicit-mode experiment without assuming a state. They reach cursors **324,619** and **27,885**, respectively, and stop explicitly on fresh `moCompSheetmetalFeat_c` objects. Neither resolved list is complete, so this work adds no whole-stream or stored-byte credit. All 919 part CMgr framing completions and current document coverage totals remain unchanged.

`template_persisted_context.py` reproduces the two probes in `template-sheet-metal-persisted-context-corpus.json`, retaining configuration names, resolved-stream hashes, root intervals/hashes, completed template records and CMgr provenance. `template_persisted_context_evidence.py` checks the native state propagation and source-derived endpoints. Tests verify the exact saved i32/hash, both template bodies, unchanged unverified historical context and rejection of unsupported state values, missing positions and mismatched source bytes. Other resolved-feature dependencies, complete lists and semantic/document/project coverage remain open.

Validation: all **1,085 regression tests pass** (122.231 seconds); specification lint reports zero findings (551 headings, 96 anchors).

Evidence: C-369.

