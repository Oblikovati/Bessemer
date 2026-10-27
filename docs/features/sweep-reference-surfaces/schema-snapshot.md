### 11.233 Sweep reference surfaces and edge data

*Status: B / ? — native-bound modern serializer and two populated roots verified; raw option meanings, historical variants and whole-document semantics remain partial.*

In ordinary external-XML model archives with file version greater than `0x27CB`, `moSweepRefSurface_c` selects serializer `0x4CB15500` in `sldmodu.dll`. It writes the reference-surface base, then direct `moSweepData_c` at runtime `+0x820`. The primary vtable's archive-backend selector at `+0xAB0` points to `0x4B29C5A0`; its `33 C0 C3` body returns zero, establishing inline surface-body framing in this context.

The direct sweep-data serializer is `0x4CB57060`, exactly the same native body used by ordinary sweep features. Its existing modern layout and version gate at `0x27D9` apply here. Constructor `0x4CB14F60` independently constructs `moSweepData_c` at `+0x820`. The following suffix belongs to the sweep-surface parent; it is not the ordinary sweep feature's suffix:

1. Five **u8** values at runtime `+0x7B4`, `+0x7B8`, `+0x7BC`, `+0x7C0`, `+0x7C4`, each loaded into a wider runtime field without Boolean normalization.
2. If sweep-data field `+0x278` is 8 or 9, two tagged `moDisplayAngularDim_c` pointers at parent `+0xBD8`, then `+0xBE0`. Native predicate `0x4CB551A0` returns nonzero for exactly these two values.
3. One **i32** edge-data count, followed by that many tagged object pointers declared as `moEdgeData_c`. This count is four bytes, not ReadCount. The native load loop consumes zero objects when the signed count is nonpositive. The diagnostic reader retains negative counts and consumes no entries, while bounding positive counts by the remaining bytes.
4. If sweep-data field `+0x258` is nonzero, one tagged `moDisplayRadialDim_c` pointer at parent `+0xBE8`.

The writer counts only nonnull edge-data pointers before writing the sequence. The native loader consumes each counted pointer, appending only nonnull objects that satisfy `IsKindOf(moEdgeData_c)` to its runtime array. The diagnostic raw view retains the consumed pointer sequence; it does not replace source values with the filtered runtime array. The final parent post-load virtual call is runtime processing and is not represented as another wire field.

`moEdgeData_c` selects serializer `0x4CB15430`. Its entire body consists of three tagged `moEndPointRef_w` pointers in runtime-field order `+0x18`, `+0x20`, `+0x28`. It does not call a feature-data base or insert an owning-feature pointer. Null pointers and inherited-map reference identity follow the ordinary tagged-object rules.

`moSweepRefSurfIdKeeper_c` selects thunk `0x4CBA2733`, bound to empty `su_CObject::Serialize`. The feature's load branch constructs this keeper when the runtime pointer is null; that construction adds no archive body bytes.

Historical byte gates are after `0x64E`, `0x65E`, `0x662`, `0x684`, `0x70D` respectively. Loading the first angular pointer additionally requires version `0xA28`; the second requires `0x2019`; both still require the angular predicate. Edge data is present after `0xFBE`; versions below `0xFC4` use a different counted-array path. The radial pointer requires `0x1FD3`. The modern context satisfies all these gates. Earlier load defaults and writer/load asymmetries do not establish a general historical decoder.

Two complete native sweep-surface roots total **6,340 bytes**: Mouse Main Housing Overmold (3,364) and the simulation car-sides sample (2,976). Both declared feature lists complete. Retained packets bind stream/root hashes, stored flag bytes, angular/radial selectors and edge counts. These results verify the selected record framing without resolving raw option meanings or claiming whole-document bit coverage.

The complete 906-context audit now contains **24,854 complete roots**, **793 complete lists** and **113 explicit stops**, with every preceding complete root record unchanged. This adds 20 roots and two exact-EOF lists totaling **1,412,780 decoded bytes**: Main Housing Overmold (1,297,367) and car-sides (115,413). Totals are **720 exact-EOF lists / 53,189,313 decoded bytes**; 73 complete lists retain four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.252.json` and `resolved-feature-list-frontier-0.253.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 852 regression tests pass (98.814 seconds); specification lint reports zero findings.

Evidence: C-275.

