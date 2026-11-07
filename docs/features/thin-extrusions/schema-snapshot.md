### 11.191 Thin extrusions, specifications and bend-ID triples

*Status: B / ? — native inheritance, six populated thin extrusions/specifications and direct bend records verified; option and thickness-control semantics remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moExtruThin_c primary table `0x4CF480A8`, complete-object locator `0x4D32F118` and serializer slot +28 select `0x4BB24DC0`. Native RTTI establishes moExtrusion_c ancestry; the serializer first calls existing complete extrusion helper `0x4BB25180`. Its inherited body therefore includes the ordinary body-feature layers and extrusion suffix, followed by:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | thin specification +860 | tagged moThinSpec_c pointer | always | Supported reader requires a nonnull moThinSpec_c. Native continuation dereferences this object when assigning dimension owners. | B |
| 2 | bend-record count | u16 | always in scope | A fixed unsigned short, **not ReadCount**: 0xFFFF denotes 65,535 records, not an escape to a wider count. Historical load begins after `0x1DA`. | V |
| 3 | bend-ID records +858 | count direct triples of u32 | count nonzero | Each record contains exactly three u32 values, 12 wire bytes, through native helper `0x4BB20BF0`. Preserve duplicates/order and all bits. No per-record tags or counts. | V / ? |

Although the runtime collection is suDynamicArray<moBendIdData_c>, this serializer writes only the u16 count and fixed triples. Generic dynamic-array width/capacity/growth metadata is absent. Readers SHALL bound count against the remaining bytes before allocating/reading. With the latest-version 265-byte empty ordinary extrusion body, a cached nonnull thin-spec pointer and zero records, the synthetic complete body is 269 bytes before outer tagged framing. A newly tagged specification adds its own object/class framing and body. This fixture proves composition, not the engineering validity of an empty extrusion.

moThinSpec_c primary table `0x4D203A90`, locator `0x4D36DBF8` and serializer slot +28 select `0x4BB367D0`. After its zero-byte ordinary su_CObject layer it reads:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | dimensions +08/+10 | two tagged moDisplayDim_c-family pointers | always in scope | Read both pointers independently of the later raw option words. | B |
| 2 | raw +2C; raw +30/+38 | i32; two f64 | always | Preserve the exact non-offset wire order and both doubles. | V / ? |
| 3 | raw +28/+40 | two i32 | always | No implicit dimension omission is inferred from these values. | V / ? |
| 4 | radial dimension +48 | tagged moDisplayRadialDim_c pointer | always in scope | Historical temporary-double representation before `0x1DA` is excluded. | B |
| 5 | raw +50/+54 | two i32 | always in scope | Older temporary-word/migration branches precede the modern minimum. | V / ? |

The null-dimension thin-spec body is **42 bytes**, including three two-byte null tags. Cached dimensions use the same tag widths; new dimensions add their own bodies. Native load assigns feature owners and may migrate older geometry after reading; offline parsing retains the stored fields and pointer identities without executing those operations.

The audit retains **six complete thin extrusions and six thin specifications**. Two extrusions contain three bend triples each, in order [2,3,5], [1,2,6], [3,4,7]; four contain no triples. Every specification stores two nonnull moDisplayDistanceDim_c pointers and one nonnull moDisplayRadialDim_c pointer. Raw +28 is zero in three and one in three; +50 is one in two and zero in four. All other raw integer members are zero. Raw +30 is 0.001 in four and 0.01 in two; the meaning/domain of +38 remains unresolved, and its stored double is retained without plausibility filtering.

Evidence: C-219.

### 11.207 Thin-boss shared serializer binding

*Status: B / ? — native primary ancestry/dispatch, inherited hooks and four populated thin-boss containing roots verified; option meanings and whole document coverage remain partial.*

moBossThin_c primary table `0x4CF4CEA8`, complete-object locator `0x4D32F3B0` and serializer slot +28 select `0x4BB24DC0`, the same serializer as moExtruThin_c. RTTI establishes the actual chain moBossThin_c → moExtruThin_c → moExtrusion_c, followed by the supported sketch/body-feature ancestry. The inherited node hooks +4B8/+4C0 select `0x4C189B60`/`0x4C18A920` for both thin classes. This binding is established from native dispatch and ancestry, not inferred from the word “boss.”

Its ordinary non-DB external-XML body for versions greater than `0x27CB` is the existing thin-extrusion layout of 11.191:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited extrusion | complete direct moExtrusion_c body | always | Existing complete body-feature and ordinary extrusion fields, with no added class tag or map slot for the base. | B |
| 2 | thin specification +860 | tagged nonnull moThinSpec_c pointer | always | Existing versioned specification layout; preserve cached identity and require the supported specification class. | B |
| 3 | bend records +858 | u16 count, count × (u32, u32, u32) | always | Count is a plain positive-width u16, including 65535; not packed ReadCount. Preserve each triple, order and duplicates. | B / ? |

No extra moBossThin_c suffix is serialized after these fields. With the tested latest empty inherited extrusion, a cached thin specification and an empty bend array, the complete body occupies **269 bytes** before outer tagged framing. `bodyfeatures.install_scoped` binds moBossThin_c to `thinextrusions.read` only in the copied feature-context map.

The extended native evidence checks five primary/ancestry bindings and ten retained native bodies, including both thin classes' inherited hooks. The thin-feature suite now has seven tests, adding a tagged thin-boss minimum with cached specification/self identity and truncation, and a source-hash-bound complete native containing root with specification class and bend-data equality. Existing tests retain high-bit specification words, u16 65535 bend counts and direct triple framing.

The audit now retains **four complete thin-boss records**, all with empty bend arrays, alongside the six previously retained moExtruThin_c records. These observations do not restrict future thin-boss bend counts or assign meanings to unresolved specification words/doubles.

Across 906 complete modern contexts the frontier now has **24,069 complete roots**, **749 complete lists** and **157 explicit stops**: 33 additional roots and four additional complete lists relative to Edition 0.212. **679 lists reach exact EOF**, spanning **44,553,795 decoded bytes**; **70 complete lists retain four-byte unknown tails**. Four additional lists reach EOF, adding 523,332 exact-EOF bytes. Completed-list endpoint/tail invariants pass. `resolved-feature-list-frontier-0.212.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-235.

### 11.208 Thin revolutions and direct thin-specification suffix

*Status: B / ? — native ancestry/base dispatch and five populated thin revolutions in complete containing roots verified; option meanings and full document/project coverage remain partial.*

moRevolutionThin_c primary table `0x4D1896D8`, complete-object locator `0x4D35E6F8` and serializer slot +28 select `0x4BB32140`. RTTI establishes moRevolution_c ancestry. The serializer first calls ordinary revolution helper `0x4BB322F0`, then reads a moThinSpec_c object. Its inherited node hooks +4B8/+4C0 select `0x4C189B60`/`0x4C18A920`, matching the supported ordinary feature layout.

The complete ordinary non-DB external-XML body for versions greater than `0x27CB` is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited revolution | complete direct moRevolution_c body | always | Existing inherited body-feature fields and revolution suffix, including the versioned +8E0 byte. No extra base class/map entry. | B |
| 2 | thin specification +8E8 | tagged nonnull moThinSpec_c pointer | always | Native ordinary branch uses ReadObject with the moThinSpec_c runtime class; retain standard new/cached identity. Supported decoder requires this exact nonnull specification type. | B |

There is **no bend-count field or bend-triple array** after row 2. Those fields belong to thin-extrusion serialization, not this serializer. Remaining native load actions update feature/dimension ownership and runtime geometry; they consume no modern archive fields. DB-stream ReadObject dispatch is outside this ordinary file-reader scope.

`thinextrusions.revolution` composes the complete ordinary revolution with its tagged specification, and is installed only in the copied feature-context map. With the tested latest empty revolution body and a cached specification, the complete body occupies **277 bytes** before outer tagged framing. A following cached self-reference begins immediately after that pointer. Two additional focused tests verify this boundary, absence of a bend count, cached specification/self identity, scoped-only registration, truncation/null-spec rejection, and a source-hash-bound complete native containing root with raw specification field equality. The combined thin-feature suite now has nine passing tests.

The audit retains **five complete thin revolutions**. All store specification raw +30 as 0.001; the remaining raw option/double values remain uninterpreted, including very small nonzero stored +38 doubles. No engineering role, enum domain or floating-point normalization is inferred from these examples.

`thin_silhouette_evidence.py` now retains six primary/ancestry bindings and twelve native bodies, including the thin-revolution-to-revolution call and all three thin-feature node-hook pairs. Existing thin-specification native evidence remains shared across extrusion, boss and revolution consumers.

Across 906 complete modern contexts the frontier now has **24,084 complete roots**, **753 complete lists** and **153 explicit stops**: 15 additional roots and four additional complete lists relative to Edition 0.213. **683 lists reach exact EOF**, spanning **44,802,506 decoded bytes**; **70 complete lists retain four-byte unknown tails**. Four additional lists reach EOF, adding 248,711 exact-EOF bytes. Completed-list endpoint/tail invariants pass. `resolved-feature-list-frontier-0.213.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-236.

### 11.236 Thin cuts and composite reference curves

*Status: B / ? — native-bound thin-cut dispatch and composite-curve layout, with six complete roots verified; scalar meanings, historical/context variants and whole-document semantics remain partial.*

This clause applies to ordinary external-XML model archives with file version greater than `0x27CB`. The installed `sldmodu.dll` primary vtable for `moCutThin_c` at `0x4CF4A7A8` selects serializer `0x4BB24DC0`, exactly the same function selected by `moExtruThin_c` and `moBossThin_c`. Native ancestry includes `moExtruThin_c`. Its archive hooks at vtable `+0x4B8` and `+0x4C0` likewise match the established thin-extrusion hooks (`0x4C189B60`, `0x4C18A920`).

Thin-cut therefore uses the existing thin-extrusion wire layout: the common extrusion body, a tagged `moThinSpec_c` pointer at runtime `+0x860`, then a **u16** bend-record count followed by that many triples of **u32** values. The selected native serializer explicitly calls common extrusion serializer `0x4BB25180` and serializes each 12-byte bend record. This reuse is backed by exact dispatch, ancestry, hooks and current native-byte comparisons, rather than a class-name assumption. The existing thin-specification layout and its supported nonnull standard specification pointer apply. Bend values remain raw; no additional thin-cut-only suffix is inserted.

`moCompositeCurve_c` has a separate layout. Its primary vtable at `0x4D2E6CF0` selects serializer `0x4CB76330`, which first calls `moReferenceCurve_c::Serialize` at `0x4C284E20`. The established ordinary reference-curve base writes reference geometry, kernel wire-body framing at runtime `+0x710`, a four-byte wire-node ID at `+0x718`, and a direct attribute object list at `+0x728`. Existing strict kernel framing and compressed Parasolid validation remain in force.

The composite-curve suffix is, in order:

1. One i32 at runtime `+0x780`, retained without an unsupported meaning.
2. One **tagged** `suObArray` pointer at `+0x778`. It uses the shared object map and can refer to an earlier array object.
3. One **direct** `suObArray` body at `+0x788`: ReadCount and tagged object pointers, with no additional class/object wrapper.
4. Four direct `su_CDWordArray` bodies at `+0x7A8`, `+0x7C8`, `+0x7E8`, `+0x808`, each independently encoded as ReadCount followed by that many u32 values.

Constructor `0x4CB75FE0` independently establishes the concrete direct container types. Do not require the four DWORD arrays to have equal counts or infer their counts from the preceding scalar. Their element meanings remain unresolved for semantic coverage accounting. The native load branch assigns the tagged `+0x778` pointer only when nonnull; the diagnostic view retains the actual stored null/reference value instead of inventing preexisting runtime state.

The installed corpus contains three complete thin-cut roots totaling **27,701 bytes**: two in tutorial mousemultibody (7,863 and 6,826) and one in camtest (13,012). It also contains three complete composite-curve roots totaling **163,300 bytes**: toaster (2,657) and two in mousemultibody (80,321 and 80,322). All three affected declared feature lists complete. The composite roots exercise present kernel bodies, nonempty object arrays and differing DWORD-array counts. Retained packets bind exact source/root hashes and branch/count summaries. These results verify selected framing without claiming whole-document or all-installed-project bit coverage.

The 906-context audit now contains **25,020 complete roots**, **797 complete lists** and **109 explicit stops**, with every preceding complete root record unchanged. This adds 35 roots and three exact-EOF lists totaling **693,683 decoded bytes**: toaster (241,244), mousemultibody (416,010) and camtest (36,429). Totals are **724 exact-EOF lists / 55,865,360 decoded bytes**; 73 complete lists retain four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.255.json` and `resolved-feature-list-frontier-0.256.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 862 regression tests pass (90.119 seconds); specification lint reports zero findings.

Evidence: C-278.

