# Model lightweight schema snapshot

Source: `../inventor-interface/docs/solidworks-file-format/specification.md`

Edition 0.302 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.11 Modern node lightweight fields and feature stages

*Status: Verified component layout; Partial meanings and surrounding objects · Evidence: [C-26], F.14*

For v > 0xC7C, the common feature serializer first calls the virtual lightweight stage at vftable offset 0x4B8 and the resolved stage at 0x4C0. In the verified feature/folder tables these are the concept implementations. The lightweight concept stage first reads the node fields below. It then reads an i32 presence flag, an optional direct node list when nonzero, and a visual-properties object. The resolved concept stage can read another node list, depending on runtime archive conditions. Feature-specific fields follow these stages; a folder appends its own version-gated i32 fields. These stages SHALL NOT be concatenated without preserving their actual class dispatch and archive conditions.

The non-database `moNode_c::SerializeLWData` body for v > 0xC7C is:

| Order | Field | Type | Condition | Meaning | Status |
|---|---|---|---|---|---|
| 1 | `name` | object | | `moNodeName_c`; body is one CString, no additional fields. | V |
| 2 | `unknown_0c` | i32 | | Stored value; loader can merge selected bits with existing runtime state. | ? |
| 3 | `flags_28` | u32 | | Flag domain open. Writer clears bit 0x1000 for storage and restores its runtime value afterward; loader sets runtime bit 0x40000000. | ? |
| 4 | `id` | i32 | | Node ID, member +8; certain import/archive conditions can replace the stored ID after reading. | V |
| 5 | `unknown_2c` | u32 | v > 0x143A | Member +0x2C. | ? |
| 6 | `unknown_20` | CString | v > 0x23CD | Member +0x20; meaning open. | ? |

A direct `moNodeList_c` body consists of ReadCount, that many object records and a final i32. The current writer writes zero for that final word; the reader consumes it without a verified semantic use. It can remove null and duplicate object pointers after reading; those runtime edits do not change the stored count. Unlike the separate resolved-feature stream's direct `suObList`, this node-list body includes the final word.

`modelnode.lightweight()` preserves stored values and counts unnamed fields as unknown. It supports only the version range above and requires the caller's archive context. `ObjectArchive(..., inherited_map=...)` copies map slots while retaining earlier object identities; appending classes or objects does not alter the source list. Missing context fails rather than guessing indices. The producer and full contents of the cached model map remain open (OI-003).

### 11.12 Modern part-root prefix and external header map

*Status: Partial root; verified prefix for 906 installed configurations · Evidence: [C-27], F.15*

For a non-database part configuration with v > 0x27BC and a separately stored `Contents/Config-N-ModelHeader`, `Contents/Config-N` begins with an MFC `moPart_c` object. Its lightweight stage begins with the following prefix. The rest of the root is not yet specified.

| Order | Field | Type | Meaning | Status |
|---|---|---|---|---|
| 1 | `lightweight_size` | i32 | Number of bytes after this word in the lightweight section. Nonnegative and within the stream. | V |
| 2 | external header operation | no bytes | Read the separate ModelHeader object (11.8), then append its self-reference, when nonnull, to this archive's load map. Append each external-reference list item afterward, in list order. These are object slots, not class declarations. | V |
| 3 | `unknown_a8` | i32 | Filed-concept member +0xA8; meaning open. | ? |
| 4 | `unknown_ac` | u32 | Filed-concept member +0xAC; meaning open. | ? |
| 5 | `node` | node LW body | Clause 11.11; continues into concept/model/part fields that remain open. | V / ? |

The byte count is written by reserving an i32 and patching it with the end position minus the position immediately after that word. A loader in a mode that skips the section reads the word, flushes its archive buffer and seeks forward by that many bytes. In ordinary loading the scope helper also preserves the section's map for later resolved-stage use. A subsequent resolved-stage scope can restore this map. Its full contents require decoding the whole lightweight section; the prefix alone does not establish that context.

The header operation uses the same object identities as the parsed header and does not copy its entire class map. The map import is gated by v > 0x27BC in the loader; the separate-header selection also depends on version, document kind, stream name and archive conditions. This clause covers the explicitly selected modern part configuration case. Readers SHALL NOT apply it to embedded or legacy model contexts merely because a nearby CString resembles a class name.

`modelprefix.read()` returns the partial root and its archive, positioned after the node fields, with `lightweight_end` identifying the verified section boundary. It validates the byte count and never seeks past unread content to claim that content is decoded. It is excluded from the coverage registry.

### 11.13 Concept lightweight continuation

*Status: Verified component layout; class bodies remain partial · Evidence: [C-26], F.16*

After the node fields of 11.11, the modern concept lightweight stage reads an i32 `has_merged_feature_list`. If nonzero, a direct node-list body follows (ReadCount, object records, final i32 as in 11.11). Next is an MFC visual-properties object, including null or reference tags. Its `moVisualProperties_c` body is shared with clause 13.6 and uses the current model archive version here. The class map continues across the node, optional list and visual properties.

`modelprefix.read(..., through_concept=True)` implements this continuation, reusing the shared visual/material reader. The resolved concept stage is not part of this continuation. The optional list can contain feature classes not yet implemented; a reader SHALL fail explicitly on those classes rather than assume the list is empty.

### 11.14 Units tables and user-unit records

*Status: Verified component layouts; member meanings and enum domains partial · Evidence: [C-28], F.16*

The modern model lightweight stage follows its filed-concept/concept fields with a `moUnitsTable_c` object (v >= 0xF98). Its body for v >= 0x69 is a direct object array: ReadCount followed by that many MFC object records. It has no trailing word. The older list-to-array conversion path is outside the current implementation. Nulls, references and nested unit objects use the same archive map.

#### 11.14.1 Common user-unit body

*Status: Verified layout; member meanings open · Evidence: [C-28], F.16*

`moUserUnits_c` and the derived classes listed below start with these fields. All offsets in field names are in-memory members, not byte offsets in the file. Their meanings remain unknown; none is inferred from a sample's numeric value.

| Order | Field | Type | Condition | Status |
|---|---|---|---|---|
| 1 | `unknown_0c` | i32 | | ? |
| 2 | `unknown_10` | i32 | | ? |
| 3 | `unknown_18` | f64 | | ? |
| 4 | `unknown_20`, `unknown_24` | i32 × 2 | | ? |
| 5 | `unknown_30` | f64 | v > 0x822 | ? |
| 6 | `unknown_28` | i32 | | ? |
| 7 | `unknown_6c` | i32 | v > 0x1AA | ? |
| 8 | `unknown_70` | i32 | v >= 0x306 | ? |
| 9 | `unknown_74` | i32 | v > 0x1041 | ? |
| 10 | `unknown_88` | f64 | v >= 0x1BD2 | ? |
| 11 | `unknown_7c` | u16 | v >= 0x1BD2 | ? |
| 12 | `unknown_08` | i32 | v > 0x1BD6 | ? |

The current body occupies 62 bytes. The native loader changes a negative stored `unknown_0c` to 2. For `unknown_30`, it assigns the stored value only when its absolute value exceeds 1e-11. The schema reader preserves both raw values; constructor defaults are not serialized fields.

#### 11.14.2 Derived bodies

*Status: Verified class-specific layouts; enum meanings open · Evidence: [C-28], F.16*

Each row starts with 11.14.1, followed by the stated suffix.

| Classes | Suffix | Status |
|---|---|---|
| `moNumberUserUnits_c`, `moFloatNumberUserUnits_c`, `moSpringConstantUnits_c`, `moStressUnits_c`, `moGravityUnits_c`, `moLinearMotorUnits_c`, `moRotaryMotorUnits_c`, `moFrequencyUserUnits_c` | None; runtime tables select the base serializer. | V |
| `moAngleUserUnits_c`, `moUnitSysUnits_c`, `moForceUnits_c` | `unknown_b0` u16. | ? |
| `moPowerUnits_c`, `moEnergyUnits_c`, `moTimeUnits_c` | [v > 0xDCE] `unknown_b0` u16. | ? |
| `moLengthUserUnits_c`, `moPercentUserUnits_c` | `unknown_b0` u16; [v > 0x926] `unknown_b8` object, declared as length units. | ? |
| `moDensityUnits_c` | [v > 0x92E] `unknown_c0`, `unknown_c4` u16 × 2; [v > 0xAC6] `has_length_unit` u8 and, if nonzero, a length-unit object; [v > 0x1FAA] `unknown_c8` u16 and `unknown_cc` i32. | V / ? |

For the length/percent serializer only, v == 0x751 maps the stored u16 through [6, 7, 8, 0, 1, 2, 9, 3, 4, 5, 10] for runtime use; values above 10 are otherwise reset to zero by the loader. The schema reader preserves the original word. The density loader before v 0x180B remaps `unknown_c0` through a native table whose domain remains open; it also resets runtime `unknown_cc` after consuming its stored value. These runtime normalizations do not remove bytes from the layout.

`modelunits.py` implements these bodies. `modelprefix.read(..., through_units=True)` reads the concept continuation and units table, then stops before the remaining model fields. Partial prefixes are excluded from coverage. Unit enum meanings, the remaining model body and merged-list feature classes remain open under OI-003.

### 11.17 Internal unit components and component references

*Status: Verified scoped layouts; reference payloads and conditional modes partial · Evidence: [C-31], F.19*

This clause covers ordinary part model archives with external XML and v > 0x27CB. Drawing-specific persistent-ID tails, clipboard/import conditions and other component-owner contexts are outside the implementation. Unknown stored values are preserved; native runtime replacements do not erase fields from this layout.

#### 11.17.1 Internal unit component

*Status: Verified internal-part branch; external models open · Evidence: [C-29], [C-31], F.19*

`moUnitComponent_c` stores an owner object pointer, followed by `has_external_model` u8. In the implemented branch the owner is null or the current moPart_c root and the flag is zero. These three bytes are the complete body when the owner is a short object reference or null. A nonzero flag introduces external-model handling that depends on the owner and runtime state; the reader rejects that branch explicitly. It also rejects non-part owners. Clause 11.19 establishes the intervening direct serializer and the model-prefix continuation through this unit component.

#### 11.17.2 Common component reference

*Status: Verified scoped layout; optional direct payloads and member meanings partial · Evidence: [C-30], [C-31], F.19*

`moCompRef_c` begins with a component object, then these fields in order:

| Order | Fields | Types | Condition | Status |
|---|---|---|---|---|
| 1 | `unknown_68`, `unknown_18`, `unknown_1c`, `unknown_20`, `unknown_30`, `unknown_70` | i32 × 6 | | ? |
| 2 | `unknown_84` | i32 | Nonzero introduces a direct old-reference payload, not implemented. | ? |
| 3 | `unknown_a0`, `unknown_34`, `unknown_48` | i32, u32, i32 | | ? |
| 4 | `unknown_90`, `unknown_94`, `unknown_98`, `unknown_9c` | i32 × 4 | | ? |
| 5 | `has_faceted_data` | i32 | If nonzero, a moFacetedData_c object follows. | V |
| 6 | `has_string_b8` | i32 | If nonzero, unknown_b8 CString follows. | V / ? |
| 7 | `has_reference_info` | i32 | v > 0x2B60; if nonzero, direct moCompRefInfo_c body follows, not implemented. | V / ? |
| 8 | `has_transform_c8` | i32 | v > 0x2BB8; if nonzero, mgXform_c body follows. | V / ? |
| 9 | `has_string_d0` | i32 | v > 0x2F55; if nonzero, unknown_d0 CString follows. | V / ? |
| 10 | `unknown_4c` | u32 | v >= 0x3EF6 | ? |

Presence words are stored i32 values, not one-byte booleans. New object declarations and references share the model's map. A recognized faceted-data tag still requires a registered complete class reader; no opaque payload is skipped.

#### 11.17.3 Component boundary-entity and edge suffixes

*Status: Verified scoped layouts; boundary references open · Evidence: [C-31], F.19*

The modern `moCompBEntRef_c` body used by `moCompFace_c` appends `has_alias` u8, an alias CString when nonzero, a boundary-reference object, then an external-reference object declared moExtObject_c (member +0xF0). Both pointers are present in both alias cases, including null pointers. Edition 0.18 corrects the previously omitted trailing pointer. It occupies two bytes when null and is distinct from the component-owner pointer. Earlier external-reference branches are outside this version range.

`moCompEdge_c` adds to that body `unknown_188` i32 flags, `unknown_158` point, `unknown_170` point, `unknown_18c` u32, an i32 reference count, and that many object pointers. The implementation accepts nonnegative counts bounded by the stream. When (flags & 6) != 6 and (flags & 8) != 0, two composite-edge object pointers follow. When both masks 0x2 and 0x4 are set the native loader returns from this stage before that optional pair, even if bit 8 is set. Ordinary part context excludes additional archive-condition and drawing-only tails.

`modelcomponents.py` implements these scoped records. The wrapper in 11.16 accepts the verified component-reference classes established here and in later extension clauses, including moCompProfile_c (11.156); unverified derived contexts remain explicit stops. The boundary-reference object bodies remain open.

### 11.19 Model unit component and direct face appearances

*Status: Verified modern part continuation; appearance member meanings partial · Evidence: [C-34], F.22*

After the units-table object in 11.13, the model serializes its embedded moFeatureRefMgr_c (+0xA98). For the supported modern range v > 0x27CB this direct call consumes **zero bytes**: it rebuilds runtime sharing from existing header references. It has no MFC object declaration or pointer tag. Next is the unit-component object pointer (+0x988), using 11.17.1 and the existing archive map. The scoped reader requires moUnitComponent_c and its internal-part branch.

The part then restores its embedded moFaceVizProps_c directly. Its wire sequence is:

| Order | Fields | Type | Status |
|---|---|---|---|
| 1 | face_count | i32 | V |
| 2 | face elements | direct face element[face_count] | V / ? |
| 3 | property_count | i32 | V |
| 4 | property elements | direct property element[property_count] | V / ? |

Both counts SHALL be nonnegative and bounded by the available bytes. They are signed 32-bit fields, not the variable-width MFC ReadCount encoding. A direct property element is unknown_10 i32 followed immediately by the moVisualProperties_c restore body of 13.6. The body has **no enclosing MFC object tag**. A direct face element is that property element, a moCompFace_c object pointer, then unknown_28, unknown_1c and unknown_18, each i32 in that order. The three-word tail is present in the scoped modern range, including unknown_28 at the native control-flow join.

The reader preserves elements in wire order; the native implementation inserts each restored element at the runtime list head. Integer member meanings remain unknown and receive no semantic coverage credit. `modelprefix.read(..., through_component=True)` stops after the unit-component pointer; `through_face_properties=True` stops after both lists, preserving the map and leaving the remaining model body unread. Assembly/drawing variants and older appearance-list layouts are outside this claim.

### 11.20 Model curvature, reference-plane data and metadata maps

*Status: Verified modern part continuation; member meanings partial · Evidence: [C-35], F.23*

This clause continues 11.19 in its ordinary part/archive context, v > 0x27CB. The following fields occur in wire order:

| Order | Fields | Type | Status |
|---|---|---|---|
| 1 | unknown_d28 | u8 | ? |
| 2 | curvature | object, declared gcCurvatureObject_c | V |
| 3 | reference_plane | object, declared moTransRefPlaneData_c | V |
| 4 | has_box | u8 | V |
| 5 | box | Box of 11.4, ten f64 | V; present iff has_box != 0 |
| 6 | is_fastener | i32 | V |
| 7 | visual_map_count | i32 | V |
| 8 | visual map entries | (unknown_key i32, moVisualProperties_c object)[visual_map_count] | V / ? |
| 9 | unknown_f10 | i32 | ? |
| 10 | is_toolbox_part | i32 | V |
| 11 | unknown_f24 | i32 | ? |
| 12 | detail_definition_count | i32 | V |
| 13 | detail definitions | (name CString, moDetailDefs_c object)[detail_definition_count] | V / ? |
| 14 | unknown_f78, unknown_f80 | f64 × 2 | ? |

The box presence flag occupies one byte; any nonzero value includes the box. The fastener and Toolbox classifications occupy four bytes each; the reader preserves their original values. Their roles are established by the save path calling isFastener and isToolboxPart. Both map counts SHALL be nonnegative and bounded by available bytes. Entries are preserved in wire order, including null pointers; runtime map insertion may reorder or omit null-valued entries. No installed sample has a nonempty detail-definition map, and the nonnull moDetailDefs_c payload remains unimplemented. Unknown derived classes fail explicitly. All object pointers share the existing model archive map.

`gcCurvatureObject_c` has a 48-byte body: five consecutive f64 curvature values (indexed 0–4), then time_low u32 and time_high u32. The timestamp is the Windows FILETIME value time_low | (time_high << 32); setCurvatureObjectTime populates it using GetSystemTime and SystemTimeToFileTime. The getter returns the same 64-bit union. The native loader computes five base-10 logarithms into separate runtime members; those cached logarithms are **not serialized**. Specific curvature-index roles and units remain open; no logarithm or normalization is applied by the schema reader.

`moTransRefPlaneData_c` has a 24-byte body, in order: unknown_08 i32, unknown_0c i32, unknown_10 u32, back_face_color u32, unknown_18 i32, unknown_1c u32. The setter setBackFaceColor establishes the fourth field at member +0x14. The other word meanings remain open and receive no semantic coverage credit. These offsets name runtime members, not absolute stream positions.

`modelprefix.read(..., through_spatial=True)` stops after is_fastener; `through_metadata=True` additionally reads through unknown_f80. The next direct manager (+0xA90) and subsequent model data remain unread. This partial reader is not registered as a complete model-stream decoder.

### 11.21 Direct boundary-reference manager

*Status: Verified modern part layout; manager identity and scalar meanings partial · Evidence: [C-36], F.24*

Immediately after 11.20, the model directly serializes its member +0xA90. Its class identity is not yet established; this clause names the member by its observed function. There is no enclosing object tag or class declaration. The first field is format i32. If format is not 2, the native helper returns without reading a manager body. If format is 2, the following fields occur:

| Order | Fields | Type | Status |
|---|---|---|---|
| 1 | map_count | su_CArchive ReadCount | V |
| 2 | entries | (name CString, boundary-reference object)[map_count] | V |
| 3 | unknown_110, unknown_114 | i32 × 2 | ? |

The map's ReadCount encoding is u16, with 0xFFFF followed by u32; it is not the i32 count used in 11.20. Each value uses the shared MFC archive map. The native loader validates nonnull map values as moBEntRef_c-derived objects. The scoped reader supports the boundary, edge, face and vertex classes of 11.18 and rejects other classes explicitly. Null values remain permitted. The two final words are direct manager members; their hexadecimal names are memory offsets, not stream offsets.

The wire entry order and every stored name/value pair are preserved, including repeated names and shared object identities. Native map insertion and later rebuilding of lookup structures do not serialize additional object pointers. The scoped modern path has no older embedded arrays or extra compatibility word. All installed modern samples use format 2. Other format values follow the observed no-body dispatch; they have no native corpus fixtures and are not claimed as independently verified formats.

`modelprefix.read(..., through_manager=True)` continues through this manager and preserves its MFC map for later stages. The following model field at +0x158 and conditional dynamic-array body are not consumed. The model's remaining lightweight and resolved bodies, including drawing/assembly variants, remain incomplete.

### 11.22 Post-manager array and model flags

*Status: Verified modern part continuation; array meanings and shrink-wrap bodies incomplete · Evidence: [C-37], F.25*

After 11.21, in the same ordinary part context with v > 0x27CB, the model reads has_array i32 (member +0x158). If nonzero, a direct dynamic-array body follows. Its first field is element_size u32. A zero size terminates this array body immediately and represents no allocated array, even when has_array was nonzero. Otherwise element_size SHALL be divisible by 24, as required by the native loader. The next fields are capacity u32, growth u32 and count u32, followed by exactly element_size × count raw bytes. The multiplication and available-byte bound SHALL be checked before reading. This is a direct body with no object tag. The array's element meanings remain unknown, and all payload bytes receive no semantic coverage credit.

The model then calls its XML virtual stage. For moPart_c, slot 0x3F8 selects the model key builder (Keywords plus the header creation-time key); slot 0x3E8 selects the part override. That override calls moModel_c::SerializeXmlData and only adds XML signatures while storing. The model implementation likewise only updates the external XML extractor while storing. **Neither function reads bytes while loading**, including when no extractor is present. No inline CString is inserted at this point; the feature XML branch in 11.15 does not describe this model call.

The following binary fields then occur:

| Order | Fields | Type | Condition | Status |
|---|---|---|---|---|
| 1 | unknown_f14 | i32 | | ? |
| 2 | has_shrink_wrap_data | i32 | | V |
| 3 | is_defeatured_state | i32 | v > 0x2F72 | V |
| 4 | shrink-wrap data | direct body | has_shrink_wrap_data != 0 | ?; not implemented |
| 5 | has_cthread | i32 | | V / ? |

The native save path derives the second field from shrink-wrap state or an applicable defeatured assembly state, derives the third from isDefeaturedState and derives the last from getHasCThread. The full domain and interpretation of the CThread indicator remain open; the schema reader preserves its stored i32. A nonzero shrink-wrap presence flag SHALL stop the scoped decoder explicitly until that direct body is implemented. The defeature flag is consumed before the optional body and is not a second presence flag. The scoped reader stops after has_cthread, before the direct embedded serializer at model member +0xC88. Remaining model lightweight/resolved content is still incomplete.

### 11.23 Embedded model collections and lightweight tail

*Status: Verified modern part context; optional payloads and member meanings partial · Evidence: [C-38], F.26*

After 11.22, the model directly serializes its embedded suObList at +0xC88: su_CArchive ReadCount followed by that many object pointers. No enclosing class/object tag precedes the list. The constructor establishes the collection type independently of the observed empty-list bytes. Native loading sets a runtime owner member on each nonnull object after restoring the list; this adds no bytes. The current corpus contains no elements, so no specific populated-list class family is claimed as verified.

Next are unknown_d50 i32 and a named display-flag-note map. The map starts with a signed i32 key count. Each entry is name CString, signed i32 note_count, then note_count moDisplayFlagNoteData_c object pointers. All counts SHALL be nonnegative and bounded by available bytes. This map uses i32 counts, unlike the preceding suObList. The schema preserves wire order and shared object identities; populated note payloads remain unimplemented and absent from the corpus.

The shared model lightweight serializer then ends with the following fields, in order, for the scoped range v > 0x27CB:

| Order | Fields | Type | Condition | Status |
|---|---|---|---|---|
| 1 | unknown_77c | byte[16] | | ? |
| 2 | unknown_1180 | i32 | | ? |
| 3 | unknown_79c | byte[16] | | ? |
| 4 | unknown_1184, unknown_1188, unknown_118c | i32 × 3 | | ? |
| 5 | has_component_import_data | i32 | | V |
| 6 | component-import data | direct body | field 5 != 0 | ?; not implemented |
| 7 | is_mixed_mode_model | i32 | v > 0x27CC | V |
| 8 | unknown_12f8 | u32 | v > 0x2B6C | ? |
| 9 | unknown_1300 | i32 | v > 0x46BB | ? |
| 10 | map_count_1308 | i32 | v > 0x46C5 | V |
| 11 | map entries | (unknown_1c u32, unknown_20 i32)[map_count_1308] | v > 0x46C5 | ? |
| 12 | mbd_3d_pdf_publish_cache | object, declared moMBD3DPDFPublishCacheData_c | v > 0x2BCE | V / ? |

The 16-byte arrays are preserved as raw bytes; their interpretation is not inferred from their size. The final map count SHALL be nonnegative and bounded by available bytes, and both values in every pair retain their stored signedness. Native insertion/reordering does not alter wire order. The mixed-mode field is identified by the isMixedModeModel call in the save path. Present direct component-import data stops the decoder explicitly. A nonnull publish-cache payload requires its class decoder and is not implemented; null and existing-object references retain ordinary MFC framing.

`modelprefix.read(..., through_collections=True)` stops after the note map; `through_model_tail=True` continues through the end of the shared model serializer. The latter remains a **partial part-root reader**: the part-specific lightweight continuation, optional model payloads, complete resolved content and field meanings listed as unknown remain open. No whole-stream coverage is inferred from reaching the end of this shared stage.

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
