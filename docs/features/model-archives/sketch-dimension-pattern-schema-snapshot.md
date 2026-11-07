# Sketch dimension and pattern schema reference

Edition 0.353 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.37 First sketch valued-dimension collection

*Status: Verified scoped modern bodies and 40 installed dimensions; remaining dimension/constraint collections partial · Evidence: [C-52], F.40*

After the five surface collections, sketch helper 0x4C5AB5A0 reads a **u16** count for the first valued-dimension collection. Each entry is an MFC object pointer expected to derive from sgValG1Dim. Unlike the preceding direct geometry collections, these entries use the shared class/object map. The supported concrete classes are sgValG1Dim and sgCircleDim, which share a serializer. Null entries and other unimplemented classes SHALL cause explicit stops.

The sgDim base begins with the Dimension_c body of 11.28, then stores i32 DcmType, u16 ConstraintStatus, u16 Invalidated and u16 Dangling. The native loader normalizes nonzero Invalidated values to 1 in memory. Readers SHALL retain the stored word. Legacy driven/temporary-transform fields are absent in the supported v > 0x27CB scope; the helper used for their gates reads the same archive version field.

The valued base appends a u16 plane-transform presence word and, when nonzero, a direct mgXform_c. The sgValG1Dim/sgCircleDim suffix then stores two sgEntHandle object pointers (Entity0 and Entity1), i32 ArcDimType0, i32 ArcDimType1 and one three-f64 vector at member 0x188. There is no direct sgSketch pointer suffix in this body. Native loading assigns sketch owners and copies/deletes temporary handle objects in memory; those operations consume no further bytes. Readers SHALL preserve archive map identities instead of treating the handles as direct untagged records.

Names follow save keys; type-code domains and the vector's geometric role remain partial. Referenced parameter/tolerance/attachment payloads retain the limitations of 11.28. This stage ends immediately before the next valued-dimension collection and does not constitute a complete sketch reader.

### 11.38 Second valued dimensions and tolerance payloads

*Status: Verified scoped G2/angle/tolerance layouts and 541 installed dimensions; field domains and later collections partial · Evidence: [C-53], F.41*

The first valued-dimension collection is followed by a u16 count and tagged sgValG2Dim-derived object pointers. Each supported entry begins with the valued base of 11.37, then two sgEntHandle pointers, four u16 words at members 0x180, 0x184, 0x1B8 and 0x1BC, an sg3DPlaneHandle pointer and a three-f64 vector at 0x1C8. The save keys for these fields are Entity0, Entity1, ArcDimType0, ArcDimType1, Quadrant, Chamfer, ActivePlane and Direction. Enum domains remain partial. Readers SHALL preserve stored Quadrant zero even though native loading replaces it with 1.

For `0x2F79 < v < 0x2FA0`, one additional u16 follows the vector. It is absent outside that interval. Shared modern bodies are confirmed for sgValG2Dim, sgLLDist, sgPntPntDist, sgPntLineDist, sgPntPntHorDist and sgPntPntVertDist. sgAnglDim delegates to the same serializer and adds no modern bytes; its remaining code normalizes runtime state. Null collection entries and unsupported classes SHALL stop explicitly. Nonnull plane-handle payloads still require a class reader.

The Dimension_c tolerance pointer of 11.28 can reference moDimTolerance_c. In the supported v > 0x27CB ordinary context its body is:

| Order | Fields | Encoding |
|---|---|---|
| 1 | Type; FitType; Max; Min | i32; i32; f64; f64 |
| 2 | UseTextScale; TextScale; unknown_88; Height | u8; f64; f64; f64 |
| 3 | UString; LString; ShowParenthsis | CString; CString; u8 |
| 4 | FitDispStyle; FitUseTextScale; FitTextScale; unknown_a8; FitTextHeight | i32; u8; f64; f64; f64 |
| 5 | unknown_5c; unknown_60; unknown_b8 | u8; u8; i32 |
| 6 | TolUppertLimit; TolLowerLimit; TolGeneralLimit | three CStrings, only for v > 0x2B6D |
| 7 | Mask | i32, only for v > 0x2B76 |

The native save-key spellings are retained above. Type/FitType and display-style domains, text unit conventions, the two unnamed scale-adjacent doubles and the mask meanings remain partial. Readers SHALL retain raw bytes/values without inferring booleans from nonzero u8 values. This modern body has no additional serialized base prefix. Older tolerance versions and later sketch constraint collections remain outside the implementation.

### 11.39 Third valued dimensions and offsets

*Status: Native-derived G3 layout with empty installed collections; 11 installed offset dimensions verified · Evidence: [C-54], F.42*

The second valued-dimension collection is followed by a u16 count and tagged sgValG3Dim-derived objects. The supported sgValG3Dim body starts with the valued base of 11.37, then three sgEntHandle pointers (Entity0, Entity1 and Entity2) and a u16 Quadrant. The loader assigns owners and normalizes zero Quadrant to 1 in memory. Readers SHALL retain the stored word and shared MFC pointer identities. Additional G3 subclasses remain unsupported.

The next modern collection is a u16 count of tagged offset-dimension objects. sgOffsetDim and sgSkOffsetDim share the valued base of 11.37 followed by u16 OffsetIndex and u16 OffsetOk. The two words are not entity pointers or i32 fields. Their index/status domains remain partial, and readers SHALL preserve noncanonical stored values rather than infer boolean normalization. Null entries and unsupported collection classes SHALL stop explicitly.

This clause applies to the ordinary modern part context with v > 0x27CB. A separate arc-length collection found in legacy loader versions 0x300 through 0x30C is absent in this scope. No legacy count is consumed between modern G3 and offset collections. Neither collection completes the remaining sketch relation/logical-dimension data.

### 11.40 Direct logical-dimension collections

*Status: Verified modern shared bodies and 480 installed logical records; roles, domains and later constraints partial · Evidence: [C-55], F.43*

The offset collection is followed by logical G1, G2 and G3 collections, in that order. Each starts with a u16 count and direct bodies. Unlike the valued-dimension collections, these direct entries have no enclosing MFC object tag or map slot. Their referenced handles and help objects still use the shared tagged object map. Readers SHALL preserve that distinction.

The logical base is sgDim (11.37), followed by u16 RefIndex and u16 Derived. It does not contain the valued base's plane-transform presence word. The logical suffixes are:

| Class | Fields after logical base | Encoding |
|---|---|---|
| sgLogG1Dim | Entity0 | sgEntHandle pointer |
| sgLogG2Dim | Entity0; Entity1 | two sgEntHandle pointers |
| sgLogG2Dim (continued) | TangentCondition0; TangentCondition1; HelpData; word_100 | u16; u16; su_CObArray pointer; u16 |
| sgLogG2Dim (continued) | unknown_110; unknown_114; unknown_118; unknown_120 | i32; i32; f64; f64 |
| sgLogG2Dim (continued) | external handles 1, 2 and 3; word_150 | three sgEntHandle pointers; u16 |
| sgLogG2Dim (continued) | LogGroupDimIndex | i32, only for v > 0x46CB |
| sgLogG3Dim | Entity0; Entity1; Entity2; external handles 1, 2 and 3 | six sgEntHandle pointers |

The scoped ordinary modern range is v > 0x27CB. Other legacy G2 branches consume additional data below this range and are not inferred from current bytes. RefIndex, Derived and all offset-named values remain stored values; reference domains, tangent codes, help-data subclasses and logical-group semantics remain partial. Readers SHALL retain raw nonboolean word values rather than replace them with native normalized state. Owner assignment, temporary-handle copying and runtime migrations do not introduce extra modern bytes.

This stage ends after the three logical collections. Later parallelism/pattern/group constraints and the sketch tail remain outside the reader; partial logical prefixes grant no complete-sketch coverage.

### 11.41 Isoparametric constraints and linear patterns

*Status: Native-derived modern bodies and synthetic populated cases; installed empty boundaries verified · Evidence: [C-56], F.44*

After the three logical collections, the sketch stores a u16 isoparametric count and direct sgIsoParam_c bodies. Each starts with the logical base of 11.40, then i32 KeepLastInd, i32 LastNumSegs, i32 Id, i32 Flags and a direct segment-state array. These entries have no enclosing MFC tag or map slot. Field names follow save keys; index domains, flag bits and segment-state values remain partial.

The segment-state array starts with u32 element_width. Zero ends the array immediately. A nonzero width is followed by two u32 metadata words, a u32 count and `element_width * count` payload bytes. The implemented typed branch requires element_width 2 and restores count u16 state values; other widths stop explicitly. Readers SHALL retain both metadata words and raw state values and SHALL reject out-of-buffer counts. The native generic helper rejects odd widths; that alone does not establish other even widths as valid typed segment states.

The next collection is a u16 linear-pattern count and direct sgLinearPattern bodies. Their base is distinct from the dimension base: i32 PattHeadPos, i32 NumVisibleEnts, i32 isCntDimAlwaysVis and a moNotePattern_c pointer. In the supported modern range, the suffix is:

| Order | Linear-pattern fields | Encoding |
|---|---|---|
| 1 | NumX; NumY; UserNumX; UserNumY; PattArrPosX; PattArrPosY | six u16 values |
| 2 | PattDirection | three-f64 vector |
| 3 | ConstructionLine0; ConstructionLine1; HadConstructionLines | two sgLineHandle pointers; u16 |
| 4 | IsEdgeForDir1; IsEdgeForDir2; EdgeDir1; EdgeDir2 | two u16 values; two moLineRef_w pointers |
| 5 | count dimension | sgLinearPattCntDim pointer |
| 6 | unknown_f0; word_d8 | f64; u16 |

All pointer fields use the shared MFC map; direct pattern entries do not allocate their own slots. Nonnull note, line-reference and count-dimension payloads require their own supported class readers. Counts, indices, flags and geometric domains remain partial. Readers SHALL preserve the stored words and floats without guessing boolean domains. The layouts use the v > 0x27CB ordinary part context; legacy pattern fields and alternate DB-stream MsgPack carriers are outside scope. Later pattern constraints and the sketch tail remain unread.

### 11.42 Pattern constraints and circular patterns

*Status: Native-derived modern bodies and synthetic populated cases; 32 installed empty collection groups verified · Evidence: [C-57], F.45*

After the linear-pattern collection, the modern sketch stores three u16-count direct collections in order: linear pattern constraints, circular patterns and circular pattern constraints. Direct entries have no enclosing MFC object tag or map slot. Their embedded pointers continue to use the shared archive map.

sgLinearPattCon and sgCircularPattCon share the logical G2 body of 11.40 followed by six u16 fields: PatternPos, NextPattConPos, LastPattConPos, ThisPattConPos, PatternMultiplier and AutoPatternPoints. Two i32 fields, XScale and YScale, follow. Despite their names, these scale fields are four-byte signed integers in this schema. Readers SHALL retain their stored widths and raw values. Index chains, multiplier domains and scaling conventions remain partial.

sgCircularPattern uses the distinct pattern base of 11.41, then:

| Order | Circular-pattern fields | Encoding |
|---|---|---|
| 1 | PattArrPos; word_9c; NumInstances; UserNumInstances | four u16 values |
| 2 | PattCentre | three-f64 point |
| 3 | ConstructionLine0; ConstructionLine1; HadConstructionLines | two sgLineHandle pointers; u16 |
| 4 | ConstructionCircle; HadConstructionCircle; isPolygon | sgArcHandle pointer; two u16 values |
| 5 | PointToUseForCentre; HadPointToUseForCentre | sgPointHandle pointer; u16 |
| 6 | IsEdgeForCenter; CircleRef; CntDim | u16; moCircleRef_w pointer; sgCircularPattCntDim pointer |

All listed fields are present in the scoped ordinary modern v > 0x27CB context. Native owner assignment and construction-handle copying introduce no additional bytes. Nonnull circle-reference/count-dimension payloads still require supported readers. Readers SHALL preserve stored words rather than infer boolean normalization. Geometric/index domains and word_9c remain partial. The subsequent display-dimension list and later constraints are outside this stage.

### 11.80 Dimension entity-help and sectioned offset records

*Status: Native serializer/constructor evidence; populated records traversed in conditional raw-kernel audit · Evidence: [C-95]*

In the ordinary v > 0x27CB scope, sgDimEntityHelpData_c stores f64 value_08, f64 value_10 and u16 word_18. Native loading gates the word on v > 0x9F7, already satisfied here. The field meanings remain partial; readers SHALL retain all stored bits and SHALL NOT promote the word to the wider runtime integer size.

sgPGMSectionedOffsetEdgeData_c stores the loop-offset-edge prefix: u16 word_8 and a tagged suObArray pointer. It then directly serializes the embedded suObArray at runtime +0x18: packed ReadCount followed by tagged objects. The embedded array has no outer class tag or object slot. Readers SHALL distinguish the first tagged array pointer from the second direct array body. A nonnull first pointer must identify a supported suObArray-derived object; unsupported classes SHALL cause an explicit stop.

sgPGMSectionHandles_c stores two tagged sgSegmentHandle pointers followed by a tagged suObArray pointer. The first two pointers can resolve to supported derived handle classes or shared archive objects. Readers SHALL retain null and repeated-reference identity. A nonnull third pointer is checked as a supported suObArray object. There are no additional direct collection counts or scalar fields in this record.

With the explicit raw-kernel variant from 11.79, the vessel fixture now completes its relation-manager prefix, bringing that conditional audit to 893 successful prefixes, 79 sketches and 25 configurations containing sketches. Thirteen configurations still stop on feature-body dependencies. The default audit remains at 892 prefixes and 77 sketches because kernel stream context is unspecified there. The 771-byte kernel payload remains opaque; these conditional object-boundary results do not prove complete geometry decoding or whole-document bit coverage.

### 11.239 Sphere backups and 3D point-to-plane dimensions

*Status: B / ? — native dispatch, fixed backup layout and one complete containing profile verified; scalar semantics and further axis dependencies remain partial.*

`mo3DSphereBackedUpData_c` selects serializer `0x4C279E00` in the hash-bound installed `sldmodu.dll`. Its RTTI ancestry includes `mo3DSurfaceBackedUpData_c`, `moBackedUpData_c` and `su_CObject`. The selected serializer calls the empty object base, then stores or restores a direct `mgPoint_c` at runtime `+0x08` (three binary64 components), followed by one binary64 at runtime `+0x20`. The wire body occupies exactly **32 bytes**, with no presence word, object tag or version gate inside this body. The decoder retains `point_08` and `raw_20`; coordinate units and the scalar's engineering domain remain unresolved. The native sample's value 0.04 alone does not establish its semantic type.

`sgPoint3DPlaneDist` derives from `sgValG2Dim` and selects the identical serializer `0x4C5A9750`. It adds no subtype suffix. The existing valued two-entity dimension body therefore applies: common valued-dimension data, two tagged entity handles, the established version-gated u16 fields, tagged 3D-plane handle, vector and legacy word. Only this proven subtype is added to the second dimension collection's accepted classes. Native geometry and dimension updates after loading add no archive fields.

`sg3DSphereHandle` derives through `sg3DSurfaceHandle` from `sgEntHandle`. Both select `0x4C591ED0`, also selected by the established plane handle. In the supported ordinary modern binary context its body is a u16 entity index, an additional i32 index only when the u16 is `0x777F`, then i32 reference ID and i32 dimension-on-CM value. Thus the direct body occupies 10 or 14 bytes; surrounding tagged-object framing is separate. Sphere handles add no suffix. Database and historical modes remain outside this reader's scope.

Golf Ball's `Contents/Config-0-ResolvedFeatures` now parses the complete `mo3DProfileFeature_c` root from **26,235 through 31,885**, a **5,650-byte** span. Source-stream and root SHA-256 values, the sphere backup values and fresh dimension/handle counts are retained in `sphere-backup-targeted-corpus.json`. The next root stops explicitly at unsupported `moFacePtAxisData_c`, diagnostic cursor 32,224; no partial next root is accepted. The previous constrained-plane checkpoint remains reproducible at Edition 0.258's boundary.

The 906-context audit now contains **25,139 complete roots**, **807 complete lists** and **99 explicit stops**, with all earlier complete root records unchanged. This adds the Golf Ball profile and one final root in each of the two routing Filter part variants. The shared point-to-plane dimension alias lets both Filter lists reach exact EOF (149,530 and 149,529 decoded bytes), adding **299,059 bytes**. Totals are **730 exact-EOF lists / 57,915,377 decoded bytes**, with **77 complete lists retaining four-byte unknown tails**. Checkpoints `resolved-feature-list-frontier-0.258.json` and `resolved-feature-list-frontier-0.259.json` preserve the comparison; exact EOF does not establish complete semantic bit coverage.

Two focused tests exercise exact binary64 bit preservation (signed zero, NaN payload and infinity), truncation at field boundaries, context rejection, hash-bound native profile parsing, nested dependency counts and the next explicit stop. Whole-document semantic bit coverage, unresolved scalar domains and all-installed-project schemas remain open; these framing results earn no additional whole-document coverage credit.

Validation: all **870 regression tests pass** (127.302 seconds); specification lint reports zero findings (463 headings, 96 anchors).

Evidence: C-281.

