### 11.16 Geometric reference wrappers

*Status: Verified scoped component layouts; inherited component references and meanings partial · Evidence: [C-30], F.18*

This clause specifies v > 0x27BC in the ordinary external-XML model context of 11.15, with the helper's lightweight-only flag (+0x834) clear. The latter flag adds configuration data in other contexts; its absence is a runtime condition, not a record tag. `modelreferences.py` requires the explicit context. It rejects embedded geometry bodies and sketch-handle context rather than guessing their extent.

#### 11.16.1 Common reference wrapper

*Status: Verified scoped layout; member meanings partial · Evidence: [C-30], F.18*

`moRefWrapper_c` stores the following in order:

| Order | Fields | Types | Condition | Status |
|---|---|---|---|---|
| 1 | component reference | object | Usually a moCompRef_c-derived object. Sketch entity handles require another sketch pointer; that context is not implemented. | V / ? |
| 2 | `unknown_60` | u16 | | ? |
| 3 | component | object | Declared moComponent_c. | V |
| 4 | `unknown_64`, `unknown_08` | i32 × 2 | | ? |
| 5 | `has_transform_10` | u16 | | V |
| 6 | `transform_10` | mgXform_c body | has_transform_10 != 0 | ? |
| 7 | `has_transform_18` | u16 | | V |
| 8 | `transform_18` | mgXform_c body | has_transform_18 != 0 | ? |
| 9 | `unknown_0c`, `unknown_20`, `unknown_6c`, `unknown_70` | i32 × 4 | | ? |
| 10 | `unknown_74` | i32 | v > 0x2FAF | ? |
| 11 | `unknown_4c`, `unknown_68`, `unknown_34`, `unknown_30` | i32 × 4 | | ? |
| 12 | `unknown_90` | i32 | v >= 0x2B73 | ? |
| 13 | `unknown_38` | i32 | v >= 0x3B02 | ? |

The transform layout is shared with 13.4. Stored i32 values are preserved even where the native loader later truncates a value to a word, merges flags or replaces a runtime value. The current wrapper has 62 bytes when both pointers are null and both transforms are absent. Actual records may contain larger pointed-to objects and transforms. The reader permits the verified component-reference classes listed in 11.17 at field 1; unverified derived/sketch contexts remain incomplete.

#### 11.16.2 Surface, face and curve suffixes

*Status: Verified scoped layouts; body-bearing cases and member meanings partial · Evidence: [C-30], F.18*

A surface-reference base appends four f64 members `unknown_a8`, `unknown_b0`, `unknown_b8`, `unknown_c0` to 11.16.1. A curve-reference base instead appends only `unknown_a8` and `unknown_b0`, both f64.

`moSingleFaceRef_w` uses the surface base, followed by `has_embedded_body` **i32**. When nonzero a kernel body and its attribute list follow; selected compressed-body framing is implemented in 11.156. After that optional body/list, the suffix is `unknown_c8` u32 and `unknown_d8` i32.

`moGeneralCurveRef_w` uses the curve base, followed by `has_embedded_body` **u8**. Its nonzero branch uses the selected common kernel-body framing in 11.156. After that optional body, the suffix is `unknown_c8` u32, `unknown_cc` i32, then three object records: `unknown_d8` (contour chooser), `unknown_d0` (component sketch-entity handle), `unknown_e0` (contour chooser). The differing face/curve presence-field widths SHALL be preserved.

`moCircleRef_w` appends to the general-curve body: `unknown_f8` point, `unknown_110` vector, `unknown_128` f64, `unknown_130` i32, then the optional frame below. `moCylinderRef_w` appends to the surface base: `unknown_c8` point, `unknown_e0` vector, `unknown_f8` f64, then the same optional frame. A point or vector is three consecutive f64; analytic roles and field units remain open.

The optional frame is a u16 `frame_mode`. If nonzero, three f64 `frame_vector` follow. If the word interpreted as **signed i16** is greater than 1, another three f64 `frame_point` follow. Thus 0xFFFF includes the vector but does not include the point. The current writer emits only mode 0 or 1; the loader also handles the point-bearing form. The schema reader preserves the original word and numeric values.

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
