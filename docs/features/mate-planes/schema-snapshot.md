### 11.361 Concentric mates and owned plane-section records

*Status: Verified framing in guarded ordinary modern part archives; feature semantics and other contexts remain partial · Evidence: C-403*

This scope adds `moMateConcentric`, `moPlaneSection_c`, `moPlaneSectionEdge_c` and the ordinary zero-payload branch of `moPlaneSectionIdKeeper_c`. It requires the completed current-part model archive, saved configuration-manager context, ordinary external-XML mode and v > 0x27CB. Direct members retain the inherited object map; no nested object tag is invented. Unsupported classes, kernel forms and contexts still stop explicitly.

**Concentric mate.** Primary table `0x4CFAA130` selects serializer `0x4B686300`. Its call chain is `0x4B685CE0` → `0x4B687F90` → moMate serializer `0x4B686010` → `0x4B6888A0` → the established model-feature body. The native RTTI ancestry and +4B8/+4C0 hooks establish the ordinary inherited concept/feature stages. The +22B0 predicate returns false, excluding the width-mate conversion branch. Helper `0x4B685CE0` has no additional archive reads in this modern scope; its older pointer/list branches remain outside the reader.

After the complete model-feature body, the modern load order is:

| Order | Member or role | Serialized form | Gate |
|---|---|---|---|
| 1 | raw +388/+38C | two i32 | v > 0xF42; always in scope |
| 2 | raw +380 | i32 | v > 0x2BB4 |
| 3 | raw word vector +368 | bounded i32 count, then count i32 | v > 0x3AFC |
| 4 | raw +3AC/+3B0 | two u16 | always |
| 5 | optional vector +3B8 | u8 presence; when nonzero, three f64 | historical v > 0xDC; always in scope |
| 6 | reference map +3C0 | bounded i32 count; each entry i32 key and tagged moRefWrapper_c pointer | v ≥ 0x27B2; always in scope |
| 7 | saved array +428 | tagged suObArray pointer | historical v > 0x66C; always in scope |
| 8 | entities +408 | bounded i32 count and that many tagged moRefWrapper_c pointers | historical v > 0x696; always in scope |
| 9 | lists +440 | bounded i32 count and that many tagged suObList pointers | v > 0xE1E; always in scope |
| 10 | optional data +468/+460/+470 | tagged moMateBushingData_c, moMateFrictionData_c, moMateAnalysisData_c pointers, in that order | all historical gates satisfied |
| 11 | raw +430/+478 | u16, i32 | all historical gates satisfied |
| 12 | raw +480/+488 | two CStrings | v > 0x12EA; always in scope |
| 13 | raw +4B0/+4BC | two i32 | respective gates v > 0x1834 and v ≥ 0x23A9; always in scope |
| 14 | optional limit/reference values | i32 +4C0, f64 +4C8, f64 +4D0, i32 referenced-ID +4D8, i32 referenced-ID +4E0 | only when stored +4BC != -1 |

The obsolete extra word between +4BC and its conditional block exists only below 0x2408 and is absent in this scope. Final IDs are serialized four-byte values, not tagged object pointers. All offset-named words, strings, map keys, limit values and referenced IDs retain unknown semantics. Counts, pointer framing and the optional vector are structurally established. Null optional data pointers are admitted; populated unsupported derived classes stop through normal tagged-object dispatch. Collection order, duplicate keys and cached identities are retained rather than reconstructed as native runtime maps.

**Plane section.** Primary table `0x4D0D05D0` selects serializer `0x4C09ADC0`. Its first call is moRefPlane serializer `0x4C283C60`, including the complete reference-geometry/atom header and tagged plane-data pointer from 11.100. It does not substitute the body-feature suffix. Constructor `0x4C0993C0` calls the moRefPlane constructor, allocates its owned member +6F8 using `0x4C0990F0`, and that constructor assigns `moPlaneSectionData_c`. The section serializer invokes this owned member directly, without an object tag or object-map slot.

The owned data serializer `0x4C09A9F0` reads mgPlane restore (root point and normal, six f64), mgBBox restore (ten f64), a bounded i32 body count, then each kernel body using mo_loadBodyFromArchive. For a valid body the native loader next directly deserializes a suObList of attributes. The supported reader admits the established complete compressed single-body Parasolid frame and its attribute list. Suppressed, absent, unsupported or unproved body forms fail before inferring validity-dependent attribute bytes. It then reads four f64 border values and a display-root point (three f64); the historical gate for these final values is v > 0xF66, always satisfied here. Runtime owner and entity-list pointers are not additional archive fields.

The section suffix is i32 +710; bounded i32 edge count and tagged edge objects; tagged moLineRef_w pointer +708; i32 +714; i32 +718; f64 +720; i32 +8B4; bounded i32 map count with `(i32, CString)` entries +8B8; bounded i32 map count with `(CString, i32)` entries +8C8; bounded i32 map count with `(i32, tagged object)` entries +8E8; a direct suObList +8F8; bounded i32 body-reference count with tagged moCompPlaneSectionBody_c entries +940; then bounded i32 pair count with `(tagged moCompPlaneSectionBody_c, tagged moVisualProperties_c)` entries +960. Native historical gates for these groups are respectively 0xF86, 0xF89, 0x11B2, 0x11E9, 0x11E2 and 0x11F4; all are satisfied in scope. The three maps retain ordered pairs and remain wholly unknown semantics, including any nested value bytes. Raw suffix words and the f64 remain unknown. Unsupported populated reference classes still fail explicitly.

`moPlaneSectionEdge_c` selects `0x4BFC8490`: a tagged moGeneralSurfRef_w pointer, bounded i32 body count and complete supported kernel frames. These bodies have no following attribute-list field in this serializer. The section keeper selects import thunk `0x4CBA2733`, resolving to `swccu.dll!su_CObject::Serialize` at `0x31D3F030`. This is not an unconditional return: with archive flag 0x10 clear, its test at `0x31D3F040` branches straight to the epilogue at `0x31D3F0BE` and consumes no bytes. The flag-0x10 path remains outside the ordinary reader.

| Installed source | Configuration | Exact EOF | Unknown bytes | Known structure bytes |
|---|---|---:|---:|---:|
| Simulation Examples/Verification/nafems_nl5a_b.sldprt | 0 | 54,459 | 17,221 | 37,238 |
| Simulation Examples/vessel weldment.sldprt | 0 | 189,654 | 71,310 | 118,344 |

The first source contains two plane sections, including one with three owned kernel bodies, two section-edge records and three entries in the direct section list. The second contains two concentric mates, each with two entity pointers; its vector, map, optional data and limit branches are empty. Synthetic tests exercise the optional vector and limit widths, exact following-byte alignment and all truncation boundaries. Combined added decoded bytes are **244,113**, comprising **88,531 unknown** and **155,582 known structure bytes**.

`resolved-mate-plane-audit-0.381.json` records **905 complete / 1 explicit stop / zero context stops** among 906 modern part configurations, with **26,782 complete roots** across the audit. All preceding **903** complete streams preserve source hash, endpoint, root count and unknown total. Complete streams contain **83,383,250 decoded bytes**, including **29,684,470 unknown** and **53,698,780 known structure bytes**. The remaining modern-part stop is `moLibraryFeature_c` in the costing machined-part sample.

Add `--mate-plane-dependencies` to the full Edition 0.380 audit/regrade commands. The public reader enables this scope; staged reads default it off. The independent command `resolved_coverage_evidence.py --mate-plane-dependencies` checks all **1,166 source rows**, source hashes, stored spans and integer attribution. Added stored part credit is **41,065 bytes**. Cumulative resolved-family known bytes become **15,012,148**, leaving **10,125,318 unknown** of 25,137,466 stored bytes. Total part unknown bytes become **53,355,484** of 227,086,352. Rounded coverage remains parts **76.5% / 71.4%**, assemblies **83.8% / 81.3%**, drawings **57.3% / 49.9%**. Whole-document semantics, other contexts, legacy formats and all installed project-format schemas remain incomplete.

`mate_plane_evidence.py` verifies seven RTTI bindings, twelve complete native function ranges including chained unwind fragments, inherited feature hooks, owned-data constructors, the ordinary keeper-provider branch, populated source endpoints and preservation of all 903 preceding positives. Four focused tests cover optional native branches, every synthetic truncation, bounded counts, context rejection, direct-member map accounting, unsupported validity-dependent body forms, source EOF, disabled-scope zero credit and appended-byte zero credit. Previous decoder sources and the C-402 verifier are frozen under `research/frozen-decoder-0.380/`; the preceding gap report is retained with its original hash.

All **1,207 regression tests pass** (189.829 seconds). Specification lint reports zero findings (585 headings, 96 anchors); whitespace checks pass apart from existing line-ending notices.

Evidence: C-403.
