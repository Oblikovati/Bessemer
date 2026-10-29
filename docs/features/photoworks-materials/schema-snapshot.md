### 11.323 Optional PhotoWorks material manager and properties

*Status: Partial material semantics · three manager/material bodies; 917 supported managers · Evidence: C-365*

The positive optional-data +0x608 branch constructs `moPWMaterialManager_c` directly. Native base-configuration Serialize 0x4C225030 selects this branch from a u8 presence value, adds condition 0x200, calls constructor 0x4C1FEF90 and invokes virtual slot +0xA0. Primary vtable 0x4D130A20 binds that load slot to 0x4C211D80. The manager body has **no tagged outer object header**.

The manager load calls ordinary `su_CObject::Serialize`, reads raw i32 +0x10 at version >=0xE44 and raw i32 +0x14 at version >=0xF1E, then serializes its directly owned +0x08 `suObList`. The modern ordinary context accepted by this extension is already above both gates. Constructor 0x4B75A320 establishes the direct list allocation. Input is two signed integers followed by packed-count tagged material pointers; counts are bounded before pointer reads. Null, unsupported-type or incomplete material objects reject this supported path.

The material list contains `moPWMaterialProperties_c`, whose primary vtable 0x4D130BC0 binds Serialize to 0x4C203E30. This tagged object owns a separately allocated property-data structure through its memory member +0x10. The data pointer itself is not serialized as a tagged object. Offsets below refer to that **property-data structure**, except the two explicitly labelled object-owned lists. In supported modern context, the exact input order is:

| Order | Wire fields |
| --- | --- |
| 1 | i32 +0x20; u32 +0x28/+0x2C/+0x30; f64 +0x38; i32 +0x40/+0x48/+0x4C |
| 2 | f64 +0x68/+0x70/+0x78/+0x80/+0x88/+0x90; i32 +0x1C0/+0x1C4; f64 +0x50/+0x58; i32 +0x60 |
| 3 | i32 +0x98/+0x9C; f64 +0xB0/+0xB8; i32 +0xA0/+0xA4/+0xA8; CStrings +0xE0/+0xE8 |
| 4 | i32 +0xF8; f64 +0x100/+0x108/+0x110/+0x118/+0x120/+0x128; u32 +0x130; f64 +0x138/+0x140/+0x150/+0x158/+0x168/+0x170/+0x178/+0x180/+0x188/+0x190/+0x198/+0x1A0; i32 +0x1A8 |
| 5 | two three-f64 vectors +0x1D8/+0x1F0; direct list at material-object +0x08; three-f64 point +0x208; direct list at material-object +0x20 |
| 6 | f64 +0xC0/+0xC8; i32 +0x1AC/+0x220; f64 +0x228/+0x230/+0x238/+0x240; i32 +0x248 |
| 7 | f64 +0x250/+0x258/+0x260/+0x268/+0x270/+0x278; CString +0x280; i32 +0xD8 |
| 8 | i32 +0x1B0/+0x1B4/+0x1B8/+0x1BC/+0x288/+0x2AC; f64 +0x2A0; i32 +0x2A8/+0x298; f64 +0x1C8; i32 +0x2C0; f64 +0x2C8; i32 +0x2D0 |
| 9 | CString +0x290; i32 +0x2D4/+0x2E0; CString +0x2F8; f64 +0x300/+0x308; i32 +0x310/+0x314; f64 +0x318 |

These are typed wire positions, not padding or a memory dump. Their application parameter meanings remain unknown and their scalar/vector/point/CString bytes are charged as unknown semantic data. Both embedded list allocations are established by material constructors 0x4B75A370 and 0x4C1FF130. Each list uses packed count plus tagged pointers, without a tagged outer list object.

Native historical gates for the table include >0xE1C (+0x90/+0x1C0), >0xE3B (+0x1C4), >0xE31 (+0x50/+0x58/+0x60), >=0xC8F (+0x1B0..+0x1BC), >=0xC9E (+0x288), >=0xCA4 (+0x2AC), >=0xCBE (+0x2A0/+0x2A8), >=0xCC3 (+0x298), >=0xD8D (+0x1C8), >=0xDB7 (+0x2C0/+0x2C8/+0x2D0), >=0xDE4 (+0x290), >=0xDF2 (+0x2D4), >=0xE6B (+0x2E0), >0xF9A (+0x2F8..+0x310) and >0x1037 (+0x314/+0x318). +0x248 is omitted only within 0xF1E..0xF28 inclusive. Every one of these gates is already resolved by the existing modern component/context minimum (>0x27CB). Earlier property versions are not admitted by the new reader.

Native load may replace raw +0x20 zero with a runtime value and change +0x28, and may rewrite stored material paths. The decoder preserves the original integers and CStrings instead. Synthetic default-plastic paths and zero fields verify this source-preserving behavior. A minimal zero-valued property body with empty lists is **601 bytes**, including **597 unknown bytes**; the four list-count bytes remain known structure.

`configurationpwmaterials` installs the material-properties reader only while reading the optional manager's material list. Its two nested lists admit mapped/null pointers and the already verified fresh `moUnitComponent_c` body; other fresh classes stop explicitly. Reader tables are restored on success and failure. The observed unit components have null owner pointers and external-model presence zero. No global material class registration or ordinary feature-reader reuse under an unverified helper state is introduced.

After list input, the manager invokes +0xE0 continuation 0x4B75B0C0 without an archive argument and assigns runtime indices at property-data +0x1D0. That member adds no source bytes. Its complete native body comprises five chained unwind ranges; the first 24-byte range alone is not treated as the whole function. Exact ranges, chain metadata and decompilation are retained. Runtime reference processing and default/path rewrites are not applied to retained source records.

The bikeframe source now reaches supported manager completion. It contains **three direct PhotoWorks material managers**, each with one tagged material-properties object. Manager spans total **2,512 bytes**, including **2,424 unknown bytes**. Their included material-property spans total **2,448 bytes**, including **2,400 unknown bytes**; these are nested spans, not disjoint additional credit. Each material's first direct list contains one unit component and its second list is empty. Three unit-component archive slots and their owner/external flags are recorded. Manager raw fields are +0x10=0 and +0x14=1 in all three observations; these values do not establish a complete domain.

The all-part audit reaches **917 supported managers / two explicit stops**, preserving all **916** prior complete rows exactly. The new stream adds **101,624 decoded bytes**, split into **47,007 known structure bytes** and **54,617 unknown bytes**. Cumulative complete manager streams contain **9,431,802 bytes**: **4,081,709 known** and **5,350,093 unknown**. Only the two nonempty decal-owned manager stages remain explicit manager failures.

The regrade adds **8,644 stored part bytes** beyond edition 0.342. Cumulative known CMgr credit becomes **874,958 stored bytes**, leaving **991,154 stored CMgr bytes unknown** in the unchanged **1,866,112-byte** denominator. Total part unknown bytes become **68,369,771**. Rounded coverage remains **69.9% / 63.4%** for parts, 83.8% / 81.3% for assemblies and 57.3% / 49.9% for drawings. Material parameter meanings, other model streams and all installed project-format semantics remain unfinished.

`configuration_pw_materials_evidence.py` revalidates native load/property code, constructor list imports, primary table slots, the full chained continuation, reused unit dependency, decompilation hashes, exact source intervals, prior positive rows and proportional stored credit. Tests preserve original paths/default values, check scalar and list boundaries with sentinels, reject invalid material types/counts/context, restore reader tables and reproduce the source and current audit. Historical replays select `pw_materials=False`; prefix reads use `pw_material_records`. Current snapshots are `configuration-manager-supported-0.343.json`, `configuration-pw-materials-corpus.json`, `configuration-pw-material-native-evidence.json` and `configuration-manager-coverage-0.343.json`.

Validation: all **1,078 regression tests pass** (119.735 seconds); specification lint reports zero findings (547 headings, 96 anchors).

Evidence: C-365.
