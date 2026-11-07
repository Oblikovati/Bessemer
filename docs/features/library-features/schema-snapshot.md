### 11.362 Library-feature configuration and cross-reference graphs

*Status: Verified ordinary modern part framing; all 906 installed modern part resolved streams now reach EOF; raw semantics and other formats remain incomplete · Evidence: C-404*

This extension requires the completed saved current-part model/configuration context, ordinary external-XML archive mode, v > 0x27CB and archive conditions 0x20/0x10000 clear. Clipboard/import, DB and archive-noise branches are not inferred from stream bytes. It adds seven local class readers; the eighth native binding is the direct moCompRefInfo_c helper used in the feature's final map. Existing reader maps are copied before installation.

**Library feature.** Primary table `0x4CF90838` selects `moLibraryFeature_c::Serialize` at `0x4BB294F0`. Native RTTI gives moBodyFeature_c ancestry, and +4B8/+4C0 hooks select the established ordinary concept/feature stages. The complete body-feature base is required first. Constructors `0x4BC07AB0`, `0x4BC07EF0` and `0x4BC08440` identify the embedded +800 suObList and +C60 suObArray. Their serializer calls are direct: no enclosing object tag or new map slot is present.

The modern ordinary field order after the inherited body is:

| Order | Member or role | Serialized form | Condition |
|---|---|---|---|
| 1 | raw mode +BA0 | u16 | always in scope; historical v > 0xA1A |
| 2 | list +800 | direct suObList: packed ReadCount and tagged objects | always in scope; historical v > 0xA82 |
| 3 | saved feature array +B98 | direct temporary suObArray | mode == 0 |
| 4 | stored time +CC0; raw +BA8 | u32 su_CTime archive value; CString | mode == 0 |
| 5 | raw +BB0/+C58/+BB4/+C5C | four u16 in this order | mode == 0; all historical gates satisfied |
| 6 | raw +B88; point +C88 | i32; three f64 | mode == 0; all historical gates satisfied |
| 7 | configurations +C60 | direct suObArray | mode == 0 |
| 8 | plane data +DA0 | tagged moXRefPlaneData_c pointer | mode == 0; historical v > 0x1110 |
| 9 | raw +DA8 | CString | always in scope; historical v > 0x12BB |
| 10 | raw +DC0/+DD0; handle +DC8 | two i32, then tagged moCompSketchEntHandle_c pointer | v > 0x2B5C |
| 11 | raw +DD4/+DD8 | two i32 | v > 0x2B5F |
| 12 | component map +E18 | bounded i32 count; each entry direct moCompRefInfo_c (two i32), then i32 map value | v > 0x2B60 |
| 13 | legacy atom-ID map | direct moAtomIdToFeatIds_c | 0x2B7B < v < 0x2BAC |
| 14 | atom-ID nodes +E38 | bounded i32 count and tagged moAtomIdToFeatIdNode_c pointers | v > 0x2BAB |
| 15 | raw +E10 | u8 | v > 0x42DF |

The ordinary branch omits the direct +BD8 serializer, imported face-visual/transform arrays and condition-0x10000 identity field. Native cleanup, default initialization, atom-map conversion and runtime feature fixups add no separate ordinary wire fields. The temporary array in row 3 is preserved in full, including all items; native retention of only its first item is not a lossless schema rule. Negative or impossible counts are rejected by this scoped implementation. Duplicate map keys and stored order are retained.

**Time operator proof.** The native store instruction at `0x4BB2976A` imports `operator<<(su_CArchive&, su_CTime)` and the load at `0x4BB2982B` imports `operator>>(su_CArchive&, su_CTime&)`. A generic-looking decompiler pointer cast on the store side is not an object pointer grammar. The load export in hash-bound `swccu.dll` is `0x31D4BC60`, which calls `0x31D343B0`; its ordinary branch reads one DWORD and advances the archive cursor by **four bytes**. Stored `0xFFFFFFFF` is normalized by native runtime code to its -1 time sentinel. The reader retains the raw u32. Timestamp roles and units for these library fields remain unassigned, so they remain unknown semantics.

**Configuration data.** `moXConfigData_c` selects `0x4BB36B10`. It reads u16 raw +20, CString raw +18, u32 raw +68, then a bounded i32 feature-map count. For each entry it reads a **u16 key through v 0x2BC8**, or an **i32 key above 0x2BC8**, followed by a tagged moXFeatureData_c pointer. It then directly serializes +28 suObArray and +48 su_CDWordArray. Constructor `0x4B42BDB0` proves these distinct container types: the second uses packed ReadCount and count u32 values, not tagged objects. Native filtering of nonpositive modern keys or null values does not remove their serialized entries. The decoder retains ordered pairs, duplicate keys, cached identities and all raw words/strings.

`moXFeatureData_c` selects `0x4BB36E60`: CString +8, u16 +10, four-byte su_CTime +18, u16 +20, four-byte su_CTime +28, then i32 +40 when v > 0x2BC8. These fields remain unknown semantics. Their widths and version gate are independently tested with distinct values and every truncation boundary.

**Cross-reference data.** `moXRefData_c` selects `0x4BB37000`: u16 +40, CString +48, tagged moRefWrapper_c pointer +10, generic tagged pointer +8, u8 +30 and point +18 (three f64). The final byte/point gate is v > 0xA9E, always satisfied here. Both pointers retain normal archive-map identity; unsupported nested objects fail explicitly. Raw metadata remains unknown. `moXRefPlaneData_c` selects `0x4BB37160` and is three i32 values at +8/+C/+10. Native input normalizes the first to an enum-like 0/1/2; the decoder preserves the stored integer and does not infer the full domain.

**Atom-ID records.** `moCompRefInfo_c::Serialize` at `0x4B517A10` has the ordinary empty su_CObject stage and two i32 values at +8/+C. The containing library map appends a third i32 value, so each direct entry occupies twelve bytes and adds no MFC slot. `moAtomIdToFeatIdNode_c` selects `0x4B513360`: i32 +8 followed by a tagged moAtomIdToFeatIds_c pointer +10. The latter selects `0x4B513430`: bounded i32 count and that many i32 IDs; bounded i32 map count with each entry an i32 key, bounded i32 list count and that many i32 IDs; then, for v > 0x2B7E, bounded i32 count and tagged moCompSolidBody_c pointers. All ID/list/map bytes remain unknown semantics. The final body-reference pointers use their complete established readers; unsupported body classes stop. The historical direct atom-map variant introduces no outer tagged header.

The costing sample `samples/tutorial/costing/machined_part.sldprt`, Config-0, now reaches **299,346-byte exact EOF**, comprising **97,789 unknown** and **201,557 known structure bytes**, with all **35 declared roots** complete. Its saved map contains **two library features, two configuration records, eight feature-data records and four cross-reference records**. Both library features have mode zero and one configuration entry; final component maps and atom-node collections are empty in this source. Synthetic populated cases verify the additional native branches without claiming populated native fixtures for those branches.

`resolved-library-audit-0.382.json` records **906 complete streams, zero explicit stops and zero context stops**, with **26,789 complete roots**. Every preceding **905** positive stream preserves source hash, endpoint, root count and unknown total. Complete streams span **83,682,596 decoded bytes**, including **29,782,259 unknown** and **53,900,337 known structure bytes**. This closes the observed modern-part resolved-feature framing frontier, not the original all-document semantic coverage objective.

Add `--library-dependencies` to the full Edition 0.381 audit/regrade commands. Public complete reads enable this scope; staged reads default it off. `resolved_coverage_evidence.py --library-dependencies` independently verifies all **1,166 source rows**, hashes, stored spans and integer attribution. Added stored part credit is **38,591 bytes**. Cumulative resolved-family known bytes become **15,050,739**, leaving **10,086,727 unknown** of 25,137,466 stored bytes. Total part unknown bytes become **53,316,893** of 227,086,352. Rounded coverage remains parts **76.5% / 71.4%**, assemblies **83.8% / 81.3%** and drawings **57.3% / 49.9%**. No whole-document or installed-project completion is claimed.

`library_feature_evidence.py` verifies eight native bindings, fifteen complete native function ranges, ordinary inherited hooks, constructor-proven containers, exact time operators and provider bytes, complete source EOF and preservation of all 905 preceding positives. Six focused tests cover configuration key-width transitions, duplicate/reference identity, direct-container map slots, time widths and unsigned values, cross-reference order, raw plane value retention, nested atom counts, library mode branches, all synthetic truncation boundaries, invalid counts/context, disabled-scope zero credit and appended-byte zero credit. The preceding C-403 verifier and decoder are frozen under `research/frozen-decoder-0.381/`; the prior gap report retains its original hash.

All **1,213 regression tests pass** (232.067 seconds). Specification lint reports zero findings (586 headings, 96 anchors).

Evidence: C-404.

## 12 Face tessellations

<a id="st-facettess-overview"></a>
