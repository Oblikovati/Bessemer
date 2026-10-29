### 11.321 Exploded-part IDs and direct visual maps in configurations

*Status: Partial document semantics · two exploded IDs, four direct visual entries and 915 supported managers · Evidence: C-363*

The remaining modern `moExplPrtId_c` dependency uses the same native input already decoded in part model streams. Its primary vtable 0x4CF16A48 binds Serialize to 0x4B684190. After ordinary zero-byte `su_CObject` input, that function reads a name CString at +0x10 and raw signed i32 +0x18. A historical tagged concept pointer at +0x20 is read only below version 0x3B3 and is excluded by the existing modern component/context scope. The additional helper construction above version 0x12CA does not introduce archive input. `configurationextras.exploded_id` reuses `partresolved.exploded_id` and retains body interval/unknown metadata; it does not infer support for every exploded-view object merely from this ID.

The direct visual map +0xB50 has different framing from a tagged visual-property map. Native base-configuration Serialize 0x4C225030 sets the helper's body number to -1, then, above version 0x144B, reads a signed i32 count. For each positive-count entry it reads a signed i32 key, allocates a visual-properties object and directly invokes exported `moVisualProperties_c::restore`, bound to 0x3CBCFDE0 in the installed SLDMFCU module. There is **no MFC class or object tag before each visual body**.

| Structure | Wire layout | Accounting |
| --- | --- | --- |
| exploded-part ID | tagged `moExplPrtId_c` header/reference; body CString name and raw i32 +0x18 | normal archive slots; raw integer remains unknown |
| direct +0xB50 map | signed i32 count; each raw i32 key followed by direct versioned visual-properties body | direct values create no MFC slots; raw keys and visual unknown fields remain unknown |

The visual input body is the established versioned `dl_light.r_moVisualProperties_c` payload. The native caller invokes restore directly; the tagged visual serializer's ordinary base contributes no additional input before that same payload. Its material/appearance subrecords retain their existing gates and unknown-byte accounting. The decoder bounds counts before reading entries and preserves source order and duplicate keys. Native tree replacement of a duplicate key is runtime state handling; applying it to the retained wire list would discard source bytes. Nonpositive signed counts consume no entries.

`configurationextras.install` locally installs only the explosion-ID class. Direct map entries call the visual-body reader without registering objects in the archive map. The extension retains modern ordinary external-XML context guards and does not widen older reader scope or global class registration. Supported manager reads enable `configuration_extras` by default; `configuration_extras=False` preserves the prior audit scope. Prefix reads use explicit `configuration_extra_records`. Existing part suffix and manager postload checks still determine completion.

`configuration_extras.py` completes the Carving Knife handle and Coping Saw manager sources. The first contains **two exploded-part IDs**, totaling **52 body bytes**, including **eight unknown bytes**. The second contains **one direct visual map with four entries**, whose key/body spans total **3,992 bytes**, including **1,760 unknown bytes**; its caller's four-byte count is outside that measured span. The observed keys are 3, 9, 34 and 40, which do not define a complete key domain. The map adds **zero archive object slots**. Synthetic duplicate-key entries verify that direct body decoding consumes no tags and leaves source order intact.

The all-part audit is now **915 supported managers / four explicit stops**, preserving all **913** previous complete rows exactly. Two new streams add **84,112 decoded bytes**, split into **42,214 known structure bytes** and **41,898 unknown bytes**. Cumulative complete manager streams contain **9,309,642 bytes**: **4,023,099 known** and **5,286,543 unknown**. The remaining failures are one legacy part-config-object body, optional data +0x608 and two nonempty decal-owned stages.

The regrade adds **7,006 stored part bytes** beyond edition 0.340. Cumulative known CMgr credit becomes **864,858 stored bytes**, leaving **1,001,254 stored CMgr bytes unknown** in the unchanged **1,866,112-byte** denominator. Total part unknown bytes become **68,379,871**. Rounded coverage remains **69.9% / 63.4%** for parts, 83.8% / 81.3% for assemblies and 57.3% / 49.9% for drawings. No full-document or installed-project-format completion is claimed.

`configuration_extras_evidence.py` revalidates native ID/visual primary tables, exact function bytes, the exported direct restore target and its configuration caller import, all source body intervals, unchanged prior positive audit rows and proportional stored-byte attribution. Tests exercise ID raw signed values and scope rejection, duplicate direct-map keys with trailing sentinels, unchanged archive slots, negative/oversized counts, absence of global registration and the full current audit. Current snapshots are `configuration-manager-supported-0.341.json`, `configuration-extras-corpus.json`, `configuration-extras-native-evidence.json` and `configuration-manager-coverage-0.341.json`.

Validation: all **1,072 regression tests pass** (117.259 seconds); specification lint reports zero findings (545 headings, 96 anchors).

Evidence: C-363.
