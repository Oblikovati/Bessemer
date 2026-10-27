### 11.238 Constrained planes and reference-plane geometry

*Status: B / ? — eight native type bindings, eight populated containing plane roots and six complete feature lists verified; scalar domains, engineering validity and other constraint variants remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, constrained plane-data objects use the established `moRefPlnData_c` body at `0x4C283D30`, including its existing file-version gates. Shared constraint serializer `0x4B8F8090` then reads or writes one tagged `moConstraintRefplaneInterfaceData_c` pointer at runtime `+0x88`. This is not an inline interface body or a nullable-array presence word.

`moConstraintMidPlaneRefplaneData_c` selects `0x4B8F8040`, whose complete five-byte body `E9 4B 00 00 00` jumps to `0x4B8F8090`. It adds no suffix beyond the common constraint data. `moConstraintRefplaneData_c` selects the common target directly. The leaf jump has no unwind-table entry; a decompiler following it can display the target body under the thunk's name, which does not create a second payload.

Interface serializer `0x4B8F8140` calls the empty object base and serializes **exactly three tagged geometry pointers**, at runtime `+0x08`, `+0x10`, `+0x18`. There is no stored slot count. Native loading accepts null slots and otherwise checks `IsKindOf(moRefPlaneGeometry_c)` before assignment. The diagnostic reader retains the actual slot pointers and inherited object-map identity.

The geometry base `moRefPlaneGeometry_c` selects `0x4C2E5390`. After the empty object base its wire body is:

1. Tagged `moRefWrapper_c` pointer at runtime `+0x08`.
2. Two binary64 values for native surface parameters U and V, associated with runtime `+0x10`.
3. Direct `mgPoint_c` at runtime `+0x20`, containing three binary64 components.

These values are retained without inferring parameter ranges, coordinate units or geometric validity from their numeric appearance. With a null or already-mapped wrapper pointer the base occupies 42 wire bytes.

The populated geometry subtypes are:

| Type | Selected serializer | Suffix after geometry base |
|---|---|---|
| `moRefPlaneCoincidentGeom_c` | `0x4C2E52D0`, five-byte jump to `0x4C2E5390` | None |
| `moRefPlaneMidPlaneGeom_c` | `0x4C2E5470` | One u8 at runtime `+0x38` |
| `moRefPlaneAngleGeom_c` | `0x4C2E5240` | One binary64 at runtime `+0x38`, then one u8 at `+0x40` |

For both byte flags, native storing emits the Boolean test of the runtime byte, while native loading sets the runtime Boolean true **only when the stored byte equals 1**. Values 2 or 255 load as false; they are not Booleanized by a nonzero test. The raw decoder preserves the byte and exposes its separate derived load value. With a null or mapped wrapper, the midplane and angle bodies occupy 43 and 51 wire bytes respectively. The coincident geometry's exact jump bytes are `E9 BB 00 00 00`.

`moConstraintCoincLineAtAnglePlaneRefplaneData_c` selects `0x4B8F8000`, which calls shared angle-parent serializer `0x4B8F7F00`. That parent calls the common constraint serializer, then writes an i32 at runtime `+0x148` and a tagged `moDisplayAngularDim_c` pointer at `+0x150`. Subsequent dimension-owner and geometry updates are runtime processing. The selected subtype initializes runtime `+0xE8` to 1 on loading without consuming another archive word. The raw i32 and double domains remain unresolved.

Eight complete containing `moRefPlane_c` roots total **19,477 bytes**, across seven configurations. Four midplane cases occur in Mouse Battery Cover and three shaft tutorial variants. Four coincident-line/angle cases occur in motor_frame, revolved_1_new and two Golf Ball plane roots. The eight roots exercise interface geometry slots, coincident/midplane/angle geometry types and dimension-reference composition. Six affected feature lists complete; Golf Ball reaches `mo3DSphereBackedUpData_c`. Exact stream/root hashes, class order, slot types and raw geometry flag bytes are retained. These results establish selected framing without claiming whole-document or all-installed-project bit coverage.

The 906-context audit now contains **25,136 complete roots**, **805 complete lists** and **101 explicit stops**, with every preceding complete root record unchanged. This adds 31 roots and six complete declared lists. Only two new lists reach exact EOF: Mouse Battery Cover (1,078,929 decoded bytes) and motor_frame (134,597), totaling **1,213,526 bytes**. The other four completed lists retain four-byte unknown tails. Totals are **728 exact-EOF lists / 57,616,318 decoded bytes** and **77 complete lists with four-byte unknown tails**. Checkpoints `resolved-feature-list-frontier-0.257.json` and `resolved-feature-list-frontier-0.258.json` retain the comparison. Neither list completion nor the unidentified tail earns additional whole-document semantic coverage credit.

Validation: all 868 regression tests pass (124.330 seconds); specification lint reports zero findings.

Evidence: C-280.

### 11.332 Top-surface IDs, parallel-plane constraints and feature states

*Status: Native-bound dependencies; four additional exact-EOF streams; semantics partial · Evidence: C-374*

The missing top-surface representation uses direct serializer **0x4BBBDE70**, not the virtual-suffix dispatcher. Its primary table is **0x4CF6BE68** (locator 0x4D334238). It reads inherited entity-ID representation via **0x4BBBBF50**, then one signed i32 at **+0x38**. This is the existing one-word `modelboundary.from_sketch_entity` grammar. Table +0x2A0 is not consulted by this serializer and must not be interpreted as a suffix function. The raw word retains unknown meaning; no topological index domain is inferred.

The newly reached constraint and parallel geometry bind to existing verified bodies:

| Class | Primary table | Serialize binding | Wire composition |
| --- | --- | --- | --- |
| moConstraintCoincLineParallelPlaneRefplaneData_c | 0x4CED26E8 | 0x4B8F8040, exact E9 delegation to 0x4B8F8090 | Ref-plane data dispatch, then tagged constraint-interface pointer at +0x88. |
| moRefPlaneParallelGeom_c | 0x4D1752A0 | 0x4C2E52D0, delegation to common geometry input | Tagged reference-wrapper pointer, two raw f64 parameters and a three-f64 point. |

Constraint base **0x4C283D30** calls `su_CObject::Serialize` and virtual payload +0x150. The concrete constraint table binds that payload to **0x4C2880A0**, the verified ordinary ref-plane data body, with no default-plane display-point insertion. Its trailing interface is the existing three tagged geometry slots (11.238). Parallel geometry uses the existing **0x4C2E5390** common body without an extra subtype suffix. Runtime geometry/plane interpretation remains separate from retained wire fields.

`featureStateData_c` belongs to `sldsmmu.dll`, primary table **0x3DC2B0A0**, locator **0x3DC73158**. Serialize **0x3DB5BD30..0x3DB5BDAC** constructs an archive-helper view and tests input/output, then reads/writes exactly two signed i32 values at **+0x08/+0x10**. It has no version gate, extra object header or inherited wire prefix within this function. `flatpatterns.feature_state` preserves both as raw words; all **eight body bytes** retain unknown semantic attribution. Synthetic negative/positive values and a following sentinel verify the eight-byte endpoint and truncation rejection.

These readers are installed locally by `resolvedfeatures.read_data` flags `top_surface_ids`, `constraint_plane_dependencies` and `feature_states`. Existing unextended scopes and global registrations remain unchanged. The targeted dependency-chain probe completes eight top-surface IDs, one constraint object, one parallel geometry and three feature-state objects. Mouse bottom housing now completes **140 roots** and consumes **1,345,977 bytes** to exact EOF; drive sled Config-1 completes **76 roots** and consumes **851,130 bytes** to exact EOF. Their original top-ID stops and intermediate plane/state stops are retained in separate corpus snapshots.

The full audit, reproduced with `resolved_persisted_context_audit.py --mbsm --selection-items --top-surface-dependencies`, reaches **836 exact-EOF streams / 70 explicit stops / zero context stops**, with **25,725 completed roots** across 906 configurations. All **832** previous complete streams preserve exact endpoints, root counts and unknown-byte totals. The four added completions are bottom housing, tutorial API formingtool, motion-path link1 and drive sled Config-1. Completed streams total **70,507,271 decoded bytes**, including **24,743,512 unknown bytes**. `resolved-top-surface-audit-0.352.json` retains every source outcome. This diagnostic adds no stored-byte registry credit and does not establish complete semantics or whole-document coverage.

`top_surface_dependencies_evidence.py` verifies four primary class bindings, exact delegation targets, common virtual payload, retained native bodies/imports and the two targeted EOF results. `top-surface-dependencies-native-evidence.json` retains those bytes and decompilation identities; `top-surface-dependencies-corpus.json` retains source hashes, root intervals/hashes, IDs and state fields. Tests check signed state/ID words, sentinel/truncation endpoints, both completed source graphs and scoped registration. Other resolved classes and full part/assembly/drawing/project bit coverage remain unfinished.

Validation: all **1,095 regression tests pass** (125.792 seconds); specification lint reports zero findings (556 headings, 96 anchors).

Evidence: C-374.

### 11.336 Point/surface planes and remaining constraint-plane variants

*Status: Native-bound ordinary plane bodies; four additional EOF streams; semantic coverage partial · Evidence: C-378*

`moPntSurfRefPlnData_c` primary table **0x4D139F50**, locator **0x4D357250**, binds Serialize +0x28 to **0x4C281950**. It calls ordinary `su_CObject::Serialize`, then virtual payload +0x150. The concrete payload is **0x4C2880A0**, the existing common ref-plane data reader without a default-plane display-point insertion. Its derived wire suffix is:

| Order | Field |
| --- | --- |
| 1 | Raw signed i32 +0x9C, then raw signed i32 +0x98. |
| 2 | Tagged moSurfaceRef_w pointer +0x88. |
| 3 | Tagged moPointRef_w pointer +0x90. |
| 4 | Raw signed i32 +0xA0 when version >=0x10BF. |
| 5 | Raw signed i32 +0xA4 and +0xA8 when version >=0x10D5. |

Native runtime-class descriptors **0x4DAEE750 / 0x4DAEE780** identify the two reference types, and native input checks their kinds after reading the suffix. Both references must be nonnull. Modern supported context (>0x27CB) resolves all three gates above; the new reader does not admit older point/surface layouts. Raw fields remain unknown, and runtime defaults are not substituted for their source values.

Four concrete constraint variants share the verified common plane/interface composition of 11.238:

| Class | Primary table | Serialize +0x28 | Additional wire input |
| --- | --- | --- | --- |
| moConstraintCoincTwoPointsPerpPlnRefplaneData_c | 0x4CED1BC8 | 0x4B8F8040 | None beyond common constraint data. |
| moConstraintPrllPlnTanOneCylinderRefplaneData_c | 0x4CED41B0 | 0x4B8F8040 | None beyond common constraint data. |
| moConstraintCoincTwoLinesRefplaneData_c | 0x4CED1550 | 0x4B8F7FA0 | Raw signed i32 +0xE8. |
| moConstraintCoincLinePerpPlaneRefplaneData_c | 0x4CED2268 | 0x4B8F7FA0 | Raw signed i32 +0xE8. |

Entry 0x4B8F8040 is the exact five-byte E9 jump to **0x4B8F8090**. All four concrete tables bind payload +0x150 to **0x4C2880A0**. The common serializer reads ordinary ref-plane data followed by the tagged three-geometry-slot constraint interface. Function **0x4B8F7FA0** calls that complete common body before reading its one signed word. No extra time value, alignment or derived padding is serialized.

`moRefPlanePerpendicularGeom_c` and `moRefPlaneTangentGeom_c`, primary tables **0x4D175340 / 0x4D1753E0**, both bind Serialize to **0x4C2E5470**. This is the existing common geometry body **0x4C2E5390**, followed by raw u8 +0x38. The runtime Boolean compares that byte with 1; the decoder retains the original byte and its unknown attribution rather than normalizing the source.

`resolvedfeatures.read_data(..., remaining_planes=True)` installs these seven classes locally. `read_supported` and `documented_bytes` now enable the verified set; setting `remaining_planes=False, mid_surfaces=False, reference_curves=False, reference_surfaces=False, geometric_dependencies=False, backup_dependencies=False, sketch_dimension_dependencies=False, isocurve_dependencies=False, weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` reproduces the preceding scope. The new point/surface reader requires the existing modern ordinary context. Synthetic tests compose the real common plane body with mapped nonnull references, signed suffix words and a following sentinel; they check the **43-byte unknown total** (23 common unknown bytes plus five raw i32 fields), truncation and null-reference rejection. The single-word constraint suffix is tested after the actual common body too.

The four targeted configurations complete **five plane objects**. Mouse Buttons contains the two-points/perpendicular and line/perpendicular variants (the latter stores +0xE8=-3); nafems_ic22 contains the point/surface variant; API part1 contains the parallel/tangent-cylinder variant; piston male contains the two-lines variant (+0xE8=-2). Their complete root counts are **87/28/24/30**, with exact-EOF sizes **670,903 / 27,022 / 25,443 / 36,324 bytes**. Source fields and root intervals/hashes are retained in `remaining-planes-corpus.json`.

The full audit reaches **851 exact-EOF streams / 55 explicit stops / zero context stops**, with **25,903 completed roots** across 906 modern configurations. All **847** prior positive streams preserve exact endpoints, root counts and unknown-byte totals. Completed streams total **72,593,626 decoded bytes**, including **25,408,970 unknown bytes** and **47,184,656 known structure bytes**. `resolved-plane-audit-0.356.json` is reproduced by the preceding audit options plus `--remaining-planes`.

The stored regrade adds **255,282 known part bytes** beyond edition 0.355. Cumulative resolved-stream credit is **13,576,135**, leaving **11,561,331 unknown bytes** in the unchanged **25,137,466-byte** family denominator. Total part unknown bytes become **54,791,497**; part coverage is **75.9% / 70.6%**, while assembly **83.8% / 81.3%** and drawing **57.3% / 49.9%** coverage remain unchanged. Failed streams, other contexts and unknown field meanings remain uncredited.

`remaining_planes_evidence.py` verifies the seven primary/payload bindings, complete native instructions/imports, exact delegation thunk, concrete reference types, all four source EOFs and preservation of 847 prior positives. `remaining-planes-native-evidence.json` retains that packet. `resolved_coverage.py --surface-dependencies --remaining-planes` generates `resolved-feature-coverage-0.356.json`; `resolved_coverage_evidence.py --remaining-planes` rechecks every source/stored span, hash and proportion in `plane-feature-coverage-native-source-evidence.json`. Frozen preceding gap reports retain their original hashes for historical replay. Full document and all installed project-format schemas remain incomplete.

Validation: all **1,104 regression tests pass** (186.178 seconds); specification lint reports zero findings (560 headings, 96 anchors).

Evidence: C-378.

