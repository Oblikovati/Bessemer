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
