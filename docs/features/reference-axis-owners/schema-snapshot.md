### 11.146 Reference-axis features and derived axis data

*Status: B — ordinary modern feature/data framing; nested unsupported reference bodies and raw parameter meanings remain partial.*

moRefAxis_c primary table `4D1588B0`, locator `4D358E78`, selects serializer `4C2828C0`. Its concept hooks select the common lightweight/resolved serializers. Runtime descriptor `4DAEE6F0` selects factory `4C270C20`, whose constructor call selects `4C289350`. The constructor calls moRefGeom_c, initializes runtime state and clears the +6F8 data pointer. The 0x700 memory allocation is not wire size.

In ordinary non-DB external-XML loading with v > 0x27CB, the axis body is the complete shared reference-geometry body from 11.100/11.139, followed by one **tagged nonnull moRefAxisData_c-family pointer** at +6F8. It has no derived body-feature extrusion/revolution/rib suffix and no independent axis-data presence word. The native load path reads the data pointer, assigns its in-memory owner at data member +48, then invokes moNode_c::SerializeDBData. That database/deferred-store operation adds no fields to this ordinary archive body; separate database semantics remain open.

The shared data class moRefAxisData_c has primary table `4D1630C8` and serializer `4C282690`. Its modern base body is exactly **112 bytes**, in this order:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | point +18 | Direct mgPoint | Always in modern scope | Three f64 values | B |
| 2 | point +30 | Direct mgPoint | Always in modern scope | Three f64 values | B |
| 3 | raw +08, +10 | f64[2] | Always in modern scope | Preserve raw values; meanings unresolved | B |
| 4 | vector +50 | Direct mgVector | Always in modern scope | Three f64 values | B |
| 5 | point +68 | Direct mgPoint | Always in modern scope | Three f64 values | B |

The owner member +48 in **memory** is absent from this wire body. Readers SHALL NOT insert an owner tag, padding or another word between fields 2 and 3 or before field 4. Point/vector types are established by the native mgPoint/mgVector save/restore calls; no endpoint, unit or raw-double interpretation is inferred from offsets alone. The old point-pair omission in versions 0x89 through 0x92, the raw-pair gate at 0x256, vector gate above 0x646 and final-point gate at 0x808 all precede the supported modern scope.

The five installed concrete data classes inherit this entire base and append these tagged pointers, without extra presence words:

| Data class | Primary table | Serializer | Pointer suffix, in file order | St. |
|---|---|---|---|---|
| moSurfaceAxisData_c | 4D1632B8 | 4C286630 | +80 surface-reference object; native load uses the generic object operator | B |
| moPlanePtAxisData_c | 4D163C68 | 4C281620 | +80 moPlaneRef_w; +88 moPointRef_w | B |
| moPlaneInterAxisData_c | 4D163980 | 4C281400 | +80 moPlaneRef_w; +88 moPlaneRef_w | B |
| moLineAxisData_c | 4D163888 | 4C27EF60 | +80 moLineRef_w-family pointer | B |
| moConFaceAxisData_c | 4D1631C0 | 4C27B560 | +80 moCompFace_c | B |

Native RTTI proves that each concrete data class derives from moRefAxisData_c. Their serializers call the base before the listed suffix. Pointers retain the inherited archive map, including duplicate plane references and derived line/surface-reference classes. Tagged data objects allocate their normal MFC slots; the native post-read owner assignment adds no serialized pointer and does not require resetting or merging those slots. The offline reader preserves the serialized graph rather than injecting a new wire owner field.

The scoped installer adds the feature, base data and five concrete data readers locally while preserving explicit probes by default. Unsupported nested reference classes/bodies continue to stop the containing graph. In particular, reaching a surface-axis data pointer does not authorize skipping an embedded reference kernel body. Remaining failures after this extension include such previously unsupported reference bodies, so not every axis-containing graph is complete.

The installed audit retains **35 freshly serialized axes** in completed roots: 11 surface-axis, nine plane/point, seven plane-intersection, six line-axis and two moConFaceAxisData_c objects. Their suffixes include moCylinderRef_w, moPlaneRef_w, moPointRef_w, moTempAxisRef_w, moLineRef_w and moCompFace_c pointers. A hash-bound native regression replays a two-plane data case through the containing root's exact end. Synthetic tests cover the 112-byte base and raw negative-zero/NaN bits, every derived pointer order, duplicate inherited identity, two complete axes sharing a tagged data object, the absence of a serialized runtime owner, truncation and null/version/context rejection. No positive installed base-only axis-data object is claimed.

The edition 0.157 frontier records **19,662 complete roots**, **377 list-complete configurations** and **529 explicit stops**, with all **906 modern model contexts** complete. **371 lists reach exact EOF**, spanning **14,852,249 decoded bytes**; six retain four-byte unknown tails. No reference-axis or listed axis-data class stop remains at this frontier. Leading dependencies are shells (37), moOneContour_c (33), circular patterns (32) and hole-wizard features (27). Full part, assembly, drawing and installed-project bit coverage remains incomplete.

Evidence: C-174.


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

