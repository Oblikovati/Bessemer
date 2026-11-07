### 11.320 Radiate, pattern-body-edge and end-cap-chamfer entity IDs

*Status: Partial document semantics · 53 native ID bodies; 913 supported managers · Evidence: C-362*

Three remaining modern ID families use native bodies equivalent to established scoped readers. Primary RTTI/vtable bindings, exact serializer code and relevant virtual suffix slots establish the following input:

| Class | Native dispatch | Wire suffix after common feature/FR representation prefix |
| --- | --- | --- |
| `moRadiateSurfIdRep_c` | Serialize 0x4BBB79B0; +0x2A0 suffix 0x4BBBF3F0 | raw i32 +0x38 and +0x3C |
| `moEndCapChamferSurfIdRep_c` | Serialize 0x4BBB79B0; +0x2A0 suffix 0x4BBBF3F0 | raw i32 +0x38 and +0x3C |
| `moPttnBodyEdgeIdRep_c` | Serialize 0x4BBBD460 | direct source entity-ID body at +0x38; raw i32 +0x48 and +0x4C |

The common prefix is helper 0x4BBBBF50 and retains the established tagged feature and external-feature-reference (`moFR_c`) pointers. The shared 0x4BBB79B0 entry calls that helper, then tail-dispatches through the concrete primary table's +0x2A0 slot. The two raw suffix parameters have no recovered application value domain; negative and other integer bit patterns are retained as unknown semantic data.

Pattern-body-edge Serialize calls the common prefix, then the embedded source entity object's virtual Serialize at +0x28 before reading its two raw words. `moEntityId_c` primary RTTI binds that direct body to 0x4BBBC390. The supported modern layout is one tagged representation pointer, not an extra tagged outer `moEntityId_c` object. Existing `modelboundary.linear_pattern` has exactly this source-ID-plus-two-word input; the new scoped reader reuses it. This correspondence is based on native body operations and pointer framing, rather than the similarity of pattern names.

`configurationremainingids.install` copies the current reader table and installs these three classes locally. It retains the existing modern ordinary external-XML context guard, rejects older/unrelated contexts before consuming bytes and does not register global class readers. The base readers preserve mapped feature, FR and source-representation object identities. Wrapper metadata retains each complete body's start/end, wire bytes and unknown-byte delta. Unsupported nested representations remain explicit failures.

`configurationmanager.read_supported` enables the extension by default. `remaining_ids=False` preserves the preceding reader scope for historical audit replays. The prefix API uses its explicit `remaining_id_records` flag. Existing modern context/version guards, older reader dispatch and postload validation remain in force.

`configuration_remaining_ids.py` completes the four formerly stopped sources. They contain **31 radiate IDs**, totaling **682 body bytes** with **372 unknown bytes**; **18 pattern-body-edge IDs**, totaling **799 bytes** with **360 unknown bytes**; and **four end-cap-chamfer IDs**, totaling **88 bytes** with **48 unknown bytes**. These are **53 object bodies / 1,569 bytes**, including **780 unknown bytes** and **789 known structure bytes**. The pattern-body-edge spans include their nested source-ID input. References are summarized as actual archive slots, without expanding or duplicating graph objects.

The all-part audit is now **913 supported managers / six explicit stops**, preserving all **909** previous complete rows exactly. Four new streams add **85,065 decoded bytes**, split into **38,336 known structure bytes** and **46,729 unknown bytes**. Cumulative complete manager streams contain **9,225,530 bytes**: **3,980,885 known** and **5,244,645 unknown**. The remaining six failures are a legacy part-config-object layout, explosion-part ID, direct visual map +0xB50, optional data +0x608 and two nonempty decal-owned stages.

The regrade adds **9,357 stored part bytes** beyond edition 0.339. Cumulative known CMgr credit becomes **857,852 stored bytes**, leaving **1,008,260 stored CMgr bytes unknown** in the unchanged **1,866,112-byte** denominator. Total part unknown bytes become **68,386,877**. Rounded coverage stays **69.9% / 63.4%** for parts, 83.8% / 81.3% for assemblies and 57.3% / 49.9% for drawings. Full document semantics and all installed project-format schemas remain unfinished.

`configuration_remaining_ids_evidence.py` revalidates native serializers, concrete suffix slots, embedded entity-ID RTTI, every exact source span, unchanged prior positive rows and proportional stored credit. Tests preserve mapped feature/FR/source identities, verify raw signed suffixes against trailing sentinels, reject invalid context/version, confirm absence of global registration, reproduce the record dataset and compare the full current audit. The current snapshots are `configuration-manager-supported-0.340.json`, `configuration-remaining-ids-corpus.json`, `configuration-remaining-ids-native-evidence.json` and `configuration-manager-coverage-0.340.json`.

Validation: all **1,069 regression tests pass** (164.802 seconds); specification lint reports zero findings (544 headings, 96 anchors).

Evidence: C-362.

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

### 11.348 Current-part weld component predicate and end-cap research continuation

*Status: Native-bound research readers; two incomplete source streams; no added coverage · Evidence: C-390*

At the Edition 0.368 checkpoint, the public restriction in 11.347 remained in force. This clause records that research continuation for ordinary modern part archives; neither `weldinternal.component` nor the end-cap reader was installed publicly at that checkpoint. Clause 11.349 adds the guarded supported composition. A recovered field boundary alone does not make its enclosing stream complete.

`moCompDetailItem_c::Serialize` at **0x4C956B10** compares its external/internal predicate with and without the archive's current owner. Its internal branch follows the existing `moCompRef_c` prefix with tagged `moDetailItem_c` pointers at runtime +E8 and +F8 and a raw i32 at +100. The nonnull case requires owner/model identity evidence. The following installed native chain establishes the current-part case:

- `setFiledOwner` at **0x4B84DCE0** stores the owner's registered model index. The owner getter at **0x4B845220** resolves that index through the global model manager.
- Ordinary `moUnitComponent_c::Serialize` at **0x4C2392A0**, with the stored external-model byte zero and no external owner wrapper, calls its +320 model setter with the archive's current owner. The setter at **0x4B84DD00** constructs or updates a model handle.
- Model-handle construction/setter and the pointer-to-index lookup preserve the same registered model identity. Lookup **0x4BEB4C90** validates a cached index against the actual object pointer before returning it; otherwise it searches the registry for that pointer. The model getter at **0x4B845C10**, through **0x4BE7B090**, uses the same global manager and index lookup as the owner getter.
- Unit virtual +168 at **0x4B84B5C0** tests for a nonnull model. Virtual +270 at **0x4B84A130** returns internal when its registered owner lookup equals that model. This is the identity branch; the part's virtual fallback is not substituted for it. In this build the relevant part fallback slots return true and false respectively, so a part class-name test alone would not prove the branch.

The research guard requires a complete ordinary current-part framing context, the original parsed `moPart_c` root present by identity in the inherited object map, an exact `moUnitComponent_c` class, stored external-model byte zero, and `component.owner is current_root`. A separately allocated equal dictionary or same-named part is rejected. The context marker records framing provenance; it does not certify unknown field meanings or emulate native runtime registration. The unit component need not equal a particular canonical unit-component instance: both installed weldment sources contain another valid instance owned by the same current part. External models, foreign owners, unresolved owners, missing provenance and other component classes remain unsupported.

The end-cap surface-ID representation has primary vtable **0x4CF73E98**, shared serializer **0x4BBB79B0**, and concrete virtual +2A0 target **0x4BBBF3F0**. Its inherited ID body and two signed i32 suffix fields match `modelboundary.end_face`; the research replay binds that reader locally as `moEndCapChamferSurfIdRep_c`.

`moEndCap_c` is bound in the installed **sldMoPartu.dll**, primary vtable **0x1804DF888**, serializer **0x180389070**. The corresponding model-DLL inventory has no primary binding for this class. In the supported modern version scope, its wire order is:

| Order | Body |
|---|---|
| 1 | Existing `moBodyFeature_c` body |
| 2 | Tagged `suObList` at runtime +800 |
| 3 | Four raw i32 fields: +808, +80C, +828, +82C |
| 4 | Four tagged `moDisplayDistanceDim_c` references: +840, +838, +848, +850, in that order |
| 5 | Raw f64 +820; raw i32 +830 |
| 6 | Tagged `moDisplayArcLengthDim_c` at +858 |

The native +828/+82C and +850 branch begins at version **0x1BCF**; +848 begins at **0x962**; +830 begins at **0xF7F**; +858 begins at **0x1FAF**. All gates are satisfied by the research reader's ordinary modern scope above 0x27CB. Earlier migrations and the final runtime virtual call consume no additional modern input. All five raw i32 fields and the f64 remain semantically unknown: **28 bytes** are explicitly excluded from value credit. Tagged reference identities and order are retained, including nulls and shared objects.

`weld_component_probe.py` preserves the initial owner-identity observations. `weld_internal_component.py` advances both weldment sources through their detail cabinets and end-cap ID representations to explicit `moEndCap_c` stops. `weld_end_cap_replay.py` then consumes their end-cap feature bodies and stops at **`moGusset_c`**, at source offsets **0x25A0D** and **0x2AF02** respectively. The original `insert_weld.sldprt` still reaches exact EOF at 29,940 bytes. Each replay retains source hashes, completed root boundaries, unknown totals and explicit stopping errors; no tail is silently accepted.

Four focused tests verify root identity/provenance rejection, scoped context propagation, ordered shared end-cap dimension references, conservative unknown accounting, every truncated payload boundary and the exact source replay outcomes. `weld_internal_evidence.py` binds the installed native code, vtable slots, source replays and decoder hashes in `weld-internal-native-evidence.json`. At Edition 0.368, public audit totals remained **871 complete / 35 explicit stops**, and stored coverage remained the Edition 0.367 report. All **1,139 regression tests passed** (158.644 seconds); lint reported zero findings. The frozen `research/frozen-decoder-0.368/resolvedfeatures.py` preserves that packet's original decoder hash. Clause 11.349 resolves the gusset and following weld-bead dependencies.

Evidence: C-390.

