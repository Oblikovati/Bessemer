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
