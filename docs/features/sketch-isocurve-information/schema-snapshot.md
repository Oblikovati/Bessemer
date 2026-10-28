### 11.344 Sketch isocurve information and the modern fixed body

*Status: Native-bound ordinary modern framing; one additional EOF stream; value meanings partial · Evidence: C-386*

`moSkIsoCurveInfo_c` primary vtable **0x4CF7ABA8** selects serializer **0x4C58D560..0x4C58D89C** in the installed C-20 model DLL. RTTI proves `su_CObject` ancestry. Following the zero-byte ordinary object prefix, the supported modern body is exactly **20 bytes**:

| Wire offset | Type | Native member | Native store label | Meaning status |
|---:|---|---|---|---|
| 0x00 | f64 | +08 | `Ratio` | Raw value; parameter interpretation uncredited |
| 0x08 | u16 | +10 | `ParamDir` | Widened runtime word; enum mapping unknown |
| 0x0A | u16 | +18 | `RefType` | Widened runtime word; enum mapping unknown |
| 0x0C | u16 | +1C | `IsoInd` | Widened runtime word; index interpretation unknown |
| 0x0E | i32 | +20 | `IsoDimId` | Stored ID; reference semantics unverified |
| 0x12 | u16 | +14 | `SkipHoles` | Widened runtime word; value semantics unverified |

The wire uses archive little-endian primitives, including the unaligned i32 at wire offset 0x0E. Native member +14 is serialized last; member layout must not be mistaken for wire order. The index and dimension-ID pair is present from version **0x66A**, and the final hole word from **0x675**. All gates hold in the existing supported ordinary context, version greater than 0x27CB. Historical omitted fields are initialized at runtime; those initialization assignments consume no input and do not widen this reader's version scope.

The six labels are native writer DB-key metadata, verified at their referenced string addresses. They are not six CString values in an ordinary archive. The ordinary load reads the numeric fields directly. `sketchisocurve.read` retains the native labels in descriptive field names but conservatively counts all **20 payload bytes as unknown** until value semantics, enums and references are established. The typed scalar decoding and names alone do not earn value credit. DB-key modes and runtime application behavior remain outside the supported path.

`resolvedfeatures.read_data(..., isocurve_dependencies=True)` installs the class locally for the existing external-sketch-entity `isocurve` pointer. The staged default remains false; the public supported and coverage readers enable it with complete saved context, declared root counts, eligible MBSM, exact EOF and bounded unknown accounting. Set `isocurve_dependencies=False, weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` to reproduce edition 0.363; historical regrade profiles explicitly pass false. Global registrations and inherited caller reader maps remain unchanged.

The installed `samples/learn/Garden Trowel/Garden Trowel_Garden_Trowel_Spade.SLDPRT` Config-0 resolved stream now reaches exact EOF at **514,962 bytes / 69 roots**, with **178,572 unknown** and **336,390 known structure bytes**. Its **14** isocurve information objects have ratios at the seven sampled positions from 0 through 1, including sixths; stored parameter-direction words take values 0 and 1. Reference-type and index words are 0, dimension IDs span 1–14, and the final hole word is 1. These are observations from this source, not complete enum definitions or general domain restrictions. Every one of the 14 bodies retains 20 unknown bytes.

The full **906-configuration** `resolved-isocurve-audit-0.364.json` reaches **870 exact-EOF streams / 36 explicit stops / zero context stops**, with **26,272 completed roots**. All **869** preceding positives preserve endpoints, root counts and unknown totals. Complete streams contain **76,150,210 decoded bytes**, comprising **26,837,717 unknown** and **49,312,493 known structure bytes**. The added stream contributes 336,390 known decoded bytes; no failed stream gains prefix-only credit.

`isocurve_info_evidence.py` verifies the installed module hash, primary/ancestry binding, complete native ranges/instructions, both version comparisons, all six referenced field labels, fourteen source bodies and preservation of every preceding positive. `isocurve-info-native-evidence.json` binds those observations to the production decoder and retained decompilation hashes. `isocurve-info-corpus.json` retains source SHA-256, completed root byte spans/hashes and all observed field values.

`resolved_coverage.py --surface-dependencies --remaining-planes --mid-surfaces --reference-curves --reference-surfaces --geometric-dependencies --backup-dependencies --sketch-dimension-dependencies --isocurve-dependencies` writes `resolved-feature-coverage-0.364.json`. `resolved_coverage_evidence.py --isocurve-dependencies` independently verifies all **1,166 source rows**, hashes, decoded sizes, stored intervals, integer proportions and audit decisions in `isocurve-feature-coverage-native-source-evidence.json`. Added stored credit is **88,292 part bytes**, taking cumulative resolved-family known bytes to **14,004,252** and leaving **11,133,214 unknown** of the unchanged 25,137,466 stored family bytes. Total part unknown bytes become **54,363,380** of 227,086,352; part coverage is **76.1% / 70.9%**. Assemblies remain **83.8% / 81.3%**, drawings **57.3% / 49.9%**. The frozen edition 0.363 gap report and historical verifier still pass. Full document and all installed project-format schemas remain unfinished.

Validation: three focused tests pass for exact field widths/order, unsigned extremes, signed ID and NaN payload preservation, truncation at field boundaries, context rejection, sentinels, local/staged registration, source EOF and zero credit after appending bytes to this newly supported stream. All **1,126 regression tests pass** (155.023 seconds); specification lint reports zero findings (568 headings, 96 anchors).

Evidence: C-386.

