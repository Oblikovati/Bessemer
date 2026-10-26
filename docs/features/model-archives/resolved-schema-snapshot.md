# Resolved manager and defaults schema snapshot

Source: `../inventor-interface/docs/solidworks-file-format/specification.md`

Edition 0.306 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.25 Resolved-section handoff, atoms and ghost bodies

*Status: Verified scoped modern prefix; remaining resolved data and optional geometry incomplete · Evidence: [C-40], F.28*

Immediately after the exact lightweight end of 11.24, the part's resolved-stage scope reads resolved_size i32. It counts bytes after this word. Negative or out-of-stream bounds SHALL fail. In the ordinary standalone part context, the scope restores the object/class map saved at the lightweight end; this is not a new MFC archive starting at map index zero. The schema reader retains the same slot sequence and object identities without re-registering the part or header objects.

The resolved model then calls the filed-concept/concept resolved functions. In the implemented ordinary load context, a direct moNodeList_c feature list follows (ReadCount, object pointers, trailing i32 as in 11.11). All installed modern configurations have an empty inline list; the separately stored ResolvedFeatures stream remains a distinct payload. The next two direct calls serialize the model's moAtomManager_c (+0xD80) and moGhostBodyIdManager_c (+0xD88). Neither manager has an enclosing object declaration or pointer tag. All consumed prefix bytes SHALL stay within the declared resolved end. The rest of the model and part resolved stages is not yet implemented.

#### 11.25.1 Atom manager and atom objects

*Status: Verified modern ordinary archive layout; embedded geometry and member meanings partial · Evidence: [C-40], F.28*

The direct atom-manager body is unknown_260 i32, atom_count i32, atom_count moAtom_c object pointers, then unknown_a8 u32. The count SHALL be nonnegative and bounded by available bytes. Null atom pointers SHALL fail: the native loader immediately assigns the manager and indexes each atom. Runtime hash/tree insertion adds no wire fields.

A moAtom_c body starts with the concept's modern lightweight fields: node fields (11.11), has_merged_list i32, optional direct merged node list, and a moVisualProperties_c object pointer. The concept resolved virtual then reads a direct feature node list. There is no independent stage-size word around these atom virtual calls. In the scoped modern path, the following fields occur:

| Order | Fields | Type | Status |
|---|---|---|---|
| 1 | atom_id | i32 | V |
| 2 | unknown_13c | u32 | ? |
| 3 | unknown_128, unknown_124 | i32 × 2 | ? |
| 4 | unknown_150 | u32 | ? |
| 5 | body_mode | i32 | V |
| 6 | body data | conditional, below | V / ? |
| 7 | unknown_168, unknown_184 | i32 × 2 | ? |
| 8 | mold_body_type | i32 | V / ?; enum domain open |
| 9 | unknown_178, unknown_180, unknown_1a8 | i32 × 3 | ? |
| 10 | unknown_194 | i32 | ? |

body_mode 1 introduces an embedded kernel body and direct attribute list and is not implemented. Mode 2 introduces one i32 body_reference. Mode 0 adds no body bytes. These stored values remain unchanged even where the native loader clamps unknown_13c or normalizes mold_body_type. Field 9 is present when hasCondition(0) or hasCondition(0x400000) is true; the implemented ordinary archive has zero condition flags, making hasCondition(0) true (11.14/C-33). Other archive contexts are not inferred from the bytes. The native manager indexes atom_id using member +0x138. The mold enum role follows fgMoldAttrib_c::isValidBodyType; its full value set remains open.

#### 11.25.2 Ghost-body manager and direct records

*Status: Verified modern scoped framing; packed flags and record meanings partial · Evidence: [C-40], F.28*

The direct ghost-body manager begins with unknown_80 i32 and packed_count u32. Its **low 28 bits** are the record count; **high 4 bits** are flags. The native loader extracts the high nibble through an arithmetic signed shift and removes it from the count; the schema preserves the entire raw word and exposes the unsigned nibble separately. Thus the count is packed_count & 0x0FFFFFFF. It SHALL be bounded by the available bytes before direct records are read. The writer sets nibble bit 0 and clears bit 1 when no bodies are available from the model's permanent partition; it clears bit 0 and sets bit 1 when bodies are available. Nibble bits 2 and 3 remain open. Unknown-byte accounting conservatively includes the whole packed word because these flag meanings are incomplete.

Each record is a **direct moGhostBodyObject_c body**, without an MFC tag or object-map insertion:

| Order | Fields | Type | Status |
|---|---|---|---|
| 1 | unknown_08, unknown_0c, unknown_10, unknown_c0 | i32 × 4 | ? |
| 2 | integer_count | i32 | V |
| 3 | unknown_a0 | i32[integer_count] | ? |
| 4 | has_geometric_data | i32 | V |
| 5 | geometric_data | moGhostGeometricData_c object pointer | V / ?; iff field 4 != 0; nonnull payload unimplemented |
| 6 | point_count | i32 | V |
| 7 | points | direct Point[point_count] | V / ?; three f64 each |

Both inner counts SHALL be nonnegative and bounded by available bytes. The native loader deduplicates integer values in a runtime unsigned array and creates runtime point objects; the schema retains every stored integer and point in wire order. No geometric-data or point payload occurs in the installed cohort. The next field after this manager is the model's relation-manager object pointer, outside the current prefix decoder. `through_resolved_concept`, `through_atom_manager` and `through_ghost_manager` select these stopping points in modelprefix.read. No whole-stream coverage is awarded.

### 11.26 Relation manager and equation records

*Status: Verified scoped layouts; dimension-handle dependencies and meanings partial · Evidence: [C-41], F.29*

After the ghost manager in 11.25, the modern ordinary part model reads a moRelMgr_c object pointer. The inherited archive map is retained, including cyclic relation/side references. This clause covers v > 0x27CB and the same ordinary context; it does not specify old relation conversions, database archives or alternate owners. The manager body is:

| Order | Fields | Type | Condition | Status |
|---|---|---|---|---|
| 1 | relations | direct suObList | | V / ? |
| 2 | cached_dimension_values | ReadCount then (name CString, value f64) pairs | | V; units open |
| 3 | unknown_170 | CString | | ? |
| 4 | unknown_178 | i32 | | ? |
| 5 | time | su_CTime, u32 archive value | | V / ?; time interpretation open |
| 6 | unknown_150, unknown_f4 | i32 × 2 | | ? |
| 7 | objects_a0 | direct suObList | | V / ? |
| 8 | unknown_1b0 | i32 | v > 0x336A | ? |
| 9 | feature_parameter_count | i32 | v > 0x336A | V |
| 10 | feature_parameters | moFeatureParameter_c object pointers[count] | v > 0x336A | V / ?; nonnull payload unimplemented |

suObList uses ReadCount and object records, without an enclosing object tag. The cache's ReadCount uses the same u16 / 0xFFFF-plus-u32 primitive. Its values are direct doubles, not object pointers. Counts SHALL be nonnegative and bounded by available bytes. The schema preserves stored map/list order and duplicate cache names rather than reproducing native hash-map insertion. The manager's runtime lists at +0x58 and +0xA0 and cache at +0x10 are independently established by its constructor. The save-only database/deferred work following serialization adds no bytes on this load path.

#### 11.26.1 Relation and equation side

*Status: Verified modern layouts and shared references; text/flag semantics partial · Evidence: [C-41], F.29*

moRelation_c stores, in order: unknown_10 CString, unknown_18 **u16**, unknown_48 i32, left moRelEquationSide_c pointer, right moRelEquationSide_c pointer, unknown_74 i32, relation_id i32, unknown_38 CString, then unknown_88, unknown_6c and unknown_8c i32. The stored word is two bytes even though the runtime member is wider. The ID role is established by the manager indexing/assigning member +0x08. Strings and scalar values are preserved, including text the native loader may trim or normalize.

moRelEquationSide_c stores a direct suObList of elements, unknown_50 CString, unknown_58 i32, is_left i32 and a parent moRelation_c pointer. Native relation loading later assigns its left side is_left = 1, right side is_left = 0 and both parent pointers. Those runtime assignments do not erase the stored values. A side may refer back to the relation currently being read; this is an ordinary object reference to an already allocated map entry, not a recursive new-object declaration. The parser SHALL preserve that identity and cycle.

#### 11.26.2 Equation elements

*Status: Verified selected layouts; dimension/global-variable references partial · Evidence: [C-41], F.29*

The common moRelElement_c body is unknown_08 CString followed by unknown_10, unknown_14, unknown_18 and unknown_1c, all i32. moRelComment_c, moRelOperator_c and moRelValue_c invoke this same body without additional wire fields. Their class identities distinguish expression roles; numeric/operator domains and the scalar/text member meanings remain open.

moRelDimension_c stores a direct suObList at +0x30, a moCompRef_c-derived object pointer, a moDimRefWrapper_c pointer, then the common element body. Its constructor proves the embedded list type. moRelGlobalVar_c instead stores a moCompXFormWrapper_c pointer, a moGlobalVarRefWrapper_c pointer, then the common element body. Unknown referenced classes stop explicitly. Full dimension-handle payloads remain the next dependency in the installed cohort.

#### 11.26.3 Relation reference wrappers

*Status: Verified scoped framing; wrapper member meanings and dimension handles partial · Evidence: [C-41], F.29*

These wrappers share the common body of 11.16.1, retaining its ordinary-context restrictions and optional transforms:

| Class | Additional fields and order | Status |
|---|---|---|
| moDimRefWrapper_c | unknown_a8 f64 **before** the common wrapper body | ? |
| moGlobalVarRefWrapper_c | common wrapper body, then unknown_a8 CString | ? |
| moCompXFormWrapper_c | common wrapper body, then two direct mgXform_c bodies (+0xA8, +0x130) | V / ? |

The leading double in the dimension wrapper SHALL NOT be moved after its common body. moCompGlobalVar_c is an additional verified common-wrapper reference class: its body is moCompRef_c (11.17.2) followed by unknown_e8 i32. It introduces no sketch-object field. Clause 11.27 adds scoped dimension-handle bodies and establishes their common-wrapper context; their underlying dimension/sketch payloads remain incomplete.

`modelprefix.read(..., through_relation_manager=True)` stops after the manager object when all referenced classes are supported. It does not skip failing relations or claim a completed prefix when a dimension handle cannot be decoded. The subsequent model arrays and remaining resolved content are still unread.

### 11.101 Model continuation: detail-definition prefix and character format

*Status: Native serializers and 906 native configuration-prefix parses; nested dimension/annotation definitions remain partial · Evidence: [C-116]*

After the tagged relation manager, model resolved serializer 0x4C2411C0 directly serializes member +B00. Constructor 0x4C01EEE0 establishes this member as an embedded suObArray: ReadCount followed by that many tagged objects, without an outer pointer-presence flag or object-map entry for the array. All 906 observed arrays are empty. The old-light conversion and model-state conversion branches between this array and the following detail definitions consume no bytes in the supported modern range (v > 0x27CB). Member +AD8 is a constructed moDetailDefs_c, serialized directly through its virtual slot 5, without an MFC object tag.

moDetailDefs_c resides in sldmfcu.dll, not sldmodu.dll. Primary RTTI table 0x3CF17440 selects serializer 0x3CB04650. Its supported scalar prefix begins with the following fields in exact wire order. Offset labels identify native members, not inferred meanings; all raw fields retain unknown-byte accounting.

| Order | Type | Native offsets in wire order |
|---|---|---|
| 1 | i32; CString; f64; u8; u32; i32; f64 | 10; 13A0; 28; 30; 20; 24; 08 |
| 2 | u8[5]; i32 | 38/14/18/1C/34; 1388 |
| 3 | u32[3]; f64[3]; u32[3]; f64[3] | EC/F0/F4; F8/100/108; 110/114/118; 120/128/130 |
| 4 | i32; f64[2]; i32[9] | 138; 140/148; E8/150/154/158/164/168/16C/170/174 |
| 5 | u8; i32[4]; u8[2]; i16[2]; i32[2] | 3C; 138C/1390/1394/1398; 40/44; E0/E2; 4C/48 |
| 6 | f64; i32; CString; i32; CString; i32[6] | 2BC0; 2BC8; 2BD8; 2BE0; 2BE8; 2BF0/178/17C/1A8/1AC/1B0 |
| 7 | i32, if v > 0x2BB3; i32[6] | 1B4; 180/184/188/18C/190/194 |
| 8 | i32 count, then count CStrings | Exclusion strings at +58; preserve order and duplicates |
| 9 | i32[3] | 198/1A0/1A4 |
| 10 | two fixed integer grids | Members beginning 228 and 288; see below |
| 11 | direct su_CStringArray, if v > 0x3B0F | B8; ReadCount and that many CStrings |

The supported range exceeds every other gate in this prefix. Each integer grid uses native row stride 0x20. For v <= 0x374E, the first grid stores two rows of five i32 values and the second stores three rows of five, at base + row*0x20 + column*4. For v > 0x374E, each stores three rows of eight i32 values. No count or row-stride padding is serialized. The +58 exclusion count is a signed i32, unlike the +B8 array's ReadCount. Readers SHALL reject negative or impossible exclusion counts and retain raw short/flag bits rather than normalize them. The older second-grid row count corrects Edition 0.101, as established by the continuation audit in 11.102.

The next field is the direct utCharFormat_c at detail-definition member +90. Its primary RTTI table 0x3CF13B08 selects serializer 0x3CA6C7C0. In this modern scope its complete wire body is CString font_name; f64 +10; i32 +18; f64 +20; u32 +30; f64 +28; f64 width_factor at +38; f64 +40/+48/+50; and u32 +58 if v >= 0x3EED. The seven doubles and two initial 32-bit words occupy 64 bytes after the CString; the versioned word adds four. The font name is established by native font-availability checking; unnamed numeric fields remain unknown. Native font replacement, empty-name defaults, finite-width normalization and registry-based width changes consume no additional archive bytes. Readers SHALL preserve the original stored string and floating-point values, including nonfinite values, independently of these runtime choices. No MFC object slot is allocated for this direct character-format body.

The corrected `--detail-prefix` audit composes all preceding supported readers and parses through this character format in all 906 modern part configurations, without errors. Detail-prefix spans total 716014 bytes and range from 561 to 889 bytes. Observed font strings are Century Gothic (901), TechnicBold (2), Arial (2) and Arial Narrow (1). The exclusion lists have 9, 10 or 20 entries; +B8 has five entries in 905 cases and is absent in the older remaining case. The evidence report retains stream hashes, version, exact start/end, character-format boundary, resolved-section end and stream size. A successful prefix SHALL NOT be reported as a complete moDetailDefs_c object. The dimension-definition continuation is specified in 11.102; further annotation, table, label and other nested definitions remain undecoded. Configuration streams and whole-document coverage remain incomplete.

### 11.102 Direct dimension-definition groups

*Status: Native constructor/serializer evidence; 906 complete groups and 9060 definition bodies; surrounding detail definitions remain partial · Evidence: [C-117]*

The direct member following the character format in 11.101 is moDimDefs_c at detail-definition offset +13B0. Constructor 0x3CA7DEA0 establishes its type; the preceding decompiler cast to moDatumDefs_c is not a type authority. moDimDefs_c constructor 0x3CA8B9A0 establishes its ten embedded definition types. Primary RTTI table 0x3CF18418 selects serializer 0x3CB087F0. In ordinary external-XML archives with zero special conditions and v > 0x27CB, the group's scalar prefix is:

| Order | Type | Native offsets in wire order |
|---|---|---|
| 1 | i32; f64[11] | 68; 08/10/18/20/28/30/38/40/48/50/58 |
| 2 | i32[5]; f64[2]; i32[2]; f64[3] | 60/64/6C/70/74; 78/80; 88/8C; 90/98/A0 |
| 3 | u32; i32; f64; i32[4] | D8; A8; B0; B8/BC/C0/C4 |
| 4 | i32[2], if v > 0x375D | D0/D4 |
| 5 | i32, if v >= 0x3EEF | CC |

The scalar prefix occupies 192, 200 or 204 bytes according to these gates. Retain raw words and doubles before native trailing-zero or style normalization. The ten direct definition bodies follow in this wire order, which differs from their memory order:

| Order | Member | Class / kind | Suffix after shared base in this modern scope |
|---|---|---|---|
| 1 | E0 | moLinearDimDef_c | u32 +108, i32 +100, then i32 +104 if v > 0x3EF8 |
| 2 | 1F0 | moAngleDimDef_c | u32 +100 |
| 3 | 2F8 | moArcLengthDimDef_c | None; native function inlines the complete shared base |
| 4 | 3F8 | moRadialDimDef_c | u32 +100, i32 +104 |
| 5 | 500 | moHoleCalloutDimDef_c | u32 +100 |
| 6 | 608 | moOrdinateDimDef_c | i32 +100, u32 +104, f64 +108 |
| 7 | 830 | moChamferDimDef_c | i32 +100/+104/+108/+10C, f64 +118/+120, i32 +110/+128 |
| 8 | 960 | moDiameterDimDef_c | i32 +100, u32 +104 |
| 9 | 718 | moAngOrdinateDimDef_c | i32 +100, u32 +104, i32 +108/+110/+114 |
| 10 | A68 | moBaseDimDef_c | None |

moBaseDimDef_c serializer 0x3CB035C0 emits the following shared base. Constructor 0x3CA8B610 establishes the allocated direct unit, character-format and line-format types used below.

| Order | Stored type | Native offsets / meaning |
|---|---|---|
| 1 | i32[3]; f64[2]; i32; f64[2]; i32[2] | 08/0C/10; 18/20; 28; 30/38; 40/44 |
| 2 | CString[2]; f64[2]; i32[8] | 48/50; 58/60; 68/6C/70/74/80/84/88/8C |
| 3 | i16; i32[2]; u32 | 90; 94/98; E0 |
| 4 | direct moLengthUserUnits_c, direct moAngleUserUnits_c | F0/F8; use their complete units grammars |
| 5 | direct utCharFormat_c | E8; use 11.101 |
| 6 | two direct uiLFConfig_c bodies | D0/D8; existing CString/line-width/end-cap grammar |
| 7 | i32[3] | C0/78/9C |

Archive condition 0x40000000 inserts a tagged moLayerData_c pointer after +E0 when the signed short +90 is not -1. That condition is outside this ordinary zero-condition reader; its absence is not inferred from a plausible next tag. All other shared-base gates precede the supported minimum. Native short and flag normalization, chamfer precision clamping, linear-style conversion, hole-callout default changes and ordinate arrow-height replacement do not add bytes. Readers SHALL retain stored values, including negative chamfer values and all short bits. Direct bodies do not allocate their own MFC class/object slots, but the length-units grammar can contain a tagged nested unit pointer that uses the existing archive map. Readers SHALL preserve that map across stages.

The `--dimension-definitions` audit completes 906 groups and 9060 constituent definition bodies without errors. Group spans total 4674694 bytes; definition bodies total 4489882 bytes, of which 3596524 remain unknown by conservative field accounting. Group sizes range from 5002 to 5230 bytes. Definition font names are Century Gothic (8889), Arial (126), Tahoma (18), TechnicBold (18) and Arial Narrow (9). Exact group/member spans, unknown-byte totals, source stream hashes and versions are retained in the report.

The older configuration exposed the incorrect second-grid row count in 11.101: consuming the native three-row second grid restores alignment through all ten definitions and changes that source's formerly empty apparent font to Century Gothic. The corrected prefix grows by 51 bytes; no bytes are skipped to obtain alignment. This continuation ends before the next direct moAnnotationDefs_c at detail-definition +1F18. The surrounding moDetailDefs_c, configuration stream, assembly/drawing archives and project payloads remain incomplete. These audit-scoped results do not change the aggregate whole-document coverage metric.
