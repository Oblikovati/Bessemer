### 11.63 Main sketch-tail offset collections

*Status: Native-derived direct collection order and sketch-offset body; 28 native group boundaries and one populated record verified · Evidence: [C-78]*

After helper 0x4C5AB5A0 returns, the main sketch serializer reads a u16 count of direct sgOffset_c bodies at runtime +0x48. These are distinct from the offset dimension bodies in 11.39. Their serializer is recovered but not yet implemented; nonzero counts stop explicitly. In modern scoped v > 0x27CB archives, a second u16-count collection follows at +0x58, containing direct sgSketchOffset_c bodies. Neither collection allocates an MFC slot for its direct entries.

sgSketchOffset_c begins with an i32 index count and that many stored u32 index values. When the count is positive, it next stores i32 word_38, i32 word_34, f64 value_40, two three-f64 points and u16 word_90. When the count is zero, that entire geometric block is absent. In both cases, a tagged sgPGMProfileData_c pointer follows, then i32 word_a4 and i32 word_a8. Readers SHALL preserve index bits, raw words, signed values and pointer identity. The scoped decoder rejects negative or impossible index counts. Nonnull unsupported profile-data bodies stop explicitly. Field semantics remain partial.

The existing relation array has a selected native logical-G2 lookup consumer. Exact sgSketchOffset_c::getOffsetRelations returns the embedded su_CUIntArray at current 0x10, whose current backing pointer and count are 0x18/0x20. The canonical serializer writes the count and four-byte entries through signed32 archive operators; the existing unsigned offline view preserves the same entry bits. Positive-count LOAD uses the same embedded array mutators. This adds no array header, object slot or field to the wire grammar.

Named getEntsForManipulators passes current array entries and the current sketch pointer at owner 0x08 to exact sgSketch::getLogG2ArrI, declared to return sgLogG2Dim*. The complete lookup sign-extends the incoming 32-bit value, multiplies by native 0x158 and adds the current backing pointer reached through sketch 0x90/container 0x08. It has no local index bounds or null checks. The 0x158 spacing is a runtime object stride, not a serialized logical-G2 record size. This proves an index-consumer relation on the reached path, not global-ID semantics, a universal file index convention, safe ranges or stable file/current ordering.

The current owner 0x08 pointer is not an additional stored sketch pointer, and its identity with an enclosing filed sketch is not established by this evidence. Later current-array values are not promised unchanged from stored bits across LOAD fixups, providers or mutation. Returned logical-G2 objects are queried repeatedly through virtual slot 0x3E8/0x4A0 with comparisons against 1/2 and pointer-nonnull tests; selected handle outputs can use the returned base or base+0x28, with further opaque conversion/provider work. Writes are sequential and may leave partial outputs. These are current pointer/output checkpoints, not stable handle pairs, safe arbitrary inputs, solver success or a complete manipulator algorithm. Saved points 0x48/0x60 receive no new role, and runtime ClickedPoint 0x78 is excluded. Evidence: C-3548.

The audit completes both collections in 28 native cases, decoding one sketch-offset record with three indices. Three cases stop on sgOffset_c count two, and one stops on a nonnull sgPGMProfileData_c body. Six sketches retain their earlier feature-body stops. Cumulative positions over the 28 completed cases total 528,919 bytes; this cohort differs from the 32-case display-list audit. Later main sketch fields remain unread and whole-stream coverage is unchanged.

### 11.357 Deform feature, matching passes and versioned geometry

*Status: Verified framing in ordinary modern part archives; raw fields remain partial semantics · Evidence: C-399*

The complete `moDeform_c` body composes native routines from four hash-bound installed modules: part serializer `0x180253590`, model ID keeper `0x4C6E0850`, geometry serializer `0x43BBC8C0` in `sldgcu.dll` and primitive restore `0x32102F70` in `sldmgu.dll`. The feature begins with the complete body-feature base and inherited concept hooks. Its constructor `0x1802517C0` constructs direct `gcDeformData_c` at +8F8 and an owned `moDeformSIdKeeper_c` at pointer +10F0. The final keeper dispatch is a direct body call, without an additional pointer tag.

**Outer feature suffix.** In wire order: tagged display dimensions +800/+808; i32 +810; tagged direction +818; six nullable direct object arrays +828, temporary array 1, temporary array 2, +830, +840, +858; i32 +860; five nullable arrays +880/+888/+890/+898/+8A0; three-f64 point +8A8; signed i32 matching-record count +8C0 and that many first-pass matching records for a positive count; then signed stored data version +8D0. Nonpositive native counts execute no record iterations and are retained. Array presence is an i32 word followed, when nonzero, by ReadCount and tagged entries. Temporary arrays are genuinely serialized and SHALL NOT be omitted merely because native loading later discards them.

The stored **feature data version**, rather than the archive version, then controls these fields:

| Stored data version | Fields in the native load order |
|---|---|
| >=1 | tagged direction +820 |
| >=2 | nullable array +838 |
| >=3 | nullable array +848 |
| >=4 | i32 +8D4 |
| >=6 | i32 +8D8 |
| always | f64 +8E0/+8E8/+8F0; i32 +864; three-f64 vector +868; direct geometry body +8F8 |
| >=8 | nullable array +850 |
| always | second pass over each previously read matching record, as specified below |
| >=7 | extension-code loop, as specified below |
| always | direct edge/face surface-ID keeper +10F0 |

Exact own moDeform_c methods bind five existing saved associations. Folded implementation addresses are qualified by the actual Deform declaring-class signatures.

| Native property | Existing saved member | Qualified operation |
|---|---|---|
| PullForce | Tagged dimension 0x800 | Current getter virtual slot 0x868(0); setter virtual slot 0x860(original double,1,0) |
| PullRadius | Tagged dimension 0x808 | Same qualified provider request through its own current pointer |
| DragPointUv | Three-f64 point 0x8A8 | Typed point assignment getter/setter; getter returns 1 after copying on normal completion |
| DeformSquare | Geometry 0xD8, absolute 0x9D0=0x8F8+0xD8 | Raw full 32-bit getter/setter; existing fixed-prefix signed32 word |
| SurfacePushPrimitiveData | Geometry 0x678/0x680, absolute 0xF70/0xF78 | Setter integer write followed by conditional typed primitive assignment |

The dimension methods dereference their current pointers without null guards. They identify numeric provider requests, not extra saved PullForce/PullRadius doubles or a guarantee of final effective values. DragPointUv names the existing three-component point; no two-dimensional reduction, U/V range, units or coordinate frame is inferred. DeformSquare transfers the whole integer without Boolean normalization or a proved square-geometry predicate.

setSurfacePushPrimitiveData stores the full incoming integer to absolute 0xF70 first. Only when its primitive-source pointer is nonnull does it invoke typed mgPrimitiveBodyData assignment at 0xF78; a null source skips that assignment rather than resetting the existing primitive. This is not an atomic pair update, a closed primitive-kind enum or a geometry-success guarantee. The geometry stored signed revision at 0x134 (absolute 0xA2C), not feature data version 0x8D0 or enclosing archive version, gates LOAD of geometry 0x678/0x680 at revision>=4. The selected STORE emits that pair without the corresponding LOAD gate. Geometry 0xD8 remains in the fixed prefix and is not newly gated by feature version 0x8D0. No outer fields are added by these nested-offset joins. Alias, callback, pointer lifetime, current/file identity and other unassigned fields remain qualified. Evidence: C-3549.

**Matching data.** `moDeformMatchData_c::Serialize` at `0x180253150` reads two sides. Each side is a nullable object array followed by two i32 words; after both sides comes a signed subrecord count and a positive-count loop of direct `moDeformMatchSubData_c` bodies. The subrecord serializer `0x1802532C0` reads two sides, each consisting of i32, tagged curve-point reference, generic tagged object reference, three-f64 point and three-f64 vector. Thus an empty-pointer subrecord still occupies **112 bytes**; its **104 scalar/geometry bytes** remain unknown. Native conversion of an endpoint object to a curve-point object changes runtime ownership and consumes no additional bytes. The decoder preserves the original shared archive-map identities.

The second matching pass `0x180252F60` reads nothing for feature data versions below 9. From version 9 it reads two `(nullable array, i32)` sides and another i32 +48. It then calls each subrecord's second pass at `0x1802530B0`; from feature data version 10 that pass reads two three-f64 vectors at +88/+A0. Earlier versions initialize runtime members without consuming those vector bytes.

The nested class `moCurvePointRef_w` has primary table `0x4D17F400`. Its serializer at `0x4C27C5E0` is a verified five-byte jump to the established endpoint-reference serializer `0x4C27D8F0`; it uses that complete grammar without an extra suffix. This additional binding is installed locally in the enabled deform scope.

**Direct geometry body.** `gcDeformData_c::Serialize` stores its own signed version at native offset +134. This is distinct from feature data version +8D0. Its fixed prefix is **372 bytes**: one i32 mode; seven three-f64 triples (two points, one vector, then two plane root/normal pairs); i32 +C0/+C4; f64 +C8/+D0; i32 +D8; f64 +E0/+E8/+F0; sixteen i32 words +F8..+134; ten f64 values +138..+180; and i32 +188. Preserve the stored mode and flags before native load-time normalization. The scoped property joins above do not change raw values or semantic coverage accounting; unassigned geometry meanings remain unresolved.

| Geometry stored version | Additional fields, in wire order | Total bytes through this version |
|---|---|---:|
| <=0 | none beyond the fixed prefix | 372 |
| >=1 | i32 +18C/+190 | 380 |
| >=2 | f64 +5C0/+5E8 | 396 |
| >=3 | fixed 30-u32 archive transform +5F0 | 516 |
| >=4 | i32 +678; primitive-body data +680 | 576 |
| >=5 | f64 +7E0 | 584 |
| >=6 | i32 +7E8 | 588 |
| >=7 | i32 +7EC | 592 |

These rows are cumulative native signed comparisons, not a table indexed by unvalidated bytes. The transform is the **fixed 120-byte archive operator**, not compact restore; unsupported perspective flag bits fail explicitly. Primitive-body restore is **56 bytes**, i32 +0, six f64 values at +8..+30, then i32 +38. The CArchive primitive export at `0x32102F60` jumps to the verified su_CArchive implementation at `0x32102F70`. Runtime geometry defaults and resetting the in-memory geometry version to 7 add no stored bytes.

**Extensions and keeper.** The extension loop reads i32 codes: `0x3D2` carries one i32, `0x13E` one f64 and `0x1466` one tagged object. Any other code terminates the native loop after that code, so its exact signed value is preserved; the installed source terminates with 0. There is no inferred length or opaque skip for unrecognized codes. The final owned `moDeformSIdKeeper_c` selects the established `moEdgeFaceSIdKeeper_c` serializer `0x4C6E0850`: direct surface-ID array +18, direct object array +50, and an additional surface-ID array +28 when archive version `>0x3746`. Its constructor at `0x4BA269A0` establishes the inherited keeper type. Reference-use/owner updates consume no additional bytes.

`samples/tutorial/molds/telephone.sldprt`, configuration 0, now reaches exact EOF at **556,067 decoded bytes**, including **248,283 unknown** and **307,784 known structure bytes**. It contains one deform object and two curve-point references, feature data version **11**, geometry version **7**, one populated matching record with one populated subrecord, and extension terminator 0. All declared feature roots and required tails complete.

`resolved-deformation-audit-0.377.json` records **898 complete / 8 explicit stops / zero context stops** among 906 modern part configurations, with **26,638 completed roots** across the audit. Every preceding 897 complete stream preserves endpoint, root count, source hash and unknown total. Complete streams now contain **81,814,688 decoded bytes**, including **29,061,608 unknown** and **52,753,080 known structure bytes**.

Add `--deformation-dependencies` to the full Edition 0.376 audit/regrade commands. The public reader enables the scope; the staged reader defaults it off. `resolved_coverage_evidence.py --deformation-dependencies` independently checks all **1,166 source rows**, hashes, stored spans and integer proportions. Added stored part credit is **63,464 bytes**. Cumulative resolved-family known bytes become **14,795,692**, leaving **10,341,774 unknown** of 25,137,466 stored bytes. Total part unknown bytes become **53,571,940** of 227,086,352. Rounded coverage remains **76.4% / 71.3%** for parts, **83.8% / 81.3%** for assemblies and **57.3% / 49.9%** for drawings. Full documents, remaining dependencies, raw semantics, legacy/assembly/drawing contexts and all installed project-format schemas remain incomplete.

`deformation_evidence.py` rechecks four DLL hashes, primary bindings and ancestry, inherited concept imports, eleven native bodies plus the curve-point leaf jump, constructor/keeper composition, the populated two-pass source and all 897 prior positives. Replay tuples are normalized to JSON arrays solely for comparison with the saved evidence representation; original stream hashes and decoder coordinates remain unchanged. Six focused tests cover geometry version boundaries 0..7, every populated geometry/matching truncation, fixed/primitive widths, unsupported perspective flags, both matching passes, all extension types and a nonzero native terminator, shared references, invalid counts, context rejection, local installation, exact source EOF, disabled-scope zero credit and appended-byte rejection. The preceding C-398 packet is reproduced through exact sources frozen under `research/frozen-decoder-0.376/`. All **1,184 regression tests pass** (163.157 seconds); specification lint reports zero findings (581 headings, 96 anchors).

Evidence: C-399.

### 11.139 Modern planar surfaces and planar ID keepers

*Status: B — native-derived ordinary framing; installed completed planar fixtures have empty geometry/keeper collections.*

The moPlanarSurface_c implementation is in **sldMoPartu.dll**, not sldmodu.dll. Its primary table `18044AC70` selects serializer `18021DEB0`. The native runtime descriptor identifies memory size 0x7C0 and factory `18021DBA0`, whose constructor call selects `18021DC90`; memory size is not wire size. The constructor calls moRefSurface_c and allocates a direct suObArray at member +0x7B0. The part table's concept hooks at +0x4B8/+0x4C0 import moConcept_c::SerializeLWData/SerializeResolvedData. Its virtual +0xAB0 returns **zero**.

The following grammar applies to v > 0x27CB in the ordinary non-DB external-XML context, with helper +0x834 clear. Other surface-derived classes can have different virtual methods and SHALL NOT receive this planar grammar merely because they share the moRefSurface_c base.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | reference geometry | Shared atom-feature inheritance and reference-geometry suffix | Always | Same inherited stages as 11.100, without the derived body-feature suffix or reference-plane data pointer | B |
| 2 | archive-backend marker | i32 | Always | Native writer records backend presence; retain the raw value | B |
| 3 | sheet count | i32 | Always | Nonnegative count; available bytes must admit all entries | B |
| 4 | sheets | Count times direct kernel body, then direct suObList | Field 3 times | Body/list pairs are inline for this class; list is read even for a suppressed body | B |
| 5 | extra-body presence | i32 | Always | Nonzero introduces field 6 | B |
| 6 | extra body | Direct common kernel body | Field 5 nonzero | Native mo_loadBodyFromArchive call | B |
| 7 | extra-body attributes | Direct suObList | Successful present field 6 | Only when field 6 successfully loads a present body; suppressed body returns zero and adds no list | B |
| 8 | raw member 748 vector | i32 count, then i32[count] | v >= 0x2BCD | v >= 0x2BCD; nonnegative count, word meanings unresolved | B |
| 9 | EdgePLCompRefs member 7B0 | Direct suObArray | Always | ReadCount and tagged object pointers, no outer tag or presence word | B |
| 10 | raw member 7B8 | i32 | Always | Modern scope; original gate v > 0x10D1 | B |

Exact own moPlanarSurface_c getEdgePLCompRefs returns the current runtime array pointer at 0x7B0. The constructor-bound array is serialized directly by its virtual serializer, so this native pointer getter does not add a tagged pointer or presence word to the existing wire body.

setEdgePLCompRefs calls deleteAll on the current destination array before reading the input count. Its signed loop then appends input pointers via SetAtGrow using the current destination count, reloading source backing/count and the current destination pointer as it proceeds. This is not a clone, a deep-copy contract, safe self-aliasing or an atomic replacement; deletion/provider/alias effects can occur before all input is examined.

getPlanarBoundingEdges first clears caller output through SetSize(0,-1,1), before reading the current source count. Each selected source reference is supplied with the actual moCompEdge_c runtime-class descriptor to an unnamed MFC ordinal 7928 call. On its nonzero result, the reference virtual slot 0x448 is called with arguments 0,0,null,null. A nonnull result is passed to typed gcEdge::getCurve(null), then to gcCurve::isPlanar(null). The getCurve result has no local null guard. If the planarity call returns nonzero, the ORIGINAL source reference is appended to output through SetAtGrow at the current output count; the curve or resolved edge is not substituted as the output item.

Source 0x7B0 and counts are re-read during traversal, and the method returns the current TOTAL output count. Repeated providers are not an immutable snapshot. This selected runtime filter proves neither that every stored item is an edge nor that all input, returned references or final geometry are planar. Valid current objects/output, provider behavior, ownership, null safety, aliases, rollback and file/current identity remain prerequisites or open limits. Raw 0x7B8 is unchanged. Evidence: C-3550.

The shared reference-geometry helper `4C283A00` first calls atom-feature serializer `4BB1EC00`. Its suffix is a direct point at +0x6B8, raw u32 flags at +0x6D4, i32 +0x6D8, tagged moFeatureRefWrapper_c pointer +0x6E0, and i32 +0x6E8 when v > 0x2F85. It does not contain moBodyFeature_c's assembly-feature/component-feature/array-778 suffix. The existing reference-plane reader and planar-surface reader now call the same explicit reference-geometry reader.

moRefSurface_c primary table `4D17A218` selects `4BB31450`. That base considers both field 2 and virtual +0xAB0 when choosing inline bodies versus backend callbacks. Since the planar class returns zero, **a nonzero backend marker does not skip the inline body or its attribute list**. Special transfer/backend branches and runtime body conversion add no additional ordinary planar wire fields. Kernel bodies use the common suppression-byte/body grammar; recognized self-describing compressed Parasolid frames are validated through their exact transmit ends. Other kernel contexts stop explicitly. The optional extra body's attribute-list decision follows the native helper's return: suppressed body returns zero; a successfully validated present body enables the list. No arbitrary opaque byte range is accepted as a body.

Each direct suObList/suObArray is ReadCount plus tagged pointers. It adds no wrapper slot, no i32 presence and no trailing moNodeList word. Root ID-keeper references, sheet attributes, optional-body attributes and the final planar array share the inherited archive map. Counts SHALL fit the minimum remaining record sizes. These framing rules do not establish attribute-class meanings.

moPlanarSurfIdKeeper_c primary table `4CF61E78` selects serializer `4BB2F8D0`. Constructor `4BB717F0` initializes suObArray members +08, +28 and +48; only **08 then 48** are serialized here. The keeper body has:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | array 08 | Direct suObArray | Always | ReadCount and tagged pointers | B |
| 2 | array 48 | Direct suObArray | Always | Same grammar, same archive map | B |
| 3 | raw 90 | u32 | Always | Preserve all bits | B |
| 4 | map-entry count | ReadCount | Always in modern scope | v > 0x87D, always satisfied in scope | B |
| 5 | entries 88 | Count times direct suObArray, u16 word count, u32[word count] | Field 4 times | Each direct array is the serialized key body; no key-object pointer tag or map slot | B |

The map suffix is helper `4BB3F980`. Readers retain entry order and the full nested arrays/word vectors; runtime map insertion and dynamic-array allocation do not serialize additional bytes. Array +28 and pointer-vector +68 are initialized runtime state, not additional modern fields. Keeper words and map associations remain semantically unresolved.

The installed audit records **54 fresh planar surfaces and 54 fresh planar ID keepers** in completed roots. Every backend marker is 1, but every sheet count, extra-body presence, raw-word-vector count, final planar-array count and keeper collection count is zero. A synthetic composition combines a complete installed compressed Parasolid body frame with inherited attribute references and a nonempty word vector; it proves present-body/list placement without claiming a populated native planar fixture. Other tests cover a suppressed sheet's attribute list, suppressed optional-body absence of a list, nested keeper map entries, extended ReadCount versus fixed u16 counts, duplicate identity, high-bit words, the 0x2BCD gate, truncation and a hash-bound installed planar-root boundary.

The edition 0.150 frontier records **18,586 complete roots** and **197 list-complete configurations**, of which **194 reach exact EOF** spanning **5,589,054 decoded bytes**. Three retain four unknown tail bytes. All **906 modern model contexts** complete; **709 lists** still stop explicitly. No planar-surface or planar-keeper stop remains at the audited frontier. The leading dependencies are attributes (62), moICE_c (44), chamfers (43), context-chain components (40) and mirror patterns (36). Complete framing remains distinct from full document bit coverage, which is still incomplete across parts, assemblies, drawings and installed project formats.

Evidence: C-167.
