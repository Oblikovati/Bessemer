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

### 11.240 Face-and-point axis data and sphere references

*Status: B / ? — two native type bindings, three complete containing axis roots and three complete declared lists verified; scalar domains, geometry validity and tails remain partial.*

`moFacePtAxisData_c` derives from `moRefAxisData_c` and selects `0x4C27DBB0`. In the supported ordinary non-DB external-XML model context with version greater than `0x27CB`, it calls common axis-data serializer `0x4C282690`, then serializes a tagged `moSurfaceRef_w` pointer at runtime `+0x80` and a tagged `moPointRef_w` pointer at `+0x88`, in that order. These are two tagged objects with no intervening presence word or count. The common body is the established 112-byte axis-data layout; its runtime owner at `+0x48` remains unserialized. With null or mapped references the derived body occupies **116 wire bytes**. Native DB dispatch selects separate archive methods and is outside this implementation's scope.

The surface reference can be `moSphereRef_w`. Its verified ancestry is `moSphereRef_w → moSurfaceRef_w → moRefWrapper_c → su_CObject`; its selected serializer is `0x4C286280`. It calls the established surface-reference serializer `0x4C2866C0`, then stores a direct three-component binary64 point at runtime `+0xC8`, followed by one binary64 at `+0xE0`. Loading these 32 suffix bytes is gated by file version greater than `0x7E9`; all versions accepted by the ordinary wrapper reader satisfy that gate. The inherited wrapper retains its own stricter context/version checks and version-dependent layout. No cylinder direction vector or optional frame follows this sphere suffix. The decoder preserves raw values as `unknown_c8` and `unknown_e0`; units, domains and geometric validity remain unresolved.

Three complete containing `moRefAxis_c` roots total **3,043 bytes**: 1,033 bytes each in NAFEMS `nafems_ic17.sldprt` and `nafems_ic18.sldprt`, and 977 bytes in Golf Ball. Exact stream/root hashes and the two reference classes are retained in `face-point-axis-targeted-corpus.json`. Both NAFEMS lists now end at exact EOF (16,456 decoded bytes each). Golf Ball parses four additional roots—axis, plane, profile and revolved cut—and completes its declared list at byte 42,761 of 42,765; its four-byte tail remains unknown.

The 906-context audit now contains **25,145 complete roots**, **810 complete lists** and **96 explicit stops**. Every previously complete root record remains unchanged. This adds six roots and three complete lists, with only two new exact-EOF lists / **32,912 decoded bytes**. Totals are **732 exact-EOF lists / 57,948,289 decoded bytes** and **78 complete lists with four-byte unknown tails**. Checkpoints `resolved-feature-list-frontier-0.259.json` and `resolved-feature-list-frontier-0.260.json` retain the comparison. No whole-document semantic coverage credit is inferred from these framing results.

Three focused tests verify surface/point order, null and shared pointer identity, absence of direct-member map slots, raw binary64 bit preservation, exact sphere suffix boundaries, truncation/context rejection and all three hash-bound native containing roots through their declared list ends. Historical/DB modes, engineering scalar meanings, unknown tails and complete document/project bit coverage remain open.

Validation: all **873 regression tests pass** (85.930 seconds); specification lint reports zero findings (464 headings, 96 anchors).

Evidence: C-282.

### 11.341 Cone references, two-point axes, dimension planes and shared handles

*Status: Native-bound ordinary modern bodies; five additional EOF streams; semantic coverage partial · Evidence: C-383*

The following primary tables and serializers are bound to the installed C-20 model DLL. Object offsets describe native members, not wire addresses. The existing ordinary modern context resolves historical gates; no legacy or DB support is added.

| Class | Primary vtable | Serialize | Modern body |
|---|---|---|---|
| `moConeRef_w` | 0x4D17E050 | 0x4C27B750 | Existing surface-reference wrapper, then raw point +C8, vector +E0, f64 +F8 and f64 +100 |
| `moTwoPtsAxisData_c` | 0x4D1635A0 | 0x4C286D20 | Existing 112-byte reference-axis data, then tagged `moPointRef_w` pointers +80 and +88 |
| `moDimPlaneEntTag_c` | 0x4CD94168 | 0x4C834650 | u16 kind, then direct plane for kind 2 or one tagged pointer otherwise |
| `sg3DParametricSurfaceHandle` | 0x4CEDA178 | 0x4C591ED0 | Existing `sgEntHandle` body, without derived suffix |
| `sgParabolaHandle` | 0x4D1D05F0 | 0x4C591ED0 | Same existing handle body, without derived suffix |

The cone serializer calls surface helper **0x4C2866C0**. Its derived suffix is **64 bytes**, with no cylinder optional-frame field. The +100 double is present from version **0x8F0**; preceding historical runtime conversion does not add input to the supported modern branch. All eight suffix doubles remain semantically unknown and uncredited.

The two-point axis calls common data helper **0x4C282690**, then consumes the two typed point-reference pointers for versions greater than **0x50**. Shared pointers and null pointers retain normal archive framing. The runtime owner gap of the common axis data is not a wire field. Both installed pointers are populated `moPointRef_w` objects.

The dimension-plane tag reads a u16 and widens it into its runtime kind. For kind **2**, it calls `mgPlane_c::restore` directly, without a tagged object header. The exported implementation at **sldmgu 0x32102AC0..0x32102B4D** reads **six f64 values**, including the final tail jump to the archive double operator: exactly **48 payload bytes**. This reader retains the six values as raw plane data and credits no value semantics. For every other stored kind, the loader consumes one generic tagged object pointer. A null pointer with no runtime plane normalizes the effective kind to **0**, including unrecognized stored kinds. A nonnull kind-1 pointer must name `moCompFace_c`; a nonnull kind-0 pointer is consumed but does not become a face reference. Other nonnull kinds fail explicitly. Stored and effective kinds are both retained rather than rewriting source bytes. All three source-observed tags use kind 1 with face references; inline planes and null-normalization branches are native-bound and covered by focused synthetic tests.

Both handle primary slots point **directly** to the existing `sgEntHandle::Serialize`, and RTTI proves that ancestry. The wire is a u16 entity index; index **0x777F** introduces a signed i32 extended index; signed i32 reference ID and dimension-on-CM fields follow. No derived class payload is appended. This local registration preserves native class identity. Runtime sketch lookup and object repair are not emulated.

`geometricfrontier.py` implements the three new layouts. `resolvedfeatures.read_data(..., geometric_dependencies=True)` installs these and both shared handles locally. The staged default remains false; `read_supported` and `documented_bytes` enable them with all preceding saved-context, root-count, eligible-MBSM, EOF and bounded-unknown guards. `geometric_dependencies=False, backup_dependencies=False, sketch_dimension_dependencies=False, isocurve_dependencies=False, weld_dependencies=False, weldment_dependencies=False, frontier_dependencies=False, alignment_dependencies=False, pattern_dependencies=False` reproduces edition 0.360; historical regrade profiles explicitly pass false. Global registrations and the caller's inherited reader map remain unchanged.

The targeted six-source replay reaches five exact EOFs:

| Installed sample (under samples/) | Decoded stream bytes | Complete roots | Unknown bytes |
|---|---:|---:|---:|
| learn/Mouse/Mouse Master.SLDPRT | 520,297 | 70 | 196,567 |
| Simulation Examples/Dynamics/ac_beam.sldprt | 93,034 | 29 | 33,257 |
| Simulation Examples/knob.sldprt | 165,766 | 44 | 69,298 |
| Simulation Examples/Verification/static_25.sldprt | 16,618 | 21 | 5,982 |
| tutorial/api/chair.sldprt | 475,808 | 56 | 233,343 |

These completed roots contain **10** newly bound helper objects: one cone, one two-point axis data object, three dimension-plane tags, three 3D surface handles and two parabola handles. Garden Trowel Handle advances inside its next root but stops at `moParabolaBackedUpData_c`; that incomplete root is not counted as a completed object or credited stream. No diagnostic cursor establishes semantic completion.

The full **906-configuration** `resolved-geometric-audit-0.361.json` reaches **864 exact-EOF streams / 42 explicit stops / zero context stops**, with **26,067 completed roots**. All **859** previous positives preserve endpoints, root counts and unknown totals. Completed streams contain **74,258,825 decoded bytes**, comprising **26,092,878 unknown** and **48,165,947 known structure bytes**. The added streams contribute 733,076 known decoded bytes.

`geometric_frontier_evidence.py` verifies both installed module hashes, all five primary/ancestry bindings, complete native serializer ranges/instructions, inherited helper calls, the exported six-double plane body, six source graphs and preservation of every prior completion. `geometric-frontier-native-evidence.json` binds those observations to the production decoder and retained decompilation hashes; `geometric-frontier-corpus.json` retains source hashes, complete root byte spans/hashes, observed tags and explicit stops.

`resolved_coverage.py --surface-dependencies --remaining-planes --mid-surfaces --reference-curves --reference-surfaces --geometric-dependencies` writes `resolved-feature-coverage-0.361.json`. `resolved_coverage_evidence.py --geometric-dependencies` independently revalidates all **1,166 source rows**, decoded sizes, hashes, physical stored spans, integer proportions and audit decisions in `geometric-feature-coverage-native-source-evidence.json`. The added stored credit is **133,754 part bytes**, taking cumulative resolved-family known bytes to **13,757,568** and leaving **11,379,898 unknown** of the unchanged 25,137,466 stored bytes. Total part unknown bytes become **54,610,064** of 227,086,352; measured part coverage is **76.0% / 70.7%**. Assemblies remain **83.8% / 81.3%**, drawings **57.3% / 49.9%**. The frozen edition 0.360 gap report and historical verifier still pass. Whole documents and all installed project formats remain unfinished.

Validation: four focused tests pass, covering cone prefix/suffix boundaries, two-point shared identity, direct-plane extent, tag normalization/type rejection, truncation, following sentinels, local registration and source graph endpoints. All **1,117 regression tests pass** (157.659 seconds); specification lint reports zero findings (565 headings, 96 anchors).

Evidence: C-383.

