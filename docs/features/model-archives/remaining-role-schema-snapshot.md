# Remaining role schema reference

Edition 0.356 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.80 Dimension entity-help and sectioned offset records

*Status: Native serializer/constructor evidence; populated records traversed in conditional raw-kernel audit · Evidence: [C-95]*

In the ordinary v > 0x27CB scope, sgDimEntityHelpData_c stores f64 value_08, f64 value_10 and u16 word_18. Native loading gates the word on v > 0x9F7, already satisfied here. The field meanings remain partial; readers SHALL retain all stored bits and SHALL NOT promote the word to the wider runtime integer size.

sgPGMSectionedOffsetEdgeData_c stores the loop-offset-edge prefix: u16 word_8 and a tagged suObArray pointer. It then directly serializes the embedded suObArray at runtime +0x18: packed ReadCount followed by tagged objects. The embedded array has no outer class tag or object slot. Readers SHALL distinguish the first tagged array pointer from the second direct array body. A nonnull first pointer must identify a supported suObArray-derived object; unsupported classes SHALL cause an explicit stop.

sgPGMSectionHandles_c stores two tagged sgSegmentHandle pointers followed by a tagged suObArray pointer. The first two pointers can resolve to supported derived handle classes or shared archive objects. Readers SHALL retain null and repeated-reference identity. A nonnull third pointer is checked as a supported suObArray object. There are no additional direct collection counts or scalar fields in this record.

With the explicit raw-kernel variant from 11.79, the vessel fixture now completes its relation-manager prefix, bringing that conditional audit to 893 successful prefixes, 79 sketches and 25 configurations containing sketches. Thirteen configurations still stop on feature-body dependencies. The default audit remains at 892 prefixes and 77 sketches because kernel stream context is unspecified there. The 771-byte kernel payload remains opaque; these conditional object-boundary results do not prove complete geometry decoding or whole-document bit coverage.

### 11.97 Base display dimension and component-feature references

*Status: Native vtable/serializer evidence; three complete linear patterns and cosmetic threads verified in scoped audit · Evidence: [C-112]*

Primary moDisplayDim_c RTTI selects serializer 0x4C835790, the shared dimension serializer already used by the supported derived display dimensions. Its primary vtable +0xA78 resolves to the zero-return function 0x4B29C5A0. The base class therefore omits the two point-reference tags, two f64 reference parameters and two i32-presence optional points at native offsets 590/598. Readers SHALL NOT substitute the derived distance/radial reference block for this class. The related virtual slots 8D8,8A8,8B0 also return zero; the last affects only historical runtime repair in the scoped path.

After the dimension-placement fields, the base-class reference continuation starts directly with u16 word_8E8, u16 word_8F0 and u32 flags_8A4, followed by the same two i32-presence nullable direct object arrays A58/A60 documented for the shared display base. The remaining units, favorites, strings, attachments, jogs and late fields are the common dimension body. There is no derived suffix after that body for moDisplayDim_c. The reader is installed in the archive's scoped registry, preserving the global registry.

Primary RTTI for moCompFeature_c and moCompBodyFeature_c selects exactly the same serializer 0x4BBB9A60. In the ordinary modern zero-condition archive, it reads the component-reference base (11.17) then direct moFRData_c at member F0: i32 feature ID and the modern u32 feature-time representation. There is no tagged FRData object, presence word or derived body-feature suffix on this path. The FRData serializer is 0x4BB48620, reused from the constructor-verified reference-plane path in 11.86. Archive conditions 1 and related masks can introduce presence/alternate reference-data branches outside this reader. Readers SHALL preserve signed ID, unsigned time and component-reference identity. Wrapper validation admits both proven component-feature classes.

With these dependencies implemented, `--complete-patterns` returns 901 relation-manager records in 906 attempts. It completes 27 profiles, 14 extrusions, five ordinary revolutions plus six revolution cuts, two ribs, three linear patterns and three cosmetic threads. The report's cumulative revolution counter is 11 and includes the six cuts. Each complete pattern has base moDisplayDim_c references at 1250/1260, distance dimensions at 1258/1268, and null 1310/1318 dimensions. These are six base-class reference use sites across the three pattern objects, not a count of distinct dimension allocations.

The three complete cosmetic-thread records each contain moCThreadData_c with eight cone values, zero instance/cache counts and absent optional point_120. These native paths now exercise the formerly synthetic-only thread-data framing in 11.93. Populated instance/cache class bodies remain separate dependencies; the existing synthetic tests cover grouped pointer framing but do not establish those class layouts. Exact pattern/thread source streams and nested body spans are retained in `complete-pattern-corpus.json`.

Five configurations still stop explicitly: four Fillet_c records and one present kernel body requiring stream/model-context framing. Completing these relation-manager prefixes does not establish complete configuration streams, assemblies, drawings or installed project formats. Whole-document bit coverage remains unchanged and below the requested target.

### 11.113 Post-view model flags, colors, sketch blocks and annotation views

*Status: Native serializers and DB-key labels; 906 complete supported continuation prefixes; unnamed meanings and later model fields remain partial · Evidence: [C-128]*

In the ordinary modern context of 11.112, the resolved model continues with:

| Order | Member | Wire representation |
|---|---|---|
| 1 | `+D24` stored value | u8. Native loading reads and discards this byte, then clears the runtime member; retain the stored byte. |
| 2 | `+CE0`, `+CE4` | Two i32 raw settings. |
| 3 | `+A80` | Tagged `moFeatColorTab_c` pointer. |
| 4 | Legacy attachment | Tagged generic object pointer. The current writer emits a null `moAttachment_c` pointer; retain any stored supported object instead of discarding it. |
| 5 | `+D3C` | i32 raw setting; native loading clamps values below 10 to 10. Preserve the original. |
| 6 | `+DB0` | i32 presence; if nonzero, a direct `moNodeList_c`, including its packed pointer count and trailing i32. |
| 7 | `+D40` | i32 raw setting; native loading clamps negative values to zero. Preserve the original. |
| 8 | `+E78` | Tagged `moSketchBlockMgr_c` pointer. Owner/initialization callbacks following loading add no serialized fields. |
| 9 | `+A40` | Tagged `suObList` pointer; entries include `moAnnotationView_c` in this corpus. A null pointer causes native runtime allocation of an empty list without consuming more bytes. |
| 10 | `+EA8`, `+EAC` | Two i32 raw settings. |
| 11 | `+EB8` | i32 count followed by u32 keys; no values are serialized. Preserve key order and duplicates, before native set insertion. |
| 12 | `+A88` | Tagged `moFlatPatternColorTab_c` pointer. |
| 13 | `+F34` | u32 raw setting. |
| 14 | `+1280` | Only when `version > 0x2BAF`: i32 count followed by u32 keys, with the same lossless ordering rule. |

All earlier gates for orders 1–13 precede the supported minimum `version > 0x27CB`. The legacy configuration-manager path before `0x3B3`, old string-array path, and transitional discarded-object array before `0x97A` are outside scope. Negative or impossible counts fail closed. The next native serialization stage is the direct object array at `+B28`, continued in 11.114.

**Feature and flat-pattern colors.** `moFeatColorTab_c::Serialize` reads an i32 count, then each entry as a direct `moVisualProperties_c::restore` payload followed by an i32 associated value. The direct visual body uses the already documented visual-properties layout and does not add a class/object slot to the archive map. The associated value's meaning remains unnamed and its four bytes count as unknown. The pre-`0x1D9` color-only layout is outside scope. `moFlatPatternColorTab_c::Serialize` is eight u32 words at `+08/+0C/+10/+14/+18/+1C/+20/+24` for supported versions; earlier gates `>0xC8F`, `>0x11D3`, `>0x12AD` precede the modern minimum. Their individual uses remain unnamed, so all 32 bytes count as unknown.

**Sketch-block manager.** `moSketchBlockMgr_c::Serialize` writes the ordinary `moFeature_c` body, 16 raw GUID bytes (`BlockGUID`), and a direct block-list map. The map adds no class/object map slot and has this modern layout:

| Order | Wire representation and evidence |
|---|---|
| 1 | i32 next ID (native writer label `NextId`). |
| 2 | Tagged `moSketchBlockMgr_c` owner pointer (`Owner`); this can refer to the containing object already inserted in the archive map. |
| 3 | i32 ID/GUID-group count (`IdGuidMapSize`); each group contains i32 ID, i32 GUID count, then that many 16-byte GUIDs. |
| 4 | i32 preferred-GUID-record count (native spelling `PreferedGuidMapSize`); each record contains 16-byte key GUID, 16-byte value GUID, i32 feature ID and CString feature name. The DB-key labels `KeyGUID`, `ValueGUID`, `FeatureId`, `FeatureName` establish those names; ordinary binary archives store raw GUID bytes without the DB string conversion. |

Preserve repeated group IDs, repeated GUIDs, repeated key GUIDs and serialized ordering. Native map construction/replacement is not a wire field. The pre-`0xA5B` integer-array group variant is outside scope; the preferred-record count is present for every supported version because its gate is `>0xBBC`.

**Annotation view.** `moAnnotationView_c::Serialize` starts with the complete direct `moView_c` body of 11.112, followed by:

| Order | Member | Modern wire representation |
|---|---|---|
| 1 | `+3D0` | i32 raw setting. |
| 2 | `+3C8` | Tagged component-reference pointer; native loading checks for `moCompRef_c` when non-null. |
| 3 | `+3E0`, initial write | Tagged `moRefWrapper_c` pointer. |
| 4 | `+3E8` | i32 raw setting. |
| 5 | `+3D8` | f64 raw setting. |
| 6 | `+3E0`, second write | Another tagged generic object pointer. Retain both pointers even though native loading overwrites the same member. |
| 7 | `+3EC`, `+3F0` | Two i32 raw settings. |
| 8 | `+2E8` | Tagged feature-comment pointer; native loading only replaces the member for a non-null pointer. |
| 9 | `+3F4` | i32 only when `version > 0x2B7E`. |
| 10 | `+400` | i32 only when `version > 0x2B9A`. |
| 11 | `+3F8` | i32 only when `version > 0x2BA0`. |
| 12 | `+3FC` | i32 only when `version > 0x2BBB`. |

All gates before order 9 precede the supported minimum; the obsolete vector at versions `0xBC7..0xC8B` is absent. The view's inherited fields are serialized through `moView_c`, not through the feature base. Its two optional view-list contexts retain the explicit unsupported-new-object boundary of 11.112.

`modelpostviews.read` installs the three manager/color handlers and annotation-view handler in a copied archive registry. `modelprefix.read(..., through_postviews=True)` composes preceding stages; `body_feature_inventory.py --post-views` reproduces `post-views-corpus.json`. All 906 prefixes complete, spanning 523,452 bytes, including 221,215 still unknown. Each has a feature-color table, sketch-block manager and non-null `+A40` list. Twenty-two color tables have one entry; 884 are empty. The lists contain 983 annotation views across 414 configurations; the other 492 lists are empty. The `+EB8` sets contain 1,835 serialized keys. Every observed block-manager ID/GUID map and preferred-GUID map is empty, all optional `+DB0` lists are absent, all flat-pattern-color pointers are null, and all observed `+1280` key sets are empty. Populated block maps, direct node lists, flat-pattern-color words and duplicate key/reference cases have native/synthetic coverage. These results establish this prefix's framing, not full document bit coverage.


### 11.115 Forwarded references, sheet-metal master and dimension-increment IDs

*Status: Native serializers, virtual-call machine code and 906 complete supported continuation prefixes; semantic coverage remains partial · Evidence: [C-130]*

For the ordinary modern part-root context following 11.114, retain the same archive map and the original node `+2C` flag word. Read these fields in order:

| Order | Model member | Wire field |
|---|---|---|
| 1 | `+238` | Direct `moForwardedFeatRefsList_c`, using packed MFC count followed by tagged object pointers. No tag or map slot is added for the direct list itself. |
| 2 | `+E30` | Tagged `moPMarkRecord_c` pointer. Its already documented body is one i32 permanent-mark ID. Ordinary conditions select this field; ghosting-disabled runtime disposal consumes no additional bytes. |
| 3 | `+1008` | i32 raw setting. |
| 4 | `+FD8` | i32 count; each entry is a tagged `moAtom_c` pointer followed by i32 raw value. Preserve null keys, duplicate keys and arbitrary stored values before native filtering/boolean normalization. |
| 5 | `+100C` (part only) | i32 presence; if nonzero, a direct sheet-metal-master body described below. |
| 6 | `+10A8` | i32 count and the flag-selected ID-map records described below. |
| 7 | `+10FC` (part only) | i32 raw setting. |
| 8 | `+1100` (part only) | i32 count, then that many direct `moDimIncrementId_c` bodies. |
| 9 | `+1120` (part only) | i32 raw setting. |
| 10 | `+1128` (part only) | i32 count, then that many direct `moDimIncrementId_c` bodies. |

All version gates selecting these fields precede the supported minimum (`version > 0x27CB`). The part-root primary-vtable callback at `+1A0` returns true, selecting the part-only fields. `modelreftail.read` requires an explicit `moPart_c` root context and an original u32 node-flag word; other root types are not inferred.

The forwarded-list serializer calls `suObList::Serialize` and then a runtime reference-repair routine with no archive argument. `moForwardedFeatRefs_c`, observed as a list entry, has one i32 feature ID at `+08`; its in-memory reference caches are not serialized here. Its primary serializer shares an implementation labeled for another class by the decompiler; primary RTTI and the caller's feature-ID lookup establish this use.

**Embedded sheet-metal master.** The model calls the sheet-metal bridge at virtual slot `+2C8`, passing a pointer-to-master and the archive. The model decompilation omits the third argument; machine code explicitly sets `R8` to the archive before the call. Concrete `smmBridge_c` dispatch creates a `moSheetMetalMaster_c` when loading and calls its serializer. Therefore a nonzero `+100C` is followed by real archive bytes, not just runtime allocation. The master is a direct object with no new MFC class/object slot:

1. Ordinary `moFeature_c` body, without an additional `moModelFeature_c` word.
2. i32 count, then tagged `moPerBodySheetMetalData_c` pointers. Although the native member is a `suObList`, this serializer writes a custom four-byte count rather than its packed-list framing.

Each per-body record calls the ordinary no-payload `su_CObject` base and reads four tagged pointers in order: `moCompFeature_c` at `+08`, `moCompSolidBody_c` at `+10`, then two `moCompFeature_c` pointers at `+18/+20`. Two i32 raw settings follow at `+28/+2C`. The native gates `>0x1060` and `>0x1314` precede this reader's minimum. Native UI-state and owner callbacks after loading consume no further fields. The retained sheet-metal module baseline is `sldsmmu.dll`, SHA-256 `ae20d533b36f105f22de4ed5984fb2245bbf03fb91a84ae473e0941a4b9e3c54`.

**Flag-selected ID map.** The model's stored node `+2C` bit `0x100` controls framing; never infer it by scanning the map payload. For supported modern versions:

| Flag state | Physical payload after i32 count |
|---|---|
| Set | Count records of four u32 words: high/low key words followed by two raw value words (16 bytes per record). |
| Clear | Count physical triples of u32 words (12 bytes per triple). If the first triple is `(0, FFFFFFFF, 0)`, native loading switches to a sentinel/pair migration path over the following triples. |

Retain all physical words, including sentinel triples and ignored/padding words, in their original order. `modelreftail.id_map` reports `wide`, `narrow` or `sentinel_triplets`; it does not replace stored records with the native hash map, drop nonpositive values or collapse duplicate keys. These record meanings remain partial, so all payload words count as unknown. Negative/impossible counts fail closed. The pre-`0x12BB` map layout is outside scope.

**Direct dimension-increment ID.** Its runtime serialization gate is `moPattern_c::isVaryInstanceCircularEnabled`, which tests the current runtime format against `0x1437`; it does not test the individual archive's version. The retained installed version-manager constructor defaults to 19000, enabling this serializer in the ordinary baseline. A runtime overridden to disable the feature is outside this reader's context.

| Order | Native member | Modern wire field |
|---|---|---|
| 1 | `+00`, `+08` | u8 handle-absent flag, then a tagged dimension-handle pointer **even when the flag is nonzero**. Retain the pointer that native loading may discard. |
| 2 | `+10` | u8 name-presence flag; if nonzero, CString name. |
| 3 | `+18/+1C/+20` | Three i32 raw values. |
| 4 | `+40`, `+48` | u8 display-dimension-absent flag, then a tagged `moDisplayDim_c` pointer, again always present as a pointer field. |
| 5 | `+24`, `+28` | u8 raw flag and f64 raw value. |
| 6 | `+50` | i32 raw value. |
| 7 | `+30`, `+38` | u8 raw flag and f64 raw value. |

All archive gates for this modern body precede `0x27CC`. Retain noncanonical flag bytes, optional strings, nonfinite doubles and both discarded pointer representations. Direct increment records add no MFC map slots; their nested tagged objects still do. The native `SerializePass2` routine has no archive extraction and adds no wire fields in this baseline.

**Nested dependencies.** `moRadiateSurfIdRep_c` uses the existing entity-representation body followed by two i32 words at `+38/+3C`; primary-vtable slot `+2A0` establishes that suffix. A standard tagged `suObArray` encountered in the increment graph uses packed object-array framing and is installed only in the copied reader registry.

The increment graph also reaches an external spline not present in the earlier sketch corpus. Spline collection mask bit `0x1` selects the external subtype, which uses the same old spline serialization body before native runtime conversion. Masks `0/1/8/9` are now supported; other subtype bits remain explicit stops. For supported spline types, the `+1F0` callback returns false, so a nonzero stored `+314` selects the secondary interpolation block only when stored control-point count `+308` is positive. This block is **not** the primary interpolation layout: read u32 parameter count, i32 width, two f64 settings, then count f64 parameters only. It has no parallel u16 words, i32 indices or count-times-width value matrix. A zero parameter count still has the width/two settings; native generation of default ones from control points consumes no more bytes. The reader retains the stored zero count and conservatively counts the secondary block as unknown.

`modelprefix.read(..., through_reference_tail=True)` composes all preceding stages. `body_feature_inventory.py --model-reference-tail` completes all 906 supported configurations, spanning 277,666 bytes including 167,702 still unknown. The retained report records source/body hashes, node flags, exact spans, raw settings and branch/collection counts. Observations include one populated forwarded list, 901 non-null permanent-mark pointers, 953 atom-map entries, 818 embedded masters, per-body records in 25 configurations, three nonempty wide ID maps (7/3/1 records), and one second increment array with 33 records; the first increment arrays are all empty. Narrow/sentinel map framing, populated direct increment records and secondary interpolation also have synthetic boundary tests. Later model-tail fields and whole-document bit coverage remain open.


