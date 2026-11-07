### 11.358 Text decorations, pattern-count annotations and thread-instance IDs

*Status: Verified framing in ordinary modern part archives; raw field meanings remain partial · Evidence: C-400*

Ten additional annotation class names are installed locally in the guarded part resolved-feature reader. All primary bindings are in the hash-bound installed `sldmodu.dll`:

| Class or class pair | Native Serialize | Body |
|---|---|---|
| `moTextDataBorder_c` | 0x4BEE9DD0 | inherited text-data base, border suffix |
| `moTextDataSymbol_c` | 0x4BEEBAA0 | same text-data base, symbol suffix |
| `sgLinearPattCntDim`, `sgCircularPattCntDim` | 0x4C58FD10 | complete valued-dimension base, then i32 pattern position |
| `moDisplayLinearPattCntDim_c`, `moDisplayCircularPattCntDim_c` | 0x4C835790 | complete shared display-dimension body, no derived suffix |
| `moSkDimHandleLinearPattCnt_c`, `moSkDimHandleCircularPattCnt_c` | 0x4C840400 | established complete sketch-dimension-handle body |
| `moCThreadInstanceId_c` | 0x4BA5BFE0 | direct object array at +8 |
| `moCTInstId_c` | 0x4BA5B1C0 | three i32 words |

**Text-data base.** Both decoration serializers call `0x4BEEB530`, which calls `0x4B680950` and the direct triple helper `0x4BEE9930`. In modern scope the base is **36 bytes**: i32 +8/+C; three f32 values +20/+24/+28; f32 +2C; u16 +30; i32 +34; f32 +38; u16 +10. The byte-free su_CObject base adds nothing. Preserve the complete stored u16 words even where native code later assigns their truth values to in-memory bools. Do not append the character-specific final u16 +40 from `moTextDataChar_c`; that field belongs to the character subclass, not this inherited base.

**Border suffix.** After the complete base: six f32 +40/+44/+48/+4C/+50/+54; u16 +60/+62; CString +80; tagged `moGenTextDataList_c` pointer +88; f32 +58/+5C; f64 +68; and a direct three-f32 triple +70. The later fields have native gates +58/+5C from archive version `0x1059`, +68 from `0x1429`, and triple +70 from `0x1BF6`; all are present throughout accepted `v > 0x27CB` scope. The nested text list is a real tagged object and can recursively contain border, symbol, line-head or character records. The complete border has **92 fixed raw bytes**, plus the CString and tagged-list bytes. Its raw bytes and complete CString serialization remain unknown semantics; the tagged pointer framing retains existing structural credit.

**Symbol suffix.** After the same base: CString +40 and CString +48; f64 +50/+60/+70; direct three-f32 triple +78; then i32 +84 only for archive version `>0x2BCA`; and f64 +58/+68 only for version `>=0x2BE2`. Thus the fixed raw portion including the base is **72, 76 or 92 bytes**, according to those gates, plus the two CString serializations. The reader preserves strings, raw signed words and floating values; names or symbol meaning are not inferred from their positions. The accepted legacy edge of the modern range can omit the last fields, and focused tests exercise the exact transitions.

**Pattern-count dimensions and display handles.** Shared count serializer `0x4C58FD10` calls the complete valued-dimension base at `0x4C5A9150`, including any nonzero-presence fixed plane transform from 11.353, then reads one signed i32 at +130. The store-side key `PatternPos` corroborates that member; no extra ordinary-stream CString key is inserted into the read grammar. The signed word remains raw unknown semantics. These classes do not add G1/G2 entity-handle suffixes.

The two display-count classes select the shared display serializer `0x4C835790`. Their native virtual slots `+0xA78`, `+0x8D8` and `+0x8A8` all select `0x4B29C5A0`, whose exact body is `xor eax,eax; ret`. This proves the ordinary base display-reference branch, without the optional point-reference branch used by distance dimensions. The reader uses the complete existing shared display body; original MFC class names and map identities remain intact. The corresponding sketch-count handles both select the established serializer `0x4C840400`, with RTTI proving `moSkDimHandle_c` and component-reference ancestry. They retain its full external/internal sketch-owner and dimension-ID grammar, not an empty alias record. Circular class bindings are proved by native code; these added source cases populate the linear variants.

**Thread instances.** Runtime descriptor `0x4DAD05D0` identifies `moCThreadInstanceId_c`, schema `0x80000001`, allocation size 40 and factory `0x4BA59240`. The factory calls constructor `0x4B9D7490`, which constructs a direct `suObArray` at +8. Its serializer calls that embedded member directly after byte-free su_CObject serialization. The stored body is therefore **ReadCount followed by tagged array entries**, without an additional object-pointer tag or presence flag for the array itself. Runtime allocation size 40 is not an archive length. Populated `moCTInstId_c` entries select `0x4BA5B1C0`: byte-free base, i32 +8/+C/+10, exactly **12 raw unknown bytes**. No particular meaning is assigned to those three words.

| Installed source | Configuration | Exact EOF | Unknown bytes | Known structure bytes |
|---|---|---:|---:|---:|
| tutorial/api/nist_ftc_08_asme1_rc_sw1702_ra.SLDPRT | 0 | 759,011 | 333,888 | 425,123 |
| tutorial/assemblymates/table.sldprt | 0 | 227,240 | 94,526 | 132,714 |

The first source contains two borders, two symbols, one linear pattern-count dimension, one matching display-count dimension and one matching sketch-count handle. The second contains two thread-instance array objects and two populated thread-instance items. Both complete every declared root and required tail to exact EOF. Combined decoded bytes are **986,251**, including **428,414 unknown** and **557,837 known structure bytes**.

`resolved-annotation-audit-0.378.json` records **900 complete / 6 explicit stops / zero context stops** among 906 modern part configurations, with **26,706 completed roots** across the audit. Every preceding 898 complete stream preserves endpoint, root count, source hash and unknown total. Complete streams now contain **82,800,939 decoded bytes**, including **29,490,022 unknown** and **53,310,917 known structure bytes**.

Add `--annotation-dependencies` to the full Edition 0.377 audit/regrade commands. The public reader enables the scope; the staged reader defaults it off. `resolved_coverage_evidence.py --annotation-dependencies` independently checks all **1,166 source rows**, hashes, stored spans and integer proportions. Added stored part credit is **81,535 bytes**. Cumulative resolved-family known bytes become **14,877,227**, leaving **10,260,239 unknown** of 25,137,466 stored bytes. Total part unknown bytes become **53,490,405** of 227,086,352. Rounded coverage remains **76.4% / 71.3%** for parts, **83.8% / 81.3%** for assemblies and **57.3% / 49.9%** for drawings. Remaining feature classes, raw meanings, complete documents and all installed project-format schemas remain open.

`annotation_extras_evidence.py` rechecks ten primary bindings, thirteen native bodies, six display virtual predicates, the thread factory/constructor chain, both populated public EOFs and preservation of all 898 preceding positives. Seven focused tests cover inherited text widths, noncanonical words, strings and shared pointers, symbol gate boundaries, every text/count-dimension truncation, extended direct-array counts, raw thread-item widths, invalid bounds, context rejection, local installation, disabled-profile zero credit and appended-byte rejection. The preceding C-399 packet is reproduced with its original hash through exact sources frozen under `research/frozen-decoder-0.377/`. All **1,191 regression tests pass** (164.862 seconds); specification lint reports zero findings (582 headings, 96 anchors).

Evidence: C-400.
