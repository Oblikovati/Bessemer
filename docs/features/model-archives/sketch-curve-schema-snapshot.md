# Sketch curve and surface schema reference

Edition 0.346 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.33 Direct sketch hatch and arc collections

*Status: Native-derived hatch framing with synthetic bodies; installed empty hatch counts and 47 arc bodies verified · Evidence: [C-48], F.36*

The text collection is followed by an i32 hatch count. Each entry is a direct sgHatch body with no category word or MFC object-map allocation. It starts with the entity base of 11.29. If flags_54 bit 1 is set, the body ends there. Otherwise, in the supported modern ordinary context it contains:

| Order | Fields | Encoding |
|---|---|---|
| 1 | four index arrays | each: i32 count, then count u32 values |
| 2 | crosshatch; hatch data | gcXhatch_c pointer; skHatchData_c pointer |
| 3 | contour count and records | i32 count; each record is i32 entity index and u16 category |
| 4 | contour chooser; object list; dword array | moContourChooser_c pointer; su_CObList pointer; su_CDWordArray pointer |

The native contour category chooses spline handles for 2, ellipse handles for 3 and arc handles otherwise. These contour records store index/category pairs, not serialized handle objects, in ordinary archives. Nonnull hatch dependencies require their own readers. Array roles, category domain and contour semantics remain partial. Readers SHALL preserve raw values and SHALL reject out-of-buffer counts. Special condition 2 stores an eight-byte contour-chooser representation instead of the pointer and is outside this scope.

The hatch collection is followed by an i32 arc count. Each arc entry starts with a u16 prototype mask: bit value 1 selects the external prototype; otherwise bit value 4 selects the underlying-offset prototype; otherwise the ordinary arc prototype is used. All restore the same direct body, without an MFC map allocation:

| Order | Arc fields | Encoding |
|---|---|---|
| 1 | common segment | 11.31 through NormalScale, without the line-specific vector/word suffix |
| 2 | Rotation | i32 |
| 3 | Center | u16; if 0xFFFF, followed by replacement i32 |
| 4 | Normal; FiletLine1; FiletLine2; VirtualPoint | four i32 values |

Save keys establish the names, including the native `Filet` spelling. Their enum/index domains and geometric meaning are not fully established. Readers SHALL retain the prototype mask and all stored values. The arc reader shares the nonminimal-owner ordinary modern scope of 11.29. Later sketch collections remain unspecified by this clause.

### 11.34 Direct ellipse and conic collections

*Status: Native-derived modern bodies and synthetic populated cases; installed empty-collection boundaries verified · Evidence: [C-49], F.37*

The arc collection is followed by an i32 ellipse count. Each direct ellipse entry has a u16 prototype mask, then the common segment body through NormalScale (11.31), i32 Rotation and five point indices: Center, MajorPoint, MinorPoint, MajorAntiPoint and MinorAntiPoint. Each index is independently encoded as u16, with an i32 replacement following 0xFFFF. There is no line-specific vector/word suffix. Bit value 1 selects an external prototype; otherwise bit value 4 selects the underlying-offset prototype; otherwise the ordinary ellipse is used. All restore the same body. Readers SHALL retain all mask bits and stored indices. Geometric/index domains remain partial.

The ellipse collection is followed by an i32 conic count. Each direct entry starts with a u16 prototype mask, the common segment body, then:

| Order | Conic fields | Encoding |
|---|---|---|
| 1 | Center; Apex; Shoulder; TopVertex | four i32 values |
| 2 | Eccentricity; Rho | f64; f64 |
| 3 | GeomType | u16 |

Conic indices have fixed four-byte storage; they do not use the ellipse/arc 0xFFFF escape. Bit value 0x40 selects a BConic prototype. External bit value 1 has priority over underlying-offset bit value 4 within either conic family. The same direct serializer restores the listed fields in every branch. Neither collection allocates MFC object-map slots for its direct entries. These layouts retain the ordinary modern nonminimal-owner scope of 11.29; legacy geometry reconstruction is outside scope.

Save keys establish the field names. Their precise geometric roles, enum domains and index constraints remain partial. The subsequent spline collection has a different serializer and is not covered by the conic body. Readers SHALL stop at unsupported later records rather than reuse this layout for splines.

### 11.36 Sketch 3D surface collections

*Status: Native-derived modern layouts and synthetic populated records; installed empty-collection boundaries verified · Evidence: [C-51], F.39*

After the spline collection, the modern sketch stores five collections in this order: planes, cylinders, spheres, cones and parametric surfaces. Each has an i32 count and direct entries with no category word or MFC map allocation. Every entry starts with the entity base of 11.29. Their suffixes are:

| Collection | Fields after entity base | Encoding |
|---|---|---|
| planes | Origin; XAxis; YAxis | three-f64 point; two three-f64 vectors |
| planes (continued) | HalfWidth; HalfHeight; CenterX; CenterY; border | four f64 values; four further f64 values |
| planes (continued) | IsAutoSizeable; has_string_188; optional string_188; unknown_190; has_point_158; optional point_158 | u8; u8; CString if present; f64; u8; three-f64 point if present |
| cylinders | Axis; Origin; Radius | three-f64 vector; three-f64 point; f64 |
| spheres | CentrePoint; Radius | three-f64 point; f64 |
| cones | Axis; Origin; Radius; Sine; Cosine | three-f64 vector; three-f64 point; three f64 values |
| parametric surfaces | no suffix | entity base only |

This clause uses the ordinary modern nonminimal-owner scope of 11.29; all listed version-gated fields are present there. Save keys support the named geometric fields. Border ordering, offset-named plane data and valid geometric domains remain partial. Readers SHALL retain stored values, including fields that native code normalizes in memory, and SHALL reject impossible collection counts. Later sketch relations and metadata are outside this stage.

