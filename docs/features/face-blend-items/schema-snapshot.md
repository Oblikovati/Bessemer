### 11.205 Face-blend item pairs and optional-point items

*Status: B / ? — native primary ancestry/base import and 15 pairs with 30 items in complete containing roots verified; pointer/point roles and full document coverage remain partial.*

These classes are defined in sldMoPartu.dll, SHA-256 `2201edfa07b1a7484acedb6db4600fa7a00e55b16090a03b92fe7d9b14cb2128`. moFaceBlendItemPair_c primary table `0x180480D98`, locator `0x1804EFA78` selects serializer `0x1802D9660`. moFaceBlendItem_c primary table `0x180480D08`, locator `0x1804EF9F0` selects `0x1802D96D0`. Both have su_CObject ancestry and begin with its imported serializer, which contributes no bytes in the ordinary binary scope used here.

The pair body in ordinary non-DB external-XML archives with version greater than `0x27CB` is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | first item +08 | tagged moFaceBlendItem_c pointer | always | Preserve null, cached and newly allocated item identity. | B |
| 2 | second item +10 | tagged moFaceBlendItem_c pointer | always | Separate occurrence; may repeat row 1's cached object. No pair count or extra direct list is serialized. | B |

The item body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | face +08 | tagged moFaceRef_c pointer | always | Retain independently from the edge pointer. | B |
| 2 | edge +10 | tagged moEdgeRef_c pointer | always | Normal tagged null/cached/new framing. | B |
| 3 | point presence | u8 | always | Zero omits row 4; any nonzero byte selects it. Keep the stored byte instead of normalizing it to one. | B |
| 4 | point +18 | three f64, mgPoint_c::restore | point presence nonzero | Native allocates a 24-byte runtime point, then restores its three coordinates. Allocation does not insert an archive length or class tag. | B / ? |

A pair with null/cached items occupies four bytes before outer framing; new item bodies add their own tagged data. An item with null/cached face and edge pointers occupies five bytes without a point, or 29 with a point. These sizes exclude outer tagged object/class framing. No version gates appear in the two derived serializers; older/special base archive contexts remain outside the supported scope.

`blends.face_item_pair` and `blends.face_item` are installed only in the copied scoped feature-reader map. Three focused tests cover raw presence bytes 0/1/255, point values, exact following-byte alignment, truncation/context rejection, pair-before-nested-item object-map allocation, repeated cached item identity, and a source-hash-bound complete native containing root with stored item fields and pair identity equality.

`face_blend_pair_evidence.py` verifies both primary COL/type/ancestry records and retains both native function bodies. The imported su_CObject base calls are bound by raw PE import descriptor/lookup entries because convenience import enumeration omits the observed slot. Their exact module/symbol and call/IAT addresses are retained in `face-blend-pair-native-evidence.json` rather than inferred from address ranges.

The audit retains **15 pairs and 30 items** in completed roots. All observed items have a nonnull moFaceRef_c, a null edge and zero point presence. Optional populated point framing is established by native branches and synthetic tests, without claiming it is represented in this native sample set.

Across 906 complete modern contexts the frontier now has **23,829 complete roots**, **741 complete lists** and **165 explicit stops**: 150 additional roots and five additional complete lists relative to Edition 0.210. **671 lists reach exact EOF**, spanning **42,342,125 decoded bytes**; **70 complete lists retain four-byte unknown tails**. Four additional lists reach EOF, adding 3,906,561 exact-EOF bytes. Completed-list endpoint/tail invariants pass. `resolved-feature-list-frontier-0.210.json` preserves the preceding checkpoint. Exact list completion does not establish full part/assembly/drawing or all-installed-project bit coverage, which remains unfinished.

Evidence: C-233.
