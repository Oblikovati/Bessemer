### 11.1756 Current hatch Angle rotation, composition and radian-input consumer

*Status: exact typed rotation/composition and initial CRT argument flow verified · Evidence: C-3441*

The current gcXhatch_c Angle field10 has a consumer in the same qualified style-copy/face-attribute chain used for Scale in11.1754. Exact helper43AEBD70 obtains the current payload and supplies Angle as the first explicit double to named mg_RotateXform, with a constructed axis vector(0,0,1) and final scalar positive zero. The resulting rotation is the left C++ operator* operand and the inverse of a restored caller transform is the right operand. This establishes operand provenance and order; it does not assign a universal coordinate frame or point-application convention to every transform.

The retained mg_RotateXform provider32224670 and inner3212CD80 bind the initial angle argument further. On the reached gated path, current Angle moves through the preserved double registers into the actual CRT cos/sin argument XMM0 without a degree conversion or other intervening angle arithmetic. Microsoft specifies the CRT sin parameter as an angle in radians. Joined with the exact native argument flow, this establishes a radian-input consumer for current Angle on this path. [Microsoft CRT sin contract](https://learn.microsoft.com/en-us/cpp/c-runtime-library/reference/sin-sinf-sinl?view=msvc-170).

This is not a closed range, finite-only policy, file-value normalization or proof that every stored origin reaches those calls unchanged. Axis/angle threshold and validity gates can bypass the trigonometric path. Later mutable thresholds and snapping branches remain separate. Floating-point rounding, exceptions, nonfinite inputs, actual providers and normal ABI behavior retain their qualifications.

As with Scale, the shared attribute-definition key slot is writable, and the payload is obtained through a pointer-valued provider. Matching current key/payload type, allocation, accessible buffers, alias/lifetime stability and successful normal calls are prerequisites; identical key-slot addresses are not a constant-value or runtime RTTI guarantee. Current Angle can differ from archived bits through other operations. The offline schema preserves the raw f64. Native bodies and two independent joins establish this scoped operation/unit role without source/container/baseline replay, frozen-proof rerun, CAD/COM/native execution, production parser/accounting changes, new population admission or byte/global coverage credit.

### 11.1758 Hatch-data DB STORE point-property construction

*Status: exact DB-only coordinate extraction and typed property insertion verified; final DB encoding remains open · Evidence: C-3443*

The skHatchData_c serializer4C5AA190 has additional work after its point-block and kernel-body STORE described in11.1755. It enters this property path only on STORE with IsDBStream nonzero; ordinary DB0 skips it. Under valid stable point-array and allocation/provider conditions, the code obtains the current embedded array size, initially sizes a temporary double vector from a native32 calculation3*N, then copies three ordered64-bit components from each current24-byte point record into that vector.

The sizing instructions truncate through EAX and sign-extend afterward, while the copy loop queries the current array size again. They do not prove safe mathematical3*N bounds or a coherent immutable snapshot for arbitrary counts, overflow, callbacks and concurrent changes. The variable stride accepted by the wire codec in11.1755 does not itself prove every loaded backing is a valid stable24-byte runtime point sequence.

Exact helper4C5849E0 traverses the temporary double vector in8-byte steps, constructs a typed msgpack11::MsgPack(double) value per element into its temporary value vector, then calls the MsgPack(vector<MsgPack>&&) constructor. The serializer builds a local string from the observed narrow ASCII literal `XHatchPoints` through imported MFC ordinal290 and passes that string and the constructed value to exact su_CDBArchive::insertMsgPack. Imported CString/provider ABI validity and normal successful construction remain prerequisites; arbitrary provider substitution or runtime DLL-loading identity is not established by the import name alone.

This identifies a typed DB property construction path, not an additional ordinary raw-stream suffix or a prescribed final MessagePack byte stream. The final DB property/container encoding, later persistence and matching LOAD consumer remain outside this proof. Typed double constructors can have their own nonfinite/numerical policy; no bit-identical final DB representation, coordinate units/frame or valid geometry is inferred.

The canonical class, current archive and working-buffer relationships are source-bound to11.1755. All raw ordinary fields, production readers, historical charges and counters remain unchanged. No source/container/baseline replay, frozen-proof rerun, CAD/COM/native execution, class/body/counter population admission or byte/global coverage credit is claimed.

### 11.1759 Hatch Override and RandomizeScale definition-adjustment branches

*Status: exact current field predicates and selected definition arithmetic verified; known outer caller bypass retained · Evidence: C-3444*

The gcXhatch_c fields Override34 and RandomizeScale30 have conditional consumers in the qualified current style/definition path. When the relevant outer helper's incoming selector is not-1, full current Override equal1 skips the current definition's processing to loop cleanup/increment. Exact2 bypasses the adjustment phase. Other full32 values can call named gcFace_w::getHatchParametersForDrawing, passing full RandomizeScale as its fourth integer argument, the current definition, an angle-output pointer and a null final pointer. No Boolean or closed enum filter is added to the stored signed words.

Within the selected helper mode1/non-null-angle-output branch, numerical case3 uses RandomizeScale nonzero to halve the current definition increment and its working start-location vector before the typed start-location update. In numerical case5, nonzero divides the increment by1.5; that case's working vector multiplication by the binary64 approximation to2/3 is independent of the RandomizeScale zero/nonzero test. These are exact branch and arithmetic roles, not a promise of stochastic behavior, physical spacing, normalized values or named enum semantics for cases3/5.

The concrete retained updateFaceHatch caller passes incoming selector-1 and therefore bypasses this Override/RandomizeScale phase. The source-bound conditional operation is not a newly observed part-file occurrence or a claim every hatch update executes it. Selected writes affect the current definition/working vectors, not an established assignment to original style Scale18 or a final geometry invariant.

Current typed payload identity, the same writable attribute-key value across reached operations, valid definitions/buffers, normal providers, aliases, allocation and object lifetime remain prerequisites. Floating-point rounding, nonfinite values and exception paths remain qualified. Other helper modes/cases and wider renderer lifecycles are not reconstructed. Raw stored words and existing serializer LOAD policies remain separate; no source/container/baseline replay, frozen-proof rerun, CAD/COM/native execution, production parser/accounting change, population admission or byte/global coverage credit is claimed.

### 11.1761 Hatch-data dynamic point-block allocation-growth metadata

*Status: actual point-codec wire/statistic/provider mapping joined to installed header terminology · Evidence: C-3446*

The metadataA/metadataB words of the nonzero-stride hatch-data point block in11.1755 have qualified allocation-request roles. MetadataA is the growth-increment request called growByCount in the installed darray.h template; metadataB is the growth-factor request called growByMultiple. They are separate from element count, record stride, final capacity and geometry/point classification.

This correspondence is verified for the actual4BA5AF00 codec, rather than transferred by template name or from the intersection array. STORE passes the backing pointer to exact suDynamicGetStats with first/second outputs that become metadataA/B and a third output that becomes stride. The raw save order is stride,A,B,count. LOAD reads A into the slot forwarded to EDX, B into R8D, count into ECX and stride into R9D for exact suDynArrayAllocSpecial. Installed darray.h uses those same statistic outputs, wire order and allocation argument positions; dynmem.h's release macro forwards them in that order to the declared four-int provider.

All wire words remain complete unsigned u32 values. The native provider's signed32 argument views preserve the same bits; they are not offline value-domain filters or proof of a particular capacity/clamping/growth formula. STORE's metadataB1/2 guard remains distinct from the selected LOAD path, which does not impose that same closed check. No additional nonnegative-A rule is inferred from an unsigned source assertion.

Stride0 carries no metadata. A nonzero stride with zero count still carries both metadata words and the count, independently of payload length. Variable stride, native32 count multiplication, backing mutation, partial reads and allocation failure retain the codec's existing limits. Earlier width2/width4 allocator roles are reconciled, not transferred as hatch source occurrences or byte credit. No allocator simulation, class/body/counter admission, source/container/baseline replay, frozen-proof rerun, CAD/COM/native execution, production parser/accounting change or global coverage gain is claimed.

### 11.1762 Hatch Solid current selector and gated coedge-boundary operation

*Status: canonical saved-member binding and selected current operation verified; caller exclusion retained · Evidence: C-3447*

The canonical gcXhatch_c signed fields are Solid at runtime20, ID24, LayerID28 and CmdLayerColor2C, not those offsets shifted by four. The saved Solid word has a direct current consumer in the qualified face-hatch helper. Full current Solid20 equal0 selects definition processing; every nonzero value selects a separate boundary-processing branch. This is a zero/nonzero predicate, not a closed Boolean file domain.

The nonzero branch additionally requires a current nonnull bucket and nonzero bucket member758. With valid returned counts and buffers it obtains the face loops through named gcFace_w::getLoops, obtains coedges of each nonnull loop through gcLoop_w::getCoEdges, and passes the selected current coedges, bucket and working transform to named gcFace_w::addEdgeToBoundary. Bucket758 remains a foreign operation gate without an inferred flag name. Named boundary processing does not establish solid-fill rendering, winding, topology, refresh, successful geometry or a final ownership/lifecycle invariant.

The concrete updateFaceHatch entry separately rejects a null style or input Solid20 nonzero with return0 before copying the style or invoking this helper. Thus its ordinary zero-Solid route does not demonstrate occurrence of the helper's nonzero branch. The conditional operation remains qualified by the current typed style payload, stable value of the writable attribute-key slot, current object/definition identity, valid arrays/coedges/transform, normal providers and callbacks. No unchanged loaded-object value, universal reachability, all-input safety or callback preservation is inferred. ID24, LayerID28 and CmdLayerColor2C receive no new operational meaning here. Stored raw values, population, historical charges and production parser/accounting remain unchanged; no data replay, native execution or coverage credit is claimed.

### 11.1764 Slot Type and exact LengthType operation selectors

*Status: canonical saved fullword fields joined to selected typed slot-data arithmetic · Evidence: C-3449*

Current canonical sgSlot saved signed32 TypeC0 and LengthTypeC4 are copied to the typed sgSlotData fields8/C. In the selected getSlotData operation, Type equal0 selects endpoint distance on the first handle. Nonzero Type selects the first handle's radius multiplied by named mg_AngleBetween applied to endpoint-minus-center vectors and the current normal. This is the installed numerical operation; it does not label every nonzero Type with a guessed enum or assert angle units, physical frame or valid geometry for arbitrary handles.

The operation obtains data28 as twice the fifth handle's getRadius result. In either selected Type branch, exact LengthType equal1 adds that data28 value to the result at data30; every other signed32 value omits that add. Named setSlotLengthType writes the complete incoming32 bits to currentC4. Exact1 is not interchangeable with generic nonzero, and neither branch supplies a closed stored enum or normalization rule. The named caller-supplied conversion methods in11.1768 require exact Type1, unlike getSlotData Type nonzero; no actual callsite or roundtrip guarantee is inferred.

A current canonical resolved slot and six valid correctly typed handles are preconditions, not a new six-nonnull file invariant or a universal resolution guarantee. The resolver repeatedly queries current sketch/manager providers; callbacks, aliases, archive overrides, object lifetime and possible changes between queries remain qualified. Null/count fallback paths can still be dereferenced or called through, so no invalid-state safety is claimed. Only selected operations under normal returns and valid floating-point inputs are assigned; there is no immutable saved/current identity, global geometric success or final-state guarantee. The existing six tagged handle occurrences and complete raw C0/C4 values remain unchanged. No source/container/baseline replay, native execution, parser/accounting change, population or byte/global coverage credit is claimed.

### 11.1766 Slot creation and ordered current-handle installation

*Status: canonical construction, saved fullword producer and ordered installer verified; final state separate - Evidence: C-3451*

Named addSlot selects the current sketch588 manager and passes the source handle array and typed sgSlotData. The retained manager path constructs a canonical sgSlot object, mutates its counter/map and inserts the object before invoking the installer. The reached installer requires current destination count0 and source count6. It copies complete32-bit data8/C into saved slotC0/C4, then processes source ordinals0 through5 in order.

For each source ordinal the installer calls that source handle's virtual0x128 and supplies the RETURNED pointer to SetAtGrow on the slot's stored C8 array at its current count. This establishes six ordered append requests, not identity with each source pointer, a nonnull copy result, fixed concrete handle categories or proof of a geometric clone. The count and buffer are current observations; callbacks, aliases, allocation and provider behavior can change subsequent state. Malformed null/count fallback paths can still reach unsafe virtual dispatch.

The installer result is ignored after manager insertion and counter mutation. No rollback, atomic operation, allocation safety, returned handle success or final six-valid-object invariant follows. Manager32-bit arithmetic can wrap. Actual reached canonical construction is distinct from arbitrary caller/archive override identity or loaded-object lifetime. These producer/installation facts complement the saved raw field codecs and current numeric selectors in11.1764. The selected current-command local geometry/storage carrier is qualified in11.1772; neither link establishes a storedC8-to-static calculateSlotEntity callsite, cross-event identity or six geometric categories. No new source/container/baseline parse, native execution, parser/accounting change, population, byte credit or phase advance is claimed.

### 11.1768 Named slot-data conversions and exact selector distinctions

*Status: three complete named conversion leaves and current-data predicates verified; actual caller/lifecycle association separate · Evidence: C-3453*

The named sgSlotData conversion methods consume caller-supplied current data. The qualified canonical getSlotData path in11.1764 copies saved full signed32 TypeC0 and LengthTypeC4 into data8/C, but no particular call from that path to these conversion leaves is newly established. Their predicates require exact Type equal1, unlike the broader Type nonzero branch in getSlotData. They do not equate all nonzero stored values with one enum member or establish a closed file domain.

Using current data offsets, convertAngleToLength writes data30=data40*data38, adding data28 only when exact LengthType equal1. It does nothing when Type differs from1. convertLengthToAngle also requires exact Type1: it starts with data30, subtracts data28 only for exact LengthType1, and writes data40=candidate/data38 only when COMISD/JBE admits an ordered candidate greater than zero. The denominator has no selected zero check; no universal finite output, arithmetic inverse or roundtrip guarantee follows.

Named onLengthTypeChanged adds data28 to data30 when LengthType equals1. Otherwise it subtracts data28 and compares the result with the incoming double. For finite ordered inputs, a result below that incoming double is replaced by it; the native store and working XMM2 are both updated before later processing. The actual COMISD/JAE behavior remains the rule for unordered inputs, rather than a universal real-number maximum formula. Subsequent recalculation requires exact Type1; exact LengthType1 subtracts data28 from the working candidate, and only the ordered-positive candidate gate permits division by data38 and assignment to data40.

The complete selected leaves write only their binary64 data30/data40 outputs; they do not assign selector fields8/C. These are named angle/length operations with verified arithmetic, not a new assertion of global physical units, coordinate frames, valid geometry or unchanged loaded/current object identity. Caller data, normal floating-point and exception behavior, nonfinite values, rounding, denormal controls, zero-denominator division and lifetimes remain qualified. There is no observed caller or universal lifecycle guarantee. Raw aliases/domains and the existing six-handle grammar remain unchanged; no source/container/baseline replay, native execution, parser/accounting change, population, coverage credit or phase advance is claimed.

### 11.1771 Hatch LayerID and command-color pre-copy boundary metadata

*Status: typed saved-member names and selected source/destination metadata checkpoints verified; final state separate - Evidence: C-3456*

Current typed gcXhatch LayerID28 and CmdLayerColor2C are fetched through exact named getHatchLayerID/getLineColorCmdColor methods and copied full32 into a current nonnull bucket at840/844. In the separately qualified reached boundary branch, named gcFace_w::addEdgeToBoundary reads those bucket words into nonvolatile integer lanes. Its selected per-record phase writes the same raw values into both source and appended destination gcBucketPoly members60/64, exposed by named getHatchLayerID/getCommandLineColor.

The writes occur before a subsequent typed gcBucketPoly::copy call. Selected later copy metadata assignments are qualified in11.1775; neither checkpoint proves whole-return preservation, refresh/render, ownership or successful geometric effect. The record offset/stride arithmetic requires valid nonoverflow counts and buffers, successful allocation and normal provider/ABI behavior. Same typed current style payload, writable-key value and bucket identity across the selected calls are preconditions, not runtime RTTI checks or callback/loaded-object lifetime invariants.

The outer updateFaceHatch input SolidNZ exclusion remains as qualified in11.1762: its normal path does not demonstrate occurrence of this conditional boundary branch. LayerID and command-color names establish metadata contracts only, without layer existence, visibility, printability, RGB packing, alpha, enum/domain, ID-generation or pixel/render implications. Current styleID24 receives no new operational meaning. Full signed32 raw values, historical scopes and charges are retained; no source/container/baseline parse, native execution, parser/accounting change, measured population, coverage credit or phase advance is claimed.

### 11.1772 Slot-command current carrier and local geometry/storage bridge

*Status: actual selected command callbacks and typed geometry/storage arguments verified; cross-event identity separate · Evidence: C-3457*

The retained selected sketch-command callbacks obtain a current state carrier through command member320 and its accessor. That accessor requests numeric2 from its provider, requires a nonnull result whose virtual88 returns exact2, then returns its member8 payload; the alternative returns null. These are exact current predicates, not a private command-class name inferred from offsets or a closed global provider-tag enum.

The common ordinal builder reads six current pointer fields from that carrier and issues ordered SetAtGrow requests at the destination's current count. In the selected calculation callback, the exact same local array address and exact same local sgSlotData address are supplied first to named calculateSlotData and then to named calculateSlotEntity. This proves within-call argument-address identity. It does not prove immutable array/data contents across either call or continuity across separate callbacks/events.

A selected commit callback forwards the current carrier into the same ordinal builder and passes the resulting local array and data to named addSlot. The canonical model creation/installer chain in11.1766 then performs its separately qualified ordered virtual128-return installation into saved C8. That link does not make returned copy pointers identical to source handles or establish their preserved geometry. No direct stored-C8-to-static-calculation call on arbitrary loaded slots is proved.

In the selected creation callback, named createSlotEntity receives the current seed handle, current double and unit-vector arguments and a local output array. After its nonzero response, returned array ordinals1 through5 are stored into carrier fields8 through28; carrier seed0 is separately held. Selected source-array ordinal types are qualified in11.1777 by seed virtual120 return1/2, not by saved TypeC0 or a universal stored-array profile. A nonnull response ordinal0 receives its selected virtual8 call with EDX1. This records the actual call without assigning a universal ownership/lifecycle label or final-state invariant.

Valid current providers, six handles, data, arrays, normal ABI returns and object lifetimes remain prerequisites. Repeated state accessor calls may yield different carriers; callbacks may change fields, backing arrays or values between operations. No six-nonnull wire invariant, original loaded/current identity, cross-event persistence, virtual-copy geometry, render result, physical unit/frame, closed enum or global geometric success follows. These are current command-operation arguments, not a new direct raw-file consumer or class/version/context admission. Production parser/accounting, historical populations/charges and project phase remain unchanged; no data replay, native execution or coverage credit is claimed.

### 11.1773 Equal-distance point current-state APIs and Processed storage distinction

*Status: canonical raw fullword accessors and local archive policy verified; state lifetime separate - Evidence: C-3458*

Canonical sgEquiDistPoints CurrentEquiDistCons20 and Processed24 have named full signed32 current-state getters/setters: isCurrentEquiDistConstraint/setCurrentEquiDistConstraint and isAlreadyProcessed/setProcessed. The exact leaf operations return or write the complete32 bits without Boolean normalization. These method names do not establish a closed Boolean domain, valid relation/index range or an engineering state machine.

The canonical serializer0x4B98DA10 STORE path emits low16 of the current20 and24 members. Its selected LOAD suffix begins at effective version>=0x1FF6: rawu16 CurrentEquiDistCons is zero-extended into runtime20; the next rawu16 Processed word is consumed into a reused local without a direct assignment to member24; ArrPos28 and MainEnt18 follow. A raw Processed value remains part of byte-faithful offline reconstruction even though this local native LOAD phase does not restore it to the state member.

The local discard does not guarantee that a preexisting Processed value is preserved across the opaque base, provider, handle callbacks or construction. No reset-to-zero, normalization or identity between the archived Processed word and a later runtime getter result is inferred. Below0x1FF6 the selected local suffix is skipped, while STORE lacks that omission gate. Inherited/provider effects and whole old-class/context admission remain separate.

Version fallback, DB/ordinary handle helpers, owner association, current providers, object maps and lifetimes remain qualified. Missing-helper errors and partial reads imply no rollback. This adds no source/container/baseline replay, native execution, parser/accounting change, population admission, coverage credit or phase advance; all historical scopes and raw values remain unchanged.

### 11.1774 Equal-distance OwnerPos current record-position consumer

*Status: canonical saved word and typed current array-index calculation verified; bounds and lifetime separate · Evidence: C-3459*

The current full32 OwnerPosD0 of sgEquiDistPointsCon feeds named getMainEqDistRel together with its currentD8 sketch pointer. The exact sgSketch::getEquiDistArrI helper sign-extends that index, reads the current sketchC8 backing pointer at8, and returns base+72*signedindex. Its two complete selected methods contain no conditional null, negative-index or bounds check. The typed pointer result does not verify object existence or dynamic class.

The separate canonical serializer join stores low16 of D0 and, on the selected LOAD branch from0x1FF6, reads rawu16 and zero-extends it into currentD0. Raw0xFFFF therefore becomes positive65535 at that LOAD checkpoint; later full32 runtime writes can produce negative indices. This is a positional consumer, not a tagged MFC identifier, file-map index, byte offset, unique relation key or valid-index-domain proof. Runtime stride72 is not a wire-body extent.

CurrentD8 must be a valid expected sketch and the array/backing/count arithmetic must address valid storage. Aliases, callbacks, constructor/reused objects, pending overrides, inherited logical-G1/base/provider effects and object lifetimes remain qualified. No unchanged loaded/current value, all-file occurrence, bounds safety, normalization, geometry or final-state guarantee follows. Raw fields, existing admission and accounting remain unchanged; no data replay, native execution or coverage credit is claimed.

### 11.1775 Boundary-poly layer and command-color copy checkpoints

*Status: exact named copy body and selected raw metadata assignments verified; whole-return preservation separate · Evidence: C-3460*

Clause11.1771 records the pre-copy style/bucket/source/destination metadata writes. The actual selected gcBucketPoly::copy body now resolves that boundary: both forward and reverse collection-copy branches converge before the metadata assignments. Original sourceRSI supplies named getHatchLayerID at source60 and getCommandLineColor at source64; their full32 returns overwrite original destinationR14 members60 and64. The callsite passes the selected current source and appended destination records previously marked with the same corresponding metadata.

These are reached copy checkpoints, not an unconditional final-preservation guarantee. Earlier subobject/container operations may change source fields through aliases or callbacks before those getters. Later copy operations allocate/copy other subobjects and call opaque providers after the metadata stores. The direct known-R14 effect audit finds these two overlapping metadata assignments, but does not exclude writes through other aliases or provider calls. Valid current source/destination identity, buffers/counts, allocation and normal ABI behavior remain prerequisites.

No transaction, lifecycle, ownership, geometry, layer existence/visibility, RGB/alpha packing, render result or refresh guarantee follows. The earlier Solid branch and known outer caller exclusion remain qualified. Current styleID24 is not newly assigned a role. Full signed32 raw values, historical scopes, parser/accounting and admission are unchanged; no source/container/baseline replay, native execution, population or byte/global coverage credit is claimed.

### 11.1776 Logical-group keys as current logical-G2 array positions

*Status: stored-key transport joined to exact post-LOAD positional lookup and selected cache branch · Evidence: C-3461*

The post-LOAD callback4C426760 in11.1770 consumes the current sgLogGroupDim receiver. CurrentB0 equal0 returns before its selected B8 runtime-handle-vector clearing. When nonzero, it captures owner88 once before clearing, stores that pointer locally and restores the captured pointer for subsequent lookups; it does not reread owner88 on every iteration. A8 tree node1C signed32 keys are passed to named sgSketch::getLogG2ArrI using that captured expected sketch pointer.

The complete getter reads the supplied sketch's current90 backing pointer at8 and calculates base+0x158*signedindex without local null or bounds guards. This identifies current logical-G2 record positions, not a unique relation ID, tagged object identifier or file offset. Runtime stride0x158 is not wire extent. Current keys reach the tree through the separately qualified raw signed32 LOAD/set insertion, with duplicate inputs still consumed as specified in11.1770.

On the nonzero path the callback clears selected existing B8 handles, then conditionally rebuilds runtime handles using returned current logical-G2 virtual providers and typed handle-copy operations. Repeated virtual queries, allocation, aliases and an opaque tree helper remain boundaries; provider slots are not assigned universal names/types or stable identities here. No complete cache contents, iterator safety, atomicity, preserved geometry or final success is inferred. ZeroB0 bypasses clearing only in this local callback, not across arbitrary inherited/provider/loader effects.

A valid captured sketch owner, current tree/backing/indices, providers and lifetimes are preconditions. The getter's current backing read does not make the captured owner immutable or valid across callbacks. There is no bounds/domain filter, all-loaded-object occurrence, extra wire field, older context admission, source replay, native execution, parser/accounting change, population, coverage credit or phase advance.

### 11.1777 Selected slot creation-source ordinal types

*Status: actual typed constructors, clone results and ordered source-profile requests verified; saved-copy identity separate · Evidence: C-3462*

The selected createSlotEntity branches distinguish exact numeric1 and2 returned by the seed handle's virtual120. This is not the saved slot TypeC0 enum and does not classify arbitrary receivers from their numeric return alone. On valid normal allocated paths, actual typed stack constructors, canonical RTTI/virtual128 targets and allocated clone classes establish the following source-array request order:

| Source ordinal | Seed virtual120 returns1 | Seed virtual120 returns2 |
|---|---|---|
| 0 | seed virtual128 returned pointer | seed virtual128 returned pointer |
| 1 | Point clone result | Point clone result |
| 2 | Line clone result | Arc clone result |
| 3 | Line clone result | Arc clone result |
| 4 | Arc clone result | Arc clone result |
| 5 | Arc clone result | Arc clone result |

These are selected creation-source profiles, not universal geometry categories for all stored slot arrays. The selected kind1 sourceordinal1 midpoint-form point/index producer is qualified in11.1778, without an all-input exact-mean or later clone-geometry guarantee. The separate kind2 named arc-query sourceordinal1 point is qualified in11.1781, without a universal angular-halfpoint or centre equivalence. Allocation failure can return null, and callbacks or overrides can change current state/vptrs. Source ordinal0's returned clone receives the selected command cleanup call; command carrier0 remains the original seed, not identity with that returned clone. Ordinals1 through5 populate the current carrier as qualified in11.1772.

The common ordinal builder and model installer in11.1766/11.1772 subsequently act on that current carrier. Saved C8 receives later virtual128 RETURNED pointers, whose identity, dynamic class and geometry are not proved for arbitrary overrides. No immutable cross-event carrier, six-valid-handle file invariant, all-loaded-slot profile, physical frame/units, radii/centre/normal/endpoint interpretation beyond actual typed operations, or final geometry/lifetime guarantee is inferred. Raw fields, parser/accounting, historical populations/charges and phase remain unchanged; no data replay, native execution, admission expansion or coverage credit is claimed.

### 11.1778 Selected slot sourceordinal1 midpoint-form producer

*Status: retained component math, binary64half and point/index/handle chain verified; actual arithmetic and overrides qualified · Evidence: C-3463*

In the selected kind1 creation source path of11.1777, canonical LineHandle virtual1B8/1C0 bind named firstPoint/secondPoint targets. Their returned locals feed actual point-minus-point, scalar-times-vector and point-plus-vector providers. The constructed local110 has the native component operation form Pfirst+0.5*(Psecond-Pfirst), with exact binary64half from read-only image bytes. This records subtraction, multiplication and addition order; it is not an exact real-arithmetic mean for every finite/nonfinite/overflow/rounding or exception state.

That local110 address is supplied to named sgSketch::addNewPoint. Its returned index is passed to the canonical sgPointHandle constructor, and the named PointHandle clone RETURN is issued as sourceordinal1's append request. The earlier current carrier and model installer links remain qualified as in11.1766/11.1772/11.1777.

Canonical endpoint method binding does not prove an arbitrary kind1 override supplies coherent same-time geometric endpoints. Current returned locals, intervening aliases/providers, index/backing/allocation validity, normal ABI returns and object lifetimes remain prerequisites. No global centre/frame/unit, immutable geometry, cross-event state identity, later virtual-copy geometry or all-loaded-slot profile is established. Raw fields, current admission, parser/accounting, historical scopes and phase remain unchanged; no data replay, native execution, population or byte/global coverage credit is claimed.

### 11.1779 IsoParam desired-count and selected removal checkpoints

*Status: actual typed producer arithmetic and selected state/index assignments verified; achieved size and final deletion separate · Evidence: C-3464*

The typed current IsoParam creation path calls helper4BBC7F90 with an incoming desired size, compared with the initially queried B8 segment-state-array size. On its increase branch, the native32 desired-minus-old difference controls appends of literal rawu16 state3. On its decrease branch it calls4BBC80C0 for requested indices from desired toward the initial old size, ignoring each removal return. These are native loop operations, not a guarantee of mathematical iteration bounds, safe negative inputs or successful resize.

At common normal completion currentAC receives the incoming desired count and the helper returns native32(desired-old). AC therefore records a desired/requested count at that checkpoint, not proven achieved B8 array length. Provider/allocation/append/removal results and callbacks may differ from that request. Signed overflow, loop behavior, negative desired/index values, valid backing storage and normal ABI returns remain qualified.

The complete selected4BBC80C0 body contains early signed index/state branches that return0 without assigning A8. Its raw state4 special case writes3 and returns0 early. On the separately reached geometry-removal branch, currentB0/owner88 lookup supplies current geometry handles; after selected virtual208 operations the helper writes the input segment index into currentA8 and raw state5 into the selected state slot. These are selected mutation checkpoints, not a universal last-valid/kept-point interpretation, deletion-success guarantee or final geometry/count invariant. Negative indices lack a proved lower-bound guard, and lookup/providers need valid current identities and lifetimes.

The separate current A8 sign/state1/3/reset-2 consumer is qualified in11.1780, without a complete state lifecycle or unchanged loaded-value claim. WireA8/AC remain complete signed32 values. No loaded-value consumer, closed state enum, Boolean/domain normalization, B4 Flags semantics or new B0 key semantics follows. Typed construction, pending overrides/maps, current object/array state, callback effects and partial failure remain separate; no rollback, old-context admission, source/container/baseline replay, native execution, parser/accounting change, measured population or coverage credit is claimed.

The primary records a failed pre-output checker attempt whose import-name expectations were corrected before the successful frozen output. Publication verifies the final frozen source/output and independent peer bytes. No separate original failed-source copy or hash was preserved; no reconstruction or assertion of complete failed-attempt source provenance is made. This audit limitation does not replace the final strict source pins or expand the operational claim.

### 11.1780 IsoParam current A8 sign and segment-state reset operation

*Status: exact current receiver/index flow, ushort assignments and A8 reset verified; success/lifecycle meanings separate · Evidence: C-3465*

After the retained model-operation call4C4A5390, the selected creator reloads its typed current IsoParam output pointer from localRBP-80 and passes the current segment indexR14D to helper4BBCA600. No success interpretation is assigned to the preceding model-operation return. Valid current typed receiver, state-array header/backing and normal providers remain prerequisites.

The complete helper sign-extends the incoming index and compares its signed32 view with the current array count. If index>=count, it returns without the selected state stores or A8 reset. Otherwise it writes rawushort1 at that current index, then compares original currentA8 with0. A8>=0 causes rawushort3 to overwrite the same selected index; negative A8 leaves the initial1 at that checkpoint. Both accepted paths then store signed32-2 into currentA8. This is an actual current A8 sign consumer and local reset, not a closed state enum or engineering name for1/3/-2.

There is no index lower-bound check. A negative index can pass the signed upper comparison and address before the buffer, including when a null backing pointer causes the count result to be treated as0. This is not an invalid-state safety guarantee or an offline admission rule. The backing pointer is reloaded for the optional second word store, and aliases/current providers/lifetimes remain qualified; no atomicity, rollback, final state or concurrency invariant follows.

Earlier removal may have assigned A8 as in11.1779, but this proof does not establish the same event or an unchanged loaded value. The earlier model operation, callbacks and pending object/context state can alter the current pointer or fields. FlagsB4 and a complete1/3/4/5 lifecycle remain unassigned. Existing rawushort/signed32 domains, source-history qualification of11.1779 and final frozen pins are retained. No new wire field, source/container/baseline replay, native execution, parser/accounting change, population, coverage credit or phase advance is claimed.

### 11.1781 Selected slot arc-query sourceordinal1 point

*Status: canonical arc query, selected component formula and point/index/handle flow verified; universal angular interpretation separate · Evidence: C-3466*

In the selected kind2 slot creation path of11.1777, sourcePoint1 receives localA8 from the seed's virtual188. The canonical ArcHandle table binds that slot to named findMidPoint, distinct from its named center at virtual180. Numeric seed-kind2 alone does not validate an arbitrary receiver's runtime class or a coherent arc state.

The localA8 query result is supplied to named addNewPoint. Its returned index feeds a PointHandle construction and sourceordinal1's named clone append request. This establishes the selected output anchor and source order; it does not replace the query with the center call or prove an all-geometry angular halfpoint.

Within one selected phase of the actual findMidPoint body, a copied current radius and selected local vector feed actual component multiplication/addition providers to construct fresh center-return + copiedRadius*selectedLocalVector, which is then copied to the method output. Earlier conditional normalization, cross/sign providers and branch choices supply that local vector. The formula does not establish global unit length, orientation, complete branch correctness, a common coordinate frame or a universal geometric midpoint for degenerate, nonfinite or tolerance-sensitive inputs.

The radius is captured earlier and the center is queried later; callbacks may make them incoherent. Current canonical ArcHandle/provider/buffer/index/allocation validity, normal calls and lifetimes remain preconditions. The current carrier and later virtual-copy/storage links retain11.1766/11.1772/11.1777 qualifications: no immutable cross-event identity, preserved later clone geometry or all-loaded-slot profile. The kind1 midpoint-FORM in11.1778 is a separate selected producer, not an interchangeable all-kind definition. Raw fields, current admission, parser/accounting and historical scopes remain unchanged; no source replay, native execution, population or byte/global coverage credit is claimed.
