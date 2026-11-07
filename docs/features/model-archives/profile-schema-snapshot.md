# Complete ordinary profile schema reference

Edition 0.358 · 2026-10-26 · generated parts: Annex A (solidworks/swstreams.py), Annex G (solidworks/swcover.py)

### 11.82 Profile-feature prefix and reuse dispatch

*Status: Native serializer evidence; 12 installed nested profile prefixes verified; profile continuation incomplete · Evidence: [C-97]*

The primary moProfileFeature_c serializer first reads the model-feature base from 11.81. Its modern prefix then stores i32 at +0x388, a tagged pattern-feature reference and i32 reuse flag at +0x3D4. Native version gates are v > 0x944, v > 0xA9B and v > 0x844 respectively, all satisfied in the ordinary v > 0x27CB scope. The writer labels the first integer AbsoluteRefPlaneByProj8000; its full semantics remain partial. Readers SHALL preserve its signed representation and the raw reuse value, not replace either integer with a byte-sized Boolean.

A nonzero reuse value selects a reused-profile path: tagged moProfileFeature_c source, tagged contour chooser, u16 at +0x3B8 and tagged line handle, followed by an additional common profile helper. Zero selects the ordinary profile path, which for ordinary archive conditions next reads a tagged sgSketch and additional profile-state helpers. Other archive conditions affect the preceding or subsequent geometry state. These continuations remain outside the prefix implementation; recognizing the reuse branch does not establish a complete profile body.

The partial probe raises ProfileFeatureStop immediately after the three modern prefix fields and never returns a completed profile object. All 12 previously blocked boss/extrusion cases reach this boundary sequentially. They store zero reuse values and null pattern-feature references. Native positions and file/stream identities are retained in the corpus report; nested object mapping is preserved throughout inherited feature parsing. The other unresolved default cases remain the revolution feature and the kernel-context stop. Successful whole manager-prefix counts remain 892, and whole-document bit coverage is unchanged.

### 11.83 Profile-owned sketches and chain collection framing

*Status: Native helper evidence; ten installed profile/sketch boundaries verified; populated chain objects unsupported · Evidence: [C-98]*

For the ordinary non-reused profile branch in 11.82, the next stored object is a tagged sgSketch pointer. The profile reader and sketch reader share the same archive map, including placeholders allocated before object bodies. Readers SHALL preserve references to already loaded sketches and cyclic owners. Reading this sketch does not complete the containing profile feature.

For scoped v > 0x27CB, the following helper stores u32 word_2B0, i32 chain_count and chain_count tagged moSketchChain_c pointers. This modern count is a signed four-byte integer, not packed ReadCount. Readers SHALL retain the stored pointer sequence, including duplicates; native insertion into a map is subsequent runtime reconstruction. Negative counts are rejected by the scoped reader. Unknown chain objects remain explicit dependencies. Earlier versions use different loop-map framing and are outside this implementation.

The main profile serializer then invokes a geometry-state helper and a contour-state helper before its next line-handle pointer. Complete recovery of these helpers is retained, but their full payloads are not implemented. The profile/sketch probe therefore always stops after the selected verified stage; it does not return a partial profile as a whole object.

Across 906 modern configurations, the sketch-stage audit reaches ten completed profile-owned sketches, then raises explicit profile partial stops. Two of the twelve prior profile cases instead encounter moCompRefPlane_c inside their sketches. The separate chain-stage audit reaches populated moSketchChain_c dependencies in all ten remaining cases, so no complete chain collection is claimed from native fixtures. Existing default manager successes remain 892. These probes extend nested-object evidence without changing whole-document coverage.

### 11.84 Profile sketch-chain bodies

*Status: Native serializer/constructor/import evidence; 13 chain entries in ten installed collections verified · Evidence: [C-99]*

In ordinary v > 0x27CB archives, moSketchChain_c begins with two direct su_CDWordArray bodies at runtime +0x08 and +0x60. Each stores packed ReadCount followed by count u32 values, with no outer object tag. The serializer then reads u32 word_28, i32 word_38, i32 word_50, i32 word_54, u32 word_58, u32 word_5C, CString string_80 and a u16 handle mask. Readers SHALL preserve the signed/unsigned distinction and raw DWORD values.

The handle mask must be less than four. Bit 0 adds a tagged sg3DPlaneHandle pointer; bit 1 adds two tagged sgPointHandle pointers. Next is u32 word_D0, u32 pair_count and pair_count pairs of u32 key/u32 value. Two tagged contourTrimData_c pointers follow, then i32 word_90, i32 word_A0 and i32 word_A4. Modern version gates are v > 0xB95 for word_D0, v > 0xC8D for the map and trim pointers, v > 0xDDC for words_90/A0 and v >= 4000 for word_A4, already satisfied by this scope. Unsupported nested handle or trim classes remain explicit dependencies. Readers SHALL preserve pair order, duplicate keys and pointer identity independently of native map reconstruction.

The embedded DWORD-array serializer calls ReadCount, allocates its element array and reads count * 4 raw bytes. Readers SHALL bound counts against remaining data before allocating. This packed framing differs from the i32 count of the containing profile chain collection in 11.83.

The installed chain-stage audit now completes all ten reachable collections, containing 13 chain entries, and raises its explicit partial-profile exception after the collection. Two other profile cases still stop inside their sketches on component reference-plane objects. The profile geometry/contour helpers and containing boss/extrusion bodies remain incomplete. Default successful manager-prefix count stays at 892; whole-document bit coverage is unchanged.

### 11.85 Suppressed profile geometry and contour maps

*Status: Native helper evidence; initial boundary interpretation corrected in 11.86; present geometry unsupported · Evidence: [C-100], [C-101]*

After the chain collection, the ordinary default-helper profile geometry-state helper reads i32 word_10 and calls the kernel-body archive helper from 11.79. A nonzero suppression byte means no restored geometry. In that case the modern continuation is i32 pair_count followed by pair_count pairs of u32 key/u32 value, then the two reference collections specified in 11.86. From v >= 0xC75 these collections are unconditional, including the suppressed-body case. The scoped reader bounds the count and preserves duplicate keys and original order. Readers SHALL retain the suppression byte and signed word_10 without inventing a Boolean field width.

For present geometry, subsequent native reads depend on whether the kernel loader actually restored a body or reported a skipped-body condition. They can include reference arrays, additional body records and context-dependent state. A framed opaque payload does not establish these runtime outcomes. Readers SHALL stop explicitly when that body-validity context is unavailable. The implemented helper therefore supports the suppressed branch only; it does not infer absence from an undecoded payload.

The following contour-state helper stores u32 word_3D8, packed ReadCount entry_count, then entry_count entries. Each entry is a tagged moOneContour_c pointer, u16 index_count and index_count u32 values. The word/collection are present for v > 0x84B, below the scoped range. This collection uses packed counts externally and a fixed u16 count inside each entry, unlike the i32 geometry pair count. Readers SHALL preserve entry order, shared contour-pointer identity and raw unsigned indices. Unknown populated contour objects remain explicit dependencies.

The original Edition 0.85 audit omitted the unconditional reference collections and therefore ended geometry parsing early and misidentified the following contour word. Those boundary claims are superseded by the corrected audits in 11.86. The stored suppressed-body and empty-pair observations remain valid. These remain partial-profile probes; default successful manager count stays at 892 and whole-document bit coverage remains incomplete.

### 11.86 Corrected geometry references and profile placement

*Status: Native control-flow/import evidence; 12 corrected placement boundaries verified; profile suffix incomplete · Evidence: [C-101]*

The modern profile geometry helper always serializes an embedded object array at state +0x168 and a dynamic array at +0x188 after the pair map. The object array uses packed ReadCount and tagged objects. The dynamic array begins with u32 element_size; zero ends it. Nonzero size must be divisible by four, followed by u32 capacity, u32 growth, u32 count and element_size * count raw bytes. Readers SHALL bound that product and retain the payload as undecoded bytes until its element semantics are established. Suppression of the kernel body does not suppress these two collections. This corrects 11.85's earlier implementation and audit interpretation.

After the contour helper, ordinary non-reused profiles store a tagged sgLineHandle pointer, tagged component reference-plane pointer, compact mgXform_c restore body, u16 extra-transform presence and, when nonzero, another compact transform. A u16 profile-state word and tagged profile-reference pointer follow. No presence word precedes the first transform in this ordinary branch. Alternate archive conditions change the reference-plane path and remain outside this reader.

moCompRefPlane_c uses the ordinary component-reference base, then direct i32 feature ID and u32 feature time, a tagged reference-geometry pointer and i32 word_100. The direct ID/time pair is the modern moFRData_c body. Native component-plane serialization gates word_100 on v > 0x4BB, already satisfied here. Readers SHALL preserve reference identity and signedness, and use the existing compact restore framing rather than the distinct display-dimension transform operator.

Corrected traversal verifies 12 profile placement boundaries, 12 nonnull component reference-plane objects and 15 chain entries. Geometry pair maps, embedded reference arrays, dynamic reference arrays and contour maps are empty in these cases; no extra placement transforms occur. The two earlier component-plane dependencies within sketches are resolved. All 12 profile probes still raise explicit partial stops after placement, before the remaining profile suffix. Default complete manager count remains 892; no whole-document bit-coverage increase is claimed.

### 11.87 Profile suffix and supported complete profile objects

*Status: Native constructor/serializer evidence; 12 complete ordinary non-reused profile objects traversed; enclosing features incomplete · Evidence: [C-102]*

After placement in 11.86, the scoped ordinary profile suffix is i32 state word_11C, i32 state word_118, u32 state word_2A0, u16 feature word_3B8, tagged moContourChooser_c pointer, direct suObArray body at state +0x128 and direct suObArray body at state +0x148. Each direct array uses packed ReadCount and tagged items without an outer object slot. For v >= 0x2B92, final i32 feature word_424 follows; below that gate it is absent. Other native gates for these fields predate the scoped range, and legacy-only bitmap/user-object branches consume no bytes here.

Native loading normalizes stored word_3B8 value 0x1C to zero in memory. Readers SHALL preserve the stored word and SHALL NOT replace its wire representation with that runtime normalization. Likewise, raw word_424 is retained independently of subsequent runtime flag changes. Constructor evidence identifies the two embedded state members as suObArray; their types are not inferred from size or empty fixtures.

The opt-in complete profile reader composes the model-feature header, ordinary non-reused sketch, chains, suppressed geometry, contours, placement and suffix. It returns only after the final supported field. The reused branch is specified in 11.161; present geometry without established body-validity context and unsupported nested classes still stop explicitly. Earlier stage probes retain partial exceptions. Complete profile objects use the containing archive map and remain separate from the unsupported enclosing boss/extrusion remainder.

The suffix-stage audit verifies 12 native profile ends. Letting the supported profile reader return completes 12 profile objects: nine containing cases then reach the inherited boss/extrusion prefix end; three encounter moCosmeticThread_c later in inherited feature data. The remaining default kernel-context and revolution dependencies persist. Default complete relation-manager count remains 892. These object-level results do not establish complete configuration streams or whole-document bit coverage.

### 11.99 Validated embedded compressed Parasolid bodies

*Status: Four native embedded bodies fully decoded through the existing Parasolid schema; 903 manager prefixes complete; other kernel forms remain context-dependent · Evidence: [C-114]*

The model-context pointer at archive-helper offset 38 is a SldModelContext_c pointer, established by the helper-object constructor. The wrapper's +4E0 method is isModelWithCompressedStream; +90 tests drawing-document context, and +548 loads a Parasolid body. The document-context implementation delegates compression selection to a document virtual flag. These runtime decisions are not inferred from an arbitrary integer in the archive. The scoped reader instead recognizes the following exact self-describing form, validating its entire nested structure before accepting it.

After u8 suppression=0, the supported form is i32 frame_length, u8 kernel_type=0, then one size-prefixed compressed chunk from 9.3. The frame length includes the kernel byte and excludes its own four bytes. The chunk's u32 length equals its remaining byte count, its GUID is the documented 16-byte compression GUID, and its block sequence ends exactly at the declared frame boundary. Every block must inflate to its declared size (1 through 0x100000 bytes); its complete zlib stream must end with no unused or unconsumed bytes. A zero/zero block pair terminates the chunk. Readers SHALL validate all these constraints, not just a plausible frame length.

Concatenated inflated blocks must form a neutral PS Parasolid transmit file from clause 10, using an installed matching schema. The parser must consume the entire transmit file without a tail, identify a partition rather than deltas, and contain exactly one BODY record. Unknown schemas, malformed block boundaries, invalid sizes, trailing compressed data or unsupported transmit structures SHALL stop the reader. The archive cursor advances over the frame only after validation succeeds. Other kernel types, uncompressed payload forms and stream/context variants remain outside this recognition rule.

The decoded result retains the original payload, compressed block sizes, inflated bytes, Parasolid layouts and node values. Unknown-byte accounting uses the same proportional rule as compressed partition streams: compressed data inherits any unknown inflated-field fraction, while envelope/block framing is known. All four observed embedded bodies have zero unknown inflated bytes, so none of their compressed payload bytes remain opaque under this typed decoding. This is format decoding, not a claim that a native geometry kernel has reconstructed or validated the shape.

The profile geometry writer at 0x4BB404F0 writes its extra-body map when the body-store helper returns success. On the supported ordinary raw path, helper 0x4C261650 returns success after completing and patching a frame; write failures throw. Thus a validated serialized body establishes the file-layout continuation independently of recreating the native load-time body pointer. After the existing pair map, direct references and dynamic reference array, the profile reader stores u32 extra-body count and count records of i32 key/kernel-body archive record. Readers SHALL retain duplicate keys and parse each nested body using its own suppression/framing rules. The no-body suppressed path still omits this block.

The four observed frames contain the following fully consumed transmit files, all schema SCH_3701250_37102_13006:

| Source | Frame length including kernel byte | Compressed chunk bytes | Inflated bytes | Parasolid nodes |
|---|---:|---:|---:|---:|
| vessel weldment, Config-0 | 772 | 771 | 1257 | 22 |
| Sustainability base plate, Config-0 | 1552 | 1551 | 2898 | 66 |
| advdrawings base plate, Config-0 | 1551 | 1550 | 2898 | 66 |
| assemblyvisualize base plate, Config-0 | 1551 | 1550 | 2898 | 66 |

Caller traces establish that the vessel frame occurs in a sketch spline, and the three base-plate frames occur in profile geometry. They are not fillet-cache records. The four frames cover 5422 compressed chunk bytes, 9951 inflated bytes and 220 decoded nodes. Exact offsets and payload/inflated hashes are retained in the evidence inventories.

The default supported-feature audit now completes 903/906 relation-manager prefixes. The three remaining paths advance through profile geometry to moRefPlane_c dependencies at 0x11B08/0x11AB1/0x11B29. Seven fillets and the previously documented supported feature objects still complete. A separate `--raw-kernel` hypothesis audit remains available and writes a distinct report; it keeps payloads opaque and still refuses the three profile continuations without the typed serialized-body evidence. Neither audit establishes complete configuration streams or full document coverage. The aggregate whole-document coverage metric is unchanged because these feature readers remain audit-scoped.

### 11.149 Ordinary modern contour and contour-chooser objects

*Status: B — native-derived direct and tagged array framing, with populated native arrays; numeric meanings remain partial.*

In sldmodu.dll, moOneContour_c primary table `4CED5A90` selects serializer `4C2802B0`; runtime descriptor `4DAED220` selects factory `4C26EB70` and constructor `4B92A0F0`. moContourChooser_c primary table `4CED5B40` selects serializer `4C27BBD0`; descriptor `4DAED3D0` selects factory `4C26EE00` and constructor `4B929760`. Both derive from su_CObject, not from the model-feature body. Their memory allocations (0x88 and 0x68) are not serialized lengths.

The constructors establish direct suObArray members at contour +20/+40 and chooser +08. A direct array is ReadCount followed by that many tagged object pointers, without its own class header/map slot. A tagged suObArray pointer instead uses the normal MFC object grammar, with a new object's body using the same count/items layout. The suObArray primary table in sldmfcu.dll (`3CDD4C10`) selects import thunk `3CB7A610`, which imports swccu.dll su_CObArray::Serialize (`31D3D0C0`). Its load path calls ReadCount (`31D36AD0`) and one object-pointer reader (`31D340D0`) per array slot. Counts SHALL fit the remaining minimum two-byte pointer slots. Preserve nulls, order and repeated reference identity.

For moOneContour_c, ordinary external-XML loading with v > 0x27CB has this order:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | arrays +20, +40 | Two direct suObArray bodies | Always | Independent counts and tagged items | B |
| 2 | profile presence +18 | i32 | Always in scope | Original gate v > 0xE0A; any nonzero value selects field 3 | B |
| 3 | profile +10 | Tagged moProfileFeature_c pointer | Presence nonzero | Do not consume a pointer for zero presence | B |
| 4 | raw +60, +64 | Two i32 | Always | Preserve signed wire values; meanings unresolved | B |
| 5 | raw +68, +78 | Two i32 | Always in scope | Original gate v > 0xDDC | B |
| 6 | raw +7C | i32 | Always in scope | Original gate v >= 4000 (decimal) | B |

Between the arrays, native helper `4B9375E0` increments edge-reference use counts. It takes no archive parameter and reads no additional wire fields. The offline reader retains serialized references without applying runtime ownership mutations. The constructor's members +08/+70/+80 are not appended fields.

For moContourChooser_c in the same scope:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | array +08 | Direct suObArray body | Always | Read before profile presence | B |
| 2 | profile presence +48 | i32 | Always in scope | Original gate v > 0xE0A | B |
| 3 | profile +40 | Tagged moProfileFeature_c pointer | Presence nonzero | Retain serialized profile identity | B |
| 4 | raw +38, +3C | Two i32 | Always in scope | Original gates v >= 0xB7D and v > 0xB9B | B |
| 5 | array mask | u16 | Always in scope | SHALL be 0 through 3; other values trigger a native file exception | B |
| 6 | arrays +28, +30 | Tagged suObArray pointers | Mask bit 0, bit 1 respectively | Read +28 before +30; these are not direct arrays or i32 presence pairs | B |
| 7 | raw +58 | i32 | Always in scope | Original gate v > 0x810 | B |

The chooser's pre-field runtime filtering and post-field profile reconciliation add no archive fields. Offline decoding preserves the original arrays rather than applying native collection filtering. The pre-0xB7D legacy integer-chain representation remains outside the supported context. Raw contour and chooser words remain semantically unknown and receive no coverage credit.

The installed frontier contains **262 fresh contours** and **54 fresh contour choosers** in completed roots. Every contour has nonzero profile presence, a moProfileFeature_c pointer and a one-item second array; its first array is populated. Every chooser has profile presence 1. Chooser direct-array counts range from 0 to 10. Masks 0, 1, 2 and 3 occur 47, 5, 1 and 1 times respectively; each present optional array contains one item. Thus both optional pointer positions and their combined case have native evidence. Tests additionally preserve noncanonical presence values, signed raw words and repeated identity, reject malformed/truncated inputs and unsupported contexts, and check a stream-hash-bound native containing-root boundary.

Edition 0.160 records **20,350 complete roots**, **409 list-complete configurations** and **497 explicit stops** across the **906 complete modern model contexts**. **403 lists reach exact EOF**, spanning **17,047,229 decoded bytes**; six retain four-byte unknown tails. No contour or chooser class stop remains at this frontier. Leading stops include hole-wizard features (38), scale features (29), base bodies (22), sheet-metal features (21) and extruded reference surfaces (21). This is partial framing progress; full part, assembly, drawing and installed-project semantic bit coverage remains incomplete.

Evidence: C-177.

### 11.161 3D profiles and reused-profile bodies

*Status: B — shared native serializer, modern reused branch and populated object boundaries verified; raw meanings and special contexts remain partial.*

Scope is ordinary non-DB external-XML model archives with version greater than `0x27CB`. `mo3DProfileFeature_c` primary vtable `0x4D0EC6E8`, locator `0x4D34FDB8`, selects `0x4BB1DE50`. That entry is exactly the five-byte instruction `E9 BB 1A 01 00`, a jump to `0x4BB2F910`, the moProfileFeature_c serializer selected by primary table `0x4D0EA4E0`. The 3D class has no additional wire suffix. RTTI also establishes moProfileFeature_c ancestry. The leaf forwarder has no Pdata record; its instruction extent is verified directly rather than derived from an unrelated function range.

Both runtime classes use the existing model-feature/profile header, including i32 absolute-reference-plane word, tagged pattern-feature pointer and i32 reuse selector at native +3D4. The complete signed reuse selector SHALL be retained. Zero selects the existing sketch/chains/geometry/contours/placement/suffix composition; any nonzero value selects the following alternate body, starting immediately after that selector.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | source profile | tagged moProfileFeature_c-derived pointer | reuse selector nonzero | Native load dereferences the source. The scoped reader rejects null; it preserves the cached or newly read object identity. | V |
| 2 | contour chooser +380 | tagged moContourChooser_c pointer | after row 1 | Pointer tag is always present; null is retained. | V |
| 3 | selection word +3B8 | u16 | after row 2 | Preserve raw word without enum or boolean normalization. | V / ? |
| 4 | selection handle +3B0 | tagged sgLineHandle pointer | after row 3 | Pointer tag is always present; all nine populated installed occurrences are sgLineHandle objects. | V |
| 5 | contour state +3D8 | u32 | after row 4 | First field of existing common contour-map helper; meaning remains partial. | V / ? |
| 6 | contour associations | ReadCount; repeated entries | after row 5 | Each entry is tagged moOneContour_c pointer, u16 index count and that many u32 indices. Preserve ordered entries, null pointers, duplicates and raw indices. | V / ? |

The modern reused load calls contour helper `0x4BB40C90` and then returns. No new sgSketch, chain collection, geometry body, placement transform, ordinary profile suffix or +424 word follows this branch. Native runtime code can share/copy sketch state from the source and update the selected handle's sketch owner; these actions consume no additional wire fields and are not performed by the offline reader. A decompiler's shared runtime setter name does not establish that +3B0 is a sketch-region pointer. Native pointer overloads and populated runtime class identities establish the line-handle observations above. Historical reuse conversions below `0xB7D` and special archive conditions remain outside scope.

The same part-scoped graph now admits `mo3DPlaneBackedUpData_c` and `sg3DPlaneHandle`, previously used by the ordinary assembly profile reader. Plane-backup primary vtable `0x4D1583C0` selects `0x4C279D70`; after its zero-byte ordinary su_CObject layer, its body is one point (three f64 values) at native +08, one vector at +20 and one vector at +38, totaling 72 bytes. Their geometric roles beyond those native types remain uncredited. Plane-handle primary `0x4D1A5F08` selects `0x4C591ED0`, the existing entity-handle body: unsigned u16 index, signed i32 replacement only for sentinel `0x777F`, then i32 reference ID and i32 DimOnCM. Neither helper has an added 3D-specific suffix or padding.

The audit retains **five complete 3D profiles**, **24 reused profiles**, **three plane backups** and **three plane handles** in complete containing roots. All observed reused objects are moProfileFeature_c; their selectors span 1 through 7. Selection handles are null in 15 cases and sgLineHandle in nine. Contour maps have zero entries in 13 cases, one in five, four in four and six in two; all 33 association entries contain one u32 index. Synthetic tests exercise additional raw values and repeated identities without claiming native semantic validity for every combination.

Edition 0.172 records **21,690 complete roots**, **584 list-complete configurations** and **322 explicit stops** across **906 complete modern model contexts**. **530 lists reach exact EOF**, spanning **27,780,577 decoded bytes**; **54 complete lists retain four-byte unknown tails**. Leading dependencies include context-unit components, reference points, flat patterns, connection points and spline tangency controls. Full part, assembly, drawing and all installed-project bit coverage remains incomplete. Object boundaries and test results grant no whole-document coverage credit.

Evidence: C-189.

