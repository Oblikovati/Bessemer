### 11.343 Spline-tangent, arc-length and plane-distance dimension classes

*Status: Exact native serializer reuse; three additional EOF streams; semantic coverage partial · Evidence: C-385*

The following native dimension classes append **no derived wire suffix**. Their primary slots select the same serializer addresses as their respective valued-dimension base classes. RTTI proves each stated ancestry, and the evidence verifier checks both derived and base primary slots independently in the installed C-20 model DLL.

| Derived class | Primary vtable | Serialize | Exact shared base body |
|---|---|---|---|
| `sgSplineTanMagnitudeDim` | 0x4D1AADE0 | 0x4C5A92E0 | `sgValG1Dim` |
| `sgArcLenDist` | 0x4D1B5D10 | 0x4C5A9E00 | `sgValG3Dim` |
| `sg3DPlane3DPlaneDist` | 0x4D1B10A8 | 0x4C5A9750 | `sgValG2Dim` |

All three serializers call the existing common valued-dimension helper **0x4C5A9150**. `sketchdimensions.valued_g1`, `valued_g2` and `valued_g3` retain their established ordinary modern layouts and conservative raw-field accounting:

| Body after common valued dimension | Modern fields |
|---|---|
| G1 | Two tagged `sgEntHandle` objects for runtime +130/+158; raw i32 +180/+184; raw vector +188 |
| G2 | Two tagged handles; raw u16 +180/+184/+1B8/+1BC; tagged `sg3DPlaneHandle` +1E0; raw vector +1C8; existing transitional u16 only when 0x2F79 < version < 0x2FA0 |
| G3 | Three tagged handles for runtime +130/+158/+180; raw u16 quadrant +1A8 |

Historical G1 handle/transform/arc-word gates and G2 word/active-plane/vector gates resolve to these existing modern bodies. G3 reads its quadrant from version **0x2DD**; the native runtime normalizes zero to one, while the schema reader retains the stored raw word. Runtime handle assignment/destruction, sketch association, geometric interpretation and DB-key contexts do not create derived payload bytes and are not emulated. This extension does not widen the supported version/archive context or establish unknown numeric meanings.

`sketchdimensions.install_additional_valued` copies the archive's reader map and installs these exact three class bindings. It also sets three local, immutable class tuples used by the G1/G2/G3 collection guards. Each collection continues to reject null entries and unrelated runtime classes. The default guards remain unchanged when the installer has not been selected. Class identity is preserved; mapped shared dimensions remain shared objects, and no alias class names or synthetic suffixes are inserted.

`resolvedfeatures.read_data(..., sketch_dimension_dependencies=True)` selects this local installer. The staged default remains false; public `read_supported` and `documented_bytes` enable it while retaining saved CMgr/model context, declared root counts, eligible MBSM, exact EOF and bounded unknown totals. Set `sketch_dimension_dependencies=False, isocurve_dependencies=False, weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` to replay edition 0.362. All historical regrade profiles explicitly pass false when the new option is absent. Global registrations and the inherited caller map remain unchanged.

The targeted five-source replay completes five newly bound dimension objects: two spline-tangent magnitude dimensions, two arc-length dimensions and one plane-to-plane distance dimension. Three streams reach exact EOF:

| Installed sample (under samples/) | Decoded stream bytes | Complete roots | Unknown bytes |
|---|---:|---:|---:|
| Simulation Examples/shovel.sldprt | 433,914 | 52 | 183,398 |
| Simulation Examples/Verification/nafems_ic21.sldprt | 31,194 | 28 | 12,611 |
| tutorial/motionstudies/redundancies/piston female.sldprt | 44,309 | 33 | 16,099 |

Coping Saw advances to **74 completed roots** before `moSculpt_c`; Garden Trowel Spade advances to **46 completed roots** before `moSkIsoCurveInfo_c`. Their new dimension objects belong to completed containing roots, but these two streams remain explicit failures and receive zero stream credit. Diagnostic cursors do not establish completion of the failing roots.

The full **906-configuration** `resolved-sketch-dimension-audit-0.363.json` reaches **869 exact-EOF streams / 37 explicit stops / zero context stops**, with **26,249 completed roots**. All **866** preceding positives preserve exact endpoints, root counts and unknown totals. Completed streams contain **75,635,248 decoded bytes**, comprising **26,659,145 unknown** and **48,976,103 known structure bytes**. The three added streams contribute **297,309 known decoded bytes**.

`sketch_dimension_frontier_evidence.py` verifies module identity, the three derived/base primary pairs, native ancestry, all three complete serializer ranges/instructions, common-helper calls, five source graphs and preservation of every preceding positive. `sketch-dimension-frontier-native-evidence.json` binds these observations to the shared reader and retained decompilation hashes. `sketch-dimension-frontier-corpus.json` records source hashes, completed root spans/hashes, new object identities and explicit next stops.

`resolved_coverage.py --surface-dependencies --remaining-planes --mid-surfaces --reference-curves --reference-surfaces --geometric-dependencies --backup-dependencies --sketch-dimension-dependencies` writes `resolved-feature-coverage-0.363.json`. `resolved_coverage_evidence.py --sketch-dimension-dependencies` independently verifies all **1,166 source rows**, hashes, decoded sizes, stored intervals, integer proportions and completion decisions in `sketch-dimension-feature-coverage-native-source-evidence.json`. Added stored credit is **55,339 part bytes**, taking cumulative resolved-family known bytes to **13,915,960** and leaving **11,221,506 unknown** of the unchanged 25,137,466 stored family bytes. Total part unknown bytes become **54,451,672** of 227,086,352. Rounded parts remain **76.0% / 70.8%**, assemblies **83.8% / 81.3%**, drawings **57.3% / 49.9%**. The frozen edition 0.362 gap report and historical verifier still pass. Full document and all installed project-format schemas remain unfinished.

Validation: three focused tests pass for default rejection versus scoped acceptance, shared mapped identity, all three collection boundaries and sentinels, unrelated-class rejection, inherited-map/global isolation, staged defaults and five source graph endpoints. All **1,123 regression tests pass** (155.408 seconds); specification lint reports zero findings (567 headings, 96 anchors).

Evidence: C-385.

