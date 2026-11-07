# Owned part continuation schema snapshot

Source: `../inventor-interface/docs/solidworks-file-format/specification.md`

Edition 0.320 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.110 Direct shared-parameter array after detail definitions

*Status: Native serializer evidence and 906 supported modern configuration prefixes; following environment-folder body unresolved · Evidence: [C-125]*

In the ordinary external-XML context of 11.109, the next field is the direct `moSharedParams_c` object at model member `+9C0`. Its constructor installs the `suObArray` base and its serializer calls `suObArray::Serialize`. Read a packed collection count (u16, or u32 following `FFFF`), followed by that many tagged object pointers using the existing archive map. The direct array itself adds no class or object map slot. Preserve null pointers, duplicate references, order and object identity.

The native loading path can deduplicate the array for smart-configuration copying and invokes a runtime callback on its entries; these operations consume no additional serialized fields. The model owner assigned at array member `+20` after loading is also a runtime update.

`modelafterdetails.read` implements only this direct array. `modelprefix.read(..., through_after_details=True)` composes it after the complete detail-definition body. The audit `body_feature_inventory.py --after-details` reaches this boundary for all 906 supported configurations, reading 109 pointer entries in total. Earlier detail-definition end and unknown-byte counters are captured before this continuation, so its bytes are not attributed to the detail-definition object.

The next tagged pointer is declared `moEnvFolder_c`. Its primary serializer shares `moFolder_c::Serialize`. The initial alias attempt failed because nested lights selected the DisplayLists context; 11.111 supplies the model-context readers and validates this continuation. This extension does not establish whole configuration-stream or document coverage, and does not change the scope to exclude any installed project format.


### 11.111 Model environment folder, light context and following units/list

*Status: Native serializer/context evidence; 906 supported modern configuration continuations · Evidence: [C-126]*

For the ordinary modern archive context of 11.110, read these fields after the shared-parameter array, preserving the existing MFC map:

| Order | Model member | Wire representation |
|---|---|---|
| 1 | `+1D8` | Tagged `moEnvFolder_c` pointer; its primary serializer is `moFolder_c::Serialize`, including the feature base and two final i32 fields described in 11.15. |
| 2 | `+9B8` | Tagged `moUnitsTable_c` pointer (nullable). The obsolete `+9B0` table occurs only before `0xF98`, outside this reader's scope. |
| 3 | `+9D0` | Tagged `suObList` pointer (nullable), using packed count and tagged entries. |
| 4 | `+9D8` legacy map | Only when `0x1BF4 < version < 0x3734`: u32 count; each entry is a CString name and three direct `mgPlane_c` records. The lower gate is always satisfied in the supported modern context. |

Each plane is six f64 values: root point x/y/z, then normal x/y/z (48 serialized bytes, despite the native object's 56-byte allocation). Retain order and repeated map names instead of applying native map replacement. Preserve raw floating-point values, including nonfinite values. The next modern model fields are ten tagged `moView_c` pointers; they are outside this continuation.

**Light serialization context.** `moLightFeature_c::Serialize` tests native global `DAT_4DADA610`. `moModel_c::serializeOldLights`, used for DisplayLists, sets it to zero around the array serialization and restores it to one. The static image initializes it to one. Ordinary configuration lights therefore require a distinct inherited framing; do not infer the mode by inspecting payload bytes.

In the supported context (`version > 0x27CB`), the model light body starts with `moModelFeature_c::Serialize`, including the feature body and its final `+354` i32. Next read the following shared light payload:

| Order | Wire fields |
|---|---|
| 1 | f64 ambient (`+388`), diffuse (`+390`), specular (`+398`); u32 color (`+3A0`). |
| 2 | i32 suppression state; i32 in-model-space (`+420`); discarded/raw i32 `+364`; i32 Cartesian-display flag (`+4E8`). |
| 3 | CString identifier name (`+4F0`). The separate username is omitted in this model context. |
| 4 | i32 PV-enabled value (`+37C`), i32 keep-light-in-scene (`+380`), then another i32 PV-enabled value for the same member. Preserve both serialized values; the native loader overwrites the earlier one. |
| 5 | f64 PV brightness (`+470`), i32 PV-shadow-enabled (`+480`), f64 PV-shadow-quality (`+488`). |
| 6 | Direct `moUserProperty_c` payload as documented for lights. The extra trailing node-ID i32 in the DisplayLists context is omitted here. |

All gates controlling these shared fields precede this reader's modern minimum. The native legacy branch that uses `moFeature_c` directly before `0x1168` is outside scope. The four supported concrete light serializers share this payload, followed by:

| Runtime class | Additional modern payload |
|---|---|
| `moAmbientLight_c` / `moLightFeature_c` | None. |
| `moDirectionLight_c` | Direction vector (three f64), f64 PV shadow softness (`+490`). |
| `moPointLight_c` | Position (three f64), f64 PV point radius (`+4A0`), i32 PV fog enabled (`+4A8`), f64 PV fog radius (`+4B0`), f64 PV fog quality (`+4B8`). |
| `moSpotLight_c` | Position and target (three f64 each); f64 cone angle, exponent, constant/linear/quadratic falloff and raw `+400`; f64 PV soft edge and PV spot radius; i32 PV fog enabled; f64 PV fog distance and quality. |

`modelafterdetails.install_scoped` copies the archive reader registry before installing environment-folder and model-light handlers. Existing DisplayLists handlers continue to use their original context. `modelprefix.read(..., through_environment=True)` selects the complete continuation above; `body_feature_inventory.py --environment-folder` reproduces the retained corpus audit.

The audit completes all 906 supported configurations: 1,004,746 environment-folder bytes, 47,304 following units/list/map bytes, plus the preceding 8,688 shared-parameter bytes. It reads 905 ambient, 1,667 direction and 11 spot light objects. No point lights occur in this corpus; their native suffix has synthetic coverage. There are 92 non-null units tables and 131 non-null object lists (all empty). Every observed legacy named-plane map is empty; populated maps, duplicate names and raw nonfinite planes have native/synthetic evidence. Full document bit coverage remains open. The following fixed model-view group is described in 11.112.


### 11.112 Fixed model-view records in resolved model data

*Status: Native serializer/constructor evidence; 906 supported modern groups and 9,060 non-null view records; nested collection bodies and field meanings remain partial · Evidence: [C-127]*

After 11.111, read ten tagged `moView_c` pointers in order. This is a fixed-size group, without a serialized count. The native loader uses eight slots only before `0x22E`, outside the supported modern context. Preserve null pointers and repeated references as stored: native code allocates a default view for a null slot and assigns each loaded view its slot index at `+358`, but those runtime updates consume no bytes and must not replace the serialized values in a lossless reader.

Although its constructor calls `moFeature_c` construction, `moView_c::Serialize` calls `su_CObject::Serialize` directly. It does not serialize the inherited node/concept/feature body. The ordinary archive branch of `su_CObject` emits no payload; archive flag `+20 & 0x10` selects a separate branch outside this reader's context. Both constructors allocate direct `mgXform_c` objects for `+360` and `+368`, rather than tagged transform pointers.

The supported ordinary modern view body is:

| Order | Native member | Wire field |
|---|---|---|
| 1 | `+350` | CString view name. |
| 2 | `+360`, `+368` | Two direct `mgXform_c` records, using 11.6; their distinct application roles remain unnamed. |
| 3 | `+380` | u16 raw setting. |
| 4 | `+384` | i32 raw setting. |
| 5 | `+370` | f64 raw setting. |
| 6 | `+378` | i32 count, then that many tagged object pointers. The allocated native container is typed for `moSectionModelData_c`; the loading call uses generic object-pointer extraction. This is not a packed MFC collection count. |
| 7 | `+358` | i32 serialized view index; retain even if it differs from its enclosing slot index. |
| 8 | `+38C` | i32 raw setting. |
| 9 | `+390` | i32 presence; if nonzero, a direct `suObList` (packed count and tagged pointers). Presence and inner count are distinct fields. |
| 10 | `+3A0` | i32 raw setting. |
| 11 | `+3B0` | f64 raw setting. |
| 12 | `+3A8` | i32 presence; if nonzero, another direct `suObList`. |
| 13 | `+3B8` | i32 raw setting. |
| 14 | `+3C0` | i32 raw setting only when `version >= 0x3764`. |

Earlier gates for orders 3–13 all precede the supported minimum (`version > 0x27CB`): `>0xEC`, `>=0x3B0`, `>0x927`, `>0xB79`, `>=0x183D`, `>0x23EF` and `>0x23F4`. Legacy layouts are not inferred. The decoded f64 settings retain nonfinite values; negative or impossible section counts fail closed.

While reading the two optional direct lists, native code sets archive-helper `+83C` and can add condition `0x200` depending on helper `+390`. Those state changes do not themselves add wire fields, but may affect a newly serialized nested object's body. `modelviews.nested_pointer` therefore accepts null/existing references and rejects new nested objects until a serializer for that context is established; ordinary global readers must not silently stand in for it. The section-data array uses normal tagged-object dispatch and likewise stops at unsupported classes. These restrictions are explicit gaps for populated collections, not proof that they are always empty.

`modelviews.read` copies the archive-local registry, installs the view serializer and reads exactly ten pointers. `modelprefix.read(..., through_views=True)` composes all preceding stages. `body_feature_inventory.py --model-views` completes all 906 supported configurations, recording source hashes, group/body spans, per-body hashes, serialized indices, collection counts and unknown-byte totals in `model-views-corpus.json`. The preceding environment continuation retains its own end position rather than absorbing these view bytes.

The group spans total 1,823,120 bytes, of which 1,794,128 are view bodies and 344,240 remain unnamed scalar bytes. Group sizes are 2,012 bytes in 901 cases, 2,084 in four, and 1,972 in the version-11000 case. Every group contains the names `*Normal To`, `*Front`, `*Back`, `*Left`, `*Right`, `*Top`, `*Bottom`, `*Isometric`, `*Trimetric`, `*Dimetric`, with stored indices 0–9. All observed section-data arrays and optional lists are empty. Synthetic cases establish populated reference lists, null/duplicate/cyclic pointers, distinct nonzero presence/count values, both sides of the final-field gate and raw nonfinite settings. The following model flags/managers are continued in 11.113; populated nested-object serializers and whole-document bit coverage remain open.


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


### 11.114 Direct model arrays and layer records

*Status: Native constructor/serializer/accessor evidence; 906 part prefixes, 19 directly reparsed drawing layer bodies; populated manager graphs have native/synthetic coverage · Evidence: [C-129]*

After 11.113, serialize eight direct `suObArray` objects in this order:

| Order | Model offset | Native version gate |
|---|---|---|
| 1 | `+B28` | `version > 0x917` |
| 2 | `+BA0` | `version > 0x917` |
| 3 | `+B50` | `version > 0x917` |
| 4 | `+B78` | `version > 0x917` |
| 5 | `+BF0` | `version > 0xF13` |
| 6 | `+BC8` | `version > 0xF13` |
| 7 | `+C18` | `version > 0xF13` |
| 8 | `+C68` | `version > 0xF23` |

Every gate precedes the supported ordinary modern minimum (`version > 0x27CB`). The constructor installs `suObArray` vtables in these inline members; they are not tagged pointers. Each body is a packed MFC count (u16, or `FFFF` followed by u32), then that many tagged object pointers using the existing archive map. The direct containers add no class/object map slots. Preserve stored null pointers, repeated references and order. Native storing can remove certain trailing entries according to their `+2E1C` member; this is a pre-write runtime operation and adds no fields. An unsupported new entry class remains an explicit reader stop; empty samples do not establish every possible entry body.

`modelarrays.read` implements this group. `body_feature_inventory.py --model-arrays` completes all 906 supported part configurations; all 7,248 arrays are empty, for 14,496 serialized count bytes. Synthetic arrays exercise populated pointers, inherited-map identity, null/duplicate references, both count encodings and truncation.

Next read a tagged `moLayerMgr_c` pointer at model `+EC8` (`version > 0xC7C`, always present as a pointer field in this scope). `modellayers.read` copies the archive registry and installs these complete modern body framings:

**`moLayer_c`**

| Order | Native member | Wire field and evidence |
|---|---|---|
| 1 | `+10` | CString name, used by `getName` and XML `UpdateUserName`. |
| 2 | `+18` | CString description, used by the description setter and XML `UpdateDescription`. |
| 3 | `+20` | 32 color bits, retained as u32; the native serializer uses a four-byte long extraction and `setColor` takes an unsigned color word. |
| 4 | `+24` | u8 line-style value; native storage is an enum-sized member, but only one byte is serialized. `setStyle(layerLineStyle)` establishes the member's role; retain unrecognized numeric values. |
| 5 | `+28` | Direct `utLineWidth_c` using restore type 1, flag 0. Supported modern versions store i16 standard width and f32 custom width. |
| 6 | `+30` | i32 layer identifier. Native lookup/XML-key formatting uses its low signed 16 bits; retain the full stored i32. |

The serializer's trailing virtual XML callback does not consume archive bytes during loading: it only queries the extractor, and the username/description updates are inside the storing branch. It is not an inline XML string or size-prefixed payload. The gates for width (`>0x438`) and ID (`>0x446`) precede the modern minimum. Nonfinite custom widths and raw style/identifier values are not normalized.

**`moLayerMask_c`** is a direct embedded `suUIntArray`: i32 count, i32 growth setting, then that many raw u32 attribute words, without an extra object tag for the embedded array. `suUIntArray::Serialize` forwards the first two words to `SetSize`; it does not use the packed MFC count. Preserve the allocation-growth word even though it does not change the number of stored values. Native `getLayerAttributes` returns these words as `swLayerMask_e`; `setIsPrintable` tests/toggles bit `0x2`, establishing the printable flag. Preserve every raw bit. Remaining attribute meanings are not established here, so the reader conservatively counts each entire attribute word as unknown while exposing the printable bit separately. The old migration replacing value 1 with 3 occurs only before `0x1BE5`, outside scope.

**`moLayerMgr_c`**

| Order | Member | Modern wire field |
|---|---|---|
| 1 | Layer collection | i32 count, then that many tagged `moLayer_c` pointers. This count is not the packed MFC array count. |
| 2 | `+08` | Tagged current-layer pointer, potentially referring to a layer already in the archive map. |
| 3 | `+30` | Tagged generic owner pointer in the ordinary condition-1-clear context. Special archives with condition 1 omit this field and are outside this reader's scope. |
| 4 | `+20` | Tagged `moLayerMask_c` pointer. |
| 5 | `+38` | i32 raw setting of unresolved meaning. |

Native loading assigns layer owners, copies temporary layer objects into a dynamic array, selects the current layer by name and computes runtime IDs. These operations consume no additional fields; decoded pointers retain the identity present in the serialized MFC graph. Earlier drawing/sheet owner variants and mask migration logic precede this reader's modern minimum. Negative/impossible collection counts and truncated bodies fail closed.

`modelprefix.read(..., through_layers=True)` composes the arrays and tagged layer-manager pointer. `body_feature_inventory.py --model-layers` completes all 906 part prefixes. All layer-manager pointers are null, adding 1,812 bytes; this is not evidence that drawings have no layers.

`native_layer_samples.py` independently scans installed modern drawings for strict new-class MFC headers naming `moLayer_c`, then parses the direct body from the verified header end. Across 20 modern drawings it finds and parses 19 bodies (657 bytes), all in `Contents/Definition`: 18 named `FORMAT` and one named `0`. The same scan finds and reparses 20 layer-mask bodies (292 bytes): 15 contain one attribute word, three contain three, one contains nine and one is empty. All growth settings are zero; the 33 attribute words contain 32 values of 3 and one of 0. Each retained body has a source-stream hash, exact span and decoded fields in `native-layer-samples.json`. This confirms those body framings without reconstructing the surrounding archive map; it does not claim whole drawing-stream or full layer-manager coverage. Populated manager/current-layer/owner cycles and masks have native/synthetic evidence. The next ordinary model stage serializes the direct `moForwardedFeatRefsList_c` at `+238`, continued in 11.115. Whole-document bit coverage remains open.


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


### 11.116 Final shared-model resolved fields

*Status: Native framing and 906 supported part configurations; nested bodies and field meanings remain partial · Evidence: [C-131]*

In the ordinary external-XML archive context with version greater than `27CB` hex, the continuation after 11.115 stores two i32 words (`+AF8`, `+D4C`), an i32 print-settings presence value and, when nonzero, a **direct** `mo3DPrintSettings_c` body. That body contains four f64 values, one i32 and a tagged component-reference pointer. Preserve the numeric fields as raw values; their meanings are unproved.

Next come a tagged `suObList` (`+A48`) and an i32 (`+EB0`). Versions greater than `27D3` add two CStrings (`+758`, `+760`); greater than `27DE` add a tagged `moCThreadRefMgr_c`; greater than `2B65` add a tagged `moBendNoteFormat_c`. These pointers use the existing archive map. A thread manager stores an i32 group count, then each i32 key, i32 holder count and tagged `moCThreadRefEntHolder_c` pointers. Versions greater than `3755` add an i32 flag; retain its original word. A bend-note format stores a CString and a tagged `su_CMapStringToString` pointer. New holder bodies remain explicit unsupported dependencies; 11.123 supplies the scoped string-map body. For other unsupported nested classes, null and existing references retain normal MFC behavior.

Versions greater than `3B05` add an i32 entry count. Each entry contains two CStrings and an i32 value count, followed by that many pairs of u32 words in low-word/high-word order. Retain serialized order and duplicate keys, even though the native runtime inserts into a map. Only versions strictly between `3EF1` and `3EF4` append a u16 geometric-tolerance decimal separator.

`sharedmodeltail.py` and the `--shared-model-end` audit parse this continuation in all 906 supported configurations: 218,493 bytes. Print settings and bend formats are absent in every native case; thread managers occur in nine. Synthetic fixtures exercise populated framing and version gates. There are still 251,640 bytes inside the declared resolved sections, belonging to the part-specific continuation. This is a shared-model boundary, not complete part, assembly, drawing, project-format or document bit coverage.

### 11.117 Part resolved suffix: mass caches, exploded-view manager and maps

*Status: Native framing and 906 exact supported configuration ends; scalar meanings partial · Evidence: [C-132], [C-133], [C-134]*

After 11.116, the ordinary modern `moPart_c` resolved serializer (`4C244920`) stores i32 `+14F0`, u8 `+1420`, i32 `+14D0` and i32 `+1524`. Next is an i32 number of **direct** `moMassPropCache_c` bodies (the current writer emits at most two). If there are two, an i32 `+1630` follows. The native constructor `4BC306E0` installs the mass-cache vtable; a misleading inferred custom-property type in the part decompilation SHALL NOT determine the body schema. Direct cache objects reserve no MFC map slots.

Each cache starts with an i32 entry count. Zero ends the cache immediately. A nonzero count is followed by that many `(i32 key, i32 body-data word, f64 value)` rows, then one direct `moMassProp_c` body. Preserve duplicate rows and all raw values. In the supported version range that mass-property body has the following ordered fields; offsets identify native members, not proved physical meanings:

| Wire type | Native offsets in serialization order |
|---|---|
| 12 f64 | `20,28,30,10,08,18,38,40,48,50,60,58` |
| 5 i32 | `1C0,1C4,1C8,1CC,1E0` |
| f64 | `00` |
| 4 i32 | `134,138,13C,140` |
| 2 CStrings | `148,150` |
| i32 | `1F0` |
| 3 f64 | `C8,D0,D8` |
| 3 direct vectors, each 3 f64 | `68,80,98` |
| i32; u16 | `15C`; `128` |

The part continues with i32 `+13F8`, a tagged `moPrtExplViewManager_c`, i32 `+1690`, i32 `+1694`, and an i32 map-presence value. A nonzero map-presence value introduces two direct collections: an i32 count of `(i32 key, tagged generic object)` rows, then an i32 count of `(i32 key, u8 value)` rows. Preserve serialized order and duplicates.

`moPrtExplViewManager_c` serializes the ordinary `moFeature_c` body, a tagged `suObArray`, a tagged `moExplPrtId_c`, f64 `+390`, i32 `+398`, i32 `+39C`, and a **direct** `su_CMapWordToOb`. Its constructor `4BABE5F0` proves the embedded map identity. That map uses `su_CArchive::ReadCount` (u16, or `FFFF` followed by u32) and that many `(u16 key, tagged generic object)` rows. It is not an i32-count map and reserves no object slot. The ID object stores a CString and an i32 `+18`. The manager's referenced array can contain the exploded-view classes continued in 11.118; other new classes still require verified readers.

The next field is the sheet-metal bridge array from 11.119. After it, if the original part lightweight member `+1614` equals zero, an i32 cached-box map count follows. Each row starts with an i32 key; **only positive keys** have a following tagged `moCachedBoundingBoxData_c`. A box-data body stores i32 `+C0`, then two records, each an i32 word, an i32 presence value and, when nonzero, two direct three-f64 points. These point pairs are not the ten-f64 `mgBBox_c` serializer. Then the part stores three i32 words (`18BC,18C0,18E8`), an i32 `+1BC8` flag and, when that flag is nonzero, an i32 count of i32 body IDs. The caller must supply the original lightweight flag; it cannot infer it from the suffix.

`partresolved.py` now parses these fields in all 906 supported native configurations: **251,640 bytes**, including **180,629 bytes with unresolved semantics**. Clause 11.118 adds the populated exploded-view dependency, and 11.119 corrects a missing middle-of-suffix bridge count. All 906 parsed ends now equal both the declared resolved end and Config-N stream end. This completes framing of the supported ordinary modern part configuration records; it does not establish all field meanings, older formats, other archive contexts or complete document coverage.

### 11.118 Part exploded views, body transforms, steps and lines

*Status: Native serializers, constructors and a populated installed fixture; framing supported, scalar meanings partial · Evidence: [C-133]*

The ordinary modern `moPrtExploder_c` body starts with the scoped `moModelFeature_c` body (the ordinary feature body plus i32 `+354`), then two i32 words (`360,364`), two tagged `suObArray` pointers (`368,370`), f64 `+378`, another tagged array (`388`), one direct ten-f64 `mgBBox_c` and a direct `suObList` (`398`). Constructor `4BABE810` allocates the box and constructs the embedded list; the box has no wire presence value. These embedded values reserve no MFC slots. Preserve the raw box values without recomputing the sphere or normalizing corners.

`moBodyXform_c` stores a direct `mgXform_c` followed by a **nonnull** tagged `moCompSolidBody_c`. Use the transform flags and optional payloads from 11.6. `moCompHolder_c` uses the same serializer (`4BBB9F90`) as `moCompRef_c`; it is also an allowed component-reference target in the scoped wrapper layout. This alias is established by its primary vtable, not inferred from the name.

The common `moPrtExplodeStep_c` serializer starts with the ordinary `moFeature_c` body, then stores: a tagged `moDirectionSpec_c`; f64 `+368`; a tagged `suObArray` (`370`); two i32 words (`378,37C`); a direct three-f64 vector (`3C8`); a direct `mgXform_c` (`3E0`); and a direct `suDWordArray` (`3A0`). The DWORD array uses the packed u16/extended-u32 count and that many u32 values. Its constructor identity is verified; retain order and duplicate values. Do not serialize the unmentioned runtime members merely because they lie between these offsets.

`moBodyExplodeStep_c` appends a **nonnull** tagged `moCompSolidBody_c` (`478`) and a tagged `suObArray` (`480`). `moPrtChainExplodeStep_c` instead appends a direct `suObArray` (`478`) and i32 `+498`; that final word's historical gate is greater than `A80` hex, always true in this scoped version range. Direct arrays and lists retain the existing archive map, enabling cycles through tagged children.

`moPrtExplodeLine_c` begins with ordinary node lightweight fields. Its node serializer dispatches virtual slots `4B8` and `4C0`; the line's primary vtable selects `4C18A1A0` (node lightweight) and `4B29DAE0` (no-byte return), respectively. Then it stores a **nonnull** tagged explode-step pointer (`40`), two direct points (`48,60`, three f64 each), two f64 values (`78,80`), three i32 words (`88,8C,90`), and two nullable tagged `moAttachPoint_c` pointers (`A0,A8`). New attachment bodies remain unsupported. This scope is above the old attachment gate `2E1` hex.

The installed `Carving Knife/Knife Handle-DEMO.SLDPRT`, `Contents/Config-0`, supplies a populated manager containing one exploder, two body transforms, two body explode steps and two explode lines. Each step's line points back to that same already-mapped step. Native reparsing verifies these identities and the exact 5,841-byte part suffix (after the 11.119 correction) with its source/body hashes. Together with the other 905 cases, the audit now reports 906 parsed prefixes and no dependency errors. Edition 0.118 still observed a four-byte mismatch in every case; 11.119 identifies and corrects the omitted bridge field. Bit semantics remain partial.

### 11.119 Part sheet-metal bridge array and exact resolved-section ends

*Status: Native virtual-call machine code, serializer/constructor evidence and 906 exact native ends; populated collection has synthetic evidence · Evidence: [C-134]*

The part serializer calls the sheet-metal bridge immediately after the two optional maps in 11.117 and before the cached-box collection. The historical version gate is greater than `1481` hex, always true above `27CB`. The bridge call's decompilation omits its third argument, but machine code at `4C2451E7` copies the archive from RSI into R8. The prologue at `4C24493E` establishes RSI from the incoming archive argument. Primary `smmBridge_c` vtable slot `+648` selects `3DB80290`, which forwards the part and archive to `3DA111D0`. This call consumes bytes and SHALL NOT be treated as a runtime-only update.

The bridge serializes an i32 count followed by that many tagged `moPerBodySheetMetalDataNew_c` pointers. Retain null entries, duplicates and serialized order. The native loader filters nulls when populating its runtime array; a lossless reader preserves the wire values. A negative or out-of-bounds count is rejected. The part owner is nonnull in this scoped call; the helper's null-owner no-byte branch is not selected from file contents.

A new per-body record has the following direct body in the supported version range:

| Wire type | Native member / interpretation |
|---|---|
| direct `suDWordArray` | `+30`: packed u16/extended-u32 count, then u32 values |
| 2 i32 | `+08`, `+0C` |
| tagged pointer | `+10`, `moSheetMetal_c` |
| tagged pointer | `+18`, `moFlatPattern_c` |
| 4 i32 | `+28`, `+20`, `+24`, `+50`, in this order |

Constructor `3DA19EF0` proves the embedded DWORD-array type. The serializer does not serialize the runtime object arrays at `+58` and `+78`. The historical gates for the last three words are respectively greater than `1435`, greater than `143D`, and at least `144D` hex, all satisfied in this scope. Raw words and array values retain unresolved semantics; new nested sheet-metal/flat-pattern bodies still require their own supported readers.

All 906 installed supported configurations have zero bridge-array count. Adding this missing **middle** field corrects the following cached-box and scalar positions and consumes exactly 3,624 additional structural bytes. The earlier four-byte remainder was a parser omission, not proved padding or a trailing field. In particular, interpretations of the later raw `18BC/18C0/18E8/1BC8` values from Editions 0.117–0.118 are superseded; regenerate their positions and values with the corrected reader.

The `--part-resolved` audit now parses all **906** configurations without errors and requires the parsed position to equal both the declared resolved end and the Config-N stream length. Unexpected trailing bytes are rejected. The part suffix totals **251,640 bytes**, of which **180,629** still have unproved meanings. Tests cover a populated bridge array with a new direct body, nulls, repeated references, packed DWORDs and truncation, plus rejection of a changed declared section size and unexpected trailing bytes. Complete framing of these supported streams does not grant whole-document or 100% bit coverage.

