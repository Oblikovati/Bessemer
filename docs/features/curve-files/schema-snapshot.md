### 11.333 Curves from stored files and point lists

*Status: Native-bound modern curve-file body; three additional exact-EOF streams; semantics partial · Evidence: C-375*

`moCurveInFile_c` primary RTTI table **0x4D2E49F8**, locator **0x4D381558**, binds Serialize +0x28 to **0x4CB74D00**. Its first call is inherited `moReferenceCurve_c::Serialize` **0x4C284E20**. The existing reference-curve grammar comprises reference geometry, its framed kernel wire body, a raw wire node ID and direct attribute list. Kernel framing and unsupported contexts retain the existing strict checks; no geometric body is skipped. The derived input calls helper **0x4CB75450** on member +0x778, whose exact field order is:

| Order | Encoding | Memory destination |
| --- | --- | --- |
| 1 | CString | Stored string at object +0x780. |
| 2 | signed i32 count | Number of stored point records. |
| 3 | max(count,0) three-f64 points | Dynamic point array at object +0x790. |
| 4 | Tagged moPointRef_w pointer, only when version >0x42CD | Object +0x798. |

The helper offsets +0x08/+0x18/+0x20 are relative to its +0x778 subobject and correspond to the object offsets above. Native input iterates only while the signed count is positive. The decoder retains a negative count with no points and bounds positive counts by remaining bytes before iteration. The writer uses the runtime array count, writes each `mgPoint_c`, then applies the same optional-pointer gate. The older extra wire-restore branch below version 0x215 is outside the supported modern context (>0x27CB).

The stored string is retained verbatim and charged as unknown semantic data; no external filename is opened or resolved. Native point type establishes the three-f64 geometry record. The optional reference pointer is present at version 0x42CE, including when null, and absent at 0x42CD. Tests exercise both sides of that gate, two populated points, signed/negative count preservation, sentinel boundaries, truncation and count overflow. `curvefiles.read` composes the existing inherited reference curve and this suffix; `resolvedfeatures.read_data(..., curve_files=True)` installs it only in the separate archive's local table.

Three configurations contain one newly decoded curve each: Simulation Examples basketball_rim Config-0/Config-1 and Dynamics basketball_rim Config-1. Each stores **15 points**, path `C:\Temp\HoopModel\curve1.sldcrv`, null optional point reference and an unsuppressed inherited kernel wire. Their complete feature/root counts are **53/53/54**, and exact-EOF stream sizes are **239,361 / 240,649 / 240,906 bytes**. These are three stored instances, not three independent curve designs. `curve-files-corpus.json` retains all source hashes, points, root intervals/hashes and optional-reference observations.

The broad replay reaches **839 exact-EOF streams / 67 explicit stops / zero context stops**, with **25,822 completed roots** across 906 configurations. All **836** previous complete streams preserve exact endpoints, root counts and unknown-byte totals. Completed streams total **71,228,187 decoded bytes**, including **24,994,671 unknown bytes**. `resolved-curve-file-audit-0.353.json` is reproduced by `resolved_persisted_context_audit.py --mbsm --selection-items --top-surface-dependencies --curve-files`. These diagnostic completions remain outside stored-coverage registration; semantic and whole-document coverage remain incomplete.

`curve_files_evidence.py` verifies primary bindings, inherited/derived/helper native bytes, decompilation identities, exact source outcomes and all prior positive audit rows. `curve-files-native-evidence.json` retains the native packet and audit hash. Tests reproduce all three source EOFs and preserve scoped registration. Remaining resolved features and complete part/assembly/drawing/installed-project schemas remain in scope.

Validation: all **1,097 regression tests pass** (147.789 seconds); specification lint reports zero findings (557 headings, 96 anchors).

Evidence: C-375.
