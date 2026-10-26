# Component and boundary reference schema snapshot

Source: `../inventor-interface/docs/solidworks-file-format/specification.md`

Edition 0.326 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

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

