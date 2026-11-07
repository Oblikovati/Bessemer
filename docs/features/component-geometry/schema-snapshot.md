### 11.160 Component axes, sketch regions and tagged text records

*Status: B — native ordinary modern record framing and populated corpus boundaries verified; raw meanings and special contexts remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. These bodies share the current MFC object map and class/object pointer tags. Native member offsets in the tables identify fields rather than wire positions. `moCompRefAxis_c` primary vtable `0x4CD8FCB8` selects serializer `0x4BBBA440`; `moSketchRegion_c` primary `0x4CEDA838` selects `0x4B96CCC0`; `sgTextHandle` primary `0x4D1D31F0` selects `0x4C591ED0`. All three bindings are in the hash-qualified sldmodu.dll of C-188.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | component-axis inherited feature | component-reference body; i32 feature ID; u32 feature time | always | Existing moCompFeature_c layout (11.17), through native helper `0x4BBB9A60`; no extra tag for the inherited layer. | V / ? |
| 2 | component-axis geometry +F8 | tagged object pointer | always after row 1 | Native accepts null or moRefGeom_c-derived geometry. Non-null load updates its runtime owner without another wire field. | V |
| 3 | sketch-region boundary entities +08 | tagged suObList pointer | always on modern load | Pointer framing, including class/map entries if newly defined, precedes the list's ReadCount and ordered tagged entries. This is not an inline direct list. | V |
| 4 | text-handle entity index +18 | u16; conditional i32 | always | The u16 is an unsigned index unless equal to `0x777F`; that sentinel introduces a signed i32 replacement. No range-based sign conversion in this modern scope. | V |
| 5 | text-handle reference ID +1C | i32 | always after row 4 | Preserve signed value. | V |
| 6 | text-handle DimOnCM +08 | i32 | always after row 5 | Preserve full stored word; no boolean coercion. | V / ? |

The component-axis empty inherited record is 82 bytes in the latest supported version, followed by the eight-byte feature ID/time and two-byte null geometry tag: 92 bytes before any outer class tag. The axis has no plane-only +100 word. The assembly resolved-tail reader and part feature reader use this same ordinary axis implementation. Component-axis loading before `0x2D1` uses a different conversion outside this scope. A native store condition can substitute a null geometry pointer, but does not remove its tag or add a presence word.

The sketch-region ordinary modern load unconditionally consumes its boundary-list pointer because the version exceeds `0x2398`. Native non-DB store emits that pointer only when its runtime list is non-null; the installed sample lists are all non-null. A synthetic null-pointer fixture therefore verifies parser framing without establishing that the native writer generates it. DB store uses a separately named inline object context and is excluded. The inherited binary su_CObject layer contributes no additional ordinary bytes in this scope.

The text-handle primary serializer slot is identical to the sgEntHandle primary slot (`0x4D1CF4E8`), so there is no text-specific suffix. Its body is ten bytes for an ordinary short index and fourteen for a sentinel/extended index. Readers SHALL preserve the `sgTextHandle` runtime identity, the unsigned short value, the signed replacement and cached pointer identities. Historical signed-short migration and missing reference/DimOnCM fields precede this scope.

The newly reachable text paths also contain tagged `utCharFormat_c` objects. Their primary vtable `0x3CF13B08` in sldmfcu.dll selects `0x3CA6C7C0`, the same serializer already specified for direct character formats in 11.101. A new tagged instance allocates its normal class/object map slots and then uses that direct body; a cached reference consumes only its pointer tag. There is no additional format presence word or nested class tag in the body. The version-gated raw +58 u32 remains present at `v >= 0x3EED`. Offline reading preserves stored font and numeric values before native font substitution or normalization.

The complete-root audit now retains **34 sketch regions**, **15 component-axis references**, **253 text handles** and **137 tagged character formats**. The region lists contain 22 one-entry, seven four-entry, four eight-entry and one five-entry cases, comprising 62 line, 23 arc and two spline handles. All 253 text handles have stored DimOnCM zero; all 137 tagged formats name Century Gothic. Populated native observations do not establish meanings for other raw values or validity of arbitrary synthetic combinations.

Edition 0.171 records **21,611 complete roots**, **574 list-complete configurations** and **332 explicit stops** across **906 complete modern model contexts**. **524 lists reach exact EOF**, spanning **27,346,909 decoded bytes**; **50 complete lists retain four-byte unknown tails**. Leading dependencies include 3D profiles, context-unit components, reference points, flat patterns and connection points. Part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these object boundaries.

Evidence: C-188.


### 11.163 Reference points and point-data variants

*Status: B — native modern feature/subtype framing and populated corpus boundaries verified; raw scalar meanings and special contexts remain partial.*

Scope is ordinary non-DB external-XML model archives with version greater than `0x27CB`. `moRefPoint_c` primary table `0x4D176A18` selects serializer `0x4C308960`, which calls the common reference-geometry serializer `0x4C283A00` before the following three tagged pointer occurrences. The inherited root point and reference geometry retain their existing layout. Offsets below identify native members rather than stream positions.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited reference geometry | common record | always | Existing body/atom reference-geometry composition from 11.100, including its versioned fields; no nested class tag for this inherited layer. | V / ? |
| 2 | point data +6F8 | tagged moRefPointData_c-derived pointer | always | Scoped reader requires non-null data. Preserve the actual runtime subclass and map identity. | V |
| 3 | profile +6F0 | tagged moProfileFeature_c pointer | always | Null/cached/new object framing; no separate presence byte. | V |
| 4 | surface reference +700 | tagged moSurfaceRef_w pointer | always | Native load reads a generic object pointer; native store uses the surface-reference overload. Preserve null and actual runtime identity. | V |

The modern point-data base uses serializer `0x4C308800`. After the zero-byte ordinary binary su_CObject layer, it reads no further fields at versions at least `0x3C2`; native store also emits none. A tagged base instance still consumes its normal MFC class/object tags and map entries. Readers SHALL NOT infer a point/vector body from its class name or omit the object-map entry because its payload is empty. The selected derived classes call this base directly and then append the following bodies, in the displayed wire order.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | moFaceCenterRefPointData_c | tagged single-face reference +10 | always | One pointer occurrence; all-null payload is 2 bytes. Serializer `0x4C308270`. | V |
| 2 | moCenterEdgeRefPointData_c | tagged curve reference +10 | always | One pointer occurrence; all-null payload is 2 bytes. Serializer `0x4C307FD0`. | V |
| 3 | moFaceEdgePointData_c | tagged curve reference +18; i32 +20 | always in this scope | Six-byte null-reference body. Preserve the raw word. Serializer `0x4C3082F0`. | V / ? |
| 4 | moFaceVertexProjectionPointData_c | tagged single-face +18; tagged point reference +20; tagged plane reference +28 | always in this scope | Three ordered pointer occurrences, including nulls and repeats; all-null payload is 6 bytes. Serializer `0x4C3083F0`. | V |
| 5 | moTwoEdgeRefPointData_c | tagged curve references +10/+18; i32 +20/+24 | always in this scope | Preserve both pointers and raw words; all-null payload is 12 bytes. Serializer `0x4C308D30`. | V / ? |
| 6 | moAlongCurveRefPointData_c | tagged curve +10; f64 +18/+28; i32 +20/+30; tagged moDisplayDim_c +38; i32 +34/+40 | always in this scope | Exact order differs from member-offset order. All-null payload is 36 bytes, including 32 raw scalar bytes. Serializer `0x4C307CD0`. | V / ? |

The gates for the selected modern derived fields precede this scope: the projection's final plane pointer begins at `0x969`, the two-edge final word after `0x97F`, and the along-curve final word at `0x1104`; other along-curve additions also precede the supported minimum. Historical base point/vector records, pre-`0x3C2` extra derived references and pre-`0x9A2` feature-level dimension/discarded-word conversions are excluded. Native data-owner updates, feature refresh and ordinary DB-only node work do not add binary fields here. Unsupported nested reference/kernel forms remain explicit stops.

`moCompRefPoint_c` primary `0x4CD90198` selects shared serializer `0x4BBBA440`, already used by the component-axis reference in 11.160. Its ordinary body is the complete component-reference base, i32 feature ID, u32 feature time and a tagged moRefGeom_c-derived geometry pointer. It has no plane-only +100 word. The scoped wrapper admits this proven moCompRef_c-derived class without an entity-handle sketch-pointer extension. Latest-version null-base composition totals 92 bytes before the outer class tag. The offline reader preserves stored pointers rather than resolving runtime geometric ownership.

The complete-root audit retains **36 reference-point features**: 11 face-center, ten along-curve, eight face/vertex projection, four center-edge and three two-edge data objects. All 36 feature-level profile and surface pointers are null. It also retains **15 component-point references**, all with null stored geometry pointers. Modern base-only and face/edge data bodies are bound to native code and exercised synthetically; they are not claimed as populated corpus variants. Per-root summaries preserve raw subtype scalars and pointer classes independently of feature completion.

Edition 0.174 records **21,942 complete roots**, **607 list-complete configurations** and **299 explicit stops** across **906 complete modern model contexts**. **553 lists reach exact EOF**, spanning **28,542,701 decoded bytes**; **54 complete lists retain four-byte unknown tails**. Leading dependencies include flat patterns, connection points, spline tangency controls, endpoint references and weldment features. Part, assembly, drawing and all installed-project bit coverage remains incomplete; these boundaries receive no whole-document coverage credit.

Evidence: C-191.

