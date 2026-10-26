# Complete detail-definition schema snapshot

Source: `../inventor-interface/docs/solidworks-file-format/specification.md`

Edition 0.313 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.103 Direct annotation-definition groups

*Status: Native constructor/serializer evidence; 906 complete groups and 9060 definition bodies; surrounding detail definitions remain partial · Evidence: [C-118]*

The next direct member of moDetailDefs_c is moAnnotationDefs_c at +1F18. Primary RTTI table 0x3CDE61E0 selects serializer 0x3CB02B40. Constructor 0x3CA50C80 establishes the ten embedded definition types and their memory offsets. The ordinary external-XML reader supports zero special archive conditions and v > 0x27CB. Its scalar prefix is i32 +10, f64 +08, i32 +14/+1C/+20/+24/+28, u32 +38, i32 +2C/+30, i32 +18 if v >= 0x2F70, then i32 +34 if v >= 0x376A. This occupies 44, 48 or 52 bytes. Other gates in the native prefix precede the supported minimum; native defaults assigned for omitted fields consume no bytes.

The ten direct definition bodies follow in this wire order. Every body first emits the shared base below. Except where a version gate is shown, the supported modern minimum exceeds all native gates for the listed suffix.

| Order | Member | Class | Suffix after shared base |
|---|---|---|---|
| 1 | 2C0 | moBalloonDefs_c | i32 +58/+5C/+60/+64/+68/+6C; CString +70/+78; u32 +80; f64 +90/+88; i32 +B4/+B8; CString +C0; f64 +98/+A0/+A8; i32 +B0 |
| 2 | 418 | moNoteDefs_c | i32 +58/+5C/+60; f64 +68/+70/+78; i32 +80 |
| 3 | 250 | moDatumDefs_c | i32 +58/+68/+6C/+60/+64/+5C |
| 4 | 1E8 | moGtolDefs_c | i32 +58 if v >= 0x3738; i32 +5C if v >= 0x3B05; u16 +60 if v > 0x3EF3 |
| 5 | 108 | moSFDefs_c | i32 +58/+5C; f64 +60/+68 |
| 6 | A0 | moWeldDefs_c | i32 +58, then direct utCharFormat_c at +60 |
| 7 | 178 | moBendNoteDefs_c | i32 +58/+5C/+60; f64 +68 |
| 8 | 4A0 | moBaseAnnotationDefs_c | None |
| 9 | 40 | moRevCloudDefs_c | f64 +58 |
| 10 | 398 | moViewLocationLabelDefs_c | i32 +58/+5C/+60/+64; f64 +68; i32 +70; f64 +78; i32 +74 |

moBaseAnnotationDefs_c primary table 0x3CDE5968 selects serializer 0x3CB03350. In this scope its shared body is i32 +08; f64 +10; i32 +18/+1C/+20/+24/+34/+40; i16 +44; two direct utLineWidth_c bodies at +2C/+38; direct utCharFormat_c at +50; then i32 +28. Each modern line-width body is i16 standard width and f32 custom width. Character-format framing is specified in 11.101. Constructor 0x3CA50F90 establishes these line-width and font objects; weld constructor 0x3CA512F0 separately establishes its additional +60 character format. These direct bodies allocate no MFC class/object entries.

Special archive condition 0x40000000 inserts a tagged moLayerData_c pointer after the signed short +44 when it is not -1. That condition is excluded by this reader's ordinary zero-condition context, not guessed from the next bytes. Older shared-base migration words and older derived variants remain outside this scope. The versioned moGtolDefs_c +60 field is a single native wchar_t: readers SHALL retain its two stored bytes as a code unit, including an unpaired surrogate, rather than parse it as a CString.

Native loading remaps balloon +68/+6C values of 8 to 3 and may set cut-list flags; view-location fields are normalized to booleans. The surface-finish writer can clamp +5C for older targets. These runtime choices do not add fields or replace the raw stored values in the schema reader. Readers SHALL retain original integers, short bits, strings and floating-point values. Unnamed scalar meanings remain unknown and retain conservative unknown-byte accounting.

The `--annotation-definitions` audit completes all 906 groups and 9060 bodies without errors. Group spans total 1770430 bytes, ranging from 1812 to 1956 bytes per group. Constituent bodies total 1723326 bytes, of which 1297338 remain unknown. The ten base character-format names per group are Century Gothic (8952), Arial (76), TechnicBold (14), Tahoma (10) and Arial Narrow (8); the additional weld font is parsed separately. The report retains source stream hashes, versions, group/member spans, base font names and per-body unknown-byte counts. Native fixture tests consume selected distinct version/size spans to their exact recorded ends without adding object-map entries.

This continuation ends after the view-location-label definition, before the following direct table-definition group at detail-definition +2410. The surrounding moDetailDefs_c and configuration streams remain partial; assembly/drawing archives and installed project payloads still require further decoding. These audit-scoped records do not establish whole-document bit coverage or alter its aggregate metric.

### 11.104 Direct table-definition groups

*Status: Native constructor/serializer evidence; 906 complete groups and 8153 table-definition bodies; surrounding detail definitions remain partial · Evidence: [C-119]*

The next direct moDetailDefs_c member is moTableDefs_c at +2410. Its primary RTTI table 0x3CF2F798 selects serializer 0x3CB0D200. Constructor 0x3CB80AE0 establishes nine embedded table-default types and a separately allocated utCharFormat_c at +328. In ordinary external-XML archives with zero special conditions and v > 0x27CB, the wire order is i32 +08; four direct definitions (general, revision, BOM, hole); direct character format +328; four direct definitions (title block, weld, bend, punch); family-table definition if v > 0x42F1; then f64 +330/+338. There is no outer tag or presence flag for any of these direct bodies.

| Order among table bodies | Member | Class | Suffix after shared general-table base |
|---|---|---|---|
| 1 | 10 | moGeneralTableDefs_c | None |
| 2 | 50 | moRevisionTableDefs_c | i32 +40/+44/+48; u32 +58; i32 +4C; CString +50 |
| 3 | B0 | moBOMTableDefs_c | u32 +40; CString +48/+50/+58 if v > 0x2B81 |
| 4 | 150 | moHoleTableDefs_c | i32 +40/+44; f64 +48/+50; i32 +58/+5C/+60/+68/+6C/+70/+74/+78/+64/+7C |
| 5 | 110 | moTitleBlockTableDefs_c | None; native function inlines the shared base |
| 6 | 1D0 | moWeldTableDefs_c | i32 +40 if v > 0x375F |
| 7 | 218 | moBendTableDefs_c | i32 +40/+44/+4C/+48 |
| 8 | 268 | moPunchTableDefs_c | i32 +40/+44; f64 +58/+60; i32 +48/+4C/+6C/+50/+70/+68 |
| 9, if v > 0x42F1 | 2E0 | moFamilyTableDefs_c | u32 +40 |

The separate +328 character format occurs between rows 4 and 5, not after the table collection. Member labels are relative to each table definition in the suffix column. Except for the explicit gates, the modern minimum exceeds the native suffix gates. Family-table serialization consumes no body in earlier versions and the parent omits its call; readers SHALL NOT emit or consume a default family-table record there.

moGeneralTableDefs_c serializer 0x3CB0A560 emits i32 +2C/+28; u32 +08; i16 +0C; i32 +38; two direct utLineWidth_c bodies at +18/+20; then direct utCharFormat_c at +10. Each modern line-width body is i16 standard width and f32 custom width; the character-format grammar is in 11.101. Constructor 0x3CB808A0 establishes the widths and allocated font object. The title-block serializer 0x3CB0D3D0 has the same wire body despite its inlined implementation. These direct records do not allocate MFC class/object slots.

Special archive condition 0x40000000 inserts a tagged moLayerData_c pointer after the signed short +0C when it is not -1. It is outside the zero-condition reader; its absence is a context requirement, not a guess from subsequent bytes. Native line-width migration in a version/build-dependent branch decrements standard-width values without reading more bytes. Older BOM loading can set a flag based on a .drwdot pathname without reading more fields. Readers SHALL retain raw stored widths, short bits, flags, strings and doubles independently of these runtime changes. Unnamed fields retain unknown-byte accounting.

The `--table-definitions` audit completes all 906 groups and 8153 table bodies without errors: 905 groups have nine bodies, and one older group has eight. Group spans total 1326330 bytes, ranging from 1280 to 1466 bytes. Constituent table bodies total 1217914 bytes, of which 860562 remain unknown. Base font names are Century Gothic (8067), Arial (66), Tahoma (8), TechnicBold (8) and Arial Narrow (4). The 906 separate group fonts are Century Gothic (887), Arial (14), Tahoma (2), TechnicBold (2) and Arial Narrow (1). The evidence report retains exact group/member/extra-font boundaries, source hashes, versions and per-body unknown-byte counts. Native fixture tests consume distinct version/size spans through their exact recorded ends while preserving object-map size.

This continuation ends after the two final doubles, before direct moLabelDefs_c at detail-definition +2750. The surrounding detail-definition object, remaining configuration data, assembly/drawing archives and installed project payloads remain incomplete. Audit-scoped framing does not establish whole-document bit coverage or alter its aggregate metric.

### 11.105 Direct view-label definition groups

*Status: Native constructor/serializer evidence; 906 complete groups and 4530 label bodies; surrounding detail definitions remain partial · Evidence: [C-120]*

The next direct moDetailDefs_c member is moLabelDefs_c at +2750. Primary RTTI table 0x3CF1C8C8 selects serializer 0x3CB0B6F0. Constructor 0x3CAF4390 establishes five embedded label-data objects and an allocated utCharFormat_c at +458. In ordinary external-XML archives with zero special conditions and v > 0x27CB, wire order is i32 +08/+0C; direct detail, section and auxiliary label data; direct character format +458; then direct orthographic and miscellaneous label data. The separate group font occurs after the first three bodies. All body-presence gates in the native group serializer precede this modern minimum.

| Order among label bodies | Member | Class | Suffix after shared view-label base |
|---|---|---|---|
| 1 | 18 | moDetailLabelData_c | u32 +CC, then i32 +C8, if v > 0x3B02; otherwise no suffix |
| 2 | E8 | moSectionLabelData_c | f64 +C8/+D0/+D8; i32 +E0/+E4/+EC/+F0/+E8 |
| 3 | 1E8 | moAuxLabelData_c | i32 +C8 |
| 4 | 2B8 | moOrthoLabelData_c | i32 +C8/+CC |
| 5 | 388 | moMiscLabelData_c | i32 +C8/+CC |

Primary tables establish distinct class identities for orthographic and miscellaneous data while both select native serializer 0x3CB1AC90. The detail, section and auxiliary serializers are 0x3CB1A720, 0x3CB1AD40 and 0x3CB1A6A0. Except for the detail gate shown above, the native suffix gates precede the supported minimum. Preserve raw words before native style/parent-view defaults; runtime assignments consume no additional bytes.

The shared moDrViewLabelData_c serializer at 0x3CB1A800 emits i32 +08/+0C/+18/+14/+1C/+20/+24/+28/+44/+2C/+38; i16 +48; CString +88/+90; u32 +5C; i32 +10/+58. It then directly serializes two character formats at +98/+A0, two utLineWidth_c bodies at +30/+3C, and four more character formats at +A8/+B0/+B8/+C0. Character-format framing is in 11.101; each modern line-width body is i16 standard width and f32 custom width. Constructor 0x3CB18C60 establishes all six allocated character formats and both embedded widths. The six fonts SHALL be consumed separately in this interleaved wire order; they are neither tagged pointers nor an MFC object collection. No class/object-map entries are allocated by these direct label records.

Special archive condition 0x40000000 inserts a tagged moLayerData_c pointer after signed short +48 when it is not -1. That condition is excluded by the ordinary zero-condition reader; its absence is not inferred from subsequent bytes. Native subtype-dependent updates to +20 and older font-copy migrations do not introduce fields in this supported range. Readers SHALL preserve original short bits, scalar values, strings and all six formats rather than substitute native normalized values. Unnamed fields remain unknown under conservative byte accounting.

The `--label-definitions` audit completes all 906 groups and 4530 label bodies without errors. Group spans total 3233040 bytes, ranging from 3246 to 3570 bytes. Constituent label bodies total 3135200 bytes, of which 2212324 remain unknown. The 27180 per-body character-format names are Century Gothic (27074), Arial (48), TechnicBold (32), Arial Narrow (18), Tahoma (6) and EuroRoman (2). The 906 separate group fonts are Century Gothic (905) and Arial Narrow (1). Thus this stage consumes 28086 complete character-format bodies. The report retains group/member/extra-font boundaries, source hashes, versions, six font names per label and per-body unknown-byte counts. Native fixture tests consume distinct version/size spans through their exact recorded ends with unchanged object-map size.

This continuation ends after miscellaneous label data, before direct moSFDataHelper_c at detail-definition +2E8. The surrounding detail-definition object, remaining configuration data, assembly/drawing archives and installed project payloads remain incomplete. Audit-scoped framing does not establish whole-document bit coverage or alter its aggregate metric.

### 11.106 Direct surface-finish and note data helpers

*Status: Native constructor/serializer evidence; 906 complete helper pairs/1812 bodies; populated note maps have native and synthetic evidence · Evidence: [C-121]*

Following the label group, moDetailDefs_c directly serializes moSFDataHelper_c at +2E8 and moNoteDataHelper_c at +4D8. Their primary RTTI tables select 0x3CB0CC20 and 0x3CB0BA70. Constructors 0x3CA7F340 and 0x3CA7F0E0 establish embedded utCharFormat_c and moAnnotationDataHelper_c members. These direct helpers do not allocate their own MFC class/object entries. The scoped reader supports ordinary external-XML archives and v > 0x27CB.

moAnnotationDataHelper_c, shared by both helpers, uses serializer 0x3CB02900. Its modern body is i32 +68/+6C/+70/+74; u8 +78; direct utCharFormat_c at +08; u8 +7C; i32 +80; f64 +88; i32 +90; u8 +94; i32 +98; f64 +A0; i32 +A8. Constructor 0x3CA7DC10 establishes its embedded character format. Every gate for this shared body precedes the supported minimum. The character-format grammar is specified in 11.101.

The surface-finish helper stores the following fields in exact wire order. Offset names are relative to this helper; they do not establish field meanings.

| Order | Stored type | Native offsets |
|---|---|---|
| 1 | i32; CString[10] | 08; 10/20/18/28/30/38/40/48/50/58 |
| 2 | u8[4]; i32; u8[4]; f64; u8 | 68/6C/70/74; 78; A0/A4/A8/AC; B0; B8 |
| 3 | direct utCharFormat_c; u8[2] | C0; 120/124 |
| 4 | direct moAnnotationDataHelper_c | 128 |
| 5 | i32; f64, if v > 0x42E3 | 7C; 80 |
| 6 | CString[2]; i32[2], if v > 0x46BF | 88/90; 98/9C |

The note helper stores the following modern body. Native 32-bit boolean-like members are frequently serialized as single u8 values; readers SHALL preserve those wire widths and original byte values.

| Order | Stored type | Native offsets |
|---|---|---|
| 1 | CString[4]; u8[6]; i32; f64; u8 | 08/18/20/28; 10/30/34/38/3C/40; 44; 50; 58 |
| 2 | i32[8]; f64[2] | 5C/60/78/7C/80/84/88/8C; 90/98 |
| 3 | u8[12]; direct utCharFormat_c | A0/A4/A8/AC/B0/B4/B8/BC/C0/12C/130/134; C8 |
| 4 | i32[2]; versioned helper map | 240/244; map at +248 if v > 0x2F4A |
| 5 | u8[3]; direct moAnnotationDataHelper_c | 138/13C/140; 150 |
| 6 | u8[2]; i32; f64; i32[2]; CString | 48/128; 14C; 68; 218/21C; 220 |
| 7 | i32[4]; f64[2]; i32; u8 | 230/234/238/23C; 70/228; 4C; 144 |
| 8 | u8, if v > 0x3373 | 148 |

Other native gates for these note fields precede the supported minimum. The helper map begins with a little-endian u64 count (native size_t), not ReadCount or i32. Each record stores i32 key and i32 kind. Kinds 1, 2 and 4 add one tagged pointer declared as moGTolDataHelper_c, moSFDataHelper_c and moNoteDataHelper_c respectively. Other kinds add no pointer bytes in the native serializer. Readers SHALL retain record order, duplicate keys, raw kind values, null pointers and repeated object identity. Native loading can discard unknown kinds or replace duplicate map entries; those runtime changes do not remove their serialized bytes. The bounded reader rejects counts above 0x7FFFFFFF or above remaining_bytes/8, avoiding the native loop's signed-int truncation. Unsupported newly declared helper classes still stop; their bodies are not skipped. The composed reader installs surface-finish/note handlers only in its copied archive-local registry.

The `--detail-data-helpers` audit completes 906 helper pairs and 1812 bodies without errors. Pair spans total 712677 bytes, of which 580541 retain conservative unknown-byte accounting. Pair sizes are 729, 740, 777 or 793 bytes. All observed note maps are empty; populated map framing has native and synthetic evidence only. Each helper contains two complete character formats, giving 3624 font records: Invalid FontFace (3276) and Segoe UI (348). These are stored font strings, not a claim about native font substitution. The report retains source hashes, versions, exact pair/member ends, map sizes, both font names per helper and per-body unknown-byte counts.

This continuation ends after note data, before direct moWeldDataHelper_c at detail-definition +730. The surrounding detail-definition object, remaining configuration data, assembly/drawing archives and installed project payloads remain incomplete. Audit-scoped helper parsing does not establish whole-document bit coverage or alter its aggregate metric.

### 11.107 Direct weld and geometric-tolerance data helpers

*Status: Native constructor/serializer evidence; 906 complete pairs/1812 helpers/1812 frames; populated tolerance/datum/indicator branches have native and synthetic evidence · Evidence: [C-122]*

The next direct moDetailDefs_c members are moWeldDataHelper_c at +730 and moGTolDataHelper_c at +9A0. Primary RTTI tables select serializers 0x3CB0D590 and 0x3CB09A60. Constructors 0x3CA7F480 and 0x3CA7E780 establish the embedded fonts, nested surface-finish/annotation helpers and allocated dialog frames. The scoped reader supports ordinary external-XML archives and v > 0x27CB; direct bodies do not add MFC slots, while explicitly tagged tolerance pointers use the existing map.

The weld helper emits the following exact order:

| Order | Stored type | Native offsets |
|---|---|---|
| 1 | discarded u8; CString[16] | 08; 10/20/30/40/50/60/70/80, then 18/28/38/48/58/68/78/88 |
| 2 | CString; u8[4]; CString; u8[2] | 90; 98/9C/A0/A4; A8; B0/B4 |
| 3 | i32[2]; u8[3]; i32[2]; u8[3] | B8/C0; C8/D0/E0; BC/C4; CC/D4/E4 |
| 4 | i32; u8[2]; CString[5]; u8[7] | D8; DC/E8; 218/220/228/230/238; 240/244/248/24C/250/254/258 |
| 5 | direct character format; u8; direct character format; u8; direct character format | F0; 150; 158; 154; 1B8 |
| 6 | two direct moSFDataHelper_c bodies | 260/268; use 11.106 |

The first stored byte is read and discarded by native loading; the reader retains it. The 16 strings are eight two-element arrays emitted by row, not ascending member address. All gates for the nested weld fonts and surface-finish bodies precede the modern minimum. Raw byte values, including noncanonical boolean values, SHALL be preserved.

The geometric-tolerance helper begins with i32 frame_count and that many direct moGTolDlgDataFrame_c bodies. It then emits i32 +30/+34; CString +38; u8 +40/+4C/+50/+54; i32 +58; u8 +68; CString +70/+78. For v >= 0x3751 it additionally emits u8 +80, CString +88/+90, u8 +98 and CString +60. Next are u8 +9C/+AC/+110/+114, direct utCharFormat_c at +B0, direct moAnnotationDataHelper_c at +118 (11.106), and u8 +A0/+44/+48/+A4/+A8. A final u8 +1E0 is present for v > 0x3714. Other gates precede this modern minimum. Native older-frame migration/default changes consume no extra bytes.

moGTolDlgDataFrame_c serializer 0x3CB09FA0 emits CString +08; u8 +28; CString +30/+38; u8 +40; CString +48/+50; six direct su_CStringArray bodies at +58/+80/+A8/+D0/+F8/+120; and i32 +20/+24. Each array uses ReadCount and preserves duplicate strings. For v >= 0x3715, it then emits u16 +148 and two tagged moGTolDlgDataTol_c pointers at +10/+18. Nonzero +148 adds three direct datum records at +150/+1C8/+240. For v > 0x3718, i32 indicator_count follows, then that many direct indicators, each i32 kind and two CStrings. The u16 remains a raw stored value, not normalized presence. Negative or impossible signed collection counts SHALL be rejected.

A tagged moGTolDlgDataTol_c has CString +08; i32 +10/+14/+18; i32 +1C presence; and, when nonzero, a direct moGtolItemTol2022_c body at +20. Constructor helper 0x3CA7EF30 proves this nested type. Nonzero signed presence includes negative values. The 2022 tolerance body stores:

- i32 value_count, then records of i32 key, i32 subkey if v >= 0x373F, f64 value, and CString text if v >= 0x334B. Preserve duplicate key/subkey records; native maps may replace them.
- Eleven i32 values: +18 is serialized twice consecutively, followed by +1C/+20/+24/+28/+2C/+30/+34/+38/+3C. Readers SHALL retain both +18 occurrences even though native loading overwrites the first.
- For v < 0x373F, one four-i32 legacy modifier record; otherwise i32 modifier_count and that many four-i32 records.
- i32 +58/+5C/+60/+64/+68; an extra discarded i32 if v < 0x3724; i32 +6C/+70; then i32 +74/+78/+7C if v > 0x3B10.

Each direct datum begins with i32 kind and u32 +40 if v > 0x373C. Kind zero adds an i32 count and that many datum items; other kinds add one datum atom. Each datum item begins with i32 kind: nonzero adds one atom, zero adds an atom vector. A vector stores u32 flags if v > 0x373C, i32 count, then atoms. An atom is CString, u32 flags, f64 value, and CString display text if v >= 0x334B. In versions 0x3B0F through 0x3B19 an atom additionally stores three discarded f64/CString pairs; readers SHALL retain those bytes. For v > 0x3B19 the outer datum instead ends with three f64/CString pairs at +48/+50, +58/+60 and +68/+70. Unknown kinds in these datum selectors follow the native nonzero branch, not a guessed tag. All records preserve order and raw values.

The scoped registry from 11.106 now also supports newly declared geometric-tolerance helpers and tolerance objects in note maps. Unsupported unrelated classes still stop. This does not replace tagged pointer identity with direct serialization.

The `--weld-gtol-helpers` audit completes 906 pairs and 1812 top-level helpers without errors, including two dialog frames per geometric-tolerance helper (1812 frames). Pair spans total 1435025 bytes, with 1135133 unknown bytes; sizes range from 1432 to 1624. Every observed frame has absent datum records, zero indicators and no present tolerance pointers. Populated branches therefore have native/synthetic evidence only. The report retains exact pair/member ends, versions, source hashes, unknown-byte totals and per-frame presence/count/class summaries. Native fixture tests consume distinct version/size spans through their exact recorded ends.

The next direct member is moCenterMarkSymDataHelper_c at detail-definition +B88. The surrounding detail-definition object, remaining configurations, assembly/drawing archives and installed project payloads remain incomplete. This audit-scoped continuation does not establish whole-document bit coverage or alter its aggregate metric.

### 11.108 Center-mark, datum-feature, datum-target and SWIFT helpers

*Status: Native constructor/serializer evidence; 906 complete four-body groups/3624 bodies; SWIFT load/store sequences agree · Evidence: [C-123]*

The next four direct moDetailDefs_c members are moCenterMarkSymDataHelper_c at +B88, moDatumFeatureDataHelper_c at +CD8, moDatumTargetDataHelper_c at +DE0 and moSwiftGtsOptions_c at +F68. Primary RTTI tables select serializers 0x3CB03C80, 0x3CB041B0, 0x3CB04340 and 0x3CAE93A0. The scoped reader supports ordinary external-XML archives with v > 0x27CB. These direct records allocate no MFC class/object entries.

The center-mark body is u8 +08; f64 +10; u8 +18; f64 +20; u8 +28/+30/+34/+38/+3C/+2C; direct moAnnotationDataHelper_c at +80 (11.106); u32 +148; u8 +40/+44/+48/+4C; f64 +78. Constructor 0x3CA7DCB0 establishes the shared annotation helper. All field gates precede the supported minimum. Retain the non-memory byte order, especially +2C after +3C, and preserve the raw u32 and original byte values.

The datum-feature body is u8 +08/+0C/+10; i32 +20/+24; CString +18/+28; i32 +30/+34/+38; then direct moAnnotationDataHelper_c at +40. Constructor 0x3CA7DD50 establishes that embedded type. All native field gates precede the supported minimum.

The datum-target body is CString +18/+20/+28/+08/+10; i32 +30/+40; f64 +50/+58; i32 +44; u8 +34/+3C/+38/+48/+4C; i32 +60; f64 +68; i32 +70; u8 +74/+90/+B0; direct mgVector_c at +78; then direct moAnnotationDataHelper_c at +C0. Constructor 0x3CA7DDB0 establishes the vector and shared helper. Its +60-through-vector gate is v > 0x27C7, already satisfied by the scoped minimum. The vector occupies three unaligned f64 values. Native normalization/default assignments do not replace these stored values.

SWIFT GTS options are a direct ordered scalar sequence, without an outer length, count, tag or presence field. The complete order, type and native member of every entry is given in [swift-options-layout.md](swift-options-layout.md), generated from native evidence by `swift_field_order.py`. Load and store sequences were extracted separately from the retained serializer and agree on all 159 entries. The latest layout contains 44 f64 and 115 i32 values, totaling 812 bytes. Its first 156 entries occupy 800 bytes in every supported version. The final entries are i32 +0C if v > 0x3739, i32 +3F8 if v >= 0x3AFF, and i32 +418 if v > 0x46B5. Omitted entries add no padding.

Native member +220 appears three times in the common sequence. Readers SHALL retain all three values in wire order rather than collapse them into a single member-keyed map; native loading overwrites the earlier values. Other native gates in this function precede the supported minimum. The independent field-order checker resolves the function's explicit pmVar1=this+220 alias and the local-i32 assignments before comparing the two sequences, and rejects unresolved operands or disagreement with the reader table. The member offsets establish storage locations and types, not semantic option names. Every SWIFT scalar remains unknown-byte-accounted until accessor/consumer evidence establishes its meaning.

The `--center-helpers` audit completes 906 groups and 3624 bodies without errors. Group spans total 1301432 bytes, with 1210352 unknown bytes. Group sizes are 1372, 1396, 1440 or 1444 bytes. SWIFT occupies 800 bytes in one older configuration, 808 in ten configurations and 812 in 895 configurations. Exact group/member spans, source hashes, versions, unknown-byte counts and SWIFT scalar counts are retained in the report. Native fixture tests consume distinct version/size groups through their exact recorded ends; synthetic tests use distinct values for all three +220 occurrences and exercise each modern tail gate.

This continuation ends after SWIFT options, before direct utLineWidthPrintData_c at detail-definition +B0. The surrounding detail-definition object, remaining configuration data, assembly/drawing archives and installed project payloads remain incomplete. These audit-scoped bodies do not establish whole-document bit coverage or alter its aggregate metric.

### 11.109 Print line widths, detail-definition tail and complete composition

*Status: Native serializer/accessor evidence; 906 complete supported modern detail-definition bodies; field meanings remain partial · Evidence: [C-124]*

After SWIFT options, moDetailDefs_c directly serializes utLineWidthPrintData_c through member +B0. Its primary RTTI table selects serializer 0x3CAF7BC0: exactly eight little-endian f32 values from native offsets +08 through +24, totaling 32 bytes without a tag, count or presence flag. Accessors getLineWeight (0x3CAF7C50) and setLineWeight (0x3CAF7ED0) establish an eight-slot array indexed by swLineWeights_e values 0 through 7. Readers SHALL retain the stored floats, including nonfinite or negative values, rather than apply unstated range constraints. These 32 bytes have established line-weight meaning and are not counted unknown. Numeric enum labels beyond their native index are not inferred.

The preceding SWIFT style update 0x3CA88440 is a direct assignment to member +3DC. It consumes no archive bytes. Detail-definition constructor 0x3CA7DEA0 independently establishes the allocated print-width type and the two character formats appearing in the tail below. This avoids treating the decompiler's reused local pointer types as a wire-type authority.

The remaining modern moDetailDefs_c tail, after the print widths, is:

| Order | Stored type | Native offsets in wire order |
|---|---|---|
| 1 | f64; i32[2]; u8[2] | 2BD0; 2BF4/2BF8; 2BFC/2C00 |
| 2 | i32[6]; f64 | 2C0C/2C10/50/2C04/2C08/80; 88 |
| 3 | f64[5]; i32[9] | 2C18/2C20/2C28/2C30/2C38; 2C40/2C44/2C48/2C4C/2C50/2C54/2C58/2C5C/98 |
| 4 | two direct utCharFormat_c bodies | A0/A8; use 11.101 |
| 5 | i32[4]; CString[4]; i32[4]; CString | 2C64/2C68/19C/1BC; 1C0/1C8/1D0/1D8; 1E0/1E4/1E8/1F8; 1F0 |
| 6 | i32, if v > 0x27D1 | 1FC |
| 7 | i32[2], if v > 0x2BB5 | 15C/160 |
| 8 | i32; CString[3], if v > 0x335E | 218; 200/208/210 |
| 9 | i32[2], if v > 0x373F | 220/224 |
| 10 | i32, if v >= 0x3B15 | 2C6C |
| 11 | i32, if v > 0x41A1 | 21C |
| 12 | i32, if v > 0x42E5 | 1B8 |

All other tail gates in serializer 0x3CB04650 precede the scoped minimum v > 0x27CB. No padding is added for absent fields. Readers SHALL preserve the two u8 values and raw unnamed scalar/string fields without native normalization; their meanings retain unknown-byte accounting. The complete supported ordinary body is now composed in this order: scalar prefix and first character format (11.101); dimension definitions (11.102); annotation definitions (11.103); table definitions (11.104); label definitions (11.105); surface-finish/note helpers (11.106); weld/geometric-tolerance helpers (11.107); center/datum/SWIFT helpers (11.108); then print widths and this tail. Older layouts and other archive contexts still stop explicitly.

The composed `detaildefinitions.read` consumes the full supported moDetailDefs_c wire body, and is installed only in the complete-stage reader's copied archive registry. The model's +AD8 instance is serialized directly and does not itself allocate an object-map entry. Nested tagged unit/helper/tolerance references retain their existing identity rules; direct parsing SHALL preserve the archive map across stages.

The `--complete-detail-definitions` audit completes all 906 supported modern part detail-definition bodies without errors. Their total extent is 16073264 bytes, of which 12773362 remain unknown by field accounting. Individual extents range from 16356 to 17904 bytes. Print-width-and-tail spans total 903622 bytes: 998 bytes in 905 cases and 432 bytes in the older remaining case. Both tail fonts are Century Gothic in every sample. Two distinct eight-float line-weight arrays occur. Evidence retains source hashes, versions, complete-object/tail ends, unknown-byte totals, widths, tail fonts, resolved-section ends and total stream sizes.

The composed reader is separately tested by starting from each selected fixture's original lightweight object map and reparsing complete detail-definition bodies to their exact staged ends. This verifies composition rather than relying only on independent stage success. Parsing still stops before the model's next direct manager at +9C0. Across these 906 configuration streams, 4171417 bytes remain after the detail-definition end. Complete object framing does not establish semantic bit coverage: unnamed fields, the remaining configuration bytes, legacy versions, assembly/drawing archives and installed project payloads remain within the unfinished scope. The aggregate whole-document metric remains unchanged while configuration decoding is audit-scoped.
