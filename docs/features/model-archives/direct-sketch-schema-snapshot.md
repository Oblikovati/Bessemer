# Direct sketch collection schema reference

Edition 0.342 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.29 Sketch point prefix and supporting references

*Status: Native-derived modern nonminimal-owner layout; installed point collections remain incomplete · Evidence: [C-44], F.32*

This clause applies to ordinary external-XML part archives with v > 0x27CB, a nonminimal sketch owner and neither archive condition 2 nor 4. It specifies a prefix, not the complete sgSketch body. Readers SHALL preserve the inherited object map and SHALL stop on an unsupported reference body, minimal-owner context or nonzero jet presence. Direct points do not introduce an MFC class tag or object-map entry.

The sketch begins with an i32 point count. Each entry is a u16 category followed by the direct point body below. Native category 1 selects an external-point prototype, 2 a frame-point prototype and other values the ordinary prototype; the same archive routine restores each. The next geometry collection is outside this clause.

| Order | Direct entity fields | Encoding |
|---|---|---|
| 1 | permanent mark; unknown_0c | i32; i32 |
| 2 | visual properties | direct moEntVisProp_c (11.12), nonminimal owner only |
| 3 | reference count; stored minimal flag | i32; u16 (0 in this scope) |
| 4 | color; constraint status; line type; width | three u16; direct utLineWidth_c restore type 2 |
| 5 | external entity | sgExtEnt_c pointer |
| 6 | flags_54; flags_56; unknown_50 | u16; u16; i32 |
| 7 | group; description presence; optional description | moGroup pointer; u8; CString when nonzero |
| 8 | unknown_68; unknown_74; unknown_78 | f64; u32; u32 |

The point appends a u16 mask, position, u16 point type and u16 geometric-reference count. Position contains two f64 values when mask bit 0x10 is set, otherwise three. Mask bit 0x20 denotes the unsupported minimal-owner branch. In the supported branch two sgEntHandle pointers follow, then u16 center-reference count, i32 unknown_108 and two further positions using the same coordinate width. A u16 jet-presence word follows; zero is followed by the u16 bidirectional external-reference count. Nonzero jet bodies remain unspecified. Other mask meanings are partial and all stored bits SHALL be retained.

The modern sgEntHandle body shared by sgLineHandle and sgArcHandle is u16 entity index, an i32 replacement index when the u16 equals 0x777F, then i32 reference ID and i32 DimOnCM. sgExtEnt_c contains four pointers (moRefWrapper_c, moDrView, moSkIntersectInfo_c, moSkIsoCurveInfo_c), then reference-axis CString in this scope.

moSketchExtRef_w contains the 11.16 wrapper followed by an i16 mode (-2, -1 or 0) and moBackedUpData_c pointer. moTempAxisRef_w contains that wrapper, two f64 values, a direct three-f64 point, three-f64 vector and u8 unknown_f0. For v > 0x371D it appends two three-f64 points and i32 unknown_128. These wrappers retain the existing component-reference restrictions; further component/feature classes are not implied supported.

### 11.31 Direct sketch line collection

*Status: Verified modern nonminimal-owner framing; some field meanings and later collections partial · Evidence: [C-46], F.34*

The first point collection of 11.29 is followed by an i32 line count. Each line starts with a u16 prototype-selection mask and the direct body below. Bit value 1 selects an external-line prototype; otherwise value 2 selects the ordinary prototype with native style normalization; otherwise value 4 selects the underlying-offset prototype; otherwise the ordinary prototype is used. The common serializer consumes the same body in every branch. These records introduce no MFC object-map entries. Readers SHALL preserve the stored category mask and style values even when the native loader normalizes its in-memory objects.

| Order | Line field | Encoding |
|---|---|---|
| 1 | entity base | direct nonminimal body of 11.29 |
| 2 | End1 | u16; if 0xFFFF, followed by replacement i32 |
| 3 | End2 | u16; if 0xFFFF, followed by replacement i32 |
| 4 | isContour; isFlipped | u16; u16 |
| 5 | NormalScale | f64 |
| 6 | vector_e0 | three f64, only when entity flags_56 has bit 0x200 |
| 7 | unknown_e8 | u16 |

This layout is scoped to v > 0x27CB, so all listed version-gated modern fields are present. Endpoint IDs refer to sketch point indices; their topology, valid range and negative extended-value semantics remain partial. NormalScale is retained even if native code would discard nonpositive values. The optional vector is controlled by the entity word, not by the collection prototype mask.

moLineBackedUpData_c contains two moPointBackedUpData_c pointers, then i32 unknown_18 and i32 unknown_1c. It has no additional serialized base prefix. Pointers use the current shared MFC map and can reference the same backup object. The next collection and the full sgSketch suffix remain outside this clause; readers SHALL stop at the declared partial boundary rather than treat a sketch as fully decoded.

### 11.32 Sketch text collection and dimension-text body

*Status: Native-derived modern layout, synthetic body tests and native empty-collection boundaries; populated native fixtures pending · Evidence: [C-47], F.35*

The line collection is followed by an i32 sgText count. Entries are direct bodies without a category word or MFC map entry. Each begins with the common segment fields from 11.31 through NormalScale, excluding the line-specific optional vector and final word. It then stores a moDimText_c pointer. In the supported no-curve branch it appends a CString Text and two i32 fields, LongDate and DateAndTime. Names follow native save keys; value domains remain unresolved. A null or differently typed dimension-text pointer is outside the supported native branch.

The modern moDimText_c body (v > 0x27CB) is:

| Order | Fields | Encoding |
|---|---|---|
| 1 | stored text; point_38; unknown_10; character format; unknown_20 | CString; three f64; i32; utCharFormat_c pointer; i32 |
| 2 | point_50; unknown_68; group count | three f64; f64; i32 |
| 3 | group sizes | group-count i32 values |
| 4 | values_88; point_counts_80 | two arrays, each containing sum(group sizes) i32 values |
| 5 | points_78 | sum(point_counts_80) direct three-f64 points |
| 6 | unknown_c4; has_curve_data | i32; i32 |
| 7 | curve arrays (only if has_curve_data is nonzero) | i32 curve count; that many three-f64 points; that many i32 values; that many three-f64 vectors; final i32 unknown_c0 |
| 8 | unknown_d0; unknown_d8; unknown_e0; unknown_24 | i32; f64; f64; i32 |
| 9 | strings_f0 | direct su_CStringArray (11.8), not an object pointer |
| 10 | unknown_118; unknown_120; unknown_124; unknown_128; unknown_130 | i32; i32; i32; f64; i32 |

The native loader removes a leading `<RTL>` marker in memory. Readers SHALL retain the stored string and marker for byte-preserving inspection. Native allocation counts and sums define array framing; readers SHALL reject negative or out-of-buffer counts rather than use them to seek through unknown bytes. Meanings of the nested arrays and most offset-named fields remain unresolved.

The same has_curve_data field controls the enclosing sgText continuation. When nonzero in the ordinary zero-condition context, native code reads a suObArray pointer and an embedded kernel body before the Text/date tail. That embedded body and nonnull character-format payloads are not implemented by this scoped reader; decoding SHALL stop explicitly at an unsupported dependency. Special archive conditions add other reference/index records and remain outside this scope. The implementation never registers a complete global sgSketch reader.

