### 11.147 Ordinary modern shell-feature suffix

*Status: B — native-derived suffix framing with populated lists and edge references; raw byte and remaining nested meanings stay partial.*

moShell_c is implemented in sldMoPartu.dll. Primary table `180480E48`, locator `1804EFB00`, selects serializer `1802DB200`. Its +0x4B8/+0x4C0 hooks import the common concept serializers from sldmodu.dll. Runtime descriptor `1806052A0` selects factory `1802D97D0`, whose constructor call selects `1802DA070`. The constructor calls moBodyFeature_c, installs the shell tables and initializes its list/dimension/reference members. Memory allocation size 0x850 is not wire size.

In ordinary non-DB external-XML loading with v > 0x27CB, the wire order is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | body-feature base | Existing complete inherited body | Always | Includes atom stages and body suffix; excludes extrusion/revolution/rib suffixes | B |
| 2 | list +820 | Tagged suObList pointer | Always on load | Retain nulls, item order and shared references | B |
| 3 | dimension +810 | Tagged moDisplayDim_c pointer | Always | Actual class can be a supported derived dimension | B |
| 4 | +818 Outward | u8 | Always | Exact full 32-bit native integer property; STORE low byte and reached LOAD zeroextends into full 32-bit. Preserve every saved byte; no closed Boolean domain or physical outward-direction guarantee. | B |
| 5 | lists +828, +830 | Two tagged suObList pointers | Always in modern scope | Original gate v >= 0x163; pointers may be independently null | B |
| 6 | edge +840 | Tagged moCompEdge_c pointer | Always in modern scope | Original gate v > 0x468 | B |
| 7 | solid body +848 | Tagged moCompSolidBody_c pointer | Always in modern scope | Original gate v > 0x7DE | B |

Named moShell_c::getOutward/setOutward move full 32-bit current member 0x818 values without normalization. The canonical serializer stores only its low byte and zeroextends the loaded byte, rather than converting nonzero to 1. Thus a native full 32-bit property is distinct from the archive width, and the property name alone supplies no closed file domain or final surface-orientation meaning. Base/provider and post-load callbacks retain their existing effects. Raw parser key raw_818 is unchanged. Evidence: C-3519.

The suffix is six tagged pointers and one byte, **13 bytes when every pointer is a short null/reference tag**. Newly serialized pointed-to objects add their normal MFC headers/bodies/map slots. No field above is a direct list body or an i32 presence/list pair. In particular, the raw byte leaves subsequent pointer tags unaligned. The constructor's internal suPtrArray at +800 and state at +808/+838 are not appended wire fields in the selected serializer.

The load path reads field 2 unconditionally. Store-side moRefHistHelperFeatData_c::iSerializeFaces filtering can omit it in other history contexts; that branch does not change this ordinary load grammar. The scoped reader consumes all six pointer positions and retains unsupported nested classes as explicit stops rather than guessing their extent.

After the suffix, virtual +0x1EB8 selects post-load callback `1802E6520`. It calls model-feature post-load handling and removes trailing items from list +828 while that list is longer than list +830, treating a null list as count zero. This is a runtime normalization, not a count/word stored after field 7. The offline reader preserves both serialized lists, their original item order and shared archive identity. It does not trim away wire evidence.

The installed frontier retains **37 freshly serialized shell features** in completed roots. All have a moDisplayDistanceDim_c pointer, raw +818 equal to 0 and populated suObList +820 bodies containing 1 through 5 items. All +828 lists are present and empty. The +830 pointer is null in 25 cases and points to an empty list in 12. Seventeen shells have moCompEdge_c +840 references; all +848 solid-body pointers are null. A native regression verifies the stream SHA-256 and exact root end for a shell with at least two list items. Synthetic tests cover six nonnull inherited pointers, noncanonical raw bytes, unequal lists preserved without native trimming, a complete feature with a newly tagged list and repeated list identity, null pointers, truncation and context/version rejection. Positive solid-body pointers are synthetic inherited references, not populated native shell-body fixtures.

The edition 0.158 frontier records **19,964 complete roots**, **389 list-complete configurations** and **517 explicit stops**, with all **906 modern model contexts** complete. **383 lists reach exact EOF**, spanning **15,556,297 decoded bytes**; six retain four-byte unknown tails. No shell-class stop remains at this frontier. Leading dependencies are circular patterns (40), moOneContour_c (33), hole-wizard features (27), scale features (23), base bodies (22), sheet-metal features (21) and extruded reference surfaces (21). Full part, assembly, drawing and installed-project bit coverage remains incomplete.

Evidence: C-175.

### 11.150 Ordinary modern scale-feature suffix and coordinate-system reference

*Status: B — native-derived suffix and reference framing; raw numeric meanings and positive native optional fields remain partial.*

moScale_c is implemented in sldMoPartu.dll. Primary table `1804D9078`, locator `1804F3840`, selects serializer `180385CB0`. Its +0x4B8/+0x4C0 hooks import the common concept lightweight/resolved serializers from sldmodu.dll. Runtime descriptor `180607080` selects factory `180385960`. That factory calls the imported moBodyFeature_c constructor and inlines the scale-specific initialization, including direct suObArray +830. Allocation size 0x858 is not serialized length.

For ordinary non-DB external-XML loading with v > 0x27CB, read the complete body-feature base, then this suffix:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +80C | i32 | Always in modern scope | Preserve the complete signed value; original vector-layout gate v >= 0x489 | B |
| 2 | vector +810 | Three f64 | Always in modern scope | Direct mgVector_c body; retain all three components independently | B |
| 3 | raw +808 | i32 | Always | Preserve the signed wire value; no inferred enumeration | B |
| 4 | coordinate system +828 | Tagged moCoordSysRef_w pointer | Always in modern scope | Original gate v > 0x4D5; null, inherited reference or new object | B |
| 5 | array +830 | Direct suObArray body | Always in modern scope | Original gate v > 0x70B; ReadCount and tagged object pointers as in 11.149 | B |

Exact moScale_c native getters/setters bind full 32-bit member 0x80C to IsUniform and member 0x808 to ScaleType. Both use signed32 archive codecs at the selected checkpoints, without API normalization. Named getScaleFactor returns a typed mgVector_c reference to member 0x810, the same embedded vector used by named archive save/restore. No Boolean/enum closure, component convention, normalization, frame, units or final transform is inferred. The historical scalar-replication branch remains separate and does not extend the modern reader to old contexts. Raw keys, saved values and original wire order remain unchanged. Evidence: C-3520.

The minimum suffix is **36 bytes**, with a short null coordinate-system tag and empty short-count array. A new pointed-to object or nonempty array increases that length through its ordinary object grammar. There is no i32 presence field before the coordinate-system pointer or direct array. The constructor's +800 and +850 state is not appended wire data. Array count bounds and nested reader failures remain explicit; unsupported object bodies are not skipped.

The native historical branch v < 0x489 instead reads one f64, sets +80C to 1 and replicates that scalar into the three vector components, then reads +808 and returns. It does not follow the modern suffix grammar. This legacy branch is documented from native evidence but remains outside the scoped reader. Modern offline decoding does not replace anisotropic components, validate the raw words as booleans or normalize numeric values.

moCoordSysRef_w primary table `4D155808`, locator `4D358570`, selects serializer `4C27C0C0` in sldmodu.dll. Its complete modern body is the ordinary reference-wrapper body (`4C2842D0`, 11.16), followed by one direct compact mgXform_c transform at +A8 (11.6). The transform's original load gate is v > 0x4D5, satisfied in modern scope. This is neither the surface-wrapper's four-double extension nor the plane-reference body. Wrapper archive-mode restrictions and unsupported embedded-reference contexts continue to apply. New tagged coordinate-system objects occupy normal MFC map slots; repeated pointers preserve identity. The direct transform adds no object slot.

The frontier contains **30 fresh scale features** in completed roots. Raw (+80C,+808) is (1,0) in 29 cases and (1,1) in one. Vectors are (1.5,1.5,1.5) in 27 cases, with one each of (2.5,2.5,2.5), (1.2,1.2,1.2) and (1,1,1). All native coordinate-system pointers are null and all direct arrays empty. Positive coordinate-system objects, repeated shared references, populated arrays, unequal vector components, signed raw extrema and noncanonical values are tested synthetically. A hash-bound native root regression verifies the composed inherited body and exact end; malformed counts, truncation and unsupported contexts are rejected.

Edition 0.161 records **20,398 complete roots**, **435 list-complete configurations** and **471 explicit stops** across all **906 complete modern model contexts**. **405 lists reach exact EOF**, spanning **17,168,239 decoded bytes**. The other **30 complete lists retain four-byte unknown tails**, up from six before scale traversal; list completion does not establish those tail meanings. No scale-class stop remains at this frontier. Leading dependencies are hole-wizard features (38), base bodies (22), sheet-metal features (21), extruded reference surfaces (21), sweeps (17) and blend cuts (15). The two raw scale words remain unknown, and this traversal does not add whole-document coverage credit. Full part, assembly, drawing and all installed project-format bit coverage remains incomplete.

Evidence: C-178.

### 11.174 Draft features and edge and face identifier keepers

*Status: B — native modern field order, helper conditions and populated draft boundaries verified; raw options, populated dynamic arrays and runtime geometry remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`, excluding archive-condition 1. moDraft_c primary table `0x180483540` in sldMoPartu.dll selects `0x1802DA600`. The serializer first calls the existing body-feature base, then reads:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | dimension +800; raw +830; direction +868; word array +828 | tagged moDisplayDim_c; i32; tagged moDirectionSpec_c; tagged su_CWordArray | always | Preserve pointer identities and full signed word. | V / ? |
| 2 | array +818 | i32 presence; direct suObArray when nonzero | always | suObArray::serializePtr framing: packed count and tagged elements after presence, without an outer array class tag or map slot. | V |
| 3 | raw +860/+840/+844 | three i32 | always | Retain original +844 before native option conversion using +840. | V / ? |
| 4 | array +820; raw +848; surface body +850; plane +858 | nullable direct array; i32; tagged moCompSurfaceBody_c; tagged moPlaneRef_w | always | Array uses the same presence/body framing as row 2. | V / ? |
| 5 | raw +870; dynamic array +878 | i32; conditional direct array body | array only when +870 nonzero | Original signed word is retained. Dynamic framing is described below. | V / ? |
| 6 | raw +880; face-group presence | i32; i32 | always in this scope | Group helper is true for the modern ordinary context; the stored presence word still determines whether row 7 occurs. | V / ? |
| 7 | raw +72C; face groups +890 | i32; i32 outer count and counted groups | group presence nonzero | Each group begins with i32 face count followed by tagged moCompFace_c pointers. Preserve empty groups and repeated faces. | V / ? |
| 8 | points +8C8 | i32 count; count × three f64 | always | Direct point values, with no class/pointer tags. Preserve coordinates. | V / ? |
| 9 | dimension +808; raw +810/+814 | tagged moDisplayDim_c; two i32 | version > 0x3746 | Older modern versions omit all ten bytes. | V / ? |
| 10 | keeper +838 | direct moDraftSIdKeeper_c body | always in modern scope | No keeper presence marker or MFC tag. Its constructor-proven type selects the inherited edge/face serializer below. | V |

Exact own-class moDraft_c getters/setters establish the following saved property associations. Every pair moves the full 32-bit current integer without Boolean conversion or clamping; the names do not establish numeric enum labels, physical directions or successful draft geometry.

| Runtime member | Exact native property | Wire |
|---|---|---|
| 0x830 | IsEdgeDraft | i32 |
| 0x840 | IsStepDraft | i32 |
| 0x848 | BodyDraft | i32 |
| 0x870 | MultipleAngles | i32 |
| 0x880 | StepSmoothSide | i32 |
| 0x810 | SecondDir | i32 |
| 0x814 | SymmetricDraft | i32 |

Selected LOAD queries admit 0x830 from 0x179, 0x840 from 0x3C6 and 0x870 from 0xB63; their selected STORE paths do not use those LOAD gates. BodyDraft 0x848 is gated independently in both directions from 0x8D7, StepSmoothSide 0x880 from 0xB87, and SecondDir 0x810/SymmetricDraft 0x814 from 0x3747. These are separate current unsigned version-provider results. Skipped local reads do not directly reset these fields; inherited work and later callbacks can still change current state. Raw keys and all saved integer bits remain unchanged offline.

MultipleAngles 0x870 also requests the direct dynamic array 0x878. STORE calls its helper when current 0x870 is nonzero and current 0x878 is nonnull. Reached LOAD sets 0x878 null for zero 0x870; for nonzero it allocates the wrapper, assigns 0x878 and calls the helper, including when allocation returns NULL. Helper success and final array contents are separate. This native STORE predicate does not permit an offline reader to omit the array after a saved nonzero 0x870.

The IsStepDraft association belongs to 0x840, not 0x844. Native LOAD below its current 0x6A4 version query sets 0x844 zero; otherwise it reads signed32 raw 0x844. With current 0x840 nonzero and signed raw 0x844<3, raw zero becomes 3 and every other such value, including negatives, becomes 6. Current840 zero or raw 0x844>=3 retains that raw value at the local assignment. STORE writes current 0x844 through the signed32 codec. This is a separate compatibility conversion, not an enum interpretation or offline normalization. Runtime 0x884 ForGraphics is not assigned a saved field here. Evidence: C-3518.

All preceding historical field gates, including the dynamic array and point collection, precede the documented minimum. Reject negative or impossible group/face/point counts. Native archive-condition `0x800` and helper +834 can clear an existing runtime +818 array before loading; they do not replace its wire presence/count framing. The reader preserves stored references and raw values without executing this cleanup or native +844 remapping. Native load-time update calls after the late dimension fields add no payload fields.

Face-group helper `0x1802B4B00` consults environment setting `0x4F0`, file version, archive-condition 1 and helper +834. In this scope, version exceeds `0xC73` and condition 1 is absent, so it returns true regardless of the environment setting. Therefore the group-presence i32 is read in every supported modern draft. A nonzero value, including a noncanonical signed value, introduces raw +72C, outer count and groups. The writer's environment-selected presence value does not justify omitting that word in an ordinary reader.

Dynamic helper `0x1802DA5A0` delegates to load `0x1802D9910`/`0x1802D9830` and store `0x1802D9940`. Framing is u32 element size; zero ends the array body. Otherwise it is u32 capacity, u32 growth/allocation word, u32 element count, then exactly element-size × count raw bytes. Nonzero element size must be divisible by eight. The existing dynamic-array reader preserves both header words and raw payload and counts payload bytes as unknown meanings. This is not packed ReadCount or a tagged double-array object; +870 presence is separate from the array's own zero-size form.

Runtime descriptor `0x1806052D0` selects draft factory `0x1802D9800`, which calls constructor `0x1802D9A30`. That constructor allocates the +838 keeper and invokes `0x1802679A0`, installing moDraftSIdKeeper_c primary table `0x180475608`. Its serializer slot is six-byte thunk `0x1803AC9DA`, which jumps through IAT `0x1803E2AB0` to sldmodu.dll export moEdgeFaceSIdKeeper_c::Serialize at `0x4C6E0850`. The import name, full export-table ordinal/address binding and inherited RTTI chain are verified. There is no additional draft-keeper suffix.

moEdgeFaceSIdKeeper_c primary table `0x4D1FB718` selects that same serializer. Its body, used both directly and for scoped tagged keeper instances, is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | surface IDs +18 | i32 count; count × direct moEntityId_c body | always | Existing entity-ID body contains a tagged representation pointer; no outer entity class tag. | V |
| 2 | references +50 | direct suObArray body | always | Packed count and tagged reference objects. No outer tag/map slot. | V |
| 3 | surface IDs +28 | i32 count; count × direct moEntityId_c body | version > 0x3746 | Entire second array, including its count, is absent in older modern files. | V |

Keeper constructor `0x4C6E0170` installs suObArray's vtable at +50. Shared mo_dynSurfIdArr helper `0x4BBBE2E0` establishes the signed count and direct entity-ID bodies. Runtime external-object replacement and edge-reference use-count updates do not add ordinary archive fields and are not executed offline. Latest empty keeper size is **10 bytes**, versus **six bytes** through `0x3746`. The null/optional-empty draft suffix is **74 bytes** latest and **60 bytes** through `0x3746`; including the latest empty body-feature base gives **325 bytes**. These synthetic minimums establish framing, not engineering validity for arbitrary values.

The complete-root audit verifies **10 drafts**. All have +818 array presence one, +820 absence, dynamic-array +870 zero, group presence one and empty direct keepers. Six have one face and one point, two have four faces and four points, one has three faces and three points, and one has two single-face groups with two points. Native tests verify stream SHA and complete containing-root spans for these group shapes. Synthetic tests cover populated dynamic arrays, empty/repeated face groups, raw values before normalization, direct/tagged keeper identity, entity-ID representations, version gates, malformed counts/stride, truncation and context rejection. Populated native dynamic arrays and nonempty native draft keepers remain unverified.

Edition 0.185 records **22,506 complete roots**, **680 list-complete configurations** and **226 explicit stops** across **906 complete modern contexts**. **619 lists reach exact EOF**, spanning **33,244,902 decoded bytes**; **61 complete lists retain four-byte unknown tails**. Revolved surfaces, cut lists, sweep cuts, annotations and other feature/chooser dependencies remain unresolved. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these boundaries.

Evidence: C-202.
