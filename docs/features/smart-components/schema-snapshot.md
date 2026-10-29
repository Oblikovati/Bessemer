### 11.359 Smart-component features, folders, items and sensor maps

*Status: Verified framing in ordinary modern part archives; raw values and application behavior remain partial · Evidence: C-401*

Ten additional class names compose the saved smart-component graph. The hash-bound installed `sldmodu.dll` selects these primary serializers:

| Class | Serialize | Stored composition |
|---|---|---|
| `moSmartComponentFeature_c` | 0x4C63E110 | model-feature base; tagged data; tagged external object |
| `moSmartComponentFeatureData_c` | 0x4C63DD60 | direct arrays/list/map, context pointer, folder pointers and raw words |
| `moSmartComponentFeatFolder_c`, `moSmartComponentCompFolder_c`, `moSmartComponentRefFolder_c` | 0x4C63DD20 | complete folder base; direct array +3B8 |
| `moSCItemData_c` | 0x4C63DAF0 | complete modern node body |
| `moSCFeatureItemData_c` | 0x4C63D890 | node base; feature pointer, strings, raw word and smart owner |
| `moSCCompItemData_c` | 0x4C63D640 | node base; transform-wrapper pointers, smart owner, string and raw word |
| `moSCRefItemData_c` | 0x4C63DB80 | node base; strings, smart owner and direct array |
| `moSensorDiaMapList_c` | 0x4CBA27FF | import thunk to the native string-to-object map serializer |

**Top feature and data.** The top feature serializes the complete model-feature base, tagged non-null `moSmartComponentFeatureData_c` at +368, then tagged `moExtObject_c` at +378. Native loading immediately dereferences the data pointer; null data fails explicitly. The +378 version gate is `v > 0xBDF`, satisfied throughout the accepted `v > 0x27CB` range. Native condition `0x80` would add i32 +360, but that condition is absent in the required ordinary external-XML context. Its runtime default is not represented as a stored word.

The data serializer writes the following in wire order:

| Order | Native member | Encoding |
|---|---|---|
| 1 | +48 | direct object array: ReadCount and tagged entries |
| 2 | +68 | second direct object array |
| 3 | +88 | tagged `moContextChainComponent_c` pointer |
| 4 | +90 | direct object list: ReadCount and tagged entries |
| 5 | +E8, +E0 | tagged feature-folder and component-folder pointers |
| 6 | +F8, +FC | two i32 words |
| 7 | +100 | direct string-to-object map body |
| 8 | +F0 | tagged reference-folder pointer |
| 9 | +130 | i32 word |

The default constructor `0x4C63BA60` and parameterized constructor `0x4C63B840` establish `suObArray` +48/+68, `suObList` +90 and `moSensorDiaMapList_c` +100. These are direct member bodies, without extra pointer tags. The modern sequence excludes the discarded legacy CString below `0xB79` and single-object replacement of the +90 list below `0xB57`; the map and final folder/word groups have gates `v > 0xB81` and `v > 0xB84`, also satisfied. Condition `0x80` would append a CString and CStringList, but contributes no bytes in ordinary scope. Raw +F8/+FC/+130 contribute **12 unknown bytes**. Runtime owner assignment and folder back-reference updates add no archive reads.

**Folders and inherited node branches.** All three folder serializers call complete `moFolder_c::Serialize`, then directly dispatch an embedded `suObArray` +3B8. Their constructors at `0x4C63B770`, `0x4C63B690` and `0x4C63BEA0` prove that member type. The inherited feature/folder virtual hooks select the established concept lightweight/resolved serializers `0x4C189B60` and `0x4C18A920`; no inherited stage is skipped.

The item classes select node lightweight hook `0x4C18A1A0` and resolved hook `0x4B29DAE0`, whose verified three-byte body is a return. Thus their complete modern node body is the established node-name/flags/ID/string grammar, without the concept feature body. The base item helper selects moNode serialization from version `0xCEA`; all accepted modern versions take that branch. Derived item wire suffixes are:

| Item | Suffix after the complete node body |
|---|---|
| feature item | tagged `moCompFeature_c` +40; CString +50; i32 +58; tagged smart-feature owner +60; CString +48 |
| component item | tagged `moCompXFormWrapper_c` +40; tagged smart-feature owner +60; CString +50; i32 +58; tagged transform wrapper +48 |
| reference item | CString +68/+70; tagged smart-feature owner +78; CString +60; direct object array +40 |

Legacy feature-item substitutions below `0xB5D`/`0xB6E`, component migration below `0xBDD` and the discarded reference-item i32 below `0x2777` are outside modern scope. The reference-item constructor `0x4C63B4B0` proves the direct +40 `suObArray`. Item strings and raw signed words remain unknown semantics. Shared owner references retain the original MFC map objects, including references back to the containing smart feature; the reader does not construct substitute owners.

**Native string-to-object map.** The sensor-map table selects a six-byte import thunk at `0x4CBA27FF`. Its verified provider is `swccu.dll`, export `su_CMapStringToOb::Serialize(su_CArchive&)` at `0x31D4AD80`, in the hash-bound module with SHA-256 `9992e50f1dc70b721f060d4f5871147882cd5e86b454e190ae3d64f3272003db`. It calls `su_CArchive::ReadCount` at `0x31D36AD0`, then reads `(CString key, tagged generic object value)` per entry. This is the ordinary ReadCount grammar, u16 with `0xFFFF` followed by u32, not the separate MFC 64-bit count grammar. Every pair is retained in wire order, including duplicate keys and null values. Native SetAt overwrites an existing key in memory; the decoder preserves all serialized occurrences. Key CString bytes remain unknown semantics, while count and object-tag framing retain structural credit.

`samples/tutorial/api/holecube.sldprt`, configuration 0, now completes every declared root and required tail to exact EOF at **19,180 decoded bytes**, with **7,348 unknown** and **11,832 known structure bytes**. The populated graph includes one feature/data pair, all three folder types, two feature items and one reference item. The data's direct arrays/list have counts **0 / 2 / 1**; its sensor map is empty and raw words are `[2,0,0]`. Populated duplicate/non-null map entries and component-item boundaries are covered by native code and synthetic tests; no populated source case for those branches is claimed.

`resolved-smart-component-audit-0.379.json` records **901 complete / 5 explicit stops / zero context stops** among 906 modern part configurations, with **26,725 completed roots** across the audit. Every preceding 900 complete stream preserves endpoint, root count, source hash and unknown total. Complete streams now contain **82,820,119 decoded bytes**, including **29,497,370 unknown** and **53,322,749 known structure bytes**.

Add `--smart-component-dependencies` to the full Edition 0.378 audit/regrade commands. The public reader enables the scope; the staged reader defaults it off. `resolved_coverage_evidence.py --smart-component-dependencies` independently checks all **1,166 source rows**, hashes, stored spans and integer proportions. Added stored part credit is **3,579 bytes**. Cumulative resolved-family known bytes become **14,880,806**, leaving **10,256,660 unknown** of 25,137,466 stored bytes. Total part unknown bytes become **53,486,826** of 227,086,352. Rounded coverage remains **76.4% / 71.3%** for parts, **83.8% / 81.3%** for assemblies and **57.3% / 49.9%** for drawings. Full documents, remaining feature classes, raw semantics, legacy/assembly/drawing contexts and all installed project-format schemas remain incomplete.

`smart_component_evidence.py` checks ten primary bindings, all item/concept hook selections, eighteen native bodies in two modules, both data constructors, all folder constructors, the reference-item array, exact map import/export/count binding, the populated source EOF and preservation of every preceding positive. Seven focused tests cover direct-container widths, duplicate map keys, null/shared values, extended counts and bounds, full folder/node composition, owner cycles, all populated body truncations, required-data rejection, context guards, local installation, disabled-profile zero credit and appended-byte rejection. The preceding C-400 packet is reproduced with its original hash through exact sources frozen under `research/frozen-decoder-0.378/`. All **1,198 regression tests pass** (163.948 seconds); specification lint reports zero findings (583 headings, 96 anchors).

Evidence: C-401.
