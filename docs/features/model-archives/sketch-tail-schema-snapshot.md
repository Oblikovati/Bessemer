# Ordinary sketch tail and root schema reference

Edition 0.354 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.43 Equal-distance point records

*Status: Native-derived modern bodies; synthetic populated tests; native populated fixtures pending · Evidence: [C-58]*

After the pattern collections of 11.42, the sketch helper reads a tagged suObList pointer at runtime member +0x118. Its items are used as display dimensions. This list remains a dependency boundary for the current sketch-prefix parser; the following layouts do not authorize skipping its payload.

For versions greater than 0x1FF5, that pointer is followed by a u16 count and direct sgEquiDistPoints bodies, then another u16 count and direct sgEquiDistPointsCon bodies. Direct entries allocate no MFC map slot. Embedded handle pointers use the existing map.

| Body | Fields in wire order | Encoding |
|---|---|---|
| sgEquiDistPoints | CurrentEquiDistCons; Processed; ArrPos; MainEnt | three u16 values; sgEntHandle pointer |
| sgEquiDistPointsCon | logical G1 body (11.40); OwnerPos; AlongEntity; FirstPoint; MidPoint; LastPoint | inherited body; u16; sgEntHandle pointer; three sgPointHandle pointers |

The scoped reader requires the ordinary modern context v > 0x27CB. The native sgEquiDistPoints loader consumes Processed without retaining it in its runtime member, although the storing branch names and emits it. Readers SHALL retain this word for byte-faithful reconstruction. Owner/sketch assignment and relation rebuilding consume no additional fields in these bodies. Word domains and geometric semantics remain partial.

For v > 0x46CB, another u16-count direct sgLogGroupDim collection follows these two collections; its body is not specified here. Neither the display-dimension list nor these later collections have been integrated into the installed-corpus sketch checkpoint. Whole-stream coverage is unchanged.

### 11.44 Logical dimension groups

*Status: Native-derived modern body; synthetic populated test; native populated fixtures pending · Evidence: [C-59]*

For v > 0x46CB, the sketch helper stores a u16 count of direct sgLogGroupDim bodies after the two equal-distance collections. Each body consists of the logical dimension base of 11.40 (not logical G1), a u16 SizeofLogGroupArr, and that many signed i32 logicalRelArrIndex values. There is no per-entry MFC tag or pointer array.

Native loading inserts indices into a set and can remove duplicates; native storing iterates that set. Readers SHALL preserve the original wire sequence and duplicate values for byte-faithful reconstruction. Relation-index validity and the geometric meaning of each group remain partial. At versions at or below 0x46CB the enclosing collection is absent; this does not imply that a tagged object of this class can be safely decoded in an unsupported context.

The intervening display list remains unresolved. A corpus probe attempts its normal tagged-list decoder and stops on its first unsupported item: 29 distance-dimension objects and three radial-dimension objects across the 32 reachable sketches. These are populated dependencies, so empty-list framing cannot advance any of these checkpoints. The other six sketches retain their earlier boss/revolution stops. No whole-stream coverage is added.

### 11.63 Main sketch-tail offset collections

*Status: Native-derived direct collection order and sketch-offset body; 28 native group boundaries and one populated record verified · Evidence: [C-78]*

After helper 0x4C5AB5A0 returns, the main sketch serializer reads a u16 count of direct sgOffset_c bodies at runtime +0x48. These are distinct from the offset dimension bodies in 11.39. Their serializer is recovered but not yet implemented; nonzero counts stop explicitly. In modern scoped v > 0x27CB archives, a second u16-count collection follows at +0x58, containing direct sgSketchOffset_c bodies. Neither collection allocates an MFC slot for its direct entries.

sgSketchOffset_c begins with an i32 index count and that many stored u32 index values. When the count is positive, it next stores i32 word_38, i32 word_34, f64 value_40, two three-f64 points and u16 word_90. When the count is zero, that entire geometric block is absent. In both cases, a tagged sgPGMProfileData_c pointer follows, then i32 word_a4 and i32 word_a8. Readers SHALL preserve index bits, raw words, signed values and pointer identity. The scoped decoder rejects negative or impossible index counts. Nonnull unsupported profile-data bodies stop explicitly. Field semantics remain partial.

The audit completes both collections in 28 native cases, decoding one sketch-offset record with three indices. Three cases stop on sgOffset_c count two, and one stops on a nonnull sgPGMProfileData_c body. Six sketches retain their earlier feature-body stops. Cumulative positions over the 28 completed cases total 528,919 bytes; this cohort differs from the 32-case display-list audit. Later main sketch fields remain unread and whole-stream coverage is unchanged.

### 11.64 Ordinary offset and PGM profile bodies

*Status: Native-derived scoped bodies and populated synthetic tests; native nested dependencies remain · Evidence: [C-79]*

The sgOffset_c direct body in 11.63 is now specified for the ordinary modern context with native reference mode zero. It starts with a u16 entry count, then count tagged moSketchExtRef_w pointers, count u32 entity-type values, and count handle records. Each handle record contains a u16 presence discriminator; only value one introduces a tagged sgEntHandle pointer. Other values consume no handle pointer. Readers SHALL retain the raw discriminator and array sequence.

The remainder is:

| Order | Fields | Encoding |
|---|---|---|
| 1 | value_30; word_38; kind_4c; word_50 | f64; three u16 |
| 2 | dimension; word_60; word_68 | tagged object pointer; two u16 |
| 3 | word_64; word_c8; word_48 | three i32 |
| 4 | points_70/88 | two three-f64 points |
| 5 | indices; vector_b0; word_a8 | only kind_4c 7–9: count i32 values; three-f64 vector; i32 |
| 6 | kernel body | only kind_4c 8–9 and v > 0x2BBE; unsupported |

Other archive reference modes use native references or omit fields and remain outside scope. The scoped decoder stops explicitly before the unsupported kernel body. Field domains and geometric meanings remain partial.

sgPGMProfileData_c begins with a direct embedded suObArray (ReadCount and tagged object pointers), followed by u16 word_28 and f64 value_30. The direct array has no enclosing tag, presence integer or map slot. Readers SHALL preserve item sharing and raw suffix values. Populated element bodies require their own readers.

The updated native audit still completes 28 groups. The three former sgOffset_c stops now enter their bodies and stop on moExtrusion_c dependencies at 0x4BE5, 0x4D81 and 0x4BBF. The former profile-class stop now reaches sgPGMLoopOffsetData_c at 0x5CB9. These advances do not establish complete populated native offset/profile bodies. The six earlier feature stops remain, and whole-stream coverage is unchanged.

### 11.65 PGM loop-offset and edge data

*Status: Native-derived bodies; nested native profile chain completed; synthetic sharing/framing tests · Evidence: [C-80]*

sgPGMLoopOffsetData_c stores i32 word_8, i32 word_c, then a direct embedded suObArray body: ReadCount followed by tagged object pointers. No array presence integer, object tag or map allocation encloses this direct array. The contained items require supported class readers.

sgPGMLoopOffsetEdgeData_c instead stores u16 word_8 followed by a tagged suObArray pointer. This array can be null, newly allocated or already mapped. Its body uses the ordinary ReadCount/object-pointer sequence when newly allocated. Readers SHALL distinguish this tagged pointer from the direct array in the enclosing loop record and SHALL preserve raw words, item sequence and shared identities. The integer meanings and element domains remain partial.

Installing these readers only in the scoped sketch-tail archive resolves the previously blocked profile chain. The native audit now completes 29 offset groups with two sketch-offset records and seven indices, totaling 552,764 cumulative archive bytes over that cohort. Three populated sgOffset cases still stop on moExtrusion_c; the six earlier boss/revolution dependencies remain. Later sketch-tail fields are unread and whole-stream coverage is unchanged.

### 11.66 Styled curves and intersection collection boundaries

*Status: Native-derived styled body/early intersection marker; 29 native empty collection pairs verified · Evidence: [C-81]*

After the two offset collections in 11.63, the modern main sketch serializer reads a u16-count collection of direct sgStyledCurve bodies at runtime +0x60, then a u16-count collection of direct sgIntersection_c bodies at +0x50. These direct entries have no outer object tags or MFC map slots. The scoped range v > 0x27CB satisfies both collection version gates.

sgStyledCurve stores an i32 count, that many tagged sgEntHandle pointers, the direct width-two dynamic state array of 11.41, four i32 values in runtime offset order +0x30/+0x40/+0x34/+0x38, u16 word_3c, f64 value_10 and f64 value_8. Native loading can discard null handles in memory; readers SHALL preserve their wire positions and original count. Readers SHALL retain dynamic-array metadata and raw scalar values. State/scalar semantics remain partial. The scoped reader rejects negative or impossible handle counts.

sgIntersection_c starts with a u16 marker at runtime +0x68. Any nonzero marker ends the body immediately in the native load path. Zero introduces the populated reference/geometry body specified in 11.237. The ordinary tagged-reference decoder now implements that body; nondefault archive-reference modes remain outside its scope. Readers SHALL preserve the raw nonzero marker.

At the original C-81 checkpoint, all 29 reachable tail cases contained zero styled curves and zero intersections. Cumulative positions total 552,880 bytes, advancing four bytes per case from the offset checkpoint. That checkpoint provided native-code and synthetic evidence for populated styled curves and nonzero intersection markers; populated zero-marker intersection evidence is now retained in 11.237. At that checkpoint, three extrusion dependencies and six earlier feature dependencies persisted, and later main-sketch fields were unread; whole-stream coverage was unchanged. Later framing progress is recorded in 11.237.

### 11.67 Sketch state and creation IDs

*Status: Native-derived ordinary modern prefix; 29 native boundaries verified · Evidence: [C-82]*

After the intersection collection in 11.66, the supported ordinary v > 0x27CB main sketch stores u16 word_12c, u16 word_128, i32 word_130, then a direct sgCreationIDs body at runtime +0x140. It has no object tag or MFC map slot.

The modern sgCreationIDs body consists of 15 signed i32 fields in runtime member order +0x08, +0x0C, +0x10, +0x14, +0x18, +0x1C, +0x20, +0x24, +0x28, +0x2C, +0x30, +0x34, +0x38, +0x40, +0x3C. Readers SHALL preserve this wire order and signed values; member +0x40 precedes +0x3C. Individual counter meanings remain partial. Earlier version gates for the listed fields fall below the scoped range.

In the ordinary context with archive condition 2 absent, no temporary condition-2 mapping follows this body. The next stored values are u16 word_37c, word_188, word_18c and word_190, in that order. Readers SHALL retain raw word values rather than substitute native defaults or runtime maximum-counter updates.

All 29 reachable native tails advance through this 76-byte stage, bringing cumulative archive positions to 555,084 bytes. Three extrusion and six earlier feature dependencies remain. Later main sketch state is unread; this does not establish a complete sketch or increase whole-stream coverage.

### 11.68 Sketch line properties and user-object list

*Status: Native-derived ordinary modern prefix; 28 native boundaries verified; one populated user-object dependency · Evidence: [C-83]*

After word_190 in 11.67, legacy spline/arc compatibility work and old discarded payload branches add no bytes in the scoped v > 0x27CB ordinary context. The main sketch then directly serializes moLineViz_c with flag zero. Its modern body is a style-name CString, i16 standard width, f32 custom width, i32 color and i32 visibility. There is no line-properties object tag or map slot.

The following fields are i32 word_568, a three-f64 point at +0x570, u32 word_56c, i32 word_578, and an i32 count of tagged sketch-tools user objects. Readers SHALL preserve signedness, raw property values, object sequence and shared identities. The native loader can remove default line properties and discard user objects after checking their runtime behavior; those operations do not omit their serialized bytes. The scoped reader rejects negative or impossible counts and stops on unsupported user-object classes. Offset-named fields and user-object payload semantics remain partial.

The audit completes this stage in 28 cases, all with empty user-object lists, totaling 532,866 cumulative archive bytes over that cohort. One other previously reachable case stops on moExtensionLineIndicator_c at 0x5D5C. The prior three extrusion and six earlier feature dependencies remain. This total is not directly comparable to the previous 29-case cohort. Later main sketch fields remain unread; whole-stream coverage is unchanged.

### 11.69 Extension-line indicator user objects

*Status: Native-derived ordinary modern body; two native indicators verified · Evidence: [C-84]*

moExtensionLineIndicator_c first serializes its sketch-tools user-object base. In the ordinary context with archive conditions 1, 2 and 4 absent, that base contains one tagged sgSketch pointer. The modern v > 0x27CB indicator then stores u16 word_18. Earlier versions use a different legacy handle sequence, outside scope.

The sketch pointer can refer back to the enclosing sketch whose object-map placeholder was allocated before its body. Readers SHALL resolve that reference through the existing MFC map without recursively decoding a new sketch body, and SHALL preserve the raw indicator word. Alternate archive conditions derive the sketch owner at runtime and omit this pointer; those contexts are not handled by this reader. Indicator word meaning remains partial.

The native audit now completes the visual-state/user-object stage in all 29 currently reachable cases, including two indicator objects, totaling 556,808 cumulative archive bytes. Three extrusion and six earlier feature dependencies remain. The next direct moSketchRelMgr_c body at sketch +0x590 is unresolved; its recovered serializer delegates to two embedded members whose types still need identification. No complete sketch or whole-stream coverage is added.

### 11.70 Local sketch relation manager

*Status: Native constructor/serializer evidence; 29 empty native managers and following entity counts verified · Evidence: [C-85]*

After the sketch-tools user-object list in 11.68, the modern main sketch directly serializes moSketchRelMgr_c at runtime +0x590. Its wire body is a direct suObList member at manager +0x78, followed by a direct EquationVarValueMap_c member at +0x40. Neither member has an outer object tag or map slot. The manager's cache at +8 and runtime sketch pointer are not serialized by this procedure.

The list uses ReadCount and tagged object pointers. The equation-variable map uses ReadCount followed by CString name/f64 value pairs. Readers SHALL preserve pair order, duplicate names, stored doubles and object identities even though native map insertion can overwrite an earlier value for the same name. Name domains, equation semantics and relation-object payload classes remain partial. The map's version gate v >= 0xD7D is satisfied throughout the scoped v > 0x27CB range.

The main sketch then stores an i32 count of direct entity bodies at +0x108, each using the ordinary entity layout from 11.29 without a tag or map slot. Older count widths differ and are outside scope. The scoped reader rejects negative or impossible counts and retains explicit stops on unsupported nested payloads.

All 29 reachable native cases have empty relation lists, equation maps and following entity collections. Cumulative archive positions total 557,040 bytes, advancing eight bytes per case. Three extrusion and six earlier feature dependencies remain. Later main-sketch data remains unread; whole-stream coverage is unchanged.

### 11.71 Point-coincidence maps and region-list pointer

*Status: Native-derived ordinary modern framing; 29 native empty-map/null-region boundaries verified · Evidence: [C-86]*

After the entity collection in 11.70, the main sketch directly serializes pnt3DcoincMap_c at runtime +0x5D0. In the scoped v > 0x27CB range its body is four pair-map groups. Each group starts with an i32 count and then count pairs of signed i32 values. There are no object tags or MFC map slots for the groups or pairs. Native loading builds forward/reverse runtime maps from each pair sequence; readers SHALL retain the original sequence, duplicate keys and signed values rather than serialize those derived maps independently.

The group order corresponds to runtime map pairs +0x08/+0x58, +0x18/+0x48, +0x28/+0x38 and +0x68/+0x78. The fourth group is present from v >= 0x1C54, below the scoped range. Individual pair domains and geometric mapping semantics remain partial. The scoped reader rejects negative or impossible counts.

A tagged suObList pointer at sketch +0x5D8 follows in this modern range. This pointer is distinct from the preceding direct maps and does use the shared MFC map. Nonnull region-item classes require supported readers. All 29 native cases currently have four empty groups and a null region pointer, advancing 18 bytes per case to cumulative archive positions totaling 557,562 bytes. Later sketch fields and the nine earlier feature dependencies remain unresolved; whole-stream coverage is unchanged.

### 11.72 Late silhouette collection boundary

*Status: Native constructor/serializer evidence; 29 empty native collection boundaries verified; long silhouette body unsupported · Evidence: [C-87]*

After the region pointer in 11.71, versions v > 0x2F50 store a u16 count of direct sgSilhouette_c records at sketch +0x110. At or below that gate the entire collection is absent. Direct entries allocate no outer MFC map slot or class tag.

Each ordinary silhouette record begins with a u16 marker at runtime +0x28. A nonzero marker ends the native load body immediately. Zero opens a longer reference/handle/state/silhouette payload; this body is recovered but not implemented. Readers SHALL preserve nonzero markers and SHALL stop explicitly on the unsupported zero-marker form rather than infer an empty body. Alternate archive conditions also affect that longer form and remain outside the scoped decoder.

All 29 reachable cases store zero silhouette entries, advancing two bytes each to cumulative archive positions totaling 557,620 bytes. This reaches the end of the main loading branch only. The common continuation still serializes block, loop and slot managers, followed by later fields; those bytes remain unread. Three extrusion and six earlier feature dependencies persist. No complete sketch or whole-stream coverage is claimed.

### 11.73 Shared block-manager prefix and loop-manager body

*Status: Native constructor/serializer evidence; 29 empty block prefixes verified; loop body tested synthetically; intervening nested manager unread · Evidence: [C-88]*

After the main sketch load/store branches, the common continuation directly serializes sgBlockManager at +0x580, sgLoopManager at +0x598 and the slot manager at +0x588. Their native gates are respectively v > 0x910, v > 0xC52 and v > 0xF25, all satisfied by the ordinary v > 0x27CB scope here. These direct calls allocate no outer archive class or object slot.

The modern block-manager prefix is i32 block_count followed by block_count pairs of i32 key and tagged sgBlock pointer. Next is i32 value_count followed by value_count triples of i32 block_id, u32 key and u32 value. Readers SHALL preserve order, duplicate keys and all stored bits, including triples whose block_id is -1: native code consumes these triples even though it omits their runtime insertion. A further direct serializer for the manager's +0x40 member follows when v > 0xA0B. That nested body remains undecoded; the prefix reader SHALL stop before it and SHALL NOT present the prefix as a complete block manager.

The modern loop manager begins with i32 slot_count. Each slot starts with i32 presence. Zero ends that slot. Every nonzero presence is followed by i32 id, i32 geometry_count, geometry_count pairs of i32 geometry_type and i32 index, four i32 fields corresponding to curve +0x110 through +0x11C, i32 at +0x130, f64 at +0x138 and a tagged display-dimension pointer. The four fields are gated by v > 0xCCC, the last integer/double by v > 0x17D8 and the pointer by v > 0x17E3; all are present in this scope. Readers SHALL retain sparse slots and raw nonzero presence values. No additional loop-manager archive reads follow these records.

The installed corpus verifies 29 empty block prefixes, each eight bytes, at cumulative archive positions totaling 557,852 bytes. The loop body is supported by complete native recovery and synthetic sparse/populated fixtures, but has not yet been reached by the sequential corpus audit because the nested block-manager body intervenes. The nine earlier feature dependencies remain. Whole-stream coverage and complete-sketch status are unchanged.

### 11.74 Nested handle-data manager and slot-manager continuation

*Status: Native constructor/serializer evidence; 29 complete empty block, loop and slot managers verified; final archive-mode branch unresolved · Evidence: [C-89]*

The nested +0x40 member of the block manager in 11.73 is sgEntHandleDataManager_c. Its direct body is i32 count followed by count pairs of tagged sgEntHandle pointer and tagged sgEntHandleData_c pointer. Native loading consumes both pointers before deciding whether the handle's runtime index is -1 and discarding the associated data. Readers SHALL preserve these pairs and their order independently of runtime insertion or deletion. Unknown populated data objects remain explicit dependencies. This body completes the block manager; the ordinary loop manager then follows as specified in 11.73.

The direct slot manager at sketch +0x588 is sgSlotManager_c. Its loading body is i32 word_10, i32 count, then count pairs of i32 index and tagged sgSlot_c pointer. Readers SHALL preserve duplicate indices and original order. The subsequent native helper rebuilds runtime state without an archive argument. No further slot-manager reads follow. Populated sgSlot_c objects remain an unsupported dependency rather than an inferred empty payload.

After the slot manager, v > 0x1177 stores i32 sketch word_134. A final tagged moProfileFeature_c pointer is conditional on native archive field +0x834 being nonzero. The meaning and ordinary-file value of that flag remain unverified here. Readers SHALL stop before that unresolved branch unless its archive context is established; parsing the preceding word does not prove a complete sketch.

`--managers` verifies 29 empty nested handle-data and loop managers at cumulative archive positions totaling 558,084 bytes. `--slots` additionally verifies 29 empty slot managers and final word_134 fields, totaling 558,432 bytes. The nine earlier feature dependencies and whole-stream coverage remain unchanged. These observations supersede the intervening-manager stop in 11.73, while retaining the final archive-mode qualification.

### 11.75 Default archive-helper state and composed sketch bodies

*Status: Native helper-state evidence; 12 complete sketch bodies in seven configurations; additional dependencies remain · Evidence: [C-90]*

The archive +0x834 field in 11.74 belongs to moArchiveHelperObject_c. Its constructor initializes the field to zero; copyArchiveHelperObject copies the existing field without consuming stream bytes. It is distinct from the separately named isSerializingSketchInfo flag at +0x844. The semantic name of +0x834 remains unknown. Readers SHALL distinguish this runtime helper state from a stored stream flag and SHALL NOT consume an extra flag byte at the end of a sketch.

For a freshly constructed default helper, word_134 is the final stored sketch field: the conditional profile-feature pointer is absent. For a helper with nonzero +0x834, a tagged moProfileFeature_c pointer follows that word. A reader supporting this branch SHALL receive the actual helper state explicitly; an unspecified state SHALL produce an explicit stop rather than a pointer-presence guess. The composition here supports the ordinary default state, not arbitrary inherited or alternate archive contexts.

The opt-in through_sketch_body reader composes 11.29 through 11.74, including all shared managers and final fields, and returns only after the supported body completes. Earlier collection probes retain their explicit partial-body exceptions. Nested object tags use the same archive map, preserving sketch cycles and later references. Unsupported nested classes still stop the containing reader, and exceeding the resolved section boundary is an error.

Sequential traversal of 906 modern part configurations completes 875 relation-manager prefixes, including 12 complete sgSketch objects in seven configurations. The other 31 configurations stop explicitly on feature, handle, backup, slot or relation-function dependencies. Earlier audits stopped at the first sketch; the composed traversal now exposes later objects and later sketches without scanning or skipping payloads. This establishes supported object boundaries, not complete configuration streams or semantic coverage of every field. Whole-stream coverage remains unchanged.

### 11.80 Dimension entity-help and sectioned offset records

*Status: Native serializer/constructor evidence; populated records traversed in conditional raw-kernel audit · Evidence: [C-95]*

In the ordinary v > 0x27CB scope, sgDimEntityHelpData_c stores f64 value_08, f64 value_10 and u16 word_18. Native loading gates the word on v > 0x9F7, already satisfied here. The field meanings remain partial; readers SHALL retain all stored bits and SHALL NOT promote the word to the wider runtime integer size.

sgPGMSectionedOffsetEdgeData_c stores the loop-offset-edge prefix: u16 word_8 and a tagged suObArray pointer. It then directly serializes the embedded suObArray at runtime +0x18: packed ReadCount followed by tagged objects. The embedded array has no outer class tag or object slot. Readers SHALL distinguish the first tagged array pointer from the second direct array body. A nonnull first pointer must identify a supported suObArray-derived object; unsupported classes SHALL cause an explicit stop.

sgPGMSectionHandles_c stores two tagged sgSegmentHandle pointers followed by a tagged suObArray pointer. The first two pointers can resolve to supported derived handle classes or shared archive objects. Readers SHALL retain null and repeated-reference identity. A nonnull third pointer is checked as a supported suObArray object. There are no additional direct collection counts or scalar fields in this record.

With the explicit raw-kernel variant from 11.79, the vessel fixture now completes its relation-manager prefix, bringing that conditional audit to 893 successful prefixes, 79 sketches and 25 configurations containing sketches. Thirteen configurations still stop on feature-body dependencies. The default audit remains at 892 prefixes and 77 sketches because kernel stream context is unspecified there. The 771-byte kernel payload remains opaque; these conditional object-boundary results do not prove complete geometry decoding or whole-document bit coverage.

### 11.237 Populated sketch intersections and loop references

*Status: B / ? — ordinary tagged-reference zero-marker bodies and ten populated native records verified; raw field meanings, identity-address modes and historical variants remain partial.*

This clause completes the zero-marker path introduced in 11.66. It applies to ordinary external-XML model archives with file version greater than `0x27CB`, ordinary archive condition flags zero and helper field `+0x834` zero. `sgIntersection_c` selects serializer `0x4C593CC0` in `sldmodu.dll`. In the main sketch collection it is a **direct** body, without an additional tagged object header or object-map slot.

Read the initial u16 invalid marker at runtime `+0x68`. Any nonzero value ends this body immediately, preserving the original value. When zero, the modern tagged-reference payload continues:

1. u32 flags at `+0x6C`, i32 base-entity/handle count, i32 last-edge count at `+0x58`.
2. Four tagged `moRefWrapper_c` pointers at `+0x10`, `+0x18`, `+0x20`, `+0x28`.
3. Two tagged `bentRefArr` pointers at `+0x30`, `+0x38`.
4. The stated number of tagged `sgEntHandle` objects, associated with runtime array `+0x40`. A nonpositive signed count consumes zero entries.
5. Direct dynamic DWORD-word groups at runtime pointer `+0x50`, using helper `0x4BB1DA60`: u32 element width; width zero ends the array, otherwise two u32 allocator metadata words, u32 group count, and `width × count` bytes. Nonzero width is divisible by four. This array's group count is independent of the earlier handle count.
6. One **direct** `moIntersectionEdge_c` body per positive handle count, associated with runtime array `+0x48`. These embedded edges are not separately tagged or mapped.
7. One u8 transform-presence value; if nonzero, direct compact `mgXform_c::restore` data at runtime `+0x60`, using the existing optional-matrix/translation/scaling/flags grammar. This is not the 30-word archive operator used by split items.
8. If version is greater than `0x2B71`, two u16 words at `+0x70`, `+0x74`, a tagged `suObList` pointer at `+0x78`, then tagged `sgArcHandle` pointer at `+0x80`.
9. If version is greater than `0x2B98`, four u16 words at `+0x88`, `+0x8C`, `+0x90`, **`+0xA0`**, a tagged `moLineRef_w` pointer at `+0x98`, then an i32 point count and that many direct `mgPoint_c` values (three binary64 components each). A nonpositive count consumes zero points. Retain the raw sequence rather than substituting native runtime uniqueness processing.
10. If version is at least `0x2BE0`, one binary64 at `+0xB0`.

The native handle reader constructs temporary handles and assigns their owning sketch before copying them into runtime storage. Sketch-owner updates, reference-use increments and temporary destruction do not add wire fields. The diagnostic view preserves the consumed tagged objects and shared-map identities. Raw flags, option words, allocator metadata, edge fields, point values and final double remain unresolved for semantic coverage accounting.

`bentRefArr` selects `0x4C58CA60`. In this tagged-reference context it delegates to `suObArray::Serialize`: ReadCount followed by tagged object pointers. Operator `0x4C585750` independently confirms that an enclosing `bentRefArr*` is an ordinary tagged `ReadObject` pointer, not an inline nullable-array marker.

`moIntersectionEdge_c` selects `0x4C58CE80`. After the empty object base it writes five i32 values at `+0x158`, `+0x15C`, `+0x160`, `+0x164`, `+0x168`, then four direct reference groups in runtime-field order `+0x38`, `+0x80`, `+0xC8`, `+0x110`. Group helper `0x4C58CC20` writes a u16 at relative `+0x00`, three i32 words at relative `+0x20`, `+0x24`, `+0x28`, then tagged `moLoopRef_c`, `moEdgeRef_c`, `moEdgeRef_c` pointers at relative `+0x08`, `+0x10`, `+0x18`. Runtime group size and the edge's `0x180` allocation size are not wire lengths.

`moLoopRef_c` selects `0x4BBBD230`: the established boundary-reference serializer `0x4BBB6F40`, followed by one u32 at runtime `+0xF0`. Native ancestry includes `moBEntRef_c`. The modern scope satisfies this suffix's historical version gate after `0x65C`.

Archive identity-address modes are distinct: the same native functions can write raw 64-bit reference values in place of tagged pointers, and `bentRefArr` then contains counted 64-bit entries. The selector depends on helper conditions 2, 1 (or the zero-condition predicate when helper `+0x834` is nonzero), `0x20`, `0x200000` and `0x8000000`. `hasCondition(0)` tests whether all archive condition flags are zero; it is not a bitwise test with mask zero. The recorded ordinary flags-zero/helper-zero evaluation selects tagged references. The decoder rejects nondefault sketch helper mode and does not infer identity-address framing from plausible-looking bytes. Condition-4 processing and historical default/owner migrations remain outside this scope.

Historical gates below the implemented modern minimum include flags after `0xDB7`, third/fourth wrapper references after `0x4F5`, explicit transform presence after `0xAE4`, the first edge word after `0x73A` and group u16 word after `0x736`. The later tail gates above are implemented at their exact boundaries.

The five affected configurations contain **ten complete zero-marker intersection bodies totaling 22,044 bytes**, inside five complete profile roots. The fixtures exercise one- and two-handle cases, corresponding direct edge records, independent dynamic word groups, shared references and present compact transforms. Exact body and containing-root spans/hashes are retained. Both side-part feature lists complete; the Mouse housings and bike frame continue to newly exposed identifier, plane-constraint or weld-corner dependencies. These are framing results, not a claim of whole-document or all-installed-project bit coverage.

The 906-context audit now contains **25,105 complete roots**, **799 complete lists** and **107 explicit stops**, with every preceding complete root record unchanged. This adds 85 roots and two exact-EOF lists totaling **537,432 decoded bytes**: the simulation FloWorks side part (268,856) and tutorial ball-valve side part (268,576). Totals are **726 exact-EOF lists / 56,402,792 decoded bytes**; 73 complete lists retain four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.256.json` and `resolved-feature-list-frontier-0.257.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 865 regression tests pass (111.705 seconds); specification lint reports zero findings.

Evidence: C-279.

