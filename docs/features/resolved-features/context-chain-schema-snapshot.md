# Context-chain schema snapshot

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

### 11.143 Ordinary context-chain components in part feature graphs

*Status: B — ordinary modern component/chain composition; external-owner modes and raw state meanings remain partial.*

moContextChainComponent_c primary table `4CE9DA10`, locator `4D31AE90`, selects serializer `4C22FCF0`. Runtime descriptor `4DAEB878` selects factory `4C21C7B0`, whose constructor call selects `4B83EFB0`. The constructor initializes the component/chain base, then installs the final context-chain table and initializes members +40/+48. Its 0x50 memory allocation is not wire size.

The supported scope is ordinary non-DB external-XML loading with v > 0x27CB and the existing ordinary chain-component owner limits. Serializer 4C22FCF0 first calls moChainComponent_c serializer 4C228AC0, which calls moComponent_c serializer 4C229CC0. The ordinary supported wire order is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | serialized owner | Tagged object pointer | Always | Null or supported part/assembly filed-owner reference; existing 11.122 base context | B |
| 2 | reference chain +20 | Tagged moReferenceChain_c pointer | Supported ordinary base path | Shared archive map | B |
| 3 | raw +30 | u32 | Supported modern base path | Existing chain-component raw word | B |
| 4 | external object +40 | Tagged moExtObject_c pointer | Always after the base | Not an i32 presence/body pair | B |
| 5 | raw +48 | i32 | Always in modern scope | Original gate v >= 0x38D; retain full value | B |

There is no unit-component u8 after the owner pointer. With short null/reference tags, the complete component body is 14 bytes, including the chain and context suffix. New pointed-to objects contribute their existing MFC class/object bodies and map slots; the component itself adds no list wrapper or count.

The serialized owner is not the same thing as the native component's external-owner member +08. moComponent_c::Serialize resolves supported filed-owner references without necessarily populating that member. The chain serializer's early return depends on the native external-owner state, not on whether the serialized pointer was nonnull. The reader therefore reuses the existing supported part/assembly/null-owner path and rejects external-owner contexts rather than selecting an early-return grammar from pointer truthiness. Constructor 4B83EFB0 initializes the external-owner member to zero.

The reference-chain and external-feature layouts are the existing ordinary schemas in 11.121–11.122. The local part reader installs their complete shared functions for moChainComponent_c, moReferenceChain_c, moExtFeatureList_c, moExtFeature_c and moReference_c alongside the new context-chain reader. It preserves explicitly supplied probes by default and does not alter global registration. A chain contains its existing tagged external-feature list and direct referenced-configuration fields; no extra part-specific list body is invented.

After reading +40/+48, the native loader can set filed ownership, resolve chain paths, replace external features or schedule reference-count checks. These runtime actions add no serialized suffix fields. Special condition-1/clipboard/import-owner handling and older component paths remain outside scope. Raw +48 is preserved as an i32 rather than reduced to a boolean or assigned an unproven state name.

The installed audit retains **182 freshly serialized context-chain components** in completed roots. All have moPart_c serialized owners, moReferenceChain_c chain objects, moExtObject_c +40 references and raw +48 equal to 1. Of their external-feature lists, 179 contain one entry and three contain two. The native regression replays a hash-bound two-feature case through the containing root's exact end. Synthetic tests distinguish the 14-byte null/reference body from unit-component framing, preserve inherited owner/chain/external-object identity and unsigned +30 bits, retain negative +48, reject truncation, and retain the explicit external-owner/version/context limits.

The edition 0.154 frontier records **19,122 complete roots**, **306 list-complete configurations** and **600 explicit stops**, with all **906 modern model contexts** complete. **300 lists reach exact EOF**, spanning **9,992,148 decoded bytes**; six retain four-byte unknown tails. No context-chain component stop remains at this frontier. The leading dependencies are mirror patterns (40), moPLine_c (39), moOneContour_c (33), shells (29), reference axes (29), hole-wizard features (23) and scale features (23). These advances do not establish full part, assembly, drawing or installed-project bit coverage, which remains incomplete.

Evidence: C-171.

