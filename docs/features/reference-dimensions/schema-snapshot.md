# Approved schema snapshot

### 11.169 Reference-dimension handles and entity dimensions

*Status: B — modern native serializers, inheritance and populated corpus boundaries verified; raw fields and engineering validity remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`, excluding archive-condition 1 and drawing-owner tails. moRefDimHandle_c primary table `0x4CDA04B8` and moFeatureDimHandle_c primary `0x4D18C950` select the identical moModDimHandle_c serializer `0x4C83DA80`. Both inherit moModDimHandle_c. The scoped reference-handle reader reuses the existing model-dimension reader (11.27): component-reference base, dimension-handle helper `0x4C833BD0`, then a tagged internal Dimension_c-derived object when the inherited external reference is null. A null internal dimension is rejected. No reference-handle-specific suffix is inferred from a different runtime class name.

The handle helper retains its existing two u16 optional-transform flags, four drawing-view tags, external/reference-dimension pointers, optional reference-dimension transform/string, signed reference ID and raw suffix. Existing pointer identity and version/context restrictions remain in force. moRefDim_c, moRefDim1E_c and moRefDim2E_c have verified Dimension_c ancestry and are admitted to the supported internal/reference-dimension type lists.

moRefDim_c primary table `0x4D166EB8` selects `0x4C83FA20`. It calls the existing Dimension_c serializer, then reads:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | plane transform +80 | direct mgXform_c body | always | Existing transform restore framing; preserve decoded fields and complete serialized bytes. No pointer tag or transform presence prefix. | V / ? |
| 2 | reference-direction presence; vector +88 | u8; three f64 | vector only when byte nonzero | Preserve the original byte and vector. | V / ? |
| 3 | text-direction presence; vector +90 | u8; three f64 | vector only when byte nonzero | Independent of row 2. | V / ? |
| 4 | raw +A4; plane tag +A8; raw +B0 | i32; tagged moDimPlaneEntTag_c pointer; i32 | always | Preserve null/new/cached pointer framing. | V / ? |
| 5 | sketch-dimension handle +B8 | tagged moSkDimHandle_c pointer | always | Null is allowed; newly introduced unsupported derived types stop. | V |
| 6 | raw +A0/+C4/+C8 | three i32 | always | Exact wire order shown. | V / ? |
| 7 | raw +D0 | i32 | version >= 0x46C7 | Older modern versions omit the word completely. | V / ? |

The native store path names DB keys such as Plane and RefDir; ordinary binary load reads the fields above directly. These key names do not introduce CString fields in this scoped layout. Native updates and geometry normalization are not executed offline. The latest-version minimum reference-dimension base is **93 bytes**, including the 29-byte minimum Dimension_c body, 34-byte direct transform, absent vectors, null pointers and +D0. At version `0x27CC` the minimum is 88 bytes because both the Dimension_c post-`0x42EE` string-presence byte and +D0 are absent.

moRefDim1E_c primary table `0x4D1370E8` selects `0x4C83DE10`; moRefDim2E_c primary `0x4D137A48` selects `0x4C83E3A0`. Both call the shared reference-dimension base before their entity suffixes:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | one-entity kind +D8; entity +E0 | u16; tagged object pointer | one-entity class | Retain the kind word and original pointer, including null. | V / ? |
| 2 | one-entity raw +F0; raw +F4 | u16; conditional u16 | +F4 only version >= 0x2F6F | No widening to native in-memory integer fields. | V / ? |
| 3 | first kind +D8; entity +E0; second kind +F0; entity +F8 | u16; pointer; u16; pointer | two-entity class | Both pointers use ordinary MFC identity; repeated objects remain repeated references. | V / ? |
| 4 | two-entity raw +108/+110/+114/+118/+11C | five u16 | two-entity class | Preserve all five occurrences and their raw values. | V / ? |
| 5 | two-entity raw +10C | u16 | two-entity class and version > 0x3747 | This final word follows +11C despite its lower member offset. | V / ? |

The one-entity and two-entity alternatives are separate suffixes; rows 1–2 do not precede rows 3–5. Latest minimum bodies are **101 bytes** and **113 bytes** respectively. All entity words are two bytes, even where native loading expands them into four-byte members. The old `0x141`–`0x159` drawing-view-tag reads and other historical migration branches are excluded. Native loading can replace a null entity's kind with 8 and normalize zero +118 to one; offline reading retains the serialized values. Kind-specific class checks and runtime suppression/conversion behavior do not establish general engineering validity for arbitrary synthetic combinations.

The complete-root audit retains **10 reference-dimension handles**, with **nine moRefDim2E_c dimensions and one moRefDim1E_c**. All ten have null external/reference-dimension pointers, reference ID -1 and zero handle transform flags. Containing-root tests verify stream SHA and exact boundaries for both populated dimension classes. Synthetic tests cover raw entity kinds and repeated pointers, signed words, independently present vectors, direct-transform bytes, version gates, truncation, context rejection and internal/external handle paths.

Edition 0.180 records **22,231 complete roots**, **647 list-complete configurations** and **259 explicit stops** across **906 complete modern contexts**. **593 lists reach exact EOF**, spanning **31,229,297 decoded bytes**; **54 complete lists retain four-byte unknown tails**. Structural-member features, coordinate systems, drafts, profile references, additional dimension/entity types and raw meanings remain open. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; these record boundaries receive no whole-document coverage credit.

Evidence: C-197.

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
