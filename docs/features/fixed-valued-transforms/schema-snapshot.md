### 11.353 Fixed sketch valued-dimension archive transforms

*Status: Verified framing in the guarded modern part context; transform semantics partial · Evidence: C-395*

The shared valued-dimension base `0x4C5A9150` in the hash-bound installed `sldmodu.dll` calls `sgDim::Serialize`, then reads a u16 plane-transform presence word. In the supported modern range a nonzero word selects the **fixed archive operator** for `mgXform_c`, followed immediately by the derived dimension suffix. The G1 serializer `0x4C5A92E0` and G2 serializer `0x4C5A9750` both call this base. The earlier reader incorrectly selected compact `restore()` framing; this clause corrects that implementation and its synthetic fixture. The resulting false MFC-tag/count diagnostics in four Plastics examples are closed without changing tag validation.

| Wire order | Field | Encoding | Condition | Accounting |
|---|---|---|---|---|
| 1 | inherited sketch dimension | complete `sgDim` body (11.37) | always | Existing mixed-width status fields and inherited map |
| 2 | plane-transform presence | u16 | always in scope | Preserve the full word; any nonzero value means present |
| 3 | plane transform | 30 little-endian u32 words, exactly 120 bytes | presence nonzero | Preserve every word; all 120 bytes remain unknown semantics |
| 4 | derived dimension suffix | existing G1/G2 grammar | always | Continues with tagged entity handles in the same map |

The `sldmgu.dll` export at `0x3221D990` jumps to `0x32220FA0`. Its archive input loop reads **30 unsigned longs with stride 4**, establishing the fixed width. Compact `restore()` at `0x321031D0` is a separate path and SHALL NOT be substituted here. The accepted reader rejects perspective flag bits `0xF0` in word 28. Unsupported context, truncated fixed words and unsupported flags fail explicitly. No optional pointer tag or compact-transform mask precedes the 120-byte body. The `valued_transforms` scope is enabled by the public complete reader; the staged reader defaults it off and stops before consuming an unsupported present transform. Historical supported profiles do not re-enable the faulty compact interpretation.

| Installed source | Configuration | Decoded stream bytes | Unknown bytes | Present fixed transforms |
|---|---|---:|---:|---:|
| Plastics Examples/screw_cap.sldprt | 0 | 40,332 | 14,802 | 4 |
| Plastics Examples/spur_gear.sldprt | 0 | 21,915 | 8,087 | 6 |
| Plastics Examples/trim_piece.sldprt | 0 | 27,391 | 9,616 | 2 |
| Plastics Examples/wrench.sldprt | 3 | 46,035 | 15,105 | 1 |

All four streams complete through every declared root and any required MBSM tail to exact EOF. Their **13 populated transforms** retain all 30 words each. The added streams contain **135,673 decoded bytes**, including **47,610 unknown** and **88,063 known structure bytes**. `resolved-valued-transform-audit-0.373.json` records **890 complete / 16 explicit stops / zero context stops** among 906 modern part configurations, with **26,540 completed roots** across the audit. Every one of the 886 preceding complete streams preserves its endpoint, root count and unknown total. Complete streams now contain **79,702,177 decoded bytes**, including **28,273,476 unknown** and **51,428,701 known structure bytes**.

Add `--valued-transforms` to the full Edition 0.372 audit/regrade commands. `resolved_coverage_evidence.py --valued-transforms` independently checks all **1,166 source rows**, hashes, stored spans and integer proportions. Added stored part credit is **14,176 bytes**. Cumulative resolved-family known bytes become **14,431,954**, leaving **10,705,512 unknown** of 25,137,466 stored bytes. Total part unknown bytes become **53,935,678** of 227,086,352. Rounded coverage remains **76.2% / 71.1%** for parts, **83.8% / 81.3%** for assemblies and **57.3% / 49.9%** for drawings. Full document, remaining contexts and all installed project-format schemas remain open.

`valued_transform_evidence.py` rechecks native bindings, operator bytes/imports, all four public source EOFs, disabled-scope zero credit and preservation of the preceding 886 positives. The focused tests cover fixed-word boundaries, every truncation length, noncanonical presence words, perspective rejection, exact source frames and appended-byte rejection. Historical C-393/C-394 packets retain their original hashes using the exact preceding sources under `research/frozen-decoder-0.372/`. The original negative replay snapshots are retained; separate Edition 0.373 scoped snapshots change only the affected failure diagnostics and partial cursor/accounting fields. All **1,163 regression tests pass** (164.570 seconds); specification lint reports zero findings (577 headings, 96 anchors).

Evidence: C-395.

