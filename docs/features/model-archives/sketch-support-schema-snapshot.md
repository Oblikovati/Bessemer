# Sketch support schema reference

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

### 11.30 Component sketch handles, point backup and silhouette references

*Status: Verified scoped modern framing; remaining referenced feature bodies and field meanings partial · Evidence: [C-45], F.33*

In the ordinary zero-condition part context of 11.29, moCompSketchEntHandle_c starts with the component-reference body of 11.17. It then contains direct i32 feature ID and u32 su_CTime representation, an sgEntHandle pointer and u32 unknown_100. The ID/time pair is direct moFRData_c, not an object pointer or the 64-bit in-memory time representation. Special-condition presence flags and legacy versions are outside this scope. Readers SHALL NOT infer those flags from the ID bytes.

sgPointHandle and sgSplineHandle use the same sgEntHandle body as the line/arc handles of 11.29. moPointBackedUpData_c consists of one direct three-f64 point; no extra base bytes precede it. moCompSilhVertex_c uses the existing component boundary-entity body of 11.17, including its alias, boundary reference and external reference.

moSilhVertexRef_c starts with the boundary-reference base of 11.18 (not the derived vertex-reference point-presence suffix). It appends three moFaceRef_c pointers, u32 flags_1a0, f64 unknown_1b0, f64 unknown_1b8 and u32 flags_1c8. Native loading interprets bits 1/2/4 of the first word as alternatives for member 0x1A0 and bit 8 as member 0x1A4; the final word selects member 0x1C8 using bits 1/2. Their geometric meanings and full bit domains remain unresolved. Readers SHALL retain the original words rather than replacing them with the native normalized members.

The opt-in point-prefix reader stops immediately after the first direct point collection. It does not constitute a complete sgSketch decoder and grants no whole-stream coverage. A referenced moBoss_c currently stops decoding rather than guessing its feature extent.

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

### 11.77 Ellipse handles, arc backups and populated slots

*Status: Native serializer evidence; populated native arc backup and slot verified; ellipse traversal reaches a later dependency · Evidence: [C-92]*

The primary sgEllipseHandle vtable dispatches directly to sgEntHandle::Serialize. Its scoped body is therefore the existing handle layout: u16 entity index, followed by i32 extended index only when the word equals 0x777F, then i32 reference ID and i32 dimension-on-CM field. There is no ellipse-specific suffix. Readers SHALL preserve the ellipse runtime class identity and the signed extended-index representation.

moArcBackedUpData_c first serializes the line-backup base: two tagged moPointBackedUpData_c pointers and two i32 fields at runtime +0x18 and +0x1C. Its suffix is i32 word_20, two tagged point-backup pointers at +0x28/+0x30, two tagged line-backup pointers at +0x38/+0x40 and one tagged point-backup pointer at +0x48. The base is stored and SHALL NOT be omitted merely because the derived serializer's call site has an incomplete recovered prototype. All seven pointer occurrences participate in the shared archive map; readers SHALL retain repeated-reference identity.

sgSlot_c first serializes the ordinary nonminimal entity base from 11.29. Its suffix is i32 Type at +0xC0, i32 LengthType at +0xC4 and exactly six tagged sgEntHandle pointers. There is no stored handle count. Readers SHALL read all six pointer occurrences, including nulls and repeated references, before returning the slot object. Alternate/minimal entity contexts remain outside the supported reader.

The composed audit now completes 892 of 906 modern part relation-manager prefixes and 77 sketches across 24 configurations. Successful prefixes contain one arc backup and one slot object. The ellipse-handle case advances to cptsSplineList_c, so its native body boundary is traversed but its containing manager remains incomplete. Fourteen configurations retain explicit unsupported dependencies. No whole-stream coverage increase is claimed.

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

### 11.238 Constrained planes and reference-plane geometry

*Status: B / ? — eight native type bindings, eight populated containing plane roots and six complete feature lists verified; scalar domains, engineering validity and other constraint variants remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, constrained plane-data objects use the established `moRefPlnData_c` body at `0x4C283D30`, including its existing file-version gates. Shared constraint serializer `0x4B8F8090` then reads or writes one tagged `moConstraintRefplaneInterfaceData_c` pointer at runtime `+0x88`. This is not an inline interface body or a nullable-array presence word.

`moConstraintMidPlaneRefplaneData_c` selects `0x4B8F8040`, whose complete five-byte body `E9 4B 00 00 00` jumps to `0x4B8F8090`. It adds no suffix beyond the common constraint data. `moConstraintRefplaneData_c` selects the common target directly. The leaf jump has no unwind-table entry; a decompiler following it can display the target body under the thunk's name, which does not create a second payload.

Interface serializer `0x4B8F8140` calls the empty object base and serializes **exactly three tagged geometry pointers**, at runtime `+0x08`, `+0x10`, `+0x18`. There is no stored slot count. Native loading accepts null slots and otherwise checks `IsKindOf(moRefPlaneGeometry_c)` before assignment. The diagnostic reader retains the actual slot pointers and inherited object-map identity.

The geometry base `moRefPlaneGeometry_c` selects `0x4C2E5390`. After the empty object base its wire body is:

1. Tagged `moRefWrapper_c` pointer at runtime `+0x08`.
2. Two binary64 values for native surface parameters U and V, associated with runtime `+0x10`.
3. Direct `mgPoint_c` at runtime `+0x20`, containing three binary64 components.

These values are retained without inferring parameter ranges, coordinate units or geometric validity from their numeric appearance. With a null or already-mapped wrapper pointer the base occupies 42 wire bytes.

The populated geometry subtypes are:

| Type | Selected serializer | Suffix after geometry base |
|---|---|---|
| `moRefPlaneCoincidentGeom_c` | `0x4C2E52D0`, five-byte jump to `0x4C2E5390` | None |
| `moRefPlaneMidPlaneGeom_c` | `0x4C2E5470` | One u8 at runtime `+0x38` |
| `moRefPlaneAngleGeom_c` | `0x4C2E5240` | One binary64 at runtime `+0x38`, then one u8 at `+0x40` |

For both byte flags, native storing emits the Boolean test of the runtime byte, while native loading sets the runtime Boolean true **only when the stored byte equals 1**. Values 2 or 255 load as false; they are not Booleanized by a nonzero test. The raw decoder preserves the byte and exposes its separate derived load value. With a null or mapped wrapper, the midplane and angle bodies occupy 43 and 51 wire bytes respectively. The coincident geometry's exact jump bytes are `E9 BB 00 00 00`.

`moConstraintCoincLineAtAnglePlaneRefplaneData_c` selects `0x4B8F8000`, which calls shared angle-parent serializer `0x4B8F7F00`. That parent calls the common constraint serializer, then writes an i32 at runtime `+0x148` and a tagged `moDisplayAngularDim_c` pointer at `+0x150`. Subsequent dimension-owner and geometry updates are runtime processing. The selected subtype initializes runtime `+0xE8` to 1 on loading without consuming another archive word. The raw i32 and double domains remain unresolved.

Eight complete containing `moRefPlane_c` roots total **19,477 bytes**, across seven configurations. Four midplane cases occur in Mouse Battery Cover and three shaft tutorial variants. Four coincident-line/angle cases occur in motor_frame, revolved_1_new and two Golf Ball plane roots. The eight roots exercise interface geometry slots, coincident/midplane/angle geometry types and dimension-reference composition. Six affected feature lists complete; Golf Ball reaches `mo3DSphereBackedUpData_c`. Exact stream/root hashes, class order, slot types and raw geometry flag bytes are retained. These results establish selected framing without claiming whole-document or all-installed-project bit coverage.

The 906-context audit now contains **25,136 complete roots**, **805 complete lists** and **101 explicit stops**, with every preceding complete root record unchanged. This adds 31 roots and six complete declared lists. Only two new lists reach exact EOF: Mouse Battery Cover (1,078,929 decoded bytes) and motor_frame (134,597), totaling **1,213,526 bytes**. The other four completed lists retain four-byte unknown tails. Totals are **728 exact-EOF lists / 57,616,318 decoded bytes** and **77 complete lists with four-byte unknown tails**. Checkpoints `resolved-feature-list-frontier-0.257.json` and `resolved-feature-list-frontier-0.258.json` retain the comparison. Neither list completion nor the unidentified tail earns additional whole-document semantic coverage credit.

Validation: all 868 regression tests pass (124.330 seconds); specification lint reports zero findings.

Evidence: C-280.

