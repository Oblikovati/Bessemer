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
