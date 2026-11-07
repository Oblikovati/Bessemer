### 11.90 Extrusion end, direction and starting-end specifications

*Status: Native serializer evidence; scoped complete extrusion audit; whole streams incomplete · Evidence: [C-105]*

The ordinary modern moBoss_c/moExtrusion_c body continues after 11.89 with a tagged moEndSpec_c pointer, f64 value_830 and i32 word_808. The scoped reader supports the ordinary external-XML archive context and versions above 0x27CB. Unsupported inherited or nested records SHALL stop the containing reader. Completion requires every inherited stage, not merely this suffix.

The end specification stores, in order: tagged direction pointer; three i32 words (08,88,8C); tagged face pointer; two i32 side types (0C,10); and two tagged display-dimension pointers (18,20). For each side in order, type 3 adds a tagged point-reference pointer (68 or 70), followed by an unconditional u8 array-presence value and, when nonzero, a direct suObArray (28 or 48). These presence values SHALL be read as single bytes.

Next are f64 value_A0; six i32 words (B8,BC,C0,C4,B0,B4); two tagged display dimensions (C8,D0); four i32 words (A8,AC,D8,DC); two direct suObArrays (E0,100); and three i32 words (128,12C,130). Side type 7 adds its tagged reference-wrapper pointer (78 then 80). A tagged starting-end specification follows, then four i32 words (148,14C,150,154). Version >=0x3B01 adds i32 word_158. Historical branches and alternate native global face-loading modes are outside this scoped layout.

The direction specification stores an i32-presence nullable direct object array, i32 word_10 and u8 primary-vector presence. Only when the primary vector exists does it store that three-double vector, u8 secondary-vector presence with optional vector, and u8 point presence with optional three-double point. Final i32 word_14 is unconditional. Readers SHALL retain the nested presence structure.

The starting-end specification begins with i32 stored type. Types 1/2 add a tagged reference wrapper. Types 3/5 add a tagged display dimension and i32 word_20; type 4 instead adds f64 value_18 and i32 word_20. Other types add no bytes in this modern path. Native normalization of stored types 4/5 to runtime type 3 SHALL NOT replace the stored discriminator in decoded evidence.

The suffix-only audit reaches all nine previously supported body boundaries. The complete-extrusion audit completes 11 extrusion objects and 14 profiles, allowing 894 of 906 relation-manager records to return. Twelve configurations retain explicit revolution, fillet, cosmetic-thread or present-kernel-body context stops. These counts belong to the opt-in audit; they neither register a general extrusion reader nor establish complete configuration streams. Whole-document bit coverage is unchanged.

### 11.91 Revolution feature, end specification and axis reference

*Status: Native serializer evidence; three complete scoped revolution objects; whole streams incomplete · Evidence: [C-106]*

The modern ordinary moRevolution_c body uses the inherited feature/atom/body stages in 11.81 and 11.88–11.89. Its suffix is a tagged display dimension at +0x800, u8 byte_81C, tagged display dimension at +0x808, i32 word_818, tagged suObList at +0x810, two i32 words (820,868), tagged moLineRef_w axis, and tagged moRevEndSpec_c end specification. Version >=0x3F05 adds u8 byte_8E0. Readers SHALL preserve both raw bytes. The native loader converts byte_8E0 to a runtime Boolean; the archive representation remains one byte. The list is present on this ordinary zero-condition path; archive condition 1 can omit it in other contexts. Legacy gates all precede the scoped versions above 0x27CB.

The revolution end specification starts with five i32 words in order (08,138,13C,0C,10), two tagged moRefWrapper_c pointers (118,120), two tagged moPointRef_w pointers (128,130), two f64 values (38,40), two i32 words (140,144), and four tagged display dimensions (18,20,28,30). These fields are unconditional in the modern scope, independent of the two side type words. Readers SHALL retain pointer identity and raw discriminator values. Older files before 0x11C3 substitute doubles for dimensions and are outside this reader's scope.

The axis moLineRef_w serializes the wrapper base from 11.16, two f64 curve parameters (A8,B0), a three-double point (C0), a three-double vector (D8), and u8 byte_F0. It does not use the additional embedded-body/pointer suffix of moGeneralCurveRef_w. The shared curve helper writes its parameters before the line geometry for versions above 0xFA3; historical relocation of those parameters is outside this modern layout.

The internal moRevolvedRadiusDim_c body uses the geometric dimension base from 11.28, then a tagged moSkDimHandleValG2_c pointer and i32 word_128. This is a model dimension, distinct from the display-dimension serializers. Its native serializer calls the same geometric helper used by ThreeDRadiusDim_c and ThreeDdiameterDim_c. The supported model-dimension handle whitelist includes this derived class.

The opt-in `--complete-revolutions` audit also enables the supported extrusion readers. It completes three revolutions, 11 extrusions and 17 profiles, returning 897 relation-manager records in 906 attempts. `complete-revolution-corpus.json` retains each revolution's source path, stream, start/end offsets and suffix flag bytes. Nine configurations still stop on rib, fillet, cosmetic-thread or present-kernel-body framing dependencies. These are object and manager-prefix results; no complete configuration stream or whole-document bit-coverage claim follows.

### 11.220 Combine/delete body features and solid references

*Status: B / ? — four combine features, three delete features/payloads and 16 solid references verified inside complete native roots; raw option meanings, populated solid-reference kernel bodies and whole-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature context, version >0x27CB. sldMoPartu.dll primary RTTI binds moCombineBodies_c (0x180466A48 / COL 0x1804ED890) to 0x18024EEE0 and moDeleteBody_c (0x18046B9B0 / COL 0x1804EDC28) to 0x18025B620. Both call the imported moBodyFeature_c serializer. Their +4B8/+4C0 hooks resolve to the ordinary moConcept_c lightweight/resolved-data pair. After the body-feature prefix, the combine suffix is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | body +800 | tagged moCompSolidBody_c | always | Target body component. | B |
| 2 | array +808 | direct suObArray | always | ReadCount and tagged objects; preserve order and duplicates. | B |
| 3 | raw +828 | i32 | always | Preserve signed stored option. | B / ? |
| 4 | raw +838 | u8 | version >0x3F07 | Preserve the exact stored byte before native nonzero-to-Boolean normalization. | B / ? |

Factory 0x18024EBD0 calls constructor 0x18024ED10, which constructs direct su_CObArray at +808 and installs suObArray's table. The runtime descriptor allocation 0x840 is not a wire size. With null/cached references and an empty array, the complete combine body is 259 bytes through version 0x3F07 and 260 thereafter.

The complete delete-body suffix is one tagged moDeleteBodyData_c pointer at +800, making its empty body 253 bytes. The payload primary 0x18046B920 / COL 0x1804EDBA0 selects 0x18025B540 and has su_CObject ancestry. Its ordinary zero-byte base is followed by a direct suObArray at +08 and i32 raw +28. The native +28 load gate (>0x1BC3) predates scope. Payload creator 0x18025B070 allocates 0x30 runtime bytes and constructs the direct array; the empty wire body is **six bytes**, not the allocation size. No operation enum or array-item restriction is inferred from observed values.

The nested moSolidRef_w is independently bound in sldmodu.dll: primary 0x4CED5050 / COL 0x4D320080 selects 0x4B90DED0, with moRefWrapper_c ancestry. Helper 0x4B90DB10 first calls wrapper serializer 0x4C2842D0, then restores u32 raw +A8 (native gate >0x1BBD, before scope). The derived body reads an **i32 body presence**. Zero ends the object. Nonzero selects one strict installed-kernel body followed immediately by a direct suObList of attributes. There is no body count or extra terminal pointer. Native partition/ownership/attribute fixups and condition 0x2000's store-time suppression do not add another field. The complete absent-body reference is 70 bytes at version 19000, or 58/62/66 bytes at wrapper checkpoints 0x27CC/0x2B73/0x2FB0. The present branch uses the existing strict kernel-body grammar and stops at unsupported kernel contexts; no opaque byte skip is used.

Native RTTI also establishes moCompSolidBody_c's moCompRef_c ancestry, admitting this already decoded component class to the wrapper's accepted component-reference contexts. All four new class bindings are confined to the copied modern feature reader map.

Six focused tests cover combine byte values 0/1/255, signed-word limits, direct-array ordering/duplicates, version boundaries and tagged minima, new payload-object map insertion and cached array identities, wrapper component identity, solid-reference raw word/full-width presence and conditional attributes, truncation/context rejection, and source-hash-bound native containing-root spans/fields for all four classes. Synthetic present-solid references exercise common suppressed-kernel framing; populated native non-suppressed bodies remain unobserved in these references.

The audit retains **four combine features**, with array counts 1/1/12/1, raw +828 equal to zero in three and 2 in one, and raw +838 zero throughout. **Three delete features/payloads** contain arrays of 2/2/1 moCompSurfaceBody_c targets and raw +28 zero. **Sixteen solid references** have body presence zero; raw +A8 values are preserved without assigning an identifier meaning. Across 906 complete modern contexts there are **24,372 complete roots**, **769 complete lists** and **137 explicit stops**: 66 additional roots and three additional lists relative to Edition 0.225. **696 lists reach exact EOF**, spanning **46,507,594 decoded bytes**, an increase of 316,244 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. `resolved-feature-list-frontier-0.225.json` preserves the prior checkpoint. Whole-document coverage credit is unchanged; complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-248.

### 11.335 Surface wrappers and projected-line subtypes

*Status: Edition 0.355 checkpoint; eight added EOF streams and measured stored credit; semantics partial · Evidence: C-377*

`moSurfaceRef_w` primary table **0x4D17CC28**, locator **0x4D35CF58**, binds Serialize +0x28 to **0x4C2866C0** in `sldmodu.dll`. It invokes common reference-wrapper input **0x4C2842D0**, then reads four f64 values at **+0xA8/+0xB0/+0xB8/+0xC0** when version >0xF22. Earlier native versions initialize defaults instead; the supported ordinary context (>0x27CB) always reads the four doubles. `modelreferences.surface` already implements this exact body for derived references and is now locally available for fresh base-class objects. The four parameters remain unknown and retain unknown-byte attribution; observed values -1/1/1/-1 do not establish their domains or meanings.

Projected-line subtypes are defined in installed **sldMoPartu.dll**, SHA-256 **2201edfa07b1a7484acedb6db4600fa7a00e55b16090a03b92fe7d9b14cb2128**. Both call imported **moFeatureData_c::Serialize**, whose ordinary input is a nonnull feature-owner pointer at +0x08. They then call **suObArray::serializePtr** for +0x10: signed i32 presence, followed by a direct packed-count tagged-object array only when present. This is not an enclosing tagged `suObArray` object. Concrete suffixes are:

| Class | Primary table | Serialize interval | Input after owner and nullable array |
| --- | --- | --- | --- |
| moPLineDraft_c | 0x1804D87C0 | 0x18037AC20..0x18037AD10 | Raw f64 +0x18, then tagged moDirectionSpec_c pointer +0x20. |
| moPLineIntersect_c | 0x1804D89D0 | 0x18037AD10..0x18037ADD1 | Raw signed i32 +0x38, then three tagged suObList pointers at +0x20/+0x28/+0x30. |

The draft input omits its f64 only for historical versions **0x159..0xA10 inclusive**; modern input includes it. No intersection suffix version gate appears in the complete native implementation. Raw subtype values remain unknown. The lists retain null/shared pointer identity and use their existing typed bodies when fresh. Native imports, complete bounded instructions and exact module identities establish the fields independently of the source values.

`resolvedfeatures.read_data(..., surface_dependencies=True)` installs these three readers in the local archive and enables the PLine feature's subtype check for draft/intersection objects. Other feature-data classes remain rejected. The complete coverage path enables this verified extension; `read_supported` and `documented_bytes` accept `surface_dependencies=False, remaining_planes=False, mid_surfaces=False, reference_curves=False, reference_surfaces=False, geometric_dependencies=False, backup_dependencies=False, sketch_dimension_dependencies=False, isocurve_dependencies=False, weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` to reproduce the edition 0.354 scope. Staged defaults and global registrations remain unchanged.

The nine targeted configurations contain **four draft objects**, **four intersection objects** and **three surface wrappers**. All eight projected-line objects have present arrays. Draft raw +0x18 is zero in all four cases. Intersection raw +0x38 is 7 or 15; their three lists are populated, including one two-item second list. These are observations, not complete value-domain claims. Eight streams reach exact EOF; the ninth, Verification nafems_ic22, advances to fresh **moPntSurfRefPlnData_c** and remains explicitly unsupported. `surface-dependencies-corpus.json` retains source hashes, root intervals/hashes, raw fields and populated collection counts.

The full modern part audit now reaches **847 exact-EOF streams / 59 explicit stops / zero context stops**, with **25,835 completed roots**. All **839** prior positives preserve exact endpoints, root counts and unknown-byte totals. Completed streams total **71,833,934 decoded bytes**, including **25,199,200 unknown bytes** and **46,634,734 known structure bytes**. `resolved-surface-audit-0.355.json` is reproduced with the earlier audit options plus `--surface-dependencies`.

`resolved_coverage.py --surface-dependencies` measures **81,710 additional known stored part bytes** beyond edition 0.354. Cumulative resolved-stream credit becomes **13,320,853** in the unchanged **25,137,466-byte** stored family denominator, leaving **11,816,613 unknown family bytes**. Total part unknown bytes become **55,046,779**; rounded coverage is **75.8% / 70.5%**, with assemblies **83.8% / 81.3%** and drawings **57.3% / 49.9%** unchanged. Unsupported contexts and streams earn zero. No full-document or installed-project completion is claimed.

`surface_dependencies_evidence.py` verifies both modules, primary bindings, complete serializer instructions/imports, common wrapper/feature-data dependencies and all previous positive audit rows. `surface-dependencies-native-evidence.json` retains those bytes. `resolved-feature-coverage-0.355.json` records every stored attribution; `resolved_coverage_evidence.py --surface-dependencies` independently verifies every source hash/span and ratio, writing `surface-feature-coverage-native-source-evidence.json`. The previous regrade remains reproducible with the extension disabled and `coverage-gaps-0.354.md` frozen under its original hash.

Tests check the different draft/intersection suffix boundaries with sentinels, signed values, null lists, truncation, all nine source outcomes and scoped registration. The complete-coverage regression checks all 906 modern decisions and the new known-byte sum, including zero credit for failed graphs. Remaining projected/surface features, unknown parameter meanings, other model streams, older/assembly/drawing contexts and all installed project formats remain unfinished.

Validation: all **1,102 regression tests pass** (144.073 seconds); specification lint reports zero findings (559 headings, 96 anchors).

Evidence: C-377.

