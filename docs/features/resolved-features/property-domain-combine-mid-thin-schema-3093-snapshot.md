### 11.220 Combine/delete body features and solid references

*Status: B / ? — four combine features, three delete features/payloads and 16 solid references verified inside complete native roots; raw option meanings, populated solid-reference kernel bodies and whole-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature context, version >0x27CB. sldMoPartu.dll primary RTTI binds moCombineBodies_c (0x180466A48 / COL 0x1804ED890) to 0x18024EEE0 and moDeleteBody_c (0x18046B9B0 / COL 0x1804EDC28) to 0x18025B620. Both call the imported moBodyFeature_c serializer. Their +4B8/+4C0 hooks resolve to the ordinary moConcept_c lightweight/resolved-data pair. After the body-feature prefix, the combine suffix is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | MainBody +800 | tagged moCompSolidBody_c | always | Named current body association; preserve stored pointer identity. | B |
| 2 | ToolBodies array +808 | direct suObArray | always | ReadCount and tagged objects; preserve order and duplicates. | B |
| 3 | OperationType +828 | i32 | always | Preserve signed stored option; enum type name does not close numeric meanings. | B / ? |
| 4 | IsMainBdyTransparentInEdit +838 | u8 | version >0x3F07 | Preserve the exact stored byte before native nonzero-to-Boolean normalization. | B / ? |

Exact own moCombineBodies_c methods bind the existing saved members to OperationType at 0x828, IsMainBdyTransparentInEdit at 0x838, MainBody at 0x800 and ToolBodies at 0x808. OperationType reads the whole current 32-bit word and has declared type booleanType_e; this supplies no numeric operation labels or closed file domain. The transparency getter machine code zero-extends the raw current byte despite its bool signature. MainBody returns the current declared moCompSolidBody_c pointer, while ToolBodies returns the address of the embedded suObArray.

The complete own setData method has four explicit arguments: operation, main-body const reference, tool-array const reference and bool. Its initial unsigned operation>2 guard returns before the frame and mutation work. The bool is the fifth ABI position including this, read from entryRSP+0x28 (currentRSP+0x90 after the frame). The reached validation/update path stores the operation before conditionally calling the old nonnull body virtual slot 0x08(1), clearing current MainBody, invoking the supplied body virtual slot 0x108 and assigning the returned pointer. It then stores the incoming bool low byte, calls deleteAll on the embedded ToolBodies array and passes selected per-item virtual slot 0x258(0) return values to addUnique. These provider results are not guaranteed to be the input pointers. Earlier validation, exceptions, aliases and callbacks can leave partial state; no atomic transaction, deep-copy semantics or geometric set-operation result is established.

The operation remains a signed32 archive word. Transparency STORE and LOAD independently require their current unsigned version query >=0x3F08 and normalize zero/nonzero to 0/1. The raw-byte getter and initial setter checkpoint are distinct from those conversions. An older LOAD bypass is not an implicit reset, and offline decoding preserves the actual stored byte. Existing typed MainBody and direct ToolBodies framing is unchanged. Evidence: C-3545.

Factory 0x18024EBD0 calls constructor 0x18024ED10, which constructs direct su_CObArray at +808 and installs suObArray's table. The runtime descriptor allocation 0x840 is not a wire size. With null/cached references and an empty array, the complete combine body is 259 bytes through version 0x3F07 and 260 thereafter.

The complete delete-body suffix is one tagged moDeleteBodyData_c pointer at +800, making its empty body 253 bytes. The payload primary 0x18046B920 / COL 0x1804EDBA0 selects 0x18025B540 and has su_CObject ancestry. Its ordinary zero-byte base is followed by a direct suObArray at +08 and i32 raw +28. The native +28 load gate (>0x1BC3) predates scope. Payload creator 0x18025B070 allocates 0x30 runtime bytes and constructs the direct array; the empty wire body is **six bytes**, not the allocation size. No operation enum or array-item restriction is inferred from observed values.

The nested moSolidRef_w is independently bound in sldmodu.dll: primary 0x4CED5050 / COL 0x4D320080 selects 0x4B90DED0, with moRefWrapper_c ancestry. Helper 0x4B90DB10 first calls wrapper serializer 0x4C2842D0, then restores u32 raw +A8 (native gate >0x1BBD, before scope). The derived body reads an **i32 body presence**. Zero ends the object. Nonzero selects one strict installed-kernel body followed immediately by a direct suObList of attributes. There is no body count or extra terminal pointer. Native partition/ownership/attribute fixups and condition 0x2000's store-time suppression do not add another field. The complete absent-body reference is 70 bytes at version 19000, or 58/62/66 bytes at wrapper checkpoints 0x27CC/0x2B73/0x2FB0. The present branch uses the existing strict kernel-body grammar and stops at unsupported kernel contexts; no opaque byte skip is used.

Native RTTI also establishes moCompSolidBody_c's moCompRef_c ancestry, admitting this already decoded component class to the wrapper's accepted component-reference contexts. All four new class bindings are confined to the copied modern feature reader map.

Six focused tests cover combine byte values 0/1/255, signed-word limits, direct-array ordering/duplicates, version boundaries and tagged minima, new payload-object map insertion and cached array identities, wrapper component identity, solid-reference raw word/full-width presence and conditional attributes, truncation/context rejection, and source-hash-bound native containing-root spans/fields for all four classes. Synthetic present-solid references exercise common suppressed-kernel framing; populated native non-suppressed bodies remain unobserved in these references.

The audit retains **four combine features**, with array counts 1/1/12/1, raw +828 equal to zero in three and 2 in one, and raw +838 zero throughout. **Three delete features/payloads** contain arrays of 2/2/1 moCompSurfaceBody_c targets and raw +28 zero. **Sixteen solid references** have body presence zero; raw +A8 values are preserved without assigning an identifier meaning. Across 906 complete modern contexts there are **24,372 complete roots**, **769 complete lists** and **137 explicit stops**: 66 additional roots and three additional lists relative to Edition 0.225. **696 lists reach exact EOF**, spanning **46,507,594 decoded bytes**, an increase of 316,244 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. `resolved-feature-list-frontier-0.225.json` preserves the prior checkpoint. Whole-document coverage credit is unchanged; complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-248.

### 11.337 Mid-surfaces, face pairs and directly owned surface-ID arrays

*Status: Native-bound ordinary modern bodies; five additional EOF streams; semantic coverage partial · Evidence: C-379*

The following layouts apply in the existing modern ordinary resolved-feature context. Object offsets identify native members, not wire offsets. Tagged pointers retain the archive's existing object-map rules; directly owned arrays have no outer object tag. Raw numeric values retain their original representation; scoped native associations below do not close unassigned domains.

| Class | Wire body after inherited body | Native serializer |
|---|---|---|
| `moMidSurface_c` | `moBodyFeature_c`; raw f64 at +800, raw u8 at +808, tagged `suObList` pointers +848 and +850 | sldMoPartu `0x18023A0C0` |
| `moMidRefSurface_c` | Reference geometry and inherited surface payload; signed i32 body presence +7B0; kernel body only when presence equals 1; tagged `moMidSurfData_c` +808; direct keeper +800; raw i32 +7C8; two signed-count arrays of tagged `moSingleFaceRef_w` at +7E8/+7F8 | sldMoPartu `0x18023C860` |
| `moMidRefSurfSIdKeeper_c` | Existing `moFaceFilletSIdKeeper_c` body, then direct `suObArray` +128 and +148 | sldMoPartu `0x180207C00` |
| `moMidSurfData_c` | Ordinary zero-byte `su_CObject` prefix; nullable direct object array +8; KnitFlag i32 +10 | sldMoPartu `0x18023CCA0` |
| `moFacePair_c` | Tagged single-face references +8/+10, then tagged `moDisplayDistanceDim_c` +18 | sldMoPartu `0x18023C6E0` |
| `moFaceToHeuristicEdges_c` | Direct `suObArray` +10, then tagged `moCompFace_c` +8 | sldMoPartu `0x180207BA0` |
| `moEdgePair_c` | Tagged `moCompEdge_c` pointers +8/+10 | sldMoPartu `0x180207B20` |
| `moBendModifiedSurfIdRep_c` | Existing mirror/source entity-ID representation, then raw signed i32 +48 | sldmodu `0x4BBBBC30` |

Three separate native class joins refine selected fields without transferring meanings between matching numerical offsets.

Exact moMidRefSurface_c getKnitFlag/setKnitFlag operate through its current typed data pointer at 0x808 and read/write the full 32-bit moMidSurfData_c member 0x10. That word uses signed32 archive codecs. The selected LOAD read requires its current unsigned version query >=0xF0E; a skipped read supplies no new reset guarantee. The name KnitFlag does not restrict the file word to Boolean values or prove successful knitting.

In moMidSurface_c, getGcFaceList selects the saved tagged list 0x848 for selector1 and list 0x850 for selector2. It traverses the selected suObList, resolves each reached nonnull payload through the typed face provider, and appends resolved faces to the caller gcEntityList. It does not clear caller output. An invalid selector, null selected list, null payload or null resolved face returns 0; a valid empty list returns 1. Earlier appends remain if a later item fails. Return1 denotes completion of this selected traversal, not complete geometry success or rollback safety. The two list selectors are not assigned front/back meanings or a count/identity relationship.

In the separate MidRef path, getPlacement reads the current data array at data 0x08 using the moFacePair_c element protocol and scans in order for the first nonnull dimension member 0x18. Neither the pair nor the array is null-guarded. For valid current objects, it calls that dimension virtual slot 0x868(0) once. On normal nontrapping floating-point completion, an ordered value in the inclusive interval [-100,+100] is returned; otherwise the readonly native default 50 is returned. A nonpositive signed array count or all dimension pointers null also selects 50. An out-of-range or unordered first candidate does not cause a search for another candidate. The saved FacePair member is the existing tagged moDisplayDistanceDim_c occurrence; this is a current provider query, not direct recovery of a raw placement scalar.

These policies establish no percentage, unit, coordinate frame, arbitrary array-element type check, stable archive/current identity or geometry result. Original moMidSurface_c raw 0x800/0x808 and other unassigned fields remain unresolved; they are not the MidRef data-pointer/member pair. Evidence: C-3546.

The mid-reference extra-body flag is present from version 0xF49, the +7C8 word from 0xFAA, and both face arrays from 0xFB1. Each face-array signed count consumes `max(count, 0)` tagged references. The keeper's second direct array is present from 0x69F; the data object's final word requires version greater than 0xF0D. All these gates are satisfied in the supported modern context. Historical face-pair direction repair through 0x6A8 consumes no additional input. These historical observations do not widen supported contexts.

The mid-reference serializer spans **three chained unwind ranges**, ending at `0x18023CC9C` (1,084 instruction bytes total). The constructor at `0x180208C60` creates its directly owned keeper through `0x180207780`; that constructor proves both embedded arrays and their `suObArray` vtables. The face/heuristic constructor at `0x180207720` independently proves its embedded array. No tagged keeper or embedded-array header is inserted. The mid-surface runtime owner assignment at +818 and mid-reference postload callback consume no wire fields and are not emulated.

`moRefSurface_c` uses the inherited reference-geometry body and surface payload at sldmodu `0x4BB31450`. Its virtual +AB0 predicate returns 1 at `0x4B29DB10`. In the ordinary context, a nonzero archive backend marker therefore routes its sheet count through the backend callback at +508: **zero inline sheet-body or attribute-list bytes** follow for those sheets. The separate extra-body flag/body and word-vector suffix still follow normally. `moMidRefSurface_c` overrides that predicate with return 0 at sldMoPartu `0x1800E3930`, preserving inline inherited sheet bodies. `planarsurfaces.surface_payload(..., external_sheets=True)` expresses the base-class branch without changing existing default readers. External sheet contents and backend resolution semantics receive no coverage credit.

`midsurfaces.py` implements these layouts. `resolvedfeatures.read_data(..., mid_surfaces=True)` installs the eight new class bindings locally, including the base reference-surface branch; global registrations remain unchanged. Staged defaults remain false. The public `read_supported` and `documented_bytes` paths enable the extension, still requiring complete inherited context, declared roots, eligible MBSM and exact EOF. Set `mid_surfaces=False, reference_curves=False, reference_surfaces=False, geometric_dependencies=False, backup_dependencies=False, sketch_dimension_dependencies=False, isocurve_dependencies=False, weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` to replay edition 0.356.

The targeted installed sources reach exact EOF: `pressurevessel.sldprt` (26,257 bytes, 23 roots), `sheet.sldprt` (191,418 bytes, 35 roots), `frequency_11.sldprt` (38,265 bytes, 23 roots), and `static_18.sldprt` (14,855 bytes, 23 roots), under the Simulation Examples sample directory. They contain **189** new helper objects: three mid-surfaces, three base reference surfaces, two mid-reference surfaces, two data objects, 14 face pairs, 14 face/heuristic objects, 51 edge pairs and 100 modified-surface ID representations. The keeper arrays contain 3/8 and 11/43 objects. Both extra mid-reference body flags and both pairs of face arrays are empty. All three base surfaces have backend marker 1 and one external sheet. A fifth completion, `Verification/nonlinear_static_12.sldprt` Config-0, contributes an 8,219-byte stream through the base-surface dependency. Positive extra mid-reference kernel bodies are native-proven but not source-observed; focused tests exercise the existing suppressed kernel-body form, without claiming every kernel representation.

`mid_surfaces_evidence.py` binds module hashes, RTTI, full chained native ranges, imports, constructors, predicate bytes and decompilations in `mid-surfaces-native-evidence.json`. `mid_surfaces.py` under research retains source hashes, object summaries and EOF endpoints in `mid-surfaces-corpus.json`. The full `resolved-mid-surface-audit-0.357.json` reaches **856 exact-EOF streams / 50 explicit stops / zero context stops**, with **25,912 completed roots** across 906 modern configurations. All 851 preceding positive streams preserve exact endpoints, root counts and unknown totals. Completed streams contain **72,872,640 decoded bytes**, comprising **25,514,409 unknown** and **47,358,231 known structure bytes**.

The stored regrade adds **27,730 known part bytes**, taking cumulative resolved-stream credit to **13,603,865** and leaving **11,533,601 unknown** in the unchanged 25,137,466-byte family denominator. Total part unknown bytes are **54,763,767**; measured part coverage is **75.9% / 70.7%**. Assembly and drawing measures remain unchanged. `resolved_coverage.py --surface-dependencies --remaining-planes --mid-surfaces` writes `resolved-feature-coverage-0.357.json`; `resolved_coverage_evidence.py --mid-surfaces` rechecks all 1,166 source rows and stored proportions in `mid-surface-feature-coverage-native-source-evidence.json`. Frozen preceding gap reports preserve historical replay. Raw meanings, failed streams, complete document schemas and all installed project formats remain unfinished.

Validation: all **1,107 regression tests pass** (163.455 seconds); specification lint reports zero findings (561 headings, 96 anchors). Three focused tests check source EOFs and object counts, local registration, shared pointers, direct keeper framing, optional kernel boundaries, external sheet framing, truncation and following sentinels.

Evidence: C-379.

### 11.191 Thin extrusions, specifications and bend-ID triples

*Status: B / ? — native inheritance, six populated thin extrusions/specifications and direct bend records verified; option and thickness-control semantics remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moExtruThin_c primary table `0x4CF480A8`, complete-object locator `0x4D32F118` and serializer slot +28 select `0x4BB24DC0`. Native RTTI establishes moExtrusion_c ancestry; the serializer first calls existing complete extrusion helper `0x4BB25180`. Its inherited body therefore includes the ordinary body-feature layers and extrusion suffix, followed by:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | thin specification +860 | tagged moThinSpec_c pointer | always | Supported reader requires a nonnull moThinSpec_c. Native continuation dereferences this object when assigning dimension owners. | B |
| 2 | bend-record count | u16 | always in scope | A fixed unsigned short, **not ReadCount**: 0xFFFF denotes 65,535 records, not an escape to a wider count. Historical load begins after `0x1DA`. | V |
| 3 | bend-ID records +858 | count direct triples of u32 | count nonzero | Each record contains exactly three u32 values, 12 wire bytes, through native helper `0x4BB20BF0`. Preserve duplicates/order and all bits. No per-record tags or counts. | V / ? |

Although the runtime collection is suDynamicArray<moBendIdData_c>, this serializer writes only the u16 count and fixed triples. Generic dynamic-array width/capacity/growth metadata is absent. Readers SHALL bound count against the remaining bytes before allocating/reading. With the latest-version 265-byte empty ordinary extrusion body, a cached nonnull thin-spec pointer and zero records, the synthetic complete body is 269 bytes before outer tagged framing. A newly tagged specification adds its own object/class framing and body. This fixture proves composition, not the engineering validity of an empty extrusion.

moThinSpec_c primary table `0x4D203A90`, locator `0x4D36DBF8` and serializer slot +28 select `0x4BB367D0`. After its zero-byte ordinary su_CObject layer it reads:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | dimensions +08/+10 | two tagged moDisplayDim_c-family pointers | always in scope | Read both pointers independently of the later raw option words. | B |
| 2 | CapEnds +2C; EndThickness +30; raw +38 | i32; two f64 | always | Preserve the exact non-offset wire order and both doubles. | V / ? |
| 3 | Direction +28; raw +40 | two i32 | always | No implicit dimension omission is inferred from these values. | V / ? |
| 4 | radial dimension +48 | tagged moDisplayRadialDim_c pointer | always in scope | Historical temporary-double representation before `0x1DA` is excluded. | B |
| 5 | AddBends +50; raw +54 | two i32 | always in scope | Older temporary-word/migration branches precede the modern minimum. | V / ? |

Exact own moThinSpec_c accessors name four existing raw scalar fields and three dimension associations. Folded implementation addresses are joined through their actual ThinSpec declaring-class signatures rather than another class's field names.

| ThinSpec member | Native property | Qualified API/wire association |
|---|---|---|
| 0x28 | Direction | Full 32-bit integer getter/setter; signed32 archive word |
| 0x2C | CapEnds | Full 32-bit integer getter/setter; signed32 archive word without Boolean normalization |
| 0x30 | EndThickness | Raw binary64 getter/setter and archive value |
| 0x50 | AddBends | Full 32-bit integer getter/setter; signed32 archive word |
| 0x08 | ThicknessDim / Thickness | Typed dimension-pointer getter; numeric getter/setter delegate to its current provider |
| 0x10 | Thickness2 | Numeric getter/setter delegate to this current tagged dimension |
| 0x48 | BendRadiusDim / BendRadius | Typed radial-dimension pointer getter; numeric getter/setter delegate to the current provider |

All three numeric dimension getters forward to current virtual slot 0x868 with EDX=0, without null guards. BendRadius and Thickness2 setters forward the original input double through slot 0x860 with R8D=1 and R9D=0. Thickness saves its original dimension 0x08 pointer and input, performs conditional opaque provider work, then forwards that saved input through the saved pointer's slot 0x860 with the same arguments. It does not promise that a separately re-read 0x08 or final effective value is unchanged. These are dimension-backed numeric requests, not extra raw Thickness/Thickness2/BendRadius doubles in the wire body.

Direction, CapEnds and AddBends are full-width integer names, not Boolean/enum-domain restrictions. EndThickness is a separate directly stored double; no units, positivity, finite range, ordering or successful thin-feature geometry is inferred. Raw 0x38/0x40/0x54 remain unnamed. Normal ABI, pointer validity and lifetime remain prerequisites; opaque calls can change other state. The existing old 0x48 allocated-double path and modern tagged-pointer path are distinct; the new role joins do not admit older contexts or assume one immutable metadata query. The LOAD rules are verified using the retained 189-byte native serializer prefix joined to only its missing 629-byte tail, covering all five chained fragments of the 818-byte routine. This adds native evidence, not wire bytes, and leaves offline values and parsing unchanged. Evidence: C-3547.

The null-dimension thin-spec body is **42 bytes**, including three two-byte null tags. Cached dimensions use the same tag widths; new dimensions add their own bodies. Native load assigns feature owners and may migrate older geometry after reading; offline parsing retains the stored fields and pointer identities without executing those operations.

The audit retains **six complete thin extrusions and six thin specifications**. Two extrusions contain three bend triples each, in order [2,3,5], [1,2,6], [3,4,7]; four contain no triples. Every specification stores two nonnull moDisplayDistanceDim_c pointers and one nonnull moDisplayRadialDim_c pointer. Raw +28 is zero in three and one in three; +50 is one in two and zero in four. All other raw integer members are zero. Raw +30 is 0.001 in four and 0.01 in two; the meaning/domain of +38 remains unresolved, and its stored double is retained without plausibility filtering.

Evidence: C-219.
