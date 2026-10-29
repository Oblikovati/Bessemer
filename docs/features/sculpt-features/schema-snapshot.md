### 11.356 Sculpt owned trim data, edge IDs and bounding-box center records

*Status: Verified framing in ordinary modern part archives; raw fields remain partial semantics · Evidence: C-398*

The installed `moSculpt_c` serializer is `0x18010D070` in hash-bound `sldMoPartu.dll`. It serializes the complete body-feature base, then directly dispatches selected members of owned `moSculptData_c` at pointer +818 and two embedded body choosers. Its primary concept hooks retain the established lightweight and resolved archive bodies. Reading the sculpt class's own tagged data pointer is not a substitute for the parent-written member sequence.

The constructor chain is explicit: `moSculpt_c` constructor `0x18010C960` calls `0x18010C870`, which invokes parameterized `moTrimSurfData_c` constructor `0x18035F9A0` before assigning the sculpt-data table. That trim constructor establishes direct `suObArray` +20 and owned `moEntRefKeeper_c` pointer +78. The keeper's serializer thunk `0x1803AD148` selects imported `suObArray::Serialize`. The sculpt constructor also initializes two direct `moBodyChooser_c` objects at +820 and +8A8. Their established serializer `0x4C27AAA0` writes a direct object array followed by a tagged atom-feature owner. These member types are proved by construction and dispatch, rather than inferred from runtime object sizes.

After the complete inherited feature, the modern wire sequence is:

| Order | Native member | Encoding |
|---|---|---|
| 1 | owned trim-data +20 | direct object array: ReadCount, then tagged objects |
| 2 | owned trim-data +78 | direct entity-keeper array body: ReadCount, then tagged objects |
| 3 | feature +820 | direct body chooser: object array, then tagged owner pointer |
| 4 | feature +8A8 | second direct body chooser, same grammar |
| 5 | feature array group +960 | signed i32 group count, then that many direct `suDWordArray` bodies for a positive count |
| 6 | owned data +C0, +C4, +D0, +C8, +CC | five i32 words, in this wire order |

The group count is preserved exactly. Native nonpositive counts execute no group iterations; each positive group is a ReadCount followed by that many u32 elements. The existing native-verified `suDWordArray` → `su_CDWordArray::Serialize` dispatch provides the element framing. Preserve every unsigned high bit, duplicate and group boundary. The five final words contribute **20 unknown bytes**; each DWORD element contributes **4 unknown bytes**. No MFC class declaration or pointer tag precedes the direct arrays or body choosers.

The legacy extra-array branches below `0x1419` and `0x1446` are outside the supported `v > 0x27CB` range. The final words have native gates +C0 always, +C4 from `0x141C`, +D0 from `0x142C`, +C8 above `0x1FB9`, and +CC above `0x1FFC`; all five are present in modern scope. Native initialization and owner-setting operations add no archive bytes. `moSculptData_c` itself selects the byte-free su_CObject serializer thunk `0x1803ACE78`; the parent explicitly serializes the members listed above.

Two nested edge-ID classes, `moRightSculptEdgeIdRep_c` and `moLeftSculptEdgeIdRep_c`, both select `0x4BBBD460` in `sldmodu.dll`. RTTI proves `moRef2EdgeIdRep_c` ancestry. The serializer calls the established entity-representation base at `0x4BBBBF50`, directly serializes its embedded identifier at +38, then reads i32 +48 and +4C. This is the existing `thinextrusions.silhouette_id` grammar, installed locally for the two additional names.

The nested `moBBoxCenterData2_c` primary table `0x4CE87D40` selects `0x4C27A9F0`: byte-free su_CObject base, i32 +8, three-f64 point +10, then f64 +28, exactly **36 raw unknown bytes**. Native code proves the same field sequence as the existing bounding-box-center body reader; the additional class name is installed only in the enabled feature scope.

`samples/learn/Coping Saw.SLDPRT`, configuration 0, now completes all declared roots and the required tail to exact EOF at **608,676 decoded bytes**, with **234,563 unknown** and **374,113 known structure bytes**. Its populated sculpt suffix contains two trim-array entries, two keeper entries, body-chooser arrays of six and four entries, four DWORD groups `[[0],[1],[2],[3]]`, and final words `[0,0,0,0,2]`. The complete graph contains one sculpt object, **50 right sculpt-edge IDs, 48 left sculpt-edge IDs and ten bbox-center-data-2 records**. Earlier explicit stops at those nested classes do not establish any earlier partial-root credit.

`resolved-volume-feature-audit-0.376.json` records **897 complete / 9 explicit stops / zero context stops** among 906 modern part configurations, with **26,602 completed roots** across the audit. Every preceding 896 complete stream preserves endpoint, root count, source hash and unknown total. Complete streams now contain **81,258,621 decoded bytes**, including **28,813,325 unknown** and **52,445,296 known structure bytes**.

Add `--volume-feature-dependencies` to the full Edition 0.375 audit/regrade commands. The public reader enables this scope; the staged reader defaults it off. `resolved_coverage_evidence.py --volume-feature-dependencies` independently checks all **1,166 source rows**, hashes, stored spans and integer proportions. Added stored part credit is **74,725 bytes**. Cumulative resolved-family known bytes become **14,732,228**, leaving **10,405,238 unknown** of 25,137,466 stored bytes. Total part unknown bytes become **53,635,404** of 227,086,352. Part coverage becomes **76.4% / 71.3%**; assemblies remain **83.8% / 81.3%** and drawings **57.3% / 49.9%**. Full documents, remaining feature dependencies, raw semantics, legacy/assembly/drawing contexts and all installed project-format schemas remain incomplete.

`volume_features_evidence.py` checks native bindings, inherited concept dispatch, the constructor/keeper chain, eight native bodies, the shared DWORD-array provider, the populated source EOF and preservation of every preceding positive. Five focused tests cover direct-body boundaries, shared map identities, signed words/nonpositive group counts, unsigned DWORD retention, every suffix truncation, invalid bounds, context rejection, local aliases, fixed bbox width, disabled-profile zero credit and appended-byte rejection. The preceding C-397 packet retains its original hash through exact sources frozen under `research/frozen-decoder-0.375/`. All **1,178 regression tests pass** (166.834 seconds); specification lint reports zero findings (580 headings, 96 anchors).

Evidence: C-398.
