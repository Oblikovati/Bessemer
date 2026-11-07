### 11.246 3D bends, folds and unfolds

*Status: B / ? — four native type bindings, seven populated containing feature roots and two additional exact-EOF lists verified; populated native unfold maps/bodies, special transforms and engineering domains remain partial.*

`moSM3dBend_c` selects serializer `0x3DB666B0` in the hash-bound installed `sldsmmu.dll`. It calls established bend-group serializer `0x3DB68EC0`, then stores the following in the supported ordinary external-XML modern context (version `>0x27CB`):

1. Tagged `moProfileFeature_c` pointer at runtime `+0x8A0`, binary64 at `+0x898`, i32 at `+0x884`, and a direct point (three binary64 components) at `+0x8A8`.
2. A direct `suObList` associated with runtime `+0x908`, with `ReadCount` and that many tagged object pointers. There is no list-object tag or map slot around this direct body.
3. Tagged boundary-component reference at `+0x8D0`, tagged `sgLineHandle` at `+0x8D8`, i32 words `+0x8E0,+0x8C0,+0x970`, then tagged `moCompFeature_c` pointer at `+0x8C8`.

All historical field gates in this suffix precede the supported range. The old extra binary64 at `+0x888` for versions below `0x678` is absent. The modern body occupies **58 bytes** with empty list and null/mapped pointers. Newly serialized pointed objects and list entries add their own bodies. Native loading associates a nonnull line handle with the profile sketch and resets runtime `+0x880`; those operations add no fields.

Before storing, native code transforms a copy of the runtime point through the profile transform, constructs a temporary `suObList`, filters entries on the virtual predicate at `+0x1958` and adds surviving objects uniquely. The decoder preserves the serialized point, list order, duplicate entries and pointer identity before runtime normalization. Point units/frame semantics and raw option domains remain open. The five native roots exercise nonnull profiles, component faces, line handles and component features; their direct lists contain six total entries, including extrusion, edge-flange, base-flange, ICE and 3D-bend features.

`moFold_c` and `moUnFold_c` derive from `moFoldUpDown_c`. Shared serializer `0x3DB61410` uses the **body-feature base directly**, followed by:

1. Direct `suObArray` at runtime `+0x800`.
2. Tagged boundary-component reference at `+0x820`.
3. i32 raw value at `+0x838`.
4. Tagged `moCompFeature_c` pointer at `+0x840`, then tagged `moFoldUpDownFeatXformData_c` pointer at `+0x858`.

The pointer load gates (`>0xC5F` and `>0x12FA`) are satisfied throughout the supported range. Constructor `0x3D72F370` proves the direct array type; its runtime sheet-metal pointer at `+0x828` is not an additional field of this serializer. With empty array and mapped/null pointers the common suffix occupies **12 bytes**. Final helper `0x3D748BC0` filters runtime array members based on their `+0x290` state after reading or writing; its body has no archive argument or serialization calls. The decoder retains the original direct-array entries.

`moFold_c` selects the complete five-byte leaf at `0x3DB61570`: `E9 9B FE FF FF`, jumping to `0x3DB61410`. It adds no suffix. The leaf has no unwind-table entry. `moUnFold_c` selects `0x3DB6B880`, calls the common serializer and adds:

1. Direct `su_CMapObToInt` at runtime `+0x890`.
2. i32 declared raw-body count associated with runtime body list `+0x888`. Positive counts read that many raw bodies; nonpositive counts read none. Selected compressed raw frames use the strict validator from 11.244; unsupported forms stop explicitly.
3. A fixed `mgXform_c` archive-operator body at `+0x8C0`.
4. Tagged `moCompEdge_c` pointer at `+0x948`, then binary64 at `+0x950`.

The map/body-list gate (`>0x82D`) and transform/edge/double gate (`>0x23A5`) precede the supported modern range. The temporary historical list used by versions `0x829..0x82D` is absent here. Constructor `0x3D72F7C0` proves the object-to-int map and runtime body-list types.

The map class selects `0x31D492F0` in `swccu.dll`. Its wire body is native u32 `ReadCount` encoding, followed by **tagged generic object key, i32 value** for every entry. The decoder retains ordered raw associations, including repeated pointer keys and signed values. Native loading assigns each value through key lookup, so repeated keys can overwrite earlier runtime values; the raw decoder does not discard those earlier wire entries. An empty direct map occupies two bytes and adds no tagged map slot. Malformed counts are bounded before allocation or iteration.

The unfold transform imports bind to `sldmgu.dll` exports `??5@YAAEAVsu_CArchive@@AEAV0@AEAVmgXform_c@@@Z` at `0x3221D990` and the matching output operator at `0x3221D9A0`. The input export is a five-byte jump (`E9 0B 36 00 00`) to `0x32220FA0`. Both retained native operators iterate **30 u32 words**, a **120-byte fixed prefix**, then call the transform pointer restore/save machinery. This is distinct from compact `mgXform_c::restore`. The raw reader preserves all 30 words and stops explicitly when `word28 & 0xF0` is nonzero, because those special kinds require additional payload support. Native matrix/scale normalization and reserved-word resets do not change the retained wire values. With empty map/body list, ordinary fixed transform and a null/mapped edge, the unfold suffix after the common fold layer occupies **136 bytes**.

Five complete `moSM3dBend_c` roots total **75,922 bytes**. Paper Airplane additionally supplies one **8,102-byte unfold** and one **7,339-byte fold**, bringing the seven containing feature roots to **91,363 bytes**. Both fold-family records contain three direct array entries; their raw `+0x838` values are 0 for unfold and 1 for fold. The native unfold map is empty, its declared body count is 0, and its nontrivial fixed transform is retained word-for-word. Populated map/raw-body composition is tested synthetically; no populated native unfold map or raw-body list is claimed.

The 906-context audit now contains **25,458 complete roots**, **821 complete lists** and **85 explicit stops**, with every prior complete root record unchanged. This adds **27 roots** and two exact-EOF lists: Right Blade (212,324 decoded bytes) and Paper Airplane (203,036), totaling **415,360 bytes**. Totals are **743 exact-EOF lists / 60,907,232 decoded bytes** and **78 complete lists with four-byte unknown tails**. Checkpoints `resolved-feature-list-frontier-0.265.json` and `resolved-feature-list-frontier-0.266.json` retain the comparison. No whole-document semantic bit coverage is inferred from these framing results.

Five focused tests verify raw floating-point bits, fixed field order, direct-list duplicate identity, normal/extended map count encodings and repeated raw keys, fold references, positive/nonpositive body counts, fixed-transform boundaries and special-kind stops, truncation/context rejection and all seven source/root-hash-bound native features through both declared list ends. Historical/DB modes, populated native unfold maps/raw bodies, special transform tails, engineering meanings and complete document/project bit coverage remain unresolved.

Validation: all **899 regression tests pass** (87.108 seconds); specification lint reports zero findings (470 headings, 96 anchors).

Evidence: C-288.
