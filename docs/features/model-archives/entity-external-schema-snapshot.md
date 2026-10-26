# Entity and external-reference schema snapshot

Source: `../inventor-interface/docs/solidworks-file-format/specification.md`

Edition 0.327 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.8 Header streams

*Status: Verified layouts in observed versions; Partial meanings · 2,201 streams · Evidence: [C-21], [C-22], F.11*

`Header2` and `Contents/Config-<n>-ModelHeader` each hold a `moHeader_c` object record (11.5). The archive version `v` comes from `_MO_VERSION_<n>/…`, interpreted as unsigned u32. The corpus exercises v = 9000, 10000, 11000, 18000 and 19000. The tables give the reader's version conditions; branches outside these observed versions are binary-derived and do not establish verification on older files.

The header maintains user and company lists, revision logs, ID allocation, external references and self-reference, update stamps, attachments and document-state values. Names come from accessors listed in [C-22]. Names with `unknown_` retain member offsets, not stream offsets; their meanings remain open (OI-021).

#### 11.8.1 Header body

*Status: Verified layout; Partial meanings · Evidence: [C-21], [C-22], F.11*

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | `saved_while_model_exists` | i32 | v > 0x371B | `getSavedWhileModelExists()`. | V |
| 2 | `users` | object | | `su_CStringArray`; `UserName()` and `CurrentUser()` index this array. | V |
| 3 | `companies` | object | | `su_CStringArray`; `CompanyName()` indexes it. | V |
| 4 | `revision_logs` | object | | `suObList` of `moLogs_c` objects, 11.8.2. | V |
| 5 | `unknown_50` | u32 | v ≥ 0xF1 | Stored su_CTime; exact role open (OI-021). | ? |
| 6 | `next_id` | i32 | | `getNewID()` returns and increments this member; `getHighestID()` returns this value minus one. | V |
| 7 | `external_references` | object | | `moExtObjectList_c`, a list of object records. | V |
| 8 | `update_stamp` | u32 | v > 0x96 | `getCurrentUpdateStamp()`. | V |
| 9 | `self_reference` | object | v > 0xD8 | `moExtObject_c`, 11.8.3. | V |
| 10 | `body_stamp` | u32 | v > 0x30B | `getCurrentBodyStamp()`. | V |
| 11 | `unknown_58` | i32 | v > 0x4E4 | Reader discards the member value after reading when v > 0x2B72; role open (OI-021). | ? |
| 12 | `unknown_68` | i32 | v > 0x500 | Meaning open (OI-021). | ? |
| 13 | `creation_time` | u32 | v ≥ 0x506 | su_CTime; `getCreationTime()`. | V |
| 14 | `unknown_60` | i32 | v ≥ 0x531 | Meaning open (OI-021). | ? |
| 15 | `header_usage` | i32 | v > 0x859 | `headerUseCode_e`; complete enum domain open (OI-021). | B |
| 16 | `unknown_c0` | i32 | v > 0x91E | Meaning and bit domain open (OI-021). | ? |
| 17 | `linked_attachments` | object | v > 0xAA1 | `su_CStringArray`; `getLinkedAttachments()`. | V |
| 18 | `component_count` | i32 | v > 0xB8C | `getNumberOfComps()`; the native empty assembly stores 1, so this is not established as the number of inserted components. | B |
| 19 | `license_flags` | u32 | v > 0xCF6 | `getLicenseFlag()`; bit domain open (OI-021). | B |
| 20 | `update_time_stamp` | u32 | v > 0xE5C | su_CTime; `getCurrentUpdateTimeStamp()`. | V |
| 21 | `replaced_components` | object | v > 0xF3A | `suObList`; `getReplacedComponents()`. | V |
| 22 | `is_copied` | i32 | v > 0x128F | `setIsCopied()`. | V |
| 23 | `unknown_64` | i32 | v ≥ 0x1304 | Meaning open (OI-021). | ? |
| 24 | `has_box`, `box` | i32, Box | v > 0x130E | The mgBBox_c body (11.4) follows only when `has_box` is nonzero; `getBBox()`. | V |
| 25 | `last_open_times` | i32[3] | v > 0x2783 | `getLastOpenTime(index)` addresses the three members; units open (OI-021). | B |
| 26 | `license_type` | u32 | v > 0x2BC2 | `getLicenseType()`; enum domain open (OI-021). | B |
| 27 | `unknown_9c` … `unknown_b0` | i32[6] | v > 0x2FA3 | Active-content values returned by `getActiveContentFlag()`; individual meanings open (OI-021). | ? |
| 28 | `icon_3dx_override` | i32 | v > 0x336F | `sw3DXIconOverride`; `get3DXIconOverride()`. Enum domain open (OI-021). | B |
| 29 | `unknown_f0` … `unknown_104` | i32[6] | v > 0x3B0A | First five are returned by `getExtRefFlags()`; detailed meanings open (OI-021). | ? |
| 30 | `has_annotation` | i32 | v > 0x3B1F | `setHasAnnotation()`. | V |
| 31 | `has_graphics_feature` | i32 | v > 0x3DB8 | `setHasGfxFeature()`. | V |
| 32 | `unknown_98` | i32 | v > 0x3F01 | Meaning open (OI-021). | ? |

After `users`, the reader can temporarily replace v if the final string contains `SOLIDWORKS_INTERNAL_D95A9473_4FF5_42DF_A100_ECC502EF9F7D_HEADER_VERSION_` followed by a decimal version. It removes that marker string from the runtime user list and restores the original version after field 14. This branch is binary-derived; no corpus header exercises it. `modelheader.py` retains the stored string and records the effective embedded version rather than modifying the archived data.

SOLIDWORKS also has a runtime option to skip loading a header saved while the model exists. The complete-stream reader here operates with that shortcut disabled. A four-byte runtime-shortcut read is not evidence that the remaining payload is absent or undocumented padding.

#### 11.8.2 Header collections and revision logs

*Status: Verified layout; Partial log meanings · Evidence: [C-21], [C-22], F.11*

A `su_CStringArray` body is a ReadCount followed by that many CStrings. ReadCount is u16 unless it is `0xFFFF`, in which case the following u32 is the count. A `suObList` or `moExtObjectList_c` body is ReadCount followed by that many object records. All records use the archive-wide object/class map, including references to earlier objects; resetting the map between lists is incorrect.

`moLogs_c` first serializes its list of stamp objects (11.7.2), then i32 `id` and CString `unknown_50`. `getTheRevisionLog(id)` and `updateRevisionLog(id, stamp)` select logs by that ID. The CString's exact role remains open (OI-021). `moCStringHandle_c` contains a single CString; it serializes neither its runtime dirty flag nor a separate integer string index. Repeated handles use ordinary object references (11.5).

#### 11.8.3 External file references

*Status: Verified modern layout; Partial meanings · Evidence: [C-21], F.11*

The modern `moExtObject_c` body starts with a direct `moSharedFileDef_c` body. For v ≥ 0xFA7, that body holds `path` and `name` as object records of class `moCStringHandle_c`; before that threshold, they are direct CStrings. Next are `unknown_20` u16, `unknown_08` u8 and `timestamp` u32. A u8 `unknown_34` occurs only for 0x1427 < v < 0x146C. The tail is CString `unknown_38` [v > 0x1FB3], CString `imported_path` [v > 0x23D9], CString `pdm_id` [v > 0x2F82], and u8 `unknown_35` [v ≥ 0x3330]. The `getImportedPath()` and `getPdmId()` accessors identify the latter strings. Unnamed fields remain open (OI-021).

After the shared file definition, the external object holds u8 `flags`, u32 `timestamp`, i32 `unknown_48` [v > 0x62], i32 `unknown_4c` [v > 0x6C], then i32 `unknown_78` and a direct `moReferencedConfiguration_c` body [v > 0x8DD]. The configuration body is CString `name`, i32 `id`; it has no object tag. The remaining fields are i32 `unknown_54` [v > 0xAD0], u32 `last_modification_time` [v ≥ 0xFD2], and i32 `unknown_50` [v > 0x3F12]. All u32 timestamps here use su_CTime's archive representation.

This is the external-object layout for v ≥ 0xF18. The legacy `moObjFileDef_c` path, optional old fields and conversion behavior below that version are not ported or credited by the decoder (OI-021). The corpus's modern archive versions avoid that branch. The `flags` byte and every `unknown_` value remain excluded from semantic coverage.

### 11.18 Boundary entities, entity IDs and feature references

*Status: Verified selected scoped layouts; representation family incomplete · Evidence: [C-32], [C-33], F.20–F.21*

These layouts use the ordinary part/archive context and version range of 11.17. Old entity conversions, alternate-owner serialization, temporary chooser payloads and direct model data are not implemented. F.20 records the earlier alignment failures; F.21 documents their correction. Complete document/model-stream coverage remains open.

#### 11.18.1 Boundary-reference base and face suffix

*Status: Verified scoped layout; meanings and optional bodies partial · Evidence: [C-32], F.20*

`moBEntRef_c` and `moEdgeRef_c` share the following body. Entity IDs are direct bodies, not enclosing moEntityId_c object tags.

| Order | Fields | Types | Condition | Status |
|---|---|---|---|---|
| 1 | `unknown_60`, `unknown_64` | u32 × 2 | | ? |
| 2 | `entity_count` | u32 | | V |
| 3 | `unknown_e8`, `unknown_70` | u32 × 2 | | ? |
| 4 | `unknown_28`, `unknown_30` | f64 × 2 | | ? |
| 5 | vertex reference | object | Declared moVertexRef_c; null allowed. | V / ? |
| 6 | entity IDs | entity-ID body[entity_count] | Each body is one representation object pointer, 11.18.2. | V |
| 7 | `unknown_48` | u32 | Legacy count retained; its old array is not stored in this version range. | ? |
| 8 | `temporary_count` | i32 | Nonzero direct temporary contour-chooser payload is not implemented. | V / ? |
| 9 | `unknown_ec`, `has_direct_model_data` | i32 × 2 | v > 0x332C; nonzero presence introduces an unimplemented direct body. | ? |

The reader preserves raw words where the native loader normalizes type/state pairs, including unknown_64 when unknown_60 is 1. A face reference, `moFaceRef_c`, appends an i32 record count and that many direct 16-byte records: i32 unknown_08, u32 unknown_0c, u32 unknown_10, i32 unknown_14. This suffix is read only when archive condition 0x80 is absent; the scoped parser selects that path. Negative or out-of-bounds counts are rejected.

A vertex reference, `moVertexRef_c`, appends has_point i32 and, if nonzero, one direct Point body (three f64, 11.4). The point is not an MFC object pointer. This suffix applies to the scoped modern ordinary-part path [C-36], F.24. For drawings above v 0x3357 the native serializer additionally reads four persistent-ID words; those drawing fields and their context are not implemented by this part decoder. Null vertex references and references to existing objects still use normal MFC pointer tags.

#### 11.18.2 Entity-ID representations

*Status: Verified selected layouts; representation family incomplete · Evidence: [C-32], F.20*

The modern `moEntityId_c` direct body consists of one MFC object pointer, declared moEntityIdRep_c. The representation base contains an embedded feature keeper whose serialized body is **two object pointers**: the feature, then a moFR_c external feature reference. They use the same MFC map. The keeper's in-memory vtable and pointers are not raw on-disk pointer values.

| Representation class | Body after the two keeper pointers | Status |
|---|---|---|
| `moEntityIdRep_c` | None. | V |
| `moFromSktEntSurfIdRep_c`, `moFilletSurfIdRep_c` | unknown_38 i32; runtime tables select the same serializer. | ? |
| `moEndFaceSurfIdRep_c` | unknown_38 i32, unknown_3c i32. | ? |
| `moDerivedSurfIdRep_c` | Direct source entity-ID body, then unknown_48 i32. | V / ? |
| `moMirPatternSurfIdRep_c` | Direct source entity-ID body; no unknown_48 suffix. | V |

Additional modern classes verified by C-34 use the same two keeper pointers and the following suffixes. This table applies only to the ordinary archive context and v > 0x27CB; it does not specify their older conversion branches.

| Representation classes | Suffix after keeper | Status |
|---|---|---|
| moSurfaceIdRep_c | None. | V |
| moSimpleSurfIdRep_c, moSimpleEdgeIdRep_c, moSegmEdgeIdRep_c, moEndEdgeIdRep_c, moTopEdgeIdRep_c, moThinOffsetSurfIdRep_c, moBottomEdgeIdRep_c, moStartEdgeIdRep_c | unknown_38 i32. | ? |
| moNetSideSurfIdRep_c, moSweepSideSurfIdRep_c, moWzdHoleSurfIdRep_c, moUnBendSurfIdRep_c | unknown_38, unknown_3c i32. | ? |
| moEndFace3IntSurfIdRep_c, moFromSktEnt3IntSurfIdRep_c, moSideFace3IntSurfIdRep_c | unknown_38, unknown_3c, unknown_40 i32. | ? |
| moIndirectSurfIdRep_c, moShellSurfIdRep_c | Direct source entity-ID body. | V |
| moPLineSurfIdRep_c, moPunchSurfIdRep_c, moCirPatternSurfIdRep_c, moCopySurfIdRep_c, moPLineProjIdRep_c, moDerivedEdgeIdRep_c | Direct source entity-ID body, unknown_48 i32. | V / ? |
| moLPatternSurfIdRep_c, moDimPatternSurfIdRep_c, moPLineInterSurfIdRep_c | Direct source entity-ID body, unknown_48, unknown_4c i32. | V / ? |
| moMacroFeatRefSurfIdRep_c | Direct source entity-ID body, unknown_48, unknown_4c, unknown_50 i32. | V / ? |

These classes identify representation families; the meanings of their integer members remain open. Unknown subclasses SHALL fail explicitly. The family is not exhausted by these selected classes.

#### 11.18.3 External feature reference

*Status: Verified modern body; member meanings partial · Evidence: [C-32], F.20*

`moFR_c` stores a moExtObject_c external-model pointer, unknown_10 i32, and a su_CTime value represented by u32 on disk. The 8-byte in-memory time member does not imply an 8-byte serialized value. Later sharing, timestamp substitution and runtime flags do not add bytes in the scoped modern path. Legacy version branches are not implemented. `modelboundary.py` reads these records and rejects temporary chooser/direct model-data branches; no whole-stream coverage is awarded.

