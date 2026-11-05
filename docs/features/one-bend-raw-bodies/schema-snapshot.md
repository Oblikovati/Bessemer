### 11.243 Edge flanges and nested sheet-metal records

*Status: B / ? — seven native type bindings and 11 populated containing edge-flange roots verified; raw engineering domains, positive single-bend raw-kernel branches and other sheet-metal classes remain partial.*

The supported context is an ordinary external-XML modern model archive, file version greater than `0x27CB`, with sheet-metal helper state `+0x850` equal to 0 or 1. Unsupported helper states stop. The following runtime offsets describe object ownership; they are not file offsets.

`moEdgeFlange_c` selects `0x3DB5F3D0` in the hash-bound installed `sldsmmu.dll`. It calls established bend-group serializer `0x3DB68EC0`: body-feature base, common bend data, three i32 words at `+0x840,+0x844,+0x848`, direct object array `+0x858`, and u32 `+0x878`. It then serializes one **direct owned `moEdgeFlangeData_c` body**, associated with pointer member `+0x880`, followed by direct `suObArray` `+0xAF8`. A final i32 at `+0xB38` loads only for version `>=0x372F`; older accepted versions initialize it to 0 without consuming a word. The array's older gate (`>0xB3A`) is satisfied throughout the supported range.

Constructor `0x3D8BF0E0` proves the `+0xAF8` array type. It also constructs an embedded data copy at `+0x888` and another array at `+0xB18`; the selected serializer does not append either of these as another body. Loading allocates the owned `+0x880` data object and calls its virtual serializer directly, so no tagged data-object header or map slot appears at that point. Feature flag updates, legacy sketch-constraint removal and data flags `+0xB0,+0xB4` are runtime processing, not extra fields.

`moEdgeFlangeData_c` selects `0x3DB5E4A0`. Its inherited `moFeatureData_c` serializer `0x4B50C220` stores a tagged feature-owner pointer at `+0x08`; native loading checks feature ancestry. The modern direct data body then follows this order:

1. Tagged `moSheetMetal_c` pointer at `+0x18` only when helper `+0x850` is 0.
2. Tagged distance dimension `+0xB8`, i32 `+0xC0`, then binary64 values `+0x28,+0x38,+0x40,+0x48`.
3. i32 words `+0x64,+0x6C,+0x70,+0x74,+0x7C`; tagged wrapper `+0x88`; tagged point reference `+0x98`; i32 `+0x80`; tagged wrapper `+0x90`; tagged point reference `+0xA0`.
4. i32 words `+0xAC,+0xA8,+0xC4,+0xC8,+0x68`; tagged distance dimension `+0x138`; tagged wrapper `+0x140`; i32 words `+0x148,+0x60,+0x50`.
5. i32 `+0x78` only for version `>0x2F5F` (otherwise loaded runtime value 0), then direct `suObArray` `+0x170`.
6. Tagged distance dimension `+0x198`; i32 words `+0x168,+0x58,+0x54,+0x204`; i32 `+0x240` only for version `>0x3356`.
7. Tagged wrapper `+0xD0`; i32 words `+0xD8,+0xE8`; binary64 `+0xE0`; i32 words `+0x190,+0x268`; finally i32 `+0x26C` only for version `>0x42CB`.

Exact own moEdgeFlange_c::getData supplies the typed current member 0x880 used by this feature's direct data serialization. Separate canonical feature/data class identities and complete native serializer routines bind nine new property roles in moEdgeFlangeData_c; shared accessor addresses alone do not transfer Hem or other class meanings.

| Data member | Native property | Qualified representation |
|---|---|---|
| 0x148 | FaceRefForFlangeAngleType | Enum-typed full32 APIs and signed32 wire word; numeric labels remain open |
| 0x60 | UseSMEnv | Raw full32 getter/setter and signed32 wire word |
| 0x58 | CylBase / setCylBaseFlag | Raw full32 getter/setter and signed32 wire word |
| 0x54 | LoftedBase / setLoftedBaseFlag | Raw full32 getter/setter and signed32 wire word |
| 0x78 | OnNonPlanarFace | Raw full32 getter/setter and signed32 wire word |
| 0xD8 | LockAngle | Raw full32 getter/setter and signed32 wire word |
| 0x48 | ReliefDepth | Raw binary64 setter and existing double wire codec; no getter asserted |
| 0x38 | ReliefRatio | Raw binary64 setter and existing double wire codec; no getter asserted |
| 0x40 | ReliefWidth | Raw binary64 setter and existing double wire codec; no getter asserted |

The guard for 0x58/0x54 queries current unsigned effective version at least 0xC84 BEFORE testing archive mode, and therefore controls BOTH STORE and LOAD. The analogous common guard for LockAngle 0xD8 is at least 0x10C6. These are not LOAD-only tests. By contrast, selected STORE writes UseSMEnv and OnNonPlanarFace without their corresponding LOAD tests: LOAD requires 0xB46 for 0x60 and 0x2F60 for 0x78; the reached older 0x78 branch writes zero. FaceRefForFlangeAngleType reads its word at a current LOAD query at least 0xB3B. These independent native checkpoints do not admit additional reader contexts or guarantee entire-object reset/preservation through callbacks.

VertexNormalToFlange and VertexParallelToBase methods bind runtime 0xB0/0xB4, but establish no additional direct saved fields here. This is not a universal nonpersistence claim. Full raw words and binary64 bits remain preserved offline. Names do not prove Boolean domains, actual cylinder/loft geometry, enforced angle constraints, physical units, positive/finite relief values or final feature success. Evidence: C-3566.

All lower historical gates and temporary compatibility arrays in this serializer precede the supported modern range. Its post-load updates do not consume additional fields. Constructor `0x3D8BEE10` proves the `+0x170` array type; other constructor-created arrays/maps are not additional modern wire fields. With empty direct arrays and null/mapped pointers, the data body is 156 bytes before the three late i32 gates, plus four bytes for each satisfied gate; helper state 1 omits the two-byte sheet-metal pointer. At version 19,000 this gives 168 bytes for helper state 0 or 166 for state 1. Native feature validity and engineering option domains remain incompletely modeled; the decoder retains raw values.

`moMultiEdgeFlangeDataPerEdge_c` selects `0x3DB64240`. After the empty object base it stores tagged profile `+0x08`, tagged component edge `+0x10`, i32 `+0x20` and tagged `moEdgeGeom_c` pointer `+0x70`. For version `>=0x42CC`, it also stores i32 `+0xC8`; any nonzero value adds tagged distance dimension `+0xD0`. The older geometry gate (`>0xBAD`) is satisfied throughout the supported context. With mapped/null pointers this body occupies 10 bytes before the late gate, 14 bytes with a zero late flag, or 16 with a nonzero flag and mapped/null dimension.

`moEdgeGeom_c` selects `0x3DB5F760`: i32 `+0x08`, direct vectors `+0x10,+0x28`, then direct points `+0x40,+0x58,+0x70`. Each vector or point has three binary64 components. The body is exactly **124 bytes**, with no count or presence word. `moLinearEdgeGeom_c` selects the five-byte leaf `0x3DB63370`, whose exact bytes `E9 EB C3 FF FF` jump to `0x3DB5F760`; it adds no suffix. The leaf has no unwind-table entry. Raw components and scalar bits are preserved without inferring units or geometric validity.

`moAbsoluteRefPlnData_c` selects existing plane serializer `0x4C283D30`. Its native `+0x150` data hook is `0x4C2880A0`, and its verified ancestry excludes `moDefaultRefPlnData_c`; therefore the default-plane-only display-root point is absent. The existing ordinary `moRefPlnData_c` layout and its version gates apply without an absolute-plane suffix.

`moOneBend_c` selects `0x3DB64540`, calling established common sheet-metal serializer `0x3DB68FB0` (body-feature base, common bend data and three i32 words `+0x840,+0x844,+0x848`). Its supported modern body continues with:

1. Generic tagged sketch associated with runtime handle `+0x878`, generic tagged boundary reference `+0x860`, tagged angular dimension `+0x868`, tagged dimension `+0x870`, u16 `+0x858`, i32 `+0x8F8`, binary64 `+0x8B8`.
2. i32 words `+0x8C0,+0x8C8,+0x8CC,+0x8C4,+0x8D0`; tagged dimensions `+0x8D8,+0x8E0`; i32 `+0x8E8`; u8 `+0xA10`.
3. Only for helper state 0: a generic tagged sketch associated with handle `+0x880`; if nonnull, its direct line-handle body; then tagged `suObList` `+0x888`. The direct handle uses the established entity-handle serializer and adds no object-map slot.
4. Generic tagged reference `+0xA90`, tagged component-feature references `+0xAA0,+0xAA8`, i32 `+0xAC8`, tagged conic-bend data `+0xAD0`, two direct compact `mgXform_c` bodies at `+0x900,+0x988`, then i32 words `+0xAD8,+0xADC,+0xA30,+0xB64,+0xA34`.
5. Generic tagged reference `+0xA98`, tagged component-feature references `+0xAB0,+0xAB8`, then the direct line-handle body associated with `+0x878` if its earlier sketch pointer was nonnull.
6. i32 declared raw-body count for `+0xAE0`; a second i32 count for `+0xB00` only for version `>0x335A`. Native loops read bodies only for positive counts; negative counts are retained as nonpositive declarations. Positive counts enter `mo_loadBodyFromArchiveRaw` and currently stop explicitly before any raw body is accepted.
7. i32 optional raw-body presence at `+0xB40` for version `>=0x334C`, then at `+0xB50` for version `>=0x334F`. Any nonzero presence requires a raw kernel body and currently stops explicitly.
8. i32 `+0xA88` only for version `>0x3734`, then i32 words `+0xA58,+0xA5C,+0xD54`. Lower legacy compatibility words and the helper-specific pre-`0x141E` conversion are outside the supported range.

Exact own moOneBend_c methods add eight saved associations, distinct from the unjoined runtime API mappings listed below:

| Member | Native property | Qualified association |
|---|---|---|
| 0xAC8 | GeomType | Enum-typed raw full32 getter/setter and signed32 wire word |
| 0xA30 | IsCreateSimpleBendGeometry | Raw full32 getter/setter and signed32 wire word |
| 0x8D0 | ReliefType | Enum-typed full32 setter only and signed32 wire word |
| 0x8D8 | ReliefWidth | Existing tagged display-dimension provider, setter request below |
| 0x8E0 | ReliefOffset | Existing tagged display-dimension provider, setter request below |
| 0x804 = inline 0x800+0x04 | UseDefaultRad | Own full32 setter only and existing direct common-bend signed32 field |
| 0x8E8 | UseDefaultRelief | Own full32 setter only and signed32 wire word |
| 0x868 | AngleDim / selected getAngle query | Declared angular-dimension pointer getter and existing tagged codec |

getAngle first accepts runtime override 0xA18 only when current 0xA28 is nonzero and the override compares ordered at least positive zero. Otherwise it queries nonnull current 0x868 through virtual slot 0x868(0), or returns positive zero when null. Unordered override comparisons fall back on nontrapping completion; positive infinity is not excluded by that comparison. This does not create a saved nonnegative/finite angle field. The own public setAngle(double,double*) is a five-byte tail to iSetAngle(double,double*). That method loads current AngleDim member 0x868 and returns when it is null; otherwise it tail-dispatches to imported moDisplayDim_c::setDimVal(double,double*) on that current receiver. XMM1 and R8 are carried unchanged through these bodies. The public tail does not initialize R8; it forwards the incoming pointer argument: the pointer argument's direction, use, validity and expected value are unproved. This is a guarded dimension request, not a direct numeric write, allocation, physical-unit claim or guarantee of final angle/geometry. Evidence: C-3583.

ReliefWidth/Offset setters can allocate a missing GENERIC moDisplayDim_c, passing the result of moScalerDim_c::GetThisClass and the original double to its constructor. They do not call a distance-dimension constructor. Subsequent safeSetFeatureOwner and conditional model/configuration-manager rebuild requests remain opaque. On normal continuation the setter RELOADS the current display pointer and performs unguarded virtual slot 0x860(original input,1,0). Allocation, callbacks and current identity therefore carry no safe-null or final-value guarantee; no physical unit or actual model-dimension class is inferred solely from the constructor request.

The relief STORE group (0x8D0/0x8D8/0x8E0/0x8E8) has an unsigned current-version query GREATER than 0x3F0. LOAD instead requires its independently queried group version at least 0x423, with a further 0x426 test for UseDefaultRelief and a reached older literal-one branch. These guards are not symmetric. GeomType LOAD requires a current unsigned query at least 0x7D0, otherwise its reached branch assigns zero; IsCreateSimpleBendGeometry uses a separate current unsigned query at least 0x7FC without a new skipped-read reset claim. Angular pointer 0x868 has its existing 0x1DA modern-pointer versus older scalar compatibility boundary. No additional reader context is admitted.

ReliefRatio 0x8F0, UseReliefRatio 0x8EC, TempAngle 0xA18, TempRadius 0xA20 and IsFixedFaceRequired 0xAC0 remain runtime API mappings without direct selected codec joins. AttributeIndex projects a virtual result equal to 15, rather than naming a saved scalar. Raw archive values, residual fields, numeric domains and final geometry remain qualified. Evidence: C-3575.

The actual canonical OneBend primary slot 0x2660 selects named getType. Its complete 83-byte routine gives saved member 0xA10 a scoped consumer role. The routine reads its current FULL32 word; values other than 3 are returned unchanged. For value 3, current reference 0x860 null returns 2; otherwise it passes that reference and the moCompFace_c runtime-class descriptor to an opaque helper, projecting the helper's nonzero result to return 1 and zero result to return 0.

The wire representation remains one unsigned byte: selected STORE writes the current low byte within its independent unsigned version-greater-than-0x3F0 group, while LOAD reads from its separate query at least 0x674 and zero-extends the byte into the runtime32 member. This is not a full32 wire field or a guarantee that getType equals an earlier stored byte. The declared type_e name supplies no engineering labels for numeric codes. The helper's class-test semantics, dynamic references, separate AttributeIndex wrapper and raw 0xD54 remain outside this proof. Evidence: C-3577.

moConicBendData_c now has a static ordinary non-DB grammar. Zero-offset primary table 0x3DC610E0, locator 0x3DC76C20 and complete 256-byte native serializer 0x3DB5D940 bind the class; installed runtime-class metadata advertises versionable schema 1. Existing OneBend tagged member 0xAD0 uses typed operators for this class. Outer MFC/context admission remains as previously scoped.

The body is exactly 72 bytes when both shared save-provider IsDBStream queries select zero, with the verified ordinary point/vector input providers:

| Wire offset | Runtime member | Field | Wire representation |
|---|---|---|---|
| 0 | 0x08 | Point | Three binary64 components, 24 bytes |
| 24 | 0x20 | Vector | Three binary64 components, 24 bytes |
| 48 | 0x38 | Raw scalar | f64 |
| 56 | 0x40 | Raw scalar | f64 |
| 64 | 0x48 | Raw byte-backed state | Full signed32 word |
| 68 | 0x49 | Raw byte-backed state | Full signed32 word |

STORE zero-extends each current runtime byte to signed32 without Boolean normalization, so its local emitted range is 0..255. LOAD instead reads the entire signed32 word and sets the corresponding runtime byte to 0/1 by a nonzero test. Offline readers retain the original signed32 file values; the runtime byte offsets are not overlapping file offsets or valid-file Boolean restrictions. The class routine adds no base-object body, own version test or optional tail.

Point/vector coordinate frames, scalar meanings and the two states' engineering roles remain open. The DB save branch, external fresh-object admission/factory behavior and populated native fixture coverage are not established by this static ordinary grammar. No production-reader dispatch or corpus endpoint is changed here. Evidence: C-3579.

Eleven complete containing edge-flange roots total **367,477 bytes** across five configurations. Within these completed roots, fresh-object counts are **19 each** for single bends, per-edge data, linear geometry and absolute-plane data. Every observed single-bend raw-body count and presence is zero. Exact source/root hashes, independent data/feature array counts, raw data fields and nested counts are retained. Native positive raw-kernel branches and populated conic-bend fixture coverage remain open; the static ordinary conic body is specified above.

The 906-context audit now contains **25,413 complete roots**, **816 complete lists** and **90 explicit stops**, with all earlier complete root records unchanged. This adds 34 roots across six configurations, including one standalone single-bend root in sheet.sldprt. No additional list reaches EOF; totals remain **738 exact-EOF lists / 60,010,312 decoded bytes**, with **78 complete lists retaining four-byte unknown tails**. Next stops include `moUiBend_c`, `moSM3dBend_c`, `moCornerFeat_c` and `moFlattenBends_c`. Checkpoints `resolved-feature-list-frontier-0.262.json` and `resolved-feature-list-frontier-0.263.json` retain the comparison. These framing results do not establish complete semantic document/project bit coverage.

Six focused tests verify geometry order and binary64 bits, per-edge flag/version boundaries, direct-data minimum sizes and helper states, all single-bend raw-tail gate boundaries and unsupported-body stops, direct feature composition and the final-word gate, plus all 11 hash-bound native containing roots and next stops. Complete engineering semantics, historical/DB contexts, further sheet-metal feature types and full document/project coverage remain unresolved.

Validation: all **886 regression tests pass** (104.144 seconds); specification lint reports zero findings (467 headings, 96 anchors).

Evidence: C-285.
