# Approved schema snapshot

### 11.164 Endpoint references and spline tangency controls

*Status: B — native modern layouts, direct-handle types and populated corpus boundaries verified; raw scalar meanings and special contexts remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`, with sketch-control archive conditions 1, 2 and 4 clear. `moEndPointRef_w` primary table `0x4D17D008` selects `0x4C27D8F0`; the serializer calls point-reference helper `0x4C281C60`, which calls common reference wrapper `0x4C2842D0`. Thus the endpoint inherits the complete wrapper from 11.16 and its three-f64 point at +A8 before the following suffix. The runtime class and cached pointer identities remain distinct from the base point-reference class.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited point reference | reference wrapper; point | always | Existing common wrapper, including conditional transforms and versioned raw words, followed by three f64 point coordinates. No additional class tag for the inherited layers. | V / ? |
| 2 | raw +E8 | f64 | always in this scope | Preserve the stored value. Historical load before `0x1FB` reads an i32 discriminator instead; that conversion is excluded. | V / ? |
| 3 | raw +F0 | i32 | always in this scope | Preserve complete signed word. Historical load omits it through `0x726`. | V / ? |

The latest-version null-component, no-transform endpoint body is 98 bytes before an outer object/class tag: 62-byte wrapper, 24-byte point and 12-byte raw suffix. Wrapper version gates still apply; this fixed example is not a replacement for their grammar. The native base resets a runtime +E0 pointer without consuming an archive field. The suffix values are not assigned parameter-range or boolean semantics solely from the runtime class name.

`moSplineTangencyControl_c` primary table `0x4D1CF2A0` selects serializer `0x4C5C2690`. It calls derivative-control helper `0x4C5B2260`, which calls sketch-control helper `0x4C5C4690`. In the stated ordinary mode the common sketch-control helper reads exactly one tagged sketch pointer. The derivative helper then serializes two embedded handles directly, before the tangency point/vector suffix.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | sketch +08 | tagged sgSketch pointer | always in this scope | Preserve null or actual cached/new object identity. Special conditions can source the owner from runtime context instead and are excluded. | V |
| 2 | spline handle +40 | direct sgSplineHandle body | always | Existing entity-handle grammar: unsigned u16 index, signed i32 replacement only for sentinel `0x777F`, then i32 reference ID and i32 DimOnCM. No class tag or object-map entry. | V / ? |
| 3 | point handle +18 | direct sgPointHandle body | always after row 2 | Same handle grammar, read as a separate record even when values match the spline handle. Native member-offset order does not determine wire order. | V / ? |
| 4 | point +128 | three f64 | always | Preserve all coordinates; geometric role beyond the native point type remains partial. | V / ? |
| 5 | vector +140 | three f64 | always | Preserve all coordinates without normalization. | V / ? |

The null-sketch, short-index tangency body is 70 bytes. Each escaped handle index adds four bytes, so a two-escaped-handle body is 78 bytes. Neither direct handle allocates a map slot; only an outer tagged control and its tagged sketch pointer can do so. Native runtime-class descriptor `0x4DAF6970` selects factory `0x4C5C2130`, which allocates `0x160` bytes and calls derivative constructor `0x4C5B2020`. That constructor establishes sgPointHandle at +18 and sgSplineHandle at +40. Both handle primary serializer slots select `0x4C591ED0`, establishing the shared direct-body grammar. The factory's other points/defaults, the sketch watcher and runtime validity state add no wire fields to this serializer.

Native load assigns the sketch to the embedded handles, checks runtime entity resolution and creates watcher state. Offline reading preserves the two stored handle records without resolving sketch entities or executing watcher updates. Historical tangency coordinate transformation before `0x96C` and nonordinary sketch-owner modes are excluded. Complete pointer framing does not establish validity of arbitrary synthetic index/reference combinations.

The complete-root audit retains **14 endpoint references** and **40 spline tangency controls**. All endpoints reference moCompEdge_c; +E8 is 0.5 in 13 cases and 0.0 in one, while +F0 is zero in all 14. All 40 controls have sgSketch owners. Per-root summaries retain both direct handle records and point/vector coordinates; synthetic tests exercise escaped indices, signed extrema, equal-but-separate direct records, map accounting and exact/truncated boundaries.

Edition 0.175 records **22,083 complete roots**, **618 list-complete configurations** and **288 explicit stops** across **906 complete modern model contexts**. **564 lists reach exact EOF**, spanning **29,438,247 decoded bytes**; **54 complete lists retain four-byte unknown tails**. Remaining dependencies include flat patterns, connection points, weldment features, coordinate systems and draft features. Part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these record boundaries.

Evidence: C-192.

### 11.209 Spline control-polygon and curvature controls

*Status: B / ? — native ancestry/base dispatch, direct-handle construction and 12 populated records of each control in complete containing roots verified; full spline subtype and document coverage remain partial.*

moSplineControlPolygonControl_c primary table `0x4D1CC130`, locator `0x4D3675E0` selects serializer `0x4C5B1180` and has moSketchToolsUserObject_c ancestry. Runtime descriptor `0x4DAF6518` records allocation size `0x40` and factory `0x4C5B0FA0`. The factory calls base constructor `0x4C5C4450`, installs the polygon-control primary table, then constructs an sgSplineHandle directly at +18. Its shared handle serializer `0x4C591ED0` is independently bound by the existing endpoint/tangency evidence. Runtime allocation is not serialized length.

For ordinary non-DB external-XML archives with version greater than `0x27CB`, with native sketch-control conditions 1/2/4 clear, the complete polygon-control body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | sketch +08 | tagged sgSketch pointer | always in this scope | Shared sketch-tools helper `0x4C5C4690` reads this pointer. Special condition modes that obtain a sketch from runtime context are excluded. | B |
| 2 | spline handle +18 | direct sgSplineHandle body | always | Existing sgEntHandle encoding: u16 index; if index equals `0x777F`, replace it with an immediately following i32; then i32 reference ID and i32 dimension-on-CM word. No handle class tag, object-map slot or sketch pointer is inserted inside this direct body. | B / ? |

The minimum complete body is 12 bytes with null/cached sketch and a short index, or 16 bytes with an escaped index. Native post-load iSetSketch assigns runtime ownership and consumes no additional archive fields.

moSplineCurvatureControl_c primary table `0x4D1CC250`, locator `0x4D367670` selects serializer `0x4C5B1650` and has moSplineDerivativeControl_c ancestry. Its serializer calls existing derivative helper `0x4C5B2260`, whose constructor-bound handles and wire order were already established for tangency controls. Its complete body in the same scope is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | derivative base | sketch pointer, direct spline handle +40, direct point handle +18 | always | Exactly the existing derivative-control body; two separate direct handle occurrences with independent escape-index encoding. | B |
| 2 | point +C0 | three f64 | always | Native mgPoint_c::restore. | B / ? |
| 3 | vector +D8 | three f64 | always | First native vector restore. | B / ? |
| 4 | vector +F0 | three f64 | always | Second native vector restore, independently retained. | B / ? |
| 5 | raw +108 | f64 | always | Preserve stored value; exact engineering role remains unassigned. | B / ? |

The curvature suffix is exactly 80 bytes, giving a minimum complete body of 102 bytes with short indices and null/cached sketch. Each escaped inherited handle adds four bytes. Native coordinate conversion before version `0x96C` is outside this modern scope; no runtime transform or watcher structure is serialized after row 5.

Both readers are installed only in copied scoped maps. The spline-control suite now has eight tests, including direct-handle short/escape indices, signed handle words, cached sketch/self identities, no-extra-map-slot accounting, exact following-byte boundaries, curvature point/vector/scalar order, truncation/context rejection, and source-hash-bound complete native containing-root spans with stored handle/geometry equality for each new class.

The audit retains **12 polygon controls** and **12 curvature controls** in complete roots. All polygon controls reference sgSketch and have reference ID -1 and dimension-on-CM word zero; eight use index 0 and four use index 4. Observed curvature spline/point index pairs are (3,13), (2,9) and (1,8), each four times. Raw curvature +108 is zero in eight records and 0.6260720325952998 in four. These observations do not establish index limits or scalar meaning.

The frontier now has **24,101 complete roots**, **753 complete lists** and **153 explicit stops** across 906 complete modern contexts: 17 additional complete roots relative to Edition 0.214, with unchanged list counts. **683 lists reach exact EOF**, spanning **44,802,506 decoded bytes**; **70 complete lists retain four-byte unknown tails**. All completed-list endpoint/tail invariants pass. The next observed failure includes unsupported sketch spline subtype mask `0x10`; no unsupported spline body is skipped or credited as complete. `resolved-feature-list-frontier-0.214.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-237.
