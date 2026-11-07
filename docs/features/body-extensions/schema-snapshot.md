### 11.354 Cavity, transformed-stock and sketch-hole feature bodies

*Status: Verified framing in ordinary modern part archives; raw fields remain partial semantics · Evidence: C-396*

Three primary class bindings close two cavity and two sketch-hole configurations. The installed model DLL has the SHA-256 recorded in C-20; `sldMoPartu.dll` has SHA-256 `2201edfa07b1a7484acedb6db4600fa7a00e55b16090a03b92fe7d9b14cb2128`. Complete inherited bodies, shared MFC identities, saved configuration-manager context and exact EOF remain required.

| Class | Module | Primary vtable | Serializer | Modern body |
|---|---|---|---|---|
| `moCavity_c` | sldMoPartu.dll | 0x1804D11D0 | 0x180374780 | complete body-feature base, then the cavity suffix below |
| `moXformStock_c` | sldmodu.dll | 0x4D1F6B88 | 0x4BB37210 | complete stock body at 0x4BB344D0; no modern stored suffix |
| `moSketchHole` | sldmodu.dll | 0x4CF432B8 | 0x4BB32ED0 | complete extrusion body at 0x4BB25180; no stored suffix |

RTTI confirms the respective `moBodyFeature_c`, `moStock_c` and `moExtrusion_c` ancestry. The model classes select the established lightweight/resolved concept serializers at vtable offsets `+0x4B8/+0x4C0`. The part DLL selects imported concept thunks at those offsets. The verifier resolves their import names rather than assuming empty inherited bodies.

The cavity suffix follows the complete body-feature base in this wire order:

| Order | Native member | Encoding | Modern treatment |
|---|---|---|---|
| 1 | +0x810 | i32 | Raw signed word |
| 2 | +0x818 | three f64 | Raw vector, 24 bytes |
| 3 | +0x830 | i32 | Raw signed word |
| 4 | +0x834 | i32 | Raw signed word |
| 5 | +0x838 | tagged object pointer | Nullable `moFaceRef_c`, same MFC map |
| 6 | +0x848 | tagged object pointer | Nullable `moCoordSysRef_w`, same MFC map |

The first four fields contribute **36 unknown bytes** per suffix. Native load gates for the first pair, +830, +834, +838 and +848 are respectively `v >= 0x489`, `v > 0xCE`, `v >= 0x2E6`, `v >= 0x3B4` and `v > 0x4D5`, all satisfied throughout the accepted modern range. The +838 load uses generic ReadObject; the matching native store overload establishes the face-reference type. Runtime reference-use calls and owner/property changes consume no bytes. Ancient migration bodies and save-time environment handling are excluded from this read-only modern grammar.

The transformed-stock serializer calls the complete stock serializer. Its extra body-reference field is confined to versions below `0x182`; its extra external-object field to `0x1AF..0x3AD`; later helper work is confined to versions below `0x3A7`. Thus there is no suffix in the accepted modern range. The two source cavity roots contain nested transformed-stock records, so registering the cavity suffix alone does not establish their endpoints. Existing stock transforms retain all 30 raw words and continue to reject unsupported perspective bits.

The sketch-hole serializer calls complete extrusion serialization and then performs only in-memory flag/property migration. It adds no stored bytes after the inherited extrusion body. Readers SHALL consume that full body rather than assigning an empty sketch-hole record.

| Installed source | Configuration | Exact EOF | Unknown bytes |
|---|---|---:|---:|
| Simulation Examples/Dynamics/sps_coil1.sldprt | 0 | 112,570 | 10,235 |
| tutorial/advdrawings/bowl bottom.sldprt | 0 | 206,451 | 49,519 |
| tutorial/swutilities/bracket_a.sldprt | 0 | 269,813 | 107,186 |
| tutorial/swutilities/bracket_b.sldprt | 0 | 257,368 | 103,789 |

The four streams contain **846,202 decoded bytes**, including **270,729 unknown** and **575,473 known structure bytes**. `resolved-body-extension-audit-0.374.json` records **894 complete / 12 explicit stops / zero context stops** among 906 modern part configurations, with **26,579 completed roots** across the audit. Every one of the preceding 890 complete streams preserves endpoint, root count, source hash and unknown total. Complete streams now contain **80,548,379 decoded bytes**, including **28,544,205 unknown** and **52,004,174 known structure bytes**.

Add `--additional-body-features` to the full Edition 0.373 audit/regrade commands. The public reader enables this scope; the staged reader defaults it off. `resolved_coverage_evidence.py --additional-body-features` independently checks all **1,166 source rows**, hashes, stored spans and integer proportions. Added stored part credit is **214,793 bytes**. Cumulative resolved-family known bytes become **14,646,747**, leaving **10,490,719 unknown** of 25,137,466 stored bytes. Total part unknown bytes become **53,720,885** of 227,086,352. Part coverage is **76.3% / 71.2%**; assemblies remain **83.8% / 81.3%** and drawings **57.3% / 49.9%**. Whole documents, remaining part dependencies, legacy/assembly/drawing contexts and all installed project-format schemas remain incomplete.

`body_extensions_evidence.py` rechecks primary bindings, ancestry, inherited concept dispatch, five native serializer bodies, four source EOFs and preservation of every preceding positive. Five focused tests cover signed raw values, shared/null reference identity, all cavity-suffix truncations, context rejection, local installation, full inherited delegation, disabled-profile zero credit and appended-byte rejection. The preceding C-395 packet retains its original hash through exact sources frozen under `research/frozen-decoder-0.373/`. The source graphs contain two cavity objects, two transformed-stock objects and twenty sketch-hole objects. All **1,168 regression tests pass** (190.961 seconds); specification lint reports zero findings (578 headings, 96 anchors).

Evidence: C-396.
