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

