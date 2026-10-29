### 11.324 Decal property input and nested association arrays

*Status: Partial opt-in reader; two component-face dependencies · Evidence: C-366*

`moDecalProperties_c` primary RTTI table 0x4CEEFBE8, locator 0x4D326960, binds Serialize slot +0x28 to 0x4BA20740 (native interval 0x4BA20740..0x4BA214F1). Ordinary input calls `su_CObject::Serialize`, then reads the sequence below. Data offsets refer to the structure reached through object member +0x10; object offsets refer to the decal object itself. These are memory offsets, not wire offsets.

| Order | Wire input |
| --- | --- |
| 1 | data i32 +0x48/+0x4C; f64 +0x68/+0x70/+0x78/+0x80/+0x88; i32 +0x98/+0x9C; f64 +0xB0/+0xB8; i32 +0xA0/+0xA4/+0xA8; CStrings +0xE0/+0xE8 |
| 2 | object i32 +0x44; CString +0x48; i32 +0x50; signed i32 index count, then max(count,0) u32 values |
| 3 | data i32 +0xF8; f64 +0x100/+0x108/+0x110/+0x118/+0x120/+0x128; u32 +0x130; f64 +0x138/+0x140/+0x150/+0x158/+0x168/+0x170/+0x178/+0x180/+0x188/+0x190/+0x198/+0x1A0; i32 +0x1A8 |
| 4 | two three-f64 vectors at data +0x1D8/+0x1F0; object i32 +0x68; direct object-owned list +0x08 |
| 5 | data three-f64 point +0x208; direct object-owned list +0x20; data f64 +0xC0/+0xC8; i32 +0x1AC/+0x1B0/+0x1B4/+0x1B8/+0x1BC/+0x2AC; f64 +0x1C8 |
| 6 | object f64 +0x70/+0x78; i32 +0x80/+0x84/+0x88/+0x8C/+0x90/+0x94 |

Historical gates are >0xBA1 for point +0x208, >=0xBA3 for the second list, >=0xBBB for data +0xC0/+0xC8, >=0xBDE for +0x1AC, >=0xC8F for +0x1B0..+0x1BC, >=0xCA5 for +0x2AC, >=0xD8D for +0x1C8, >=0x1190 for object +0x70/+0x78/+0x80/+0x84/+0x88, >0x12B2 for +0x8C and >=0x1BC3 for +0x90/+0x94. Below 0x1190 native input replaces data +0x140 using +0x118 after reading it. The new opt-in reader requires ordinary external-XML context and version >0x27CB, resolving these branches, and preserves source values without runtime conversions.

Both direct lists use packed counts and tagged pointers. A minimal zero-valued body with empty lists/CStrings is **423 bytes**, including **415 unknown bytes**. The four-byte index count and two two-byte list counts establish structure. All scalar parameters, index values, vectors, point and strings remain semantically unknown. This probe grants no whole-stream credit.

Condition-200 map helper 0x4BA25CC0 reads signed i32 object count, then tagged decal pointer plus raw i32 value for each positive-count entry. It next reads signed association count; each association is raw i32 key, signed i32 pointer count, then that many tagged decal pointers. Negative counts execute no iterations. Scene-map associations contain one pointer instead; they are not interchangeable. Native map construction may discard nonpositive values or merge keys; retained records preserve every wire entry. Raw values/keys remain unknown. Positive counts are bounded against remaining bytes and reader tables are restored after success/failure.

`configurationdecals` is opt-in through archive flag `configuration_decals`; supported-manager defaults are unchanged. Its direct lists currently admit mapped/null pointers and the verified fresh unit component. Fresh component-face references stop explicitly pending dependency verification under this helper condition. No complete source decal body is claimed.

The probe reaches the first direct list in both remaining sources. `sps_pc_board.sldprt` retains [4841,5561), **720 bytes**, including **696 unknown bytes**, and stops at fresh `moCompFace_c` tag 0x15A7. `usb_case2.sldprt` retains [2999,3643), **644 bytes**, including **620 unknown bytes**, and stops on the same class. Intervals include consumed list count and class header at failure; these are prefix observations. Stored paths are preserved. Supported-manager totals remain **917/919**; stored coverage is unchanged.

`configuration-decal-native-evidence.json` retains module hash, primary binding, serializer/map native bytes, decompilation hash and source intervals. `configuration_decals_evidence.py` verifies those identities and intervals; `configuration-decal-corpus.json` retains both probes. Tests check scalar-body sentinels/truncation, nested association arrays, shared pointer identity, count bounds, reader restoration and exact source stops.

Validation: all **1,081 regression tests pass** (123.401 seconds); specification lint reports zero findings (548 headings, 96 anchors).

Evidence: C-366.

### 11.325 Component-face prefixes inside decal lists

*Status: Opt-in prefix only; fresh face-reference bodies remain open · Evidence: C-367*

The decal list's first fresh class in both remaining files is `moCompFace_c`. Its primary table 0x4CE065A8, locator 0x4D2FFE60, binds Serialize +0x28 to 0x4BBB7E80, the same serializer as `moCompBEntRef_c` (primary table 0x4CE8F970). This function tests helper condition mask 0x2006607 before ordinary input. The enclosing decal map adds condition 0x200; that bit is included in the mask. With no filed-concept object in this ordinary external-XML scope, the condition selects the component-reference input path.

The inherited `moCompRef_c` Serialize binding is 0x4BBBA690 at primary table 0x4CE91018. Its ordinary input grammar remains the component pointer and versioned fields documented in 11.17. Modern boundary-reference input then reads alias-presence u8, the alias CString when present, a boundary-reference pointer, and the external-reference pointer. The boundary pointer is also present in the alias case at these versions. Native load retains runtime reference-count/owner operations without adding wire bytes. The retained native functions, primary RTTI bindings and decompilation hashes are checked against the installed module by `configuration_decal_faces_evidence.py`.

Archive flag `configuration_decal_faces` enables this dependency only within the already opt-in decal reader. The local list table admits fresh component-face and unit-component bodies, along with mapped/null pointers; it restores the previous table afterward. Unsupported fresh dependencies still stop. This does not expand default supported-manager scope or grant any stored coverage credit.

The two probes advance **124 bytes each** beyond 11.324. `sps_pc_board.sldprt` reaches cursor **5685**, stopping at fresh `moFaceRef_c` tag **0x1624**. `usb_case2.sldprt` reaches cursor **3767**, stopping at the same class at **0xEA6**. These endpoints include the consumed fresh-class header; neither component-face nor decal object is complete. `configuration-decal-face-corpus.json` retains the exact larger prefixes, source hashes and explicit errors. `configuration-decal-face-native-evidence.json` binds them to the native evidence. The original component-face stops are still reproduced with the extension disabled.

Tests reproduce both extended and original boundaries and verify mapped component-face identity and local reader restoration. Supported managers remain **917/919**, and full document/project bit coverage remains unfinished.

Validation: all **1,082 regression tests pass** (180.818 seconds); specification lint reports zero findings (549 headings, 96 anchors).

Evidence: C-367.

### 11.326 Complete decal dependency graphs and all-part manager framing

*Status: Complete supported manager framing in all 919 installed parts; partial semantics · Evidence: C-368*

The two decal-owned stops are resolved by scoped reuse of modern face-reference and entity-ID input. `moFaceRef_c` primary table 0x4CF97008 binds Serialize to 0x4BBBCE10, calling boundary input 0x4BBB6F40 (base table 0x4CE87020). The boundary grammar is 11.18: raw u32 +0x60/+0x64, u32 entity count, raw u32 +0xE8/+0x70, two f64 parameters, vertex-reference pointer, direct entity-ID records, raw u32 +0x48, temporary-record count and modern +0xEC/direct-model fields. Nonempty unsupported temporary/direct-model branches still reject. Native load normalizes some in-memory flags; source values remain preserved.

The face suffix reads its signed i32 record count when version >0x109B and helper condition 0x80 is absent, then four 32-bit words per record (+0x08/+0x0C/+0x10/+0x14). The decal map adds 0x200 and does not add 0x80, so this ordinary input includes the count and records. The supported modern version minimum exceeds this gate. Raw record words retain unknown semantics. The reader bounds counts before iteration.

| Nested class | Primary table | Serialize / suffix | Additional input (moFR row gives its body) |
| --- | --- | --- | --- |
| moEndFaceSurfIdRep_c | 0x4CF71658 | 0x4BBB79B0 / 0x4BBBF470 | two raw i32 words |
| moNetSideSurfIdRep_c | 0x4CF723C8 | 0x4BBB79B0 / 0x4BBBF3F0 | two raw i32 words |
| moFromSktEntSurfIdRep_c | 0x4CF6BBC0 | 0x4BBBDE70 | one raw i32 word |
| moSimpleSurfIdRep_c | 0x4CF6A680 | 0x4BBBDE70 | one raw i32 word |
| moFromSktEnt3IntSurfIdRep_c | 0x4CF74EB8 | 0x4BBB79B0 / 0x4BBBED80 | three raw i32 words |
| moFR_c | 0x4CF5BE70 | 0x4BB48950 | external-model pointer, raw i32 and u32 archive time |

The shared 0x4BBB79B0 dispatcher calls inherited representation input and virtual suffix slot +0x2A0. The retained packet verifies each concrete suffix binding. Entity representations contain a feature pointer and external feature-reference pointer, as in 11.18. Fresh face/vertex references and these specific ID classes are installed only in the decal's nested list scope; unsupported fresh classes retain explicit failures. Default `read_supported` now enables `decals=True`, while prefix reads require `decal_records=True`. Historical replays explicitly disable this extension. The earlier face-only and reference-only probes remain independently reproducible.

The circuit-board CMgr consumes exactly **6,367 bytes**, including **3,755 unknown bytes**. Its single decal body is [4841,6281), **1,440 bytes**, including **1,080 unknown bytes**; direct list counts are 1 and 0. The USB-case CMgr consumes exactly **7,794 bytes**, including **4,205 unknown bytes**. Its single decal body is [2999,4240), **1,241 bytes**, including **972 unknown bytes**; both direct list counts are 1. Both ordinary tagged roots pass the existing postload graph guard and exact EOF check. Their stored paths and raw parameters are retained.

The all-part audit now completes **919/919 managers**, with no explicit manager stops. Every one of the prior **917** positive rows is unchanged. Complete manager streams total **9,445,963 decoded bytes**, comprising **4,087,910 known structure bytes** and **5,358,053 unknown bytes**. The two added streams contribute 14,161 decoded bytes, split into 6,201 known and 7,960 unknown. Nested decal spans are included in those totals, not additional credit. All-manager framing completion does not establish complete semantic decoding or whole-document coverage.

The stored-byte regrade adds **2,139 known part bytes**, raising cumulative CMgr credit to **877,097** in the unchanged **1,866,112-byte** denominator; **989,015 stored CMgr bytes remain unknown**. Total part unknown bytes become **68,367,632**. Rounded coverage remains 69.9% / 63.4% for parts, 83.8% / 81.3% for assemblies and 57.3% / 49.9% for drawings. No whole document is complete, and all installed project-format semantics remain in scope. `configuration-manager-coverage-0.346.json` retains the stored attribution.

`configuration-decal-reference-native-evidence.json` retains primary RTTI/suffix bindings, native bytes and decompilation identities. `configuration_decal_references_evidence.py` verifies those against the installed module, exact source lengths/hashes, completed decal spans and all prior positive audit rows. `configuration-decal-id-corpus.json` retains both completed probes; `configuration-manager-supported-0.346.json` retains the 919-file audit. Tests replay the two dependency graphs, exact EOF/body totals and original disabled-scope stops, and recheck known-structure credit across the complete installed part corpus.

Validation: all **1,083 regression tests pass** (122.786 seconds); specification lint reports zero findings (550 headings, 96 anchors).

Evidence: C-368.
