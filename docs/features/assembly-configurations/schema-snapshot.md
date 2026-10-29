### 11.120 Modern assembly root dispatch and shared lightweight prefixes

*Status: Native dispatch and all 174 ordinary shared-model lightweight tails after 11.122; assembly-specific data remains incomplete · Evidence: [C-135], [C-136], [C-137]*

The installed assembly corpus contains 183 Config-N records: 174 modern `moAssembly_c` roots, six modern `moRouteAssembly_c` roots and three older ordinary roots (version 10000). The scoped reader below accepts only `moAssembly_c` with version greater than `27CB` hex. Routing roots and older records remain explicit unsupported cases; an assembly filename alone does not establish the root class or archive context.

Primary `moAssembly_c` vtable `4CE6AA80` selects root serializer `4B680F10`, lightweight virtual `4B691960` at `+4B8`, and resolved virtual `4B694B00` at `+4C0`. The modern root dispatches both virtual stages. The lightweight stage constructs the same section-scope helper as the part, then calls shared model lightweight serializer `4C23BD90`; the resolved stage independently constructs its scope and calls shared model resolved serializer `4C2411C0`. Assembly-specific continuations SHALL NOT be replaced by the part continuations.

The verified lightweight prefix is the MFC root class/object record, i32 section byte size, native header-object map insertion, i32 `+A8`, u32 `+AC`, and node lightweight fields from 11.11. The external Config-N-ModelHeader supplies self/external references in the existing proven insertion order, with no added class slots or inline bytes. New root objects are mapped before their fields, preserving cycles through owners and references. The shared continuation uses the concept prefix and units described for ordinary model archives. Its unit component may refer to the **assembly** root; that owner allowance is archive-local, while the existing part path retains its part-owner restriction.

`modelprefix.read(..., root_class='moAssembly_c')` parses this node prefix in all 174 supported ordinary configurations: **16,696 bytes**, with **4,176 bytes retaining unresolved meanings**. The complete header map remains attached to the returned archive. Selecting shared stages up to `through_model_tail=True` reuses the native common model order, without entering part-specific fields. Edition 0.120 completed that shared tail in three configurations (**7,213 cumulative prefix bytes**, **4,275 unknown bytes**) and stopped the others at nested references/components. Clauses 11.121–11.122 advance this to all 174 configurations; the current corpus reports reflect the later readers. These are still partial lightweight records, not complete assemblies.

`moPointRef_w`, reached while investigating the latter cases, shares the `moRefWrapper_c` serializer (`4C2842D0`) and appends one direct three-f64 point at `+A8` through serializer `4C281C60`. The point's historical gate is greater than `78` hex, always true in this scope. Clearing its runtime `+E0` member consumes no bytes. Its coordinate-frame meaning is not established by this layout alone.

The assembly option is limited to the shared lightweight stages. Selecting part appearances, the part lightweight suffix or resolved part stages with an assembly root is rejected. This avoids interpreting assembly-specific data using a part schema. No whole-assembly, routing, drawing or project-format coverage is claimed. Continue the ordinary assembly body from `moReference_c` and the separate assembly lightweight suffix before entering resolved data.

### 11.121 Ordinary assembly references and external-feature links

*Status: Native reference serializers; expanded native corpus in 11.122; raw meanings and optional dependencies remain partial · Evidence: [C-136], [C-137]*

In the ordinary external-XML context above version `27CB` hex, `moReference_c` serializer `4B68C5C0` first serializes the ordinary feature body. It then stores a u8 presence value and optional direct `mgXform_c` (`+358`), a u8 presence value and optional direct ten-f64 `mgBBox_c` (`+368`), and a tagged `moExtObject_c`. Retain original nonzero presence values, transform encodings and box values; do not apply native normalization.

The rest of the reference body follows this order. Offsets identify native members whose meanings may remain unproved.

| Wire fields | Native members / notes |
|---|---|
| u16, u16, u32 | `3F8,400,3D8`; native normalizes `400` to a boolean, but retain the stored u16 |
| direct referenced configuration | CString name, i32 ID; no object-map slot |
| u8 presence, optional direct transform | consumed and discarded by the native loader; preserve it |
| u8 presence, optional transform array | when nonzero: i32 count, then each u8 presence and optional direct transform; preserve holes |
| i32, u32 | `448,3E8` |
| tagged smart-part pointer; i32 | `moSmartPartData_c`; `450` |
| two tagged pointers; i32 | `moSMCInstanceData_c`, `moSmartInsertComponentData_c`; `4A8` |
| tagged toolbox pointer; two i32 | `moToolboxPartSpecArray_c`; `4AC,44C` |
| tagged XML-rules pointer | `moXmlRulesModelData_c` |
| five i32 | `4D0,410,414,4B8,4D4` |
| u32 time; i32 | `su_CTime` wire value; `4E8` |
| tagged external-feature pointer; i32 | `moExtFeature_c`; `548` |
| two CStrings; three i32; u32 | `4F8,500`; `51C,528,380`; `554` |
| two u32, only above `2B6C` | `3EC,3F0` |
| tagged wrapper, only above `2F53` | `moRefWrapper_c` |
| tagged stack-part pointer, then three i32, only above `2F7D` | `moStackPartData_c`; `494,498,49C`; retain the pointer even though the current writer stores null and the loader discards it |
| i32, only above `42D2` | `3D4` |

The direct `moReferencedConfiguration_c` serializer is in **sldarchiveu.dll**, at `31D08140`, not in sldmodu.dll. Native code serializes a CString at `+08` and signed 32-bit value at `+10`; the exported `getId` accessor confirms the latter identity. Its ordinary `su_CObject` base adds no bytes. The in-memory object size is not a wire-size rule. `su_CTime` likewise retains its established u32 archive representation rather than its larger in-memory storage.

`moExtFeature_c` serializer `4C187DC0` calls base `4C1887A0`. In the supported ordinary **handle-name** context, the base stores a direct shared-file definition (11.8), a direct referenced configuration, u8 `+40`, i32 `+44`, and a tagged **nonnull** `moCStringHandle_c`. The feature adds a nullable tagged `moReference_c` at `+50` and i32 `+58`. Runtime reconstruction, reference registration and counter updates consume no additional bytes. The tagged reference can point back to the still-being-loaded enclosing reference; reserve object slots before parsing fields.

The native name-context flag at `4DB7E519` lies in the loader-zero-filled part of `.data`; retained static references are the two checks in the base serializer. Its nonzero mode stores a direct CString instead of a string-handle pointer. That alternate context is not selected or guessed from bytes by this reader. The successful installed fixtures and cyclic-pointer regression establish the selected handle context; other archive modes remain unsupported.

`assemblyreferences.py` is installed only in the assembly root's local reader registry. The smart-part, toolbox, pattern and chain continuations are defined in 11.122. Other unsupported new instance, smart-insert, XML-rules, stack-part and smart-relation bodies fail explicitly; null and already-mapped references still use normal MFC semantics. Condition `80` would add a final u16 after the reference body; it is outside this ordinary context and is not silently consumed.

Edition 0.121 completed shared model lightweight tails in **152 of 174** ordinary modern configurations, covering **796,031 cumulative prefix bytes** with **493,459 unknown bytes**. Their graphs contain **665** complete ordinary references: 658 have a transform and box, seven have a transform without a box; none exercise the discarded-transform or transform-array branches. The unit-stage audit also completes 152 cases (**747,158 bytes**, **474,531 unknown bytes**). Its model-tail stops were 14 reference patterns, five smart-part records, one toolbox specification and two chain components; 11.122 resolves these installed dependency cases. The 149 newly advanced cases stop 166 bytes before their lightweight end; the three earlier cases stop six bytes before it. These are measured remaining spans, not decoded padding. Routing/legacy assembly cases, assembly-specific lightweight/resolved continuations and complete document bit coverage remain open.

### 11.122 Assembly reference patterns, smart parts, toolbox records and chains

*Status: Native serializers and all 174 ordinary assembly shared lightweight tails; assembly-specific suffix and bit semantics remain incomplete · Evidence: [C-137]*

These records use the same ordinary context and version floor as 11.121. They are installed only in the assembly archive's reader registry. Class and object pointers retain their original map identities, including repeated references. Direct arrays and configurations reserve no class/object slots.

A `moReferencePattern_c` starts with the complete reference body from 11.121 and appends i32 `+560`, a **nonnull** tagged `moPatternItemId_c`, a tagged seed `moReference_c`, and i32 `+578`. Retain the raw `578` word although the native loader normalizes it to a boolean. Additional fields are i32 `+57C` above `332D` hex, i32 `+580` above `42D0`, and CString `+558` at or above `42E2`. Keep the base-reference end separate from the complete derived-object end when recording spans.

Two concrete pattern-item serializers are verified. `moLPatternItemId_c` stores i32 `+20`, i32 `+24`, u8 `+08`, and f64 `+10`. `moCirPatternItemId_c` instead stores i32 `+18`, i32 `+1C`, u8 `+08`, and f64 `+10`. Both bodies are 17 bytes in this scope. Older circular-index migration and the byte/double gates precede the supported version floor; do not apply those runtime migrations to modern stored values. The ordinary `su_CObject` base adds no bytes. Only these concrete new-class bodies are implemented; other derived pattern-ID bodies remain unsupported.

`moSmartPartData_c` stores a CString (`+10`), an **i32** count followed by that many tagged `moSmartRelation_c` pointers, then i32 `+64` and i32 `+68`. Null relations are discarded by native runtime insertion; a lossless reader preserves serialized nulls, duplicates and order. Assignment of the owning reference is a runtime callback and does not add an owner pointer to the body. New smart-relation bodies remain unsupported.

`moToolboxPartSpecArray_c` begins with a direct object-array body: packed u16/extended-u32 count and tagged item pointers. It then stores eight CStrings at `20,30,38,40,48,50,58,60`, i32 `+7C`, CString `+70`, and i32 `+80`, in that order. The loader's assignment of `+78 = 1` is not a wire field. Each `moToolboxPartSpec_c` body stores i32 `+08`, CStrings `+10` and `+18`, f64 `+20`, and i32 values `+28` and `+2C`. Their physical/property meanings remain unproved; preserve raw values, including nonfinite doubles.

The supported `moChainComponent_c` path begins with the modern component base's **single tagged owner pointer**. It does not have the additional unit-component byte from 11.17. The reader accepts a null owner or an already-supported part/assembly owner, then reads a tagged `moReferenceChain_c` and u32 `+30`. External-owner resolution and legacy external-model paths remain unsupported; do not select their early-return behavior from pointer truthiness alone.

A modern ordinary `moReferenceChain_c` stores u16 `+24`, a tagged `moExtFeatureList_c`, i32 `+20`, then a direct referenced configuration (CString name and i32 ID) at `+40`. Its default constructor proves that embedded type. The modern ordinary owner is supplied from archive context rather than an extra wire pointer. The tagged feature list uses an **i32** count followed by tagged `moExtFeature_c` entries; it is not the packed-count object-array serializer. Native reference counting, replacement/canonicalization and configuration registration do not add bytes. Those runtime changes are not applied to the lossless wire graph.

The shared lightweight model-tail audit now succeeds in **all 174** ordinary modern assembly configurations: **1,091,222 cumulative prefix bytes**, of which **696,908** retain unresolved meanings. The unit-stage totals are **1,035,313** and **675,266** bytes. The passing graphs contain 878 ordinary references and 127 reference patterns (1,005 reference-base bodies), 99 linear and 28 circular pattern IDs, 15 smart-part records with empty relation lists, and 16 toolbox arrays containing 232 specifications (14 arrays of 15, two of 11). Two chain components each contain a chain with one external-feature list entry. Exact per-object body spans, raw-byte hashes, unknown-byte counts and list cardinalities are retained in the corpus report.

All earlier ordinary dependency stops are resolved for this corpus. Edition 0.122 still stopped before the assembly-specific lightweight suffix: 171 cases had 166 bytes remaining, three had six bytes remaining. Clause 11.123 supplies that suffix; those bytes were not padding. Six routing roots, three older assembly configurations, resolved assembly data, drawing schemas and installed project formats remain part of the incomplete overall objective. Complete shared-prefix framing does not grant 100% bit coverage.

### 11.123 Assembly lightweight suffix and MFC string maps

*Status: Native MFC/assembly code and 174 exact ordinary assembly lightweight ends; map values and box meanings remain partial · Evidence: [C-138]*

After the shared lightweight model tail, the ordinary assembly serializer stores u8 `box_1A10_presence` and, if nonzero, a direct ten-f64 `mgBBox_c`, then u8 `box_1A18_presence` and its optional box. The second presence field's historical gate is greater than `17FA` hex, always satisfied above `27CB`. Preserve the stored presence bytes and raw box values. Runtime allocation, self-reference construction and visual-property defaults do not add bytes here.

Next are two **direct** `su_CMapStringToString` bodies at `+1498` and `+14D0`, in that order. Both are present above `23DA`, always true in scope. Only versions strictly below `2B65` in this supported range append a discarded i32 compatibility word. Preserve that word; the current writer does not emit the historical branch.

The map's `su_CArchive` wrapper at `31D376D0` dispatches to its MFC serializer when archive flag `10` hex is clear, as in this ordinary context. Thunk `31D4C76D` imports **mfc140u.dll ordinal 12804** through IAT `31D504B8`. The installed MFC export address table resolves that ordinal to `18023CE50`, `CMapStringToString::Serialize`. The default PE-library export inventory had stopped at its 8,192-entry ordinal-scan safety threshold; its truncated symbol list did not prove that the ordinal was absent. The retained resolver checks the raw ordinal range and reads the export-address-table entry directly. Native function bytes, module hashes and the separate ordinal-name catalog establish the identity.

The map body is `CArchive::ReadCount`, followed by that many key/value pairs, each a direct MFC string. The **64-bit MFC** count uses these alternatives:

| Prefix | Count value |
|---|---|
| u16 other than `FFFF` | that u16 |
| u16 `FFFF`, u32 other than `FFFFFFFF` | that u32 |
| u16 `FFFF`, u32 `FFFFFFFF`, u64 | that u64 |

This is distinct from the scoped `su_CArchive::ReadCount` u16/u32 helper at `31D36AD0`. Do not reuse or extend that helper globally based on the MFC map's layout. `mfcmaps.py` retains the selected count-prefix width and rejects a count that cannot fit the remaining key/value data.

MFC strings use the familiar u8/u16/u32 length hierarchy and optional `FF FE FF` UTF-16 marker, but a u32 length of `FFFFFFFF` introduces an additional u64 length. The marker is followed by the complete length hierarchy for the UTF-16 code-unit count. `string64` retains whether the marker was present and the original prefix width, including noncanonical short values encoded through longer escapes. UTF-16 code units, including unpaired surrogates, remain lossless. Single-byte strings use a Latin-1 byte projection; the runtime Windows code-page interpretation is not claimed. Lengths are checked against available payload before reading or allocation.

Preserve serialized pair order and duplicate keys. The native runtime inserts into a map and replaces matching keys; the schema reader keeps the wire sequence. Application-specific meanings of both maps' strings remain open, so their serialized strings retain unknown-byte accounting. A direct map reserves no MFC slot. A tagged `su_CMapStringToString` uses the same body after its normal class/object record; its MFC `CArchive` virtual is slot **2**, not the `su_CObject` slot 5. This also supplies the previously missing nested-map body for the bend-note format in 11.116.

`assemblylightweight.py` and `through_assembly_lightweight=True` complete all **174** ordinary modern assembly lightweight sections exactly at their declared ends. The new suffixes total **28,404 bytes**, with **27,360** raw box bytes still of unresolved meaning. In 171 cases both boxes are present; in three both are absent. All 348 native maps are empty. Cumulative lightweight prefixes total **1,119,626 bytes**, including **724,268 unknown bytes**. The reader rejects a changed section boundary. Populated maps, duplicate keys, all count/string escape widths, Unicode edge cases and the tagged bend-map composition have native code plus synthetic tests, rather than populated installed fixtures.

The cursor now stops at the resolved-section size word. This result completes framing only for the supported ordinary assembly lightweight stage. Resolved assembly data, routing/older roots, drawing/project schemas, field meanings and full-document bit coverage remain unfinished.

### 11.124 Ordinary assembly resolved prefixes

*Status: Native shared-model dispatch and bounded corpus prefixes; assembly resolved suffix incomplete · Evidence: [C-139], [C-140]*

The ordinary modern assembly resolved virtual at `4B694B00` enters scope 2 through `4C21E6B0` and calls the shared model serializer `4C2411C0`. That serializer first calls resolved concept `4C18AD80`, then the atom manager at model offset `+D80`, ghost manager at `+D88`, and relation manager. The part-specific relation-preservation predicate does not select its preservation branch for ordinary assembly roots. The archive retains the complete lightweight class/object map: resolved pointers can refer back to earlier objects.

`assemblyresolved.py` composes the already documented shared records in the following wire order. `modelprefix.read(..., root_class='moAssembly_c', assembly_resolved_stage=...)` selects an inclusive stopping stage and completes the assembly lightweight section before entering it. Other roots or unknown stages are rejected.

| Stage | Serialized continuation |
|---|---|
| `concept` | i32 resolved byte count and direct resolved node list (11.25). |
| `atom_manager` | Direct atom manager, retaining raw flags and tagged atom identities. |
| `ghost_manager` | Direct ghost manager, packed count/flags and ghost-body records. |
| `relations` | Tagged `moRelMgr_c`, using the existing archive map. |
| `details` | Direct object array at `+B00`, then direct `moDetailDefs_c`; the latter reserves no object-map slot. |
| `environment` | Shared post-detail parameters, environment folder and following units/list fields. |
| `views` | Ten tagged shared-model views in their established order. |
| `postviews` | Feature colors, dynamic node list, sketch-block manager, annotation views and conditional maps. |
| `arrays` | Eight direct object arrays in model order. |
| `layers` | Tagged layer-manager pointer. |

The assembly-local registry installs the complete reference-plane readers from 11.100 and the tagged detail-definition body where required. It does not replace the archive map. Every completed stage records its start, end and unknown-byte delta; an overrun of the declared resolved end is an error. `end` remains the declared resolved boundary, while `prefix_end` is the actual parsed position. Reaching a stage does not imply that the resolved section is complete.

All **174** supported ordinary modern assembly configurations reach the relation-manager boundary: **1,140,850 cumulative prefix bytes**, including **736,688 unknown bytes**. The resolved prefixes alone span **21,224 bytes**. All inline resolved feature lists and atom managers are empty; ghost managers contain **61 bodies** across 15 configurations, and two configurations contain six relations each. Empty resolved feature lists do not imply an assembly has no features elsewhere.

Edition 0.124 reached the layer-manager pointer in **169 of 174** configurations: **4,725,712 cumulative prefix bytes**, including **3,288,523 unknown bytes**. In edition 0.124 the other five stopped explicitly at a new `moCameraFeature_c`; their partial cursors received no successful-prefix credit. Clause 11.125 supplies this dependency and advances the current report to all 174 configurations. The native reports retain input and prefix hashes, object-map sizes and individual stage spans. Three older configurations and six routing roots remain outside this ordinary modern scope. Shared-model continuation after layers, assembly-specific resolved fields, raw field meanings, drawing/project schemas and full-document bit coverage remain unfinished.

### 11.125 Modern camera features and attachment points

*Status: Native serializer/constructor evidence; seven complete camera bodies in five assembly configurations · Evidence: [C-140]*

Scope: ordinary external-XML model archives, version greater than `27CB`, newly constructed `moCameraFeature_c`. Its primary vtable `4CE8C080` has Serialize at slot 5, `4B7B7120`. Both recovered constructors initialize `+5D0` to zero; after the `moModelFeature_c` base, the reader therefore invokes direct attributes `4B7B67F0` at `+378` with arguments `false, true`. The runtime reuse branch selected by a nonzero `+5D0`, older camera layouts and independent attribute serialization are outside this schema. The constructor selector is not an extra wire field.

A camera body consists of the model-feature base (feature body plus raw i32 `+354`) followed by these direct attribute fields. Offsets below are relative to the attributes record, not to the enclosing camera. The direct records reserve no MFC map slots.

| Order | Attribute offsets | Wire fields |
|---|---|---|
| 1 | `10,14,18,1C,20,24` | Six i32 words. |
| 2 | `88,90,98,B8` | Three f64 values, then i32. |
| 3 | `C0,C8` | Tagged attachment pointer, then f64. |
| 4 | `140,A0,160` | i32, point (3 f64), i32. The native copy from `A0` to `148` consumes no additional bytes. |
| 5 | `168,170` | Tagged attachment pointer, then f64. |
| 6 | `D0,D8,E0,E8,EC,F0` | Three f64 values, two i32 words, tagged reference pointer. |
| 7 | `F8` | Direct faceted reference, defined below. |
| 8 | `1D8,1E0,1E8,1F0` | i32, three f64 values. |
| 9 | `1F8` aspect specification | Two f64 values (width/height inputs), then i32 mode. Preserve zero denominators and raw inputs; native recomputation of a ratio consumes no bytes. |
| 10 | `218,220,228` | i32, f64, i32. |
| 11 | `230,238,240` | Tagged attachment pointer, f64, f64. |

The three attachment/value pairs use helper `4B7BBB90`: in this version scope each stores a tagged object and an f64. The loader accepts `moAttachPoint_c` derivatives and discards incompatible runtime types; the schema preserves the wire object when a complete reader exists, without fabricating a replacement. `F0` uses helper `4B7BBC60`, a tagged object without a following f64. Raw scalar meanings remain open and retain unknown-byte accounting.

The direct faceted reference (`4B7B5F90`) begins with a u32. Preserve all bits: low four bits select the payload, bit 4 is the stored enable flag, and bit 5 supplies the direction reversal flag for kind 3. After the selected payload comes one CString in every supported modern version.

| Low four bits | Payload before final CString |
|---|---|
| 0 | None. |
| 1 | i32 point count followed by that many points. |
| 2 | i32 point count followed by that many points, then i32 edge word. |
| 3 | Vector, then point. |
| 4 | Point, then vector. |

Negative or out-of-bounds point counts are errors. Other low-nibble values are unsupported rather than inferred from surrounding bytes. The native writer emits kinds 0–4; the schema retains the original u32 including higher bits. No runtime hit objects, transforms or computed identities created during load add bytes to this direct record. The final CString remains of unresolved application meaning.

A modern `moAttachPoint_c` body (`4C27A640`) stores two points (`+08,+28`), i32 (`+20`), a tagged drawing-view pointer (`+50`), two more points (`+58,+70`), and u8 (`+88`). Its ordinary model context has neither condition 1 nor condition 2 set; special contexts can add a CString and remain unsupported. `moParametricPoint_c` (`4C280A80`) adds two f64 values (`+108,+110`), a tagged reference (`+118`), u16 (`+124`) and i32 (`+130`). For versions greater than `335C`, drawing roots additionally store five points; the present reader requires an explicit part/assembly root and rejects drawing/unknown roots before consuming bytes. Retain object identities and raw words rather than normalizing them to runtime booleans.

`camerafeatures.py` installs through the archive-local environment registry. The model prefix supplies its explicit root class. The native assembly corpus now parses all **174** ordinary modern configurations through the layer-manager pointer: **4,878,787 cumulative prefix bytes**, including **3,394,014 unknown bytes**. It contains **seven** camera bodies totaling **4,570 bytes**, with **3,945 unknown bytes**; one camera has a parametric attachment, and all seven faceted records have kind zero. Populated faceted kinds 1–4, repeated attachment identities, raw flags, invalid counts and truncation have native code plus synthetic tests. Complete resolved assembly suffixes and full-document coverage remain open.

### 11.126 Assembly shared reference tail and resolved suffix

*Status: Native root predicates and serializers; 174 shared-model ends; exact stream coverage expanded in 11.127 · Evidence: [C-141], [C-142]*

Scope: ordinary modern `moAssembly_c` roots, version greater than `27CB`, with the inherited lightweight archive map. The shared resolved serializer `4C2411C0` uses virtual `+1A0` to select part-only fields. The primary part vtable `4D126A00` resolves this slot to `4B29DB10` (`mov eax,1; ret`); the assembly vtable `4CE6AA80` resolves it to `4B29C5A0` (`xor eax,eax; ret`). The difference changes byte consumption and cannot be implemented by simply passing assemblies to the former part-only reader.

After the tagged layer manager, assemblies store the direct forwarded-feature array at `+238`, tagged permanent-mark record, raw i32 `+1008`, and an i32 atom-map count followed by tagged atom/i32 value pairs. Next is the node-flag-selected ID map from 11.115: preserve the original node flags, count and ordered rows. Assemblies omit the `+100C` word and its optional sheet-metal master, and omit both `+10FC/+1100` and `+1120/+1128` dimension-increment groups. These fields are absent, rather than zero-valued placeholders. The remaining shared model tail follows 11.116, including its existing version gates and nested-map rules.

`modelreftail.py` now accepts explicit ordinary part or assembly roots and rejects other roots. `assembly_resolved_stage='reference_tail'` and `'model_tail'` select inclusive boundaries after these continuations. Invalid or normalized boolean node flags are rejected before entering the new assembly stages. All **174** supported ordinary assembly configurations complete both continuations. Their reference tails total **2,784 bytes**, with **696 unknown bytes**: every forwarded array, atom map and ID map is empty, and all ID maps select the wide mode. Populated map forms remain covered by the existing native code and synthetic tests. Cumulative prefixes through the shared model tail total **4,923,967 bytes**, including **3,411,294 unknown bytes**.

The assembly resolved virtual `4B694B00` then stores the following suffix. All historical gates controlling these fields are already satisfied within the supported version scope. Tagged records retain normal class/object map allocation; direct arrays/lists reserve no map slots.

| Order | Assembly offset | Wire field |
|---|---|---|
| 1 | `1558,1568` | i32, u16. |
| 2 | `1598,1600,1740,1738` | Tagged object array, mirror-assembly data, object array, object array. |
| 3 | `1748,1590` | Two i32 words. |
| 4 | `1588,1860` | Tagged exploded-view manager and assembly timings. |
| 5 | `1A20` | i32. |
| 6 | `18C0,18E0` | Two direct object arrays, proved by assembly constructors `4B69B740`/`4B69C010`. |
| 7 | `1708` | i32 smart-replacement presence. A nonzero value selects a further payload in `LoadSmartReplacementData` (`4B67D8A0`); this payload remains unsupported and causes an explicit stop. |
| 8 | `1938` | i32 presence; any nonzero value introduces two direct `suObList` bodies (`1940,1988`). Preserve the original presence word. |
| 9 | `166C` | Final i32. |

`moAssemblyTimings_c` (`4B680E70`) stores four i32 words. The current native writer emits zeros and the loader discards the values; the schema retains all four raw words. `moMirrorAssemblyData_c` (`4B689CD0`) stores a tagged external object, two direct transforms, a plane root point and a plane normal. Neither record adds wire data for the ordinary zero-byte `su_CObject` base.

`moExplViewManager_c` (`4B6842F0`) has the same modern framing as the part manager in 11.117: feature base, tagged views array, tagged ID, f64 and two i32 words, then direct packed-count `(u16 key, tagged value)` map. Its constructor `4BAB5B70` proves that the embedded `+360` member is `su_CMapWordToOb`. `moExplId_c` uses the same serializer `4B684190` as the part ID: CString then i32. These are archive-local aliases to the existing complete readers, not new global registrations.

An assembly exploder (`moAsmExploder_c`, `4B680A00`) stores the model-feature base, two i32 words (`360,364`), tagged array (`368`), another tagged array which native load converts to a runtime set (`378`), f64 (`380`), i32 (`388`), tagged array (`390`), a direct ten-f64 box (`3A8`), then a direct list (`3B0`). Retain the converted/discarded array and its pointer identity. The constructor `4BAB5A80`, reached through the retained runtime-class factory, allocates the box and constructs the list; there is no box-presence word in this wire layout.

`moCompXform_c` (`4B682E70`) stores a direct transform followed by a required nonnull component pointer. Modern `moRefXform_c` (`4B68B420`) stores those fields followed by a second direct transform. Preserve the tagged component identity.

A component explode step (`moCompExplodeStep_c`, `4B682450`) first calls the shared assembly step serializer `4B6847C0`. In modern scope that base stores the feature body, tagged direction (`368`), f64 (`370`), second tagged direction (`378`), f64 (`380`), i32 (`388`), tagged array (`390`), two i32 words (`398,39C`), vector (`3C0`), direct transform (`3D8`), vector (`460`), point (`478`), point (`510`), three vectors (`528,540,558`), point (`570`), and three vectors (`588,5A0,5B8`). The derived step adds a required component pointer (`608`) and two tagged arrays (`610,618`). Preserve the two serialized direction pointers independently even when native load subsequently synthesizes a missing runtime direction.

`moExplodeLine_c` (`4B684570`) shares the modern part-line framing from 11.118: node lightweight body, required step pointer, two points, two f64 values, three i32 words and two attachment pointers. Cyclic references to an enclosing step retain the existing map identity. `moCompHolder_c` reuses the component-reference body already established in 11.118. `moPlaneRef_w` reuses the direct plane-reference body in 11.100. `moCompRefAxis_c` (`4BBBA440`) stores the component-feature body followed by a tagged reference-geometry pointer; unlike the component-plane record, it adds no final `+100` word. Geometric wrappers accept this native component-axis context, while unsupported nested geometry still stops explicitly.

`assembly_resolved_stage='assembly_tail'` selects the complete supported suffix and requires both the declared resolved end and physical stream end to equal the final cursor. A changed size word or appended bytes is an error. Every successfully decoded nested suffix object records its body span, unknown-byte delta and class; the corpus audit hashes those spans.

Edition 0.126 completed **171 of 174** ordinary modern configuration streams: **5,373,729 cumulative bytes**, including **3,684,148 unknown bytes**. The assembly-specific suffixes alone span **529,113 bytes**, of which **327,823 retain unresolved meanings**. All 171 observed smart-replacement and final-list presence words are zero; nonzero final-list presence is tested synthetically, and nonzero smart replacement is rejected. Completed graphs contain 171 exploded-view managers, 41 assembly exploders, 503 component transforms, 128 reference transforms, 244 component explode steps, 488 direction specifications, 250 component holders, 659 explode lines, 142 plane references, eight exploded-view IDs and 171 timings records. Mirror/axis bodies have native code and synthetic checks but no successful populated record in this report.

Edition 0.126 stopped two configurations at `moExplodeLineProfileFeature_c` and one at `moFaceBlendSurfIdRep_c`; clause 11.127 supplies those dependencies and completes all 174 supported streams. Six routing roots and three older configurations remain excluded from this modern ordinary reader. This completes configuration framing only where the reader succeeds; raw meanings, other streams, drawings and all installed project formats still require further work before full-document bit coverage can be claimed.

### 11.127 Exploded-line profiles and complete ordinary assembly configuration framing

*Status: Native dependency serializers and 174 exact configuration stream ends; semantic coverage remains partial · Evidence: [C-142]*

`moFaceBlendSurfIdRep_c` has primary vtable `4CF6B918`, Serialize slot 5 at `4BBBDE70`. It calls the common entity-representation serializer `4BBBBF50` and adds one i32 (`+38`). Its ordinary modern body therefore matches `from_sketch_entity` in 11.18: tagged feature pointer, tagged external feature-reference pointer, raw i32. `modelboundary.py` registers this verified alias, and the assembly suffix retains its normal body-span tracking. There is no additional face-blend-specific array or object-map slot.

The other former stops are `moExplodeLineProfileFeature_c` objects, primary Serialize `4BB24BE0`. They first call the complete profile serializer `4BB2F910`, whose ordinary non-reused layout is defined in 11.82–11.87. Every supported version here is greater than `27AB` and adds an i32 automatic-element count after that profile body, followed by that many tagged `moAutoExpldeLineElementData_c` pointers. This array is direct and reserves no class/object map slot itself. Preserve nulls, duplicate pointers and order. Negative or out-of-bounds counts are rejected. New automatic-element bodies remain unsupported; null or existing pointers retain the normal archive-map behavior.

`assemblyprofiles.py` composes a complete supported sketch reader for the profile dependency. It follows the established direct collection order: points; lines; texts; hatches; arcs; ellipses; conics; splines; surface collections; the three valued-dimension collections; offsets; logical groups; isoparametric state; linear patterns; following pattern constraints; then the complete ordinary sketch tail. Each collection keeps its existing version gates and dependency checks. The profile header, chains, geometry, contours, placement and suffix use the existing complete profile reader. The shared reused-profile branch follows 11.161; unsupported nested records still raise an explicit error.

The sketch-tail helper is newly constructed with profile mode disabled, as in the ordinary part path. The composed reader selects that mode for this nested sketch and restores any outer mode on both success and failure. It does not consume an extra profile-feature pointer at the sketch end. Every completed sketch records its start, end and unknown-byte delta, independently of its surrounding profile.

Two additional native classes occur in these sketches:

| Class | Primary Serialize | Modern ordinary body |
|---|---|---|
| `mo3DPlaneBackedUpData_c` | `4C279D70` | Point (`+08`), vector (`+20`), vector (`+38`): 72 bytes. The ordinary `su_CObject` base consumes no bytes. Field roles beyond their types remain unresolved, so these bytes retain unknown accounting. |
| `sg3DPlaneHandle` | `4C591ED0` | The existing `sgEntHandle::Serialize` body: u16 entity index, with `777F` followed by i32 as the escape, then i32 reference ID and i32 dimension-on-CM word. No plane-specific suffix. |

These dependencies are installed in the profile's archive-local registry. Existing object identities remain intact across the containing exploded view, profile, sketch and following suffix. The complete-prefix audit retains separate sketch hashes and spans; no raw scanning or guessed padding is used to find the next record.

All **174** supported modern ordinary assembly configuration streams now end exactly at both the declared resolved boundary and physical stream end: **5,491,729 bytes**, of which **3,761,424 remain of unresolved meaning**. Their assembly-specific suffixes total **567,762 bytes**, with **350,130 unknown bytes**. The completed corpus includes three face-blend ID representations and two exploded-line profiles. The profiles span **18,142 bytes**, including **9,630 unknown bytes**; their sketches span **17,308 bytes**, including **9,256 unknown bytes**, and contain 28 points, eight lines and four planes in total. Both automatic-element lists are empty. Populated lists of null/existing pointers, escaped plane indices, raw face-blend suffixes and truncation have native-code evidence plus synthetic tests.

The 183 observed assembly configurations include six routing roots covered separately in 11.128 and three older configurations outside the modern reader's scope. Exact parsing of the supported configuration streams does not establish the meanings of every stored field or cover the remaining streams in an assembly. Drawings, all installed project formats, older records, raw routing field meanings, optional automatic-element bodies and full-document bit coverage remain unfinished.

### 11.128 Modern routing assembly roots and trailer

*Status: Native root/callback/import evidence and six exact routing configuration stream ends; field meanings remain partial · Evidence: [C-144]*

Scope: ordinary external-XML `moRouteAssembly_c` roots with version greater than `27CB`. The class is implemented in `sldrtmou.dll`, not in `sldmodu.dll`. Its primary vtable `34C0E400` has Serialize at slot 5 (`34A98D10`). That serializer first calls `moAssembly_c::Serialize`, then writes its routing-specific trailer. The trailer lies after the inherited assembly resolved section; it is not part of that section's size word.

The inherited virtual callbacks are import thunks: `+4B8` (`34BA4DEC`) resolves to ordinary assembly lightweight `4B691960`, and `+4C0` (`34BA4DE6`) resolves to ordinary assembly resolved `4B694B00`. Virtual `+1A0` resolves through `34BA3544` to `moConcept_c::isPart` (`4B29C5A0`, returning zero). Raw named export-table lookup establishes those targets without relying on a truncated ordinal inventory. Routing roots therefore use the same shared/assembly field order as 11.123–11.127 and omit the same part-only reference-tail fields.

The routing-specific trailer has these fields. All native historical gates in the last column are satisfied within the supported modern scope; this reader does not infer the older assembly layout from them.

| Order | Routing offset | Wire field | Native load gate |
|---|---|---|---|
| 1 | `1A90` | Raw i32. | Version greater than `0465`. |
| 2–4 | `1A98,1AA0,1AA8` | Three CStrings. | Version at least `0F73`. |
| 5 | `1AB0` | Fourth CString. | Version at least `0F87`. |
| 6–7 | `1AC8,1AD0` | Two `su_CTime` archive words, u32 each. | Version at least `0F87`. |

The time extractor `swccu.dll:31D4BC60` reads one u32 through `31D343B0`. It widens `FFFFFFFF` to the in-memory invalid-time sentinel without consuming another word. Preserve the wire values rather than replacing them with their runtime representation. String roles, the i32's meaning and both time roles remain unresolved. `routeassembly.py` therefore retains unknown-byte accounting for the entire trailer. No trailer field reserves an MFC class/object-map slot.

`modelprefix.read(..., root_class='moRouteAssembly_c', assembly_resolved_stage='assembly_tail')` now composes the inherited assembly sections and this trailer. The ordinary assembly path still requires its resolved end to equal stream end. The routing path first requires the parsed inherited assembly suffix to equal the declared resolved end, then reads the trailer and independently requires physical stream end. `resolved['prefix_end']` remains the inherited resolved boundary; `route_tail` records the trailer's separate start/end/unknown span. Partial routing stages stop at their requested inherited boundary and do not read the trailer.

The archive-local unit-component reader accepts the derived routing root as an assembly owner. Chain-component owner checks, ordinary model reference tails, assembly suffix guards and model attachment contexts likewise accept the verified derived assembly context. Default part unit-component behavior and rejection of part-only stage selections on assembly roots remain intact. The existing archive map is retained across all stages and the trailer.

All **six** installed routing configurations now parse to their exact stream ends: five at version 19000 and one at 18000. Their complete streams total **180,028 bytes**, including **122,002 unknown bytes**. Trailers total **728 bytes**, all retaining unknown accounting. Five trailers are 28 bytes with four empty UTF-16 strings; one is 588 bytes with string lengths 59, 113, 108 and zero code units. The completed graphs contain 44 ordinary reference bodies and six each of exploded-view managers and timing records.

Together with the 174 ordinary configurations in 11.127, **all 180 modern assembly configuration streams** in this corpus now have exact lightweight, resolved and final root boundaries. They total **5,671,757 bytes**, including **3,883,426 unknown bytes**. Three version-10000 ordinary assembly configurations remain outside this reader. Exact configuration framing does not supply missing field meanings or account for other streams, drawings or project documents; the full-document coverage objective remains incomplete.
