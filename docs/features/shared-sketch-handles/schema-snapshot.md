# Approved schema snapshot

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
