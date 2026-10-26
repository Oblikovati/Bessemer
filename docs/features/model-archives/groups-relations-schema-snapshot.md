# Folder, parameter, relation and bridge schema snapshot

Source: `../inventor-interface/docs/solidworks-file-format/specification.md`

Edition 0.330 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.15 Modern feature and folder records

*Status: Partial meanings; verified ordinary archive layout; conditional archive modes open · Evidence: [C-26], [C-29], F.17*

This clause covers v > 0x27BC in the ordinary model-reading context with external XML extraction, no clipboard/import pointer pair, no archive-noise byte and no suppressed resolved-node-list stage. These conditions come from archive/runtime state, not a per-record flag. Other modes SHALL NOT be guessed from nearby byte values. The implementation requires an explicit `ordinary_external_xml` context and grants no whole-stream coverage.

#### 11.15.1 Common feature body

The archive-condition predicate is documented by C-33: with no helper it returns zero. For mask zero it tests whether the stored flags are zero. For any nonzero mask it returns flags & mask, so a truth test means **any masked bit**, not all bits. Composite masks in the serializers SHALL be interpreted accordingly.

*Status: Verified scoped layout; most member meanings open · Evidence: [C-26], [C-29], F.17*

The common feature body begins with the node lightweight body (11.11) and the concept lightweight continuation (11.13). When the native class has a child-feature-list container, a direct resolved feature node list follows (ReadCount, objects, final i32). Classes whose constructors leave that container null omit the whole direct list, including its count and final word; attributes and attribute definitions are proven examples (11.140). This is class/runtime state, not a presence byte inferred from the stream. The following fields are then stored in order:

| Order | Fields | Types | Condition | Status |
|---|---|---|---|---|
| 1 | `unknown_290`, `unknown_284`, `unknown_294` | i32, i32, u32 | | ? |
| 2 | `has_unknown_1e0` | u8 | | ? |
| 3 | `unknown_1e0` | CString | has_unknown_1e0 != 0 | ? |
| 4 | `unknown_2a0`, `unknown_2b0`, `unknown_2b4`, `unknown_2a8` | i32 × 4 | | ? |
| 5 | `unknown_1d8`, `unknown_2ac` | f64, i32 | | ? |
| 6 | `unknown_250` | object | | ? |
| 7 | `unknown_1f0`, `unknown_1ec` | CString, u8 | | ? |
| 8 | entity visual properties | direct body | moEntVisProp_c, shared with 13.6: two u16, u8, u32, layer u16, line-width body. | V / ? |
| 9 | `unknown_2e4`, `unknown_a0`, `unknown_2e8` | i32, i32, object | | ? |
| 10 | `unknown_200`, `unknown_2f8`, `unknown_2f9` | u32, u8, u8 | | ? |
| 11 | `unknown_310`, `unknown_hidden`, `unknown_304` | i32 × 3 | | ? |
| 12 | `unknown_29c`, `unknown_298` | u32 × 2 | Stored high/low FILETIME words passed to setFeatTimeStampForConfigs; domain/uses remain open. | ? |
| 13 | `unknown_90`, `unknown_2a4`, `unknown_33c` | u16, i32, i32 | | ? |

The final XML stage can read a CString in certain inline-XML archive modes. With external extraction it exchanges tags, names and descriptions with the XML extractor instead of consuming that CString from this record. The scoped reader does not infer an inline value or an optional noise byte. Unknown object fields preserve graph references; their bytes are counted once as unknown, including any nested unknown bytes.

#### 11.15.2 Folder suffixes

*Status: Verified scoped layouts; member meanings open · Evidence: [C-29], F.17*

`moFolder_c` and `moCommentsFolder_c` append `unknown_358` and `unknown_368`, both i32, after 11.15.1. `moFtrFolder_c` and `moMateReferenceGroupFolder_c` then append `unknown_380` object, `unknown_374` i32, `unknown_38c` i32 and `unknown_394` i32. Their verified IsMateFolder virtual function returns zero, so the conditional member +0x370 is absent. Do not apply this suffix to every derived folder without checking that predicate.

#### 11.15.3 Position-group folder

*Status: Verified scoped layout; reference bodies and meanings partial · Evidence: [C-29], F.17*

`moPosGroupFolder_c` inherits the common feature body through `moModelFeature_c`, not through the folder suffix in 11.15.2. The model-feature stage appends `unknown_354` i32; its older migration helper consumes no bytes in this version range. The position-group suffix is:

| Order | Fields | Types | Condition | Status |
|---|---|---|---|---|
| 1 | `unknown_380` | CString | | ? |
| 2 | `direction_count` | u16 | | ? |
| 3 | `unknown_388`, `unknown_3a0` | u16[direction_count] × 2 | Two consecutive arrays, not interleaved pairs. | ? |
| 4 | references | ReadCount, object[count] | Direct suObArray body, no final node-list word. | V / ? |
| 5 | `unknown_3a8`, `unknown_3ac` | i32 × 2 | | ? |
| 6 | `unknown_3b0` … `unknown_3b3` | u8 × 4 | v >= 0x3EEB | ? |

The native loader has three slots per direction array; it reads min(direction_count, 3) words for each and fills unused runtime slots with zero. The implemented schema accepts counts through 3 and explicitly rejects larger values; noncanonical larger-count writer/loader behavior remains open. The raw flag bytes are preserved even though the native loader normalizes them to boolean values. The reference-array elements include the unresolved geometric-reference classes listed in F.17.

### 11.16 Geometric reference wrappers

*Status: Verified scoped component layouts; inherited component references and meanings partial · Evidence: [C-30], F.18*

This clause specifies v > 0x27BC in the ordinary external-XML model context of 11.15, with the helper's lightweight-only flag (+0x834) clear. The latter flag adds configuration data in other contexts; its absence is a runtime condition, not a record tag. `modelreferences.py` requires the explicit context. It rejects embedded geometry bodies and sketch-handle context rather than guessing their extent.

#### 11.16.1 Common reference wrapper

*Status: Verified scoped layout; member meanings partial · Evidence: [C-30], F.18*

`moRefWrapper_c` stores the following in order:

| Order | Fields | Types | Condition | Status |
|---|---|---|---|---|
| 1 | component reference | object | Usually a moCompRef_c-derived object. Sketch entity handles require another sketch pointer; that context is not implemented. | V / ? |
| 2 | `unknown_60` | u16 | | ? |
| 3 | component | object | Declared moComponent_c. | V |
| 4 | `unknown_64`, `unknown_08` | i32 × 2 | | ? |
| 5 | `has_transform_10` | u16 | | V |
| 6 | `transform_10` | mgXform_c body | has_transform_10 != 0 | ? |
| 7 | `has_transform_18` | u16 | | V |
| 8 | `transform_18` | mgXform_c body | has_transform_18 != 0 | ? |
| 9 | `unknown_0c`, `unknown_20`, `unknown_6c`, `unknown_70` | i32 × 4 | | ? |
| 10 | `unknown_74` | i32 | v > 0x2FAF | ? |
| 11 | `unknown_4c`, `unknown_68`, `unknown_34`, `unknown_30` | i32 × 4 | | ? |
| 12 | `unknown_90` | i32 | v >= 0x2B73 | ? |
| 13 | `unknown_38` | i32 | v >= 0x3B02 | ? |

The transform layout is shared with 13.4. Stored i32 values are preserved even where the native loader later truncates a value to a word, merges flags or replaces a runtime value. The current wrapper has 62 bytes when both pointers are null and both transforms are absent. Actual records may contain larger pointed-to objects and transforms. The reader permits the verified component-reference classes listed in 11.17 at field 1; unverified derived/sketch contexts remain incomplete.

#### 11.16.2 Surface, face and curve suffixes

*Status: Verified scoped layouts; body-bearing cases and member meanings partial · Evidence: [C-30], F.18*

A surface-reference base appends four f64 members `unknown_a8`, `unknown_b0`, `unknown_b8`, `unknown_c0` to 11.16.1. A curve-reference base instead appends only `unknown_a8` and `unknown_b0`, both f64.

`moSingleFaceRef_w` uses the surface base, followed by `has_embedded_body` **i32**. When nonzero a kernel body and its attribute list follow; selected compressed-body framing is implemented in 11.156. After that optional body/list, the suffix is `unknown_c8` u32 and `unknown_d8` i32.

`moGeneralCurveRef_w` uses the curve base, followed by `has_embedded_body` **u8**. Its nonzero branch uses the selected common kernel-body framing in 11.156. After that optional body, the suffix is `unknown_c8` u32, `unknown_cc` i32, then three object records: `unknown_d8` (contour chooser), `unknown_d0` (component sketch-entity handle), `unknown_e0` (contour chooser). The differing face/curve presence-field widths SHALL be preserved.

`moCircleRef_w` appends to the general-curve body: `unknown_f8` point, `unknown_110` vector, `unknown_128` f64, `unknown_130` i32, then the optional frame below. `moCylinderRef_w` appends to the surface base: `unknown_c8` point, `unknown_e0` vector, `unknown_f8` f64, then the same optional frame. A point or vector is three consecutive f64; analytic roles and field units remain open.

The optional frame is a u16 `frame_mode`. If nonzero, three f64 `frame_vector` follow. If the word interpreted as **signed i16** is greater than 1, another three f64 `frame_point` follow. Thus 0xFFFF includes the vector but does not include the point. The current writer emits only mode 0 or 1; the loader also handles the point-bearing form. The schema reader preserves the original word and numeric values.

### 11.24 Part-specific lightweight appearance records and end

*Status: Verified scoped modern lightweight framing; meanings and other archive contexts partial · Evidence: [C-39], F.27*

This clause continues 11.23 for ordinary standalone part archives, v > 0x27CB, without drawing or special attribute-list conditions. The part's virtual slot 0x16C0 returns zero. With archive condition 0x80 absent, the native load path omits the optional attribute-list/map block and reads these fields:

| Order | Field | Type | Status |
|---|---|---|---|
| 1 | hatch | object, declared gcXhatch_c | V |
| 2 | material | object, declared moMaterial_c | V |
| 3 | object_1580 | object, generic | V / ?; observed uoModelData_c |
| 4 | body appearance count | i32 | V |
| 5 | body appearance elements | direct elements[count] | V / ? |
| 6 | property count | i32 | V |
| 7 | property elements | direct property elements[count] | V / ? |
| 8 | has_weldment | i32 | V |
| 9 | unknown_160c, unknown_1610, unknown_1614 | i32 × 3 | ? |
| 10 | part_map_count | i32 | V; only v > 0x27D0 |
| 11 | part map entries | (unknown_key i32, unknown_value i32)[part_map_count] | ?; only v > 0x27D0 |

All counts SHALL be nonnegative and bounded by available bytes. Unknown scalar/map values are preserved. Fields 4–7 restore the direct moBodyVizProps_c body; they do not have an outer MFC object tag. A direct property element is unknown_10 i32 followed by the same direct visual-properties body as 11.19. A body appearance element is a direct property element, a moCompAtomBody_c-derived object pointer, unknown_20 i32 and unknown_24 i32. Native AddHead calls reverse runtime list order; the schema preserves wire order. The weldment field is derived from hasWeldment in the ordinary save path, with an existing stored value used in another runtime path; no boolean normalization is applied.

After field 11, or field 9 when the map is absent by version, the scoped part lightweight reader SHALL end exactly at the size-delimited section boundary from 11.12. The returned archive retains its object map for later data. This framing completion does not establish unknown member meanings, resolved-data layouts, special attribute contexts or complete document coverage.

#### 11.24.1 Hatch, material and model-data wrapper

*Status: Verified scoped layouts; field domains and scene payload incomplete · Evidence: [C-39], F.27*

The modern gcXhatch_c body is name CString, angle f64, scale f64, then six i32 fields in this order: override, solid, id, layer_id, command_layer_color, randomize_scale. These names are identified by the native serializer's key labels (including its spelling Overide). Angle units and complete value domains remain open. The loader substitutes a default name for empty strings and resets scales outside [0.00001, 10000] to 1; the schema preserves stored values instead.

moMaterial_c contains a moDensityParameter_c object pointer. In the compatibility interval 0x2B90 < v < 0x2BAB only, seven additional f64 fields follow at members +0x10 through +0x40. Their meanings remain unknown. Modern versions outside that interval have no such tail.

The observed uoModelData_c body is a uoScene object pointer followed by unknown_10 i32. All installed instances have a null scene; nonnull uoScene payloads are not implemented. The enclosing generic pointer is preserved as an object and is not replaced with the application runtime object created by SOLIDWORKS after loading.

#### 11.24.2 Density parameter

*Status: Verified modern ordinary-part body; units and parameter flags partial · Evidence: [C-39], F.27*

moDensityParameter_c uses the following body: node lightweight fields of 11.11, a second moNodeName_c pointer (parameter_name), value f64, has_alternate_value u8, then alternate_value f64 if the byte is nonzero. The second name pointer is a distinct serialized field and SHALL NOT be omitted because the node fields already contain a name pointer. The density parameter's modern node dispatch uses the node lightweight function and a no-op resolved slot; it adds no resolved bytes at this point.

The native loader can normalize a negative primary value according to a virtual predicate and replaces the runtime primary value with alternate_value when present. The schema retains both stored values and the original presence byte. Runtime parameter-array insertion is not MFC object-map insertion and adds no bytes. Density units, full node-flag meanings and alternate-value workflow semantics remain open. No installed sample exercises the alternate-value branch.

#### 11.24.3 Component body references and additional entity IDs

*Status: Verified ordinary part bodies; scalar meanings partial · Evidence: [C-39], F.27*

moCompAtomBody_c, moCompSolidBody_c and moCompSurfaceBody_c share the common component-reference body of 11.17.2. They then store a moFaceRef_c pointer, signed i32 integer_count, integer_count i32 unknown values, unknown_108 i32, body_atom_id i32 and face_node_id i32. The count SHALL be nonnegative and bounded by available bytes. The last two IDs are identified by getAtomId on the body and getNodeId on the referenced face in the save path. The surface subclass only sets a runtime flag after reading the common body; it adds no wire bytes. The solid subclass resolves to the same serializer.

The appearance corpus also verifies five additional modern entity-ID representations using the keeper prefix of 11.18.2. moThinClosingSurfIdRep_c and moBendEndSurfIdRep_c dispatch to the two-i32 suffix at +0x38/+0x3C. moSegmLeftTrimEdgeIdRep_c and moSegmRightTrimEdgeIdRep_c directly serialize that same two-i32 suffix. moSurfExtNeighborIdRep_c uses a direct source entity ID followed by unknown_48 i32, matching the derived-surface layout. These aliases are established from primary serializer and virtual-suffix evidence, not class-name similarity.

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

### 11.115 Forwarded references, sheet-metal master and dimension-increment IDs

*Status: Native serializers, virtual-call machine code and 906 complete supported continuation prefixes; semantic coverage remains partial · Evidence: [C-130]*

For the ordinary modern part-root context following 11.114, retain the same archive map and the original node `+2C` flag word. Read these fields in order:

| Order | Model member | Wire field |
|---|---|---|
| 1 | `+238` | Direct `moForwardedFeatRefsList_c`, using packed MFC count followed by tagged object pointers. No tag or map slot is added for the direct list itself. |
| 2 | `+E30` | Tagged `moPMarkRecord_c` pointer. Its already documented body is one i32 permanent-mark ID. Ordinary conditions select this field; ghosting-disabled runtime disposal consumes no additional bytes. |
| 3 | `+1008` | i32 raw setting. |
| 4 | `+FD8` | i32 count; each entry is a tagged `moAtom_c` pointer followed by i32 raw value. Preserve null keys, duplicate keys and arbitrary stored values before native filtering/boolean normalization. |
| 5 | `+100C` (part only) | i32 presence; if nonzero, a direct sheet-metal-master body described below. |
| 6 | `+10A8` | i32 count and the flag-selected ID-map records described below. |
| 7 | `+10FC` (part only) | i32 raw setting. |
| 8 | `+1100` (part only) | i32 count, then that many direct `moDimIncrementId_c` bodies. |
| 9 | `+1120` (part only) | i32 raw setting. |
| 10 | `+1128` (part only) | i32 count, then that many direct `moDimIncrementId_c` bodies. |

All version gates selecting these fields precede the supported minimum (`version > 0x27CB`). The part-root primary-vtable callback at `+1A0` returns true, selecting the part-only fields. `modelreftail.read` requires an explicit `moPart_c` root context and an original u32 node-flag word; other root types are not inferred.

The forwarded-list serializer calls `suObList::Serialize` and then a runtime reference-repair routine with no archive argument. `moForwardedFeatRefs_c`, observed as a list entry, has one i32 feature ID at `+08`; its in-memory reference caches are not serialized here. Its primary serializer shares an implementation labeled for another class by the decompiler; primary RTTI and the caller's feature-ID lookup establish this use.

**Embedded sheet-metal master.** The model calls the sheet-metal bridge at virtual slot `+2C8`, passing a pointer-to-master and the archive. The model decompilation omits the third argument; machine code explicitly sets `R8` to the archive before the call. Concrete `smmBridge_c` dispatch creates a `moSheetMetalMaster_c` when loading and calls its serializer. Therefore a nonzero `+100C` is followed by real archive bytes, not just runtime allocation. The master is a direct object with no new MFC class/object slot:

1. Ordinary `moFeature_c` body, without an additional `moModelFeature_c` word.
2. i32 count, then tagged `moPerBodySheetMetalData_c` pointers. Although the native member is a `suObList`, this serializer writes a custom four-byte count rather than its packed-list framing.

Each per-body record calls the ordinary no-payload `su_CObject` base and reads four tagged pointers in order: `moCompFeature_c` at `+08`, `moCompSolidBody_c` at `+10`, then two `moCompFeature_c` pointers at `+18/+20`. Two i32 raw settings follow at `+28/+2C`. The native gates `>0x1060` and `>0x1314` precede this reader's minimum. Native UI-state and owner callbacks after loading consume no further fields. The retained sheet-metal module baseline is `sldsmmu.dll`, SHA-256 `ae20d533b36f105f22de4ed5984fb2245bbf03fb91a84ae473e0941a4b9e3c54`.

**Flag-selected ID map.** The model's stored node `+2C` bit `0x100` controls framing; never infer it by scanning the map payload. For supported modern versions:

| Flag state | Physical payload after i32 count |
|---|---|
| Set | Count records of four u32 words: high/low key words followed by two raw value words (16 bytes per record). |
| Clear | Count physical triples of u32 words (12 bytes per triple). If the first triple is `(0, FFFFFFFF, 0)`, native loading switches to a sentinel/pair migration path over the following triples. |

Retain all physical words, including sentinel triples and ignored/padding words, in their original order. `modelreftail.id_map` reports `wide`, `narrow` or `sentinel_triplets`; it does not replace stored records with the native hash map, drop nonpositive values or collapse duplicate keys. These record meanings remain partial, so all payload words count as unknown. Negative/impossible counts fail closed. The pre-`0x12BB` map layout is outside scope.

**Direct dimension-increment ID.** Its runtime serialization gate is `moPattern_c::isVaryInstanceCircularEnabled`, which tests the current runtime format against `0x1437`; it does not test the individual archive's version. The retained installed version-manager constructor defaults to 19000, enabling this serializer in the ordinary baseline. A runtime overridden to disable the feature is outside this reader's context.

| Order | Native member | Modern wire field |
|---|---|---|
| 1 | `+00`, `+08` | u8 handle-absent flag, then a tagged dimension-handle pointer **even when the flag is nonzero**. Retain the pointer that native loading may discard. |
| 2 | `+10` | u8 name-presence flag; if nonzero, CString name. |
| 3 | `+18/+1C/+20` | Three i32 raw values. |
| 4 | `+40`, `+48` | u8 display-dimension-absent flag, then a tagged `moDisplayDim_c` pointer, again always present as a pointer field. |
| 5 | `+24`, `+28` | u8 raw flag and f64 raw value. |
| 6 | `+50` | i32 raw value. |
| 7 | `+30`, `+38` | u8 raw flag and f64 raw value. |

All archive gates for this modern body precede `0x27CC`. Retain noncanonical flag bytes, optional strings, nonfinite doubles and both discarded pointer representations. Direct increment records add no MFC map slots; their nested tagged objects still do. The native `SerializePass2` routine has no archive extraction and adds no wire fields in this baseline.

**Nested dependencies.** `moRadiateSurfIdRep_c` uses the existing entity-representation body followed by two i32 words at `+38/+3C`; primary-vtable slot `+2A0` establishes that suffix. A standard tagged `suObArray` encountered in the increment graph uses packed object-array framing and is installed only in the copied reader registry.

The increment graph also reaches an external spline not present in the earlier sketch corpus. Spline collection mask bit `0x1` selects the external subtype, which uses the same old spline serialization body before native runtime conversion. Masks `0/1/8/9` are now supported; other subtype bits remain explicit stops. For supported spline types, the `+1F0` callback returns false, so a nonzero stored `+314` selects the secondary interpolation block only when stored control-point count `+308` is positive. This block is **not** the primary interpolation layout: read u32 parameter count, i32 width, two f64 settings, then count f64 parameters only. It has no parallel u16 words, i32 indices or count-times-width value matrix. A zero parameter count still has the width/two settings; native generation of default ones from control points consumes no more bytes. The reader retains the stored zero count and conservatively counts the secondary block as unknown.

`modelprefix.read(..., through_reference_tail=True)` composes all preceding stages. `body_feature_inventory.py --model-reference-tail` completes all 906 supported configurations, spanning 277,666 bytes including 167,702 still unknown. The retained report records source/body hashes, node flags, exact spans, raw settings and branch/collection counts. Observations include one populated forwarded list, 901 non-null permanent-mark pointers, 953 atom-map entries, 818 embedded masters, per-body records in 25 configurations, three nonempty wide ID maps (7/3/1 records), and one second increment array with 33 records; the first increment arrays are all empty. Narrow/sentinel map framing, populated direct increment records and secondary interpolation also have synthetic boundary tests. Later model-tail fields and whole-document bit coverage remain open.


### 11.130 Final item tables, document folders and selection-set data

*Status: B — native-bound modern load framing; field meanings and nested classes remain partial.*

A constructor may assign a base-class vtable and then overwrite the same primary pointer with its derived table. Readers and evidence collectors SHALL use the final assignment when selecting the complete runtime serializer. Favorite feature items finish at vtable `4CE35188`, serializer `4B65AC60`; history feature items finish at `4CE42D08`, serializer `4B6622F0`. Their earlier temporary tables select the base item serializers `4B65AEC0` and `4B662410` only. Both derived serializers call their base, then serialize the moCompFeature_c pointer at +0x40 for the modern scope. This pointer follows ordinary null/new/reference archive rules and may reference an inherited or already-read object. It is not an array count or a new list root.

The sensor-folder constructor binds vtable `4D06AA60` to serializer `4BEC2E00`. On load it calls only the common folder serializer; the sensor database export call is store-only. `moDocsFolder_c` binds `4D05DE08` directly to the common folder serializer `4BEC2AF0`, without a suffix. `moLiveSectionFolder_c` binds `4D066FF8` to `4BEC2BE0`: common folder body, then one i32 for v > 0xF59. Native code compares that word with 1 when assigning its runtime byte at +0x370. Readers preserve the original four bytes; its field meaning remains unresolved and receives no semantic credit.

Selection-set data likewise has two primary assignments: temporary moItemData_c table `4CE3C048`, then final moSelectionSetData_c table `4CE3CA68`. The final serializer `4B65E130` calls the base node-stage serializer `4B65E000`, then reads this suffix:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `raw_40` | i32 | v > 0x1BCB | Native member +0x40; preserve raw, no semantic credit | B |
| 2 | `raw_48` | CString | v > 0x1BCB | Native member +0x48; preserve text, no semantic credit | B |
| 3 | `comment` | Tagged pointer | v > 0x3720 | moFeatComment_c pointer; shared map, no reset | B |
| 4 | `items` | Direct suObList | v > 0x1BCB | ReadCount then tagged item pointers, in stored order | B |

The constructor identifies +0x50 as a direct suObList, not an array pointer. The modern base node path uses the node lightweight layout and empty resolved stage, as verified by both temporary and final vtable slots. Nonempty selection item and comment bodies still require their own class readers. An unsupported nested class causes an explicit stop.

At the edition 0.141 checkpoint, `resolved_feature_list_frontier.py` probed all declared roots with the same inherited context, recording only the last fully completed root before an error. Across **906** modern configurations, **14** stop constructing model context; **892** reach the separate list, framing **4,608 complete roots** before explicit unsupported-class stops. None reaches a complete list. The leading next dependencies are moDetailCabinet_c (821 cases), moSensor_c (50), moSurfaceBodyFolder_c (15) and moSelectionItemData_c (4). Context failures and list failures remain separate. Failed partial objects receive no credit, and the diagnostic remains outside the coverage registry.

Evidence: C-158.

