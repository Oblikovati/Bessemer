### 11.322 Version-11000 owned object, scene and manager suffix

*Status: Partial document semantics · one additional supported manager; 916 total · Evidence: C-364*

The remaining legacy part-config-object stop is an actual **version-11000** source. Native `moPartConfigObject_c` Serialize 0x4C234990 reads tagged pattern +0xA0 above 0x2390, tagged list +0x88 only above 0x2B8A and tagged object +0xA8 only above 0x4972. Version 11000 (0x2AF8) therefore has the pattern pointer but no +0x88 or +0xA8 input. Treating +0x88 as an unconditional modern pointer would consume the following part-configuration field.

`partconfigurations.part_config_object` now permits this exact version when the explicit version-11000 option is enabled. It retains the previous rejection for other unverified versions below the modern +0x88 boundary. The reader records `list_88_present=false` and `list_88=None`, consuming no substitute pointer bytes. Its other supported empty-list/optional-body restrictions remain unchanged. The measured directly owned object is **50 bytes**; it does not add a tagged outer object header or slot.

This correction advances the source to the manager's owned-data stage, whose suffix also requires version gates. `configurationintermediate.owned` accepts only version 11000 with the explicit option and ordinary modern component context. It follows native manager Serialize 0x4C22C140:

| Stage | Version-11000 input |
| --- | --- |
| owned decal +0x6D8 | signed object count and object/raw-word entries; signed association count, each raw key and signed-count tagged-pointer array |
| owned scene +0x6E0 | signed object count and scene/raw-word entries; signed association count, each raw key and one tagged scene pointer |
| conversion IDs +0x700 | signed i32 count and tagged conversion pointers |
| strings +0x728 | direct packed-count CString array |
| raw +0x750 | i32, present above 0x27BE |
| raw +0x754 | u32, present above 0x27D5 |

Fresh decal and conversion-ID bodies remain explicit dependencies. Scene bodies use a temporary local scene table, with normal tagged archive identity. Reader tables are restored after success or failure. The following input groups are absent: plane map +0x7F0 (>0x3733), named-data fields +0x808..+0x828 (>0x3765), set map +0x6B8 (>0x41A2), raw +0x858 (>0x42E8), and display-map presence (>0x46C1). External configuration-data handling still contributes zero inline cursor bytes; that fact does not establish its external-data semantics.

The owned cubical scene uses the same native dispatch and gated scalar body established in 11.313. `configurationoldscenes` accepts version 11000 only with this explicit option and the ordinary modern context guard; its established versions 9000/10000 retain their previous scope. The node tooltip is present; HDR exposure/tone mapping (>0x42EC) and field of view (>0x46BA) are absent. No file version is rewritten to reuse a modern suffix, and absent native runtime defaults are not synthesized as stored fields.

The source now completes normal tagged loading and the guarded singleton postload path. Its +0x358/+0x360 and sole root-list item refer to the same populated configuration. The owned scene and its association resolve to the same archive slot. The scene body totals **3,128 bytes**, including **1,526 unknown bytes**, and contains **six rectangular walls**. The owned-manager-through-suffix span totals **3,190 bytes**. Decal collections, conversion IDs and the string array are empty; raw +0x750 is zero and raw +0x754 is **2**, retained as its original unsigned value rather than normalized to a boolean. All excluded input groups are recorded absent. Exact field EOF is followed by the existing zero-inline-byte postload validation.

Public `read_supported` enables the exact version-11000 extension by default; `version11000=False` preserves historical replay scope. Prefix reads use `version11000_records`. Existing older reader dispatch, modern context guards, scene table isolation and unverified-version rejection remain in force.

The all-part audit now reports **916 supported managers / three explicit stops**, preserving every prior **915** complete row exactly. The new stream adds **20,536 decoded bytes**, split into **11,603 known structure bytes** and **8,933 unknown bytes**. Cumulative complete manager streams contain **9,330,178 bytes**: **4,034,702 known** and **5,295,476 unknown**. Remaining manager failures are optional data +0x608 and two nonempty decal-owned stages.

The regrade adds **1,456 stored part bytes** beyond edition 0.341. Cumulative known CMgr credit becomes **866,314 stored bytes**, leaving **999,798 stored CMgr bytes unknown** in the unchanged **1,866,112-byte** denominator. Total part unknown bytes become **68,378,415**. Rounded coverage stays **69.9% / 63.4%** for parts, 83.8% / 81.3% for assemblies and 57.3% / 49.9% for drawings. Full document and installed project-format semantics remain unfinished.

`configuration_version11000_evidence.py` revalidates exact native owned-object, manager, scene/base/data/wall function bytes and source intervals, actual pointer identities, raw suffix values, unchanged prior positive audit rows and proportional stored credit. Tests exercise the absent +0x88 field and modern +0xA8 field against sentinels, reject neighboring unverified versions and disabled scope, retain full u32 values, verify reader restoration, and reproduce the source and all-part audit. Current snapshots are `configuration-manager-supported-0.342.json`, `configuration-version11000-corpus.json` and `configuration-manager-coverage-0.342.json`.

Validation: all **1,075 regression tests pass** (119.284 seconds); specification lint reports zero findings (546 headings, 96 anchors).

Evidence: C-364.
