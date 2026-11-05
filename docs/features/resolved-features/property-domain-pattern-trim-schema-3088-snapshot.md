### 11.94 Linear-pattern header and derived suffix

*Status: Native serializer evidence; three shared-header boundaries verified; enclosing patterns incomplete · Evidence: [C-109]*

Primary moLPattern_c serializer 0x4BB28E00 first calls shared pattern serializer 0x4BB2E550, which calls the body-feature base from 11.81 and 11.88–11.89. The scoped reader returns that supported base independently of any derived feature suffix. This base completion is not a complete pattern object. The following layout applies to ordinary zero-condition external-XML archives above version 0x27CB.

The shared pattern header stores tagged seed suObList (980), tagged suObList (9D0), i32 word_9D8, tagged suObArray (818), u8 byte_9F4, i32 word_9F8 and tagged suObList (8D0). Before version 0x42CF, the next field is u8 byte_940. From 0x42CF, it is instead tagged suObList (8D8) followed by i32 word_940. Both variants then store a tagged moMatrixIndexHelper_c pointer (948). Readers SHALL preserve the raw byte/word and pointer identity. Native normalization of byte_9F4 from 2 to 1 under owner-dependent conditions does not change its stored width. Archive condition 0x2000 can substitute direct feature-reference data for the seed list; that branch is outside this zero-condition reader.

The shared moPattern_c base has the following exact own-class property map. These roles do not reorder the existing wire layout or add fields to derived suffixes.

| Runtime member | Native property | Existing wire representation |
|---|---|---|
| 9D8 | SketchInvarient (native spelling) | i32 |
| 940 | PatternEntitiesType, moPatternEntitiesType_e contract | legacy u8 or modern i32, as above |
| 1048 | PropColorCThread | u8 |
| 1120 | MissingSeedFeature | i32 |
| 1124 | MirrorSeedOnly | u8 |
| 9FC / A00 | indexed DimensionDrivenPattern, indices 0/1 | two u8 |

All seven current fields have full 32-bit accessor contracts. At the current query >=0x42CF, member 0x940 uses the signed32 codec and the existing list 0x8D8 occurrence. The older selected STORE instead writes byte(current 0x940==1), while older LOAD maps any nonzero byte to current 1 and zero to 0. These are different native transformations, not a restriction that old files contain only 0/1. The enum type alone establishes no numeric engineering labels or public-API code mapping. Exact named moPattern_c::isPttnBody compares the entire current DWORD 0x940 with 1 and returns 1 only for equality, otherwise 0. This is an exact-one predicate, not a low-bit or arbitrary-nonzero test. It binds numeric 1 to that named native predicate only; it does not close the enum, identify other codes or guarantee body geometry. The previous fullword accessors and legacy/current codec policies remain distinct. Evidence: C-3530.

A supporting interface trace identifies installed ILinearPatternFeatureData.set_PatternBodyArray (void/object, DISPID 31) and the exact native XDispatch put_PatternBodyArray literal. Its selected root/worker paths construct typed moCompSolidBody references and clear/add them through a current wrapper+0x68 owner collection at +0x798. These are runtime addresses, not file offsets. The wrapper+0x68 object's concrete type and its commit path to model 0x940 remain unbound. The trace proves neither a GUID/vtable/Invoke binding nor a public enum translation, UI-only behavior or transitive absence of model changes. No COM call was executed. The missing typed commit bridge remains a requirement before using public array names to label model 0x940.

Members 0x1048, 0x1124 and 0x9FC/0xA00 STORE low bytes and reached LOAD zeroextends them to full 32-bit; no Boolean conversion is inferred. LOAD admits 0x1048 from 0xB72 and 0x1124 from 0x3746, otherwise assigning local zero. LOAD admits 0x9D8 from 0xEB, otherwise local zero ; 0x1120 uses signed32. Other existing context, variation and version gates remain separate. Each provider query retains its own checkpoint.

Indexed DimensionDrivenPattern access uses an unsigned 0..1 guard, then index*4 addressing. An invalid getter index returns 0; an invalid setter index skips the local store. The setter index is EDX and its full 32-bit value is R8D. Only supplied indices 0/1 map the two declared saved fields. These native names do not establish a Boolean domain, actual invariant sketch, color propagation, missing seed truth, mirror behavior or final pattern geometry. The base binding is independent of derived overrides ; 0x9F4/0x9F8 and other unlisted fields remain open. Evidence: C-3527.

Immediately after this header, the shared serializer invokes an embedded collection at +0x838, followed by a packed-count object-pointer map, later embedded collections/helpers, variation records and further versioned fields. The initial probe stops before +0x838; constructor-verified maps, the following object array and shape data are documented in 11.95. Readers SHALL stop at the first unsupported stage. The header reader never skips the base to reach the derived suffix.

After the complete shared base, the moLPattern_c-specific suffix starts with four tagged display dimensions (1250,1258,1260,1268). This branch is proven by its primary vtable +0x2680 resolving to the zero-return function 0x4B29C5A0. Other derived pattern classes may omit these four tags and are not admitted by this scoped suffix reader. Next are u8 bytes 1290/1294, two generic tagged object pointers (1270,1278), two tagged line references (1280,1288), and u8 byte_129C.

The suffix continues with i32 words 12D8/12DC, tagged reference wrappers 12E0/12E8, i32 words 12F0/12F4, tagged reference wrappers 12F8/1300, i32 words 1308/130C, tagged display dimensions 1310/1318, and i32 words 1320/1324. Version >=0x3F00 adds u8 byte_1298. Readers SHALL retain raw byte values and signed words. Earlier line-reference and advanced-field gates all precede the scoped modern versions. Runtime ownership repair and flag updates add no suffix bytes.

The following exact own-class moLPattern_c properties bind selected suffix members. All eleven have full 32-bit current getter/setter contracts, independently joined to the canonical serializer rather than inferred from names or shared addresses.

| Runtime member | Exact native property | Wire |
|---|---|---|
| 0x1290 | FlipDirection1 | u8 |
| 0x1294 | FlipDirection2 | u8 |
| 0x1320 | IsControledByNum1 | i32 |
| 0x1324 | IsControledByNum2 | i32 |
| 0x12d8 | IsOffsetFromRef1 | i32 |
| 0x12dc | IsOffsetFromRef2 | i32 |
| 0x1308 | RevOffset1 | i32 |
| 0x130c | RevOffset2 | i32 |
| 0x12f0 | UseCentroid1 | i32 |
| 0x12f4 | UseCentroid2 | i32 |
| 0x1298 | SymmetricForDirection2 | u8 |

FlipDirection1/2 and SymmetricForDirection2 STORE only the low byte and reached LOAD zeroextends all 256 possible bytes to full 32-bit, without Boolean conversion. The other eight fields use signed32 codecs. The advanced offset block's LOAD gate is its current query >=0x1BFF, the IsControledByNum pair's is >=0x1C02, and member 0x1298 has independent current queries >=0x3F00 in both directions. Earlier branches/defaults and inherited pattern gates retain their existing policies. No Boolean/enum closure, physical direction, centroid formula or complete pattern behavior follows from these native names.

The SymmetricForDirection2 setter first writes its full input into 0x1298, then works through current dimensions 0x1268/0x1260 and their providers. A nonzero request passes 1 to handle virtual 0x648; a zero request first requires virtual 0x650's AX result equal to 1 before passing 0 to 0x648. No operation name is assigned to those dynamic targets, and later callbacks need not preserve the initial stored word or object identity. Offline keys and saved values remain unchanged. This is a modelling-pattern parameter group; sketch gaps remain open. Evidence: C-3521.

`--pattern-header` reaches the explicit collection boundary in all three native hex-nut-beater configurations at offsets 72811,72815,72815. Each has one seed, empty 9D0/8D0 lists, null array_818/matrix helper, byte_9F4=1, word_9D8=1 and word_9F8/word_940=0. These traversals complete three revolution cuts and 24 profiles; the cumulative revolution counter is eight, including those three cuts and five ordinary revolutions. Manager completion remains 898/906, with three intentional pattern stops plus four fillet and one kernel-context stop. No pattern or cosmetic thread is returned complete. The standalone derived-suffix reader has native and synthetic evidence, but is not yet reached by those native paths. Whole-document bit coverage is unchanged.

### 11.222 Surface-cut and trim-reference-surface features

*Status: B / ? — five surface cuts and one trim verified inside complete native roots; raw meanings, unobserved pointer combinations and full-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature context, version >0x27CB. sldMoPartu.dll primary RTTI binds moSurfCut_c (0x1804BBD88 / COL 0x1804F2548) to 0x180357290, and moTrimRefSurface_c (0x1804C54B8 / COL 0x1804F29D0) to 0x180360030. Both have moBodyFeature_c ancestry, call its imported serializer and use the ordinary inherited moConcept_c lightweight/resolved hooks. Surface-cut follows the complete body-feature prefix with:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | atom feature +808 | tagged moAtomFeature_c | always in scope | Preserve pointer identity. | B |
| 2 | raw +818,+81C | two i32 | always | Separate signed words. | B / ? |
| 3 | face +820 | tagged moFaceRef_c | always in scope | Nullable tagged occurrence. | B |
| 4 | surface +810 | tagged moConstSurfRef_w | always in scope | Existing constant-surface grammar. | B |
| 5 | plane +900 | tagged moPlaneRef_w | always in scope | Existing plane-reference grammar. | B |
| 6 | arrays +8C0,+8E0 | two direct suObArray | always in scope | Each ReadCount and tagged items, in order. | B |
| 7 | chooser +838 | direct moBodyChooser_c | always in scope | Direct object list then tagged moAtomFeature_c owner; no extra tagged chooser object. | B |

Exact moSurfCut_c::getSurface names current member 0x808 as a surface-feature pointer of declared type moAtomFeature_c*. getSheetIndex returns fullsigned32 member 0x818. Named setSurface sequentially writes its first formal pointer from RDX to 0x808, then its second formal integer from R8D to 0x818, on the original receiver. No normalization, index bounds, zero/one-based convention, pointer/index consistency check or atomic-pair update is established.

The canonical serializer writes 0x808 through its typed moAtomFeature_c pointer operator, followed by 0x818 through signed32 codecs. Selected LOAD uses a latched signed version comparison >=0xD6 for the typed pointer branch, then reads 0x818; older obsolete-pointer/list handling remains excluded. Archive identity and later provider effects are not a guarantee that the current pair remains coherent or that a surface cut succeeds. Raw 0x81C and runtime 0x828 KeepPieceIndex receive no new saved meaning.

All positive native claims in C-3529 use the original retained packets. Its two incident companions preserve one excluded duplicate 719-byte static capture and record zero qualifying missing-body source extensions. The duplicate is not new positive evidence. Raw file values, readers and accounting remain unchanged. Evidence: C-3529.

Native load gates >=0xD6, >=0x3AB, >0x4CA, >0x7F9 and >=0x70F all precede scope. Constructor 0x180356F70 binds the two suObArray members and embedded body chooser. With null/cached pointers and empty arrays/chooser, the complete body is **275 bytes**. New targets contribute normal tagged bodies and map slots.

Trim constructor 0x18035F590 allocates a moTrimSurfData_c at runtime +800 through constructor 0x18035F840 and constructs the direct body chooser at +808. The serializer does not write a tagged trim-data object: it dispatches selected members of that allocation directly. Runtime offsets prefixed data below refer to that allocated data object. After the complete body-feature prefix the modern trim sequence is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | data AllTools +20, Surfaces +48 | two direct suObArray | always in scope | Named embedded collections, serialized in this order. | B |
| 2 | data profile +18 | tagged moProfileFeature_c | always in scope | Normal archive map identity. | B |
| 3 | data direction +40 | tagged moDirectionSpec_c | always in scope | Nullable target. | B |
| 4 | data raw +10 | u32 | always in scope | Native gate >0x4DF predates scope. | B / ? |
| 5 | feature chooser +808 | direct moBodyChooser_c | always in scope | Direct object list then tagged owner. | B |
| 6 | data solid +68 | tagged moSolidRef_w | always in scope | Existing solid-reference grammar. | B |
| 7 | data keeper +78 | direct allocated moEntRefKeeper_c body | always in scope | suObArray serialization; no pointer tag or separate count prefix beyond that array grammar. | B |
| 8 | data array +A0 | direct suObArray | always in scope | Separate direct array. | B |
| 9 | data raw +08,+0C | two i32 | always in scope | Preserve signed words. | B / ? |

Exact own-data getAllTools and getSurfaces return addresses of the embedded collections at data 0x20 and 0x48 respectively. They are aliases to current collections, not snapshots. Named setOriginalTools addresses the same data 0x20 collection, invokes typed su_CObArray::SetSize with arguments (0,-1,1), then tail-calls Append with the supplied source collection reference. This proves operation order and the named association; it does not prove deep-copy semantics, closed item classes, final contents, alias safety or rollback on failure. The existing direct suObArray archive grammar is unchanged. The serializer reloads current feature 0x800 separately before dispatching each collection, so same-instance lifetime across callbacks is not promised. Evidence: C-3533.

The following exact moTrimSurfData_c property roles refer to the allocated data reached through feature 0x800. They do not add a separately tagged TrimData object or split packed u32 data 0x10 into additional wire fields.

| Saved component | Native property / writer | Qualified policy |
|---|---|---|
| data 0x10 mask 0x01 | MutualTrim | Getter projects bit0; setter zero clears it, nonzero sets it, preserving other bits. |
| data 0x10 mask 0x08 | RemovePicked | Getter projects bit3; setter zero clears it, nonzero sets it, preserving other bits. |
| data 0x10 mask 0x10, gated by 0x01 | SpacePartition | Getter is zero unless MutualTrim is set. Nonzero setter ORs 0x11, forcing both components; zero setter clears only 0x10. |
| data 0x10 mask 0x04 | setConsuming | Setter-only association; zero clears and nonzero sets 0x04, preserving other bits. |
| data 0x10 mask 0x40 | UseRollback | Getter returns (current word >> 6) & 1; setter zero clears 0x40, nonzero sets it, preserving the other 31 current bits. |
| data 0x10 high 16 bits | CompleteOpts saved fragment | Current provider synchronization below; not a complete 32-bit saved property. |
| data 0x08 | CreateSolidFlag | Raw full signed32 native getter/setter and archive word. |
| data 0x0C | CreateSolidOption | Raw full signed32 native getter/setter and archive word. |

Exact own-data moTrimSurfData_c::setUseRollback(int) reads original data 0x10 and writes old&0xFFFFFFBF for zero input or old|0x40 for nonzero input. The complementary exact useRollback query reads that word, shifts right by six and masks with one, returning only the current request bit. The same whole unsigned32 archive codec stores this component in the allocated TrimData object reached through feature 0x800. These local query/setter checkpoints do not restrict the whole file word to Boolean values, prove a successful rollback effect or guarantee a universal final state. They are separate from feature runtime UseRollback at 0x8B0. Mask 0x2 and other unassigned components remain open; the existing CompleteOpts high-fragment and version/proxy policies are unchanged. No wire field is added. Evidence: C-3531, C-3533.

CompleteOpts has distinct saved-fragment and runtime-provider policies. Its setter first writes the full 32-bit input to current 0x70->0x98 when 0x70 is nonnull, then copies only that input's low 16 bits into data 0x10's high 16 bits, preserving the low 16 bits. With null 0x70, its getter returns the current packed high 16 bits fragment. Otherwise the getter returns the full 32-bit value at current 0x70->0x98 and conditionally synchronizes that value's low 16 bits into the packed high 16 bits, again preserving the low 16 bits. The getter can therefore mutate the packed field, and its provider-backed result is not restricted to 16 bits. Provider type, numeric option meanings, aliases and later state remain separate. Offline decoding reads the stored u32 and does not call these methods.

The selected direct modern LOAD path requires its current query >=0xDF7. Separate current unsigned queries admit data 0x10 from 0x4E0 in both directions and data 0x08 from 0x1FE5 in both directions. Selected STORE writes data 0x0C without its LOAD gate; LOAD admits it from 0x2000 and has no direct reset at the skipped branch. Primitive codecs are unsigned32 for 0x10 and signed32 for 0x08/0x0C. Proxy transfers, replacement data and separate provider queries retain their original context; no single immutable query value or current-object identity is promised. Other low bits and upper-fragment numeric domains remain unassigned. No file Boolean/enum closure or successful trim, consuming or solid-creation effect follows. Evidence: C-3525.

The trim proxy/transfer path below version 0xDF7 is excluded. Later native gates >0xDF8, >0x1FE4 and >=0x2000 also precede scope, so all rows are present throughout the supported context. Constructor evidence identifies direct arrays +20,+48,+80,+A0 and an allocated entity-reference keeper +78. Array +80 is constructed but is not separately dispatched by this serializer; it must not be inserted into the wire sequence. The keeper's primary serializer thunk 0x1803AD148 resolves to sldmfcu.dll suObArray::Serialize. The body chooser independently binds through sldmodu.dll primary 0x4CE879D0 to 0x4C27AAA0: ordinary zero-byte su_CObject base, direct object list and tagged owner. Runtime allocation sizes 0xC0 for trim data and 0x8B8/0x908 for feature objects are not serialized lengths. The complete empty trim body is **281 bytes**.

Both readers are installed only in the copied modern feature-context map. Four focused tests cover full pointer/collection ordering, duplicate and cached identities, signed/u32 extremes, populated arrays/keeper/chooser, direct-data map accounting, tagged minima at 0x27CC and version 19000, cached self/following-byte alignment, truncation/context rejection and source-hash-bound complete native containing-root spans/raw fields/counts/target classes for both features.

Five native surface cuts have raw +818 = -1; raw +81C is zero in three and one in two. Both arrays and chooser lists are empty. Each has a plane-reference pointer and null atom/face/constant-surface pointers. The complete native trim has array counts (1,1,0,0), one chooser item, profile moProfileFeature_c, null direction/solid, u32 raw +10 = 458760 and raw +08/+0C zero. These observations do not define valid domains or restrict other pointer combinations.

Across 906 complete modern contexts there are **24,399 complete roots**, **774 complete lists** and **132 explicit stops**: 22 additional roots and three additional lists relative to Edition 0.227. **701 lists reach exact EOF**, spanning **46,743,994 decoded bytes**, an increase of 211,155 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. Later affected parsing reaches moOffsetRefSurface_c or trim-edge entity representations rather than skipping them. `resolved-feature-list-frontier-0.227.json` preserves the preceding checkpoint. Whole-document coverage credit is unchanged; complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-250.
