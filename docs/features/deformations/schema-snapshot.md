### 11.357 Deform feature, matching passes and versioned geometry

*Status: Verified framing in ordinary modern part archives; raw fields remain partial semantics · Evidence: C-399*

The complete `moDeform_c` body composes native routines from four hash-bound installed modules: part serializer `0x180253590`, model ID keeper `0x4C6E0850`, geometry serializer `0x43BBC8C0` in `sldgcu.dll` and primitive restore `0x32102F70` in `sldmgu.dll`. The feature begins with the complete body-feature base and inherited concept hooks. Its constructor `0x1802517C0` constructs direct `gcDeformData_c` at +8F8 and an owned `moDeformSIdKeeper_c` at pointer +10F0. The final keeper dispatch is a direct body call, without an additional pointer tag.

**Outer feature suffix.** In wire order: tagged display dimensions +800/+808; i32 +810; tagged direction +818; six nullable direct object arrays +828, temporary array 1, temporary array 2, +830, +840, +858; i32 +860; five nullable arrays +880/+888/+890/+898/+8A0; three-f64 point +8A8; signed i32 matching-record count +8C0 and that many first-pass matching records for a positive count; then signed stored data version +8D0. Nonpositive native counts execute no record iterations and are retained. Array presence is an i32 word followed, when nonzero, by ReadCount and tagged entries. Temporary arrays are genuinely serialized and SHALL NOT be omitted merely because native loading later discards them.

The stored **feature data version**, rather than the archive version, then controls these fields:

| Stored data version | Fields in the native load order |
|---|---|
| >=1 | tagged direction +820 |
| >=2 | nullable array +838 |
| >=3 | nullable array +848 |
| >=4 | i32 +8D4 |
| >=6 | i32 +8D8 |
| always | f64 +8E0/+8E8/+8F0; i32 +864; three-f64 vector +868; direct geometry body +8F8 |
| >=8 | nullable array +850 |
| always | second pass over each previously read matching record, as specified below |
| >=7 | extension-code loop, as specified below |
| always | direct edge/face surface-ID keeper +10F0 |

**Matching data.** `moDeformMatchData_c::Serialize` at `0x180253150` reads two sides. Each side is a nullable object array followed by two i32 words; after both sides comes a signed subrecord count and a positive-count loop of direct `moDeformMatchSubData_c` bodies. The subrecord serializer `0x1802532C0` reads two sides, each consisting of i32, tagged curve-point reference, generic tagged object reference, three-f64 point and three-f64 vector. Thus an empty-pointer subrecord still occupies **112 bytes**; its **104 scalar/geometry bytes** remain unknown. Native conversion of an endpoint object to a curve-point object changes runtime ownership and consumes no additional bytes. The decoder preserves the original shared archive-map identities.

The second matching pass `0x180252F60` reads nothing for feature data versions below 9. From version 9 it reads two `(nullable array, i32)` sides and another i32 +48. It then calls each subrecord's second pass at `0x1802530B0`; from feature data version 10 that pass reads two three-f64 vectors at +88/+A0. Earlier versions initialize runtime members without consuming those vector bytes.

The nested class `moCurvePointRef_w` has primary table `0x4D17F400`. Its serializer at `0x4C27C5E0` is a verified five-byte jump to the established endpoint-reference serializer `0x4C27D8F0`; it uses that complete grammar without an extra suffix. This additional binding is installed locally in the enabled deform scope.

**Direct geometry body.** `gcDeformData_c::Serialize` stores its own signed version at native offset +134. This is distinct from feature data version +8D0. Its fixed prefix is **372 bytes**: one i32 mode; seven three-f64 triples (two points, one vector, then two plane root/normal pairs); i32 +C0/+C4; f64 +C8/+D0; i32 +D8; f64 +E0/+E8/+F0; sixteen i32 words +F8..+134; ten f64 values +138..+180; and i32 +188. Preserve the stored mode and flags before native load-time normalization. All serialized geometry bytes remain unknown semantics.

| Geometry stored version | Additional fields, in wire order | Total bytes through this version |
|---|---|---:|
| <=0 | none beyond the fixed prefix | 372 |
| >=1 | i32 +18C/+190 | 380 |
| >=2 | f64 +5C0/+5E8 | 396 |
| >=3 | fixed 30-u32 archive transform +5F0 | 516 |
| >=4 | i32 +678; primitive-body data +680 | 576 |
| >=5 | f64 +7E0 | 584 |
| >=6 | i32 +7E8 | 588 |
| >=7 | i32 +7EC | 592 |

These rows are cumulative native signed comparisons, not a table indexed by unvalidated bytes. The transform is the **fixed 120-byte archive operator**, not compact restore; unsupported perspective flag bits fail explicitly. Primitive-body restore is **56 bytes**, i32 +0, six f64 values at +8..+30, then i32 +38. The CArchive primitive export at `0x32102F60` jumps to the verified su_CArchive implementation at `0x32102F70`. Runtime geometry defaults and resetting the in-memory geometry version to 7 add no stored bytes.

**Extensions and keeper.** The extension loop reads i32 codes: `0x3D2` carries one i32, `0x13E` one f64 and `0x1466` one tagged object. Any other code terminates the native loop after that code, so its exact signed value is preserved; the installed source terminates with 0. There is no inferred length or opaque skip for unrecognized codes. The final owned `moDeformSIdKeeper_c` selects the established `moEdgeFaceSIdKeeper_c` serializer `0x4C6E0850`: direct surface-ID array +18, direct object array +50, and an additional surface-ID array +28 when archive version `>0x3746`. Its constructor at `0x4BA269A0` establishes the inherited keeper type. Reference-use/owner updates consume no additional bytes.

`samples/tutorial/molds/telephone.sldprt`, configuration 0, now reaches exact EOF at **556,067 decoded bytes**, including **248,283 unknown** and **307,784 known structure bytes**. It contains one deform object and two curve-point references, feature data version **11**, geometry version **7**, one populated matching record with one populated subrecord, and extension terminator 0. All declared feature roots and required tails complete.

`resolved-deformation-audit-0.377.json` records **898 complete / 8 explicit stops / zero context stops** among 906 modern part configurations, with **26,638 completed roots** across the audit. Every preceding 897 complete stream preserves endpoint, root count, source hash and unknown total. Complete streams now contain **81,814,688 decoded bytes**, including **29,061,608 unknown** and **52,753,080 known structure bytes**.

Add `--deformation-dependencies` to the full Edition 0.376 audit/regrade commands. The public reader enables the scope; the staged reader defaults it off. `resolved_coverage_evidence.py --deformation-dependencies` independently checks all **1,166 source rows**, hashes, stored spans and integer proportions. Added stored part credit is **63,464 bytes**. Cumulative resolved-family known bytes become **14,795,692**, leaving **10,341,774 unknown** of 25,137,466 stored bytes. Total part unknown bytes become **53,571,940** of 227,086,352. Rounded coverage remains **76.4% / 71.3%** for parts, **83.8% / 81.3%** for assemblies and **57.3% / 49.9%** for drawings. Full documents, remaining dependencies, raw semantics, legacy/assembly/drawing contexts and all installed project-format schemas remain incomplete.

`deformation_evidence.py` rechecks four DLL hashes, primary bindings and ancestry, inherited concept imports, eleven native bodies plus the curve-point leaf jump, constructor/keeper composition, the populated two-pass source and all 897 prior positives. Replay tuples are normalized to JSON arrays solely for comparison with the saved evidence representation; original stream hashes and decoder coordinates remain unchanged. Six focused tests cover geometry version boundaries 0..7, every populated geometry/matching truncation, fixed/primitive widths, unsupported perspective flags, both matching passes, all extension types and a nonzero native terminator, shared references, invalid counts, context rejection, local installation, exact source EOF, disabled-scope zero credit and appended-byte rejection. The preceding C-398 packet is reproduced through exact sources frozen under `research/frozen-decoder-0.376/`. All **1,184 regression tests pass** (163.157 seconds); specification lint reports zero findings (581 headings, 96 anchors).

Evidence: C-399.
