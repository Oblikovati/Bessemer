### 11.1757 Logical-G2 legacy tangent-word gate and compatibility invalidation

*Status: exact old LOAD presence and independent current-state validation operation verified · Evidence: C-3442*

The canonical sgLogG2Dim serializer4C596040 has a selected LOAD gate for the two unsigned16 tangent-condition words at runtimeF8/FC: they are consumed only when the effective version query is at least0x3CA, and present values are zero-extended into the current runtime words. The older branch skips them without a direct reset in the selected continuation. STORE writes their low16 values without that same old-version omission. Existing modern raw-word roles and admission remain unchanged; constructor/reused/pending-object state is separate from the skipped-word checkpoint.

A subsequent separate effective-version query below0x535 enables a compatibility check only when current dcm_type80 equals exact6. The operation obtains entity0/1 through the original owner's actual virtual3E8/4A0 queries, with the second handle at offset28. For a reached entity whose numerical virtual120 kind equals3, a zero named sgEntHandle::isModifiableSpline result can select a literal1 assignment to current invalidated94. The first qualifying failure directly selects that assignment; otherwise the second entity can be checked. The numeric codes6 and3 are retained here rather than promoted to a universal enum from this branch alone.

The two version queries are independent. A combined presence/validation interval would require stable effective-version metadata across them; this proof does not assume it for arbitrary callbacks. Repeated handle resolution, current arrays/counts, providers and object lifetime remain dependencies. Missing count/storage paths can yield null and later use it through a virtual call, so this is not a safe validation routine for arbitrary malformed state. isModifiableSpline's zero/nonzero test is not a closed return-value enum or a geometry-success promise.

The invalidated94 assignment is local current-state behavior, not an all-path final-state or rollback guarantee. This compatibility block adds no inline archive fields and grants no complete old body/class admission. Existing valued-G2 Quadrant/Chamfer meanings are not transferred to this logical class. Retained bytes and an independent review support these selected branches with no source/container/baseline replay, frozen-proof rerun, CAD/COM/native execution, production parser/accounting change, new source population or byte/global coverage credit.

### 11.1760 Logical-G2 legacy suffix presence, type and runtime-default matrix

*Status: exact canonical LOAD gates and local assignment policies verified; inherited/old containing-body admission separate · Evidence: C-3445*

The canonical logical-G2 serializer4C596040 extends the earlier word/validation qualification in11.1757 with the following ordered LOAD suffix checkpoints. Each row queries the current effective version separately, using its provider/fallback path; the table is not a claim of a single immutable version value across arbitrary callbacks.

| LOAD query requires | Reached serialized suffix | Local policy |
|---|---|---|
| >=0x684 | Generic object-pointer read for current108 | Null is accepted; nonnull must pass the current runtime-class compatibility predicate against su_CObArray. Type failure reaches error/int3 before108 assignment. |
| >=0x9F8 | u16 alignment100 | Zero-extends the raw word. |
| >=0xC5C | signed32 loop indices110/114 | Two independent long codecs, in order. |
| >=0xCD1 | binary64 help parameters118/120 | Also assigns binary64+1 to current128/130; those assignments consume no additional doubles. |
| >=0xDCD | Three requested-class object reads | Each independently routes through current DB/ordinary ReadObject. Nonnull receives owner association/setter/temporary cleanup; null is passed to its setter. |
| >=0xF10 | u16 stop150 | Zero-extends the raw word. |
| >=0x46CC | signed32 word154 | Retains its own late version gate. |

The108 LOAD checkpoint initializes its local pointer to null before reading. A type failure may already have changed codec/map state before its error path; this is not transactional rejection or a class-map rollback. Below0x684 the local read and member assignment are skipped, rather than implicitly clearing an existing current108. Skipped scalar branches similarly do not supply a universal default. Constructor, reused object, pending override, earlier base/provider effects and callbacks remain separate.

The three object reads are not fixed-width inline handle bodies. Their current IsDBStream decisions, requested classes, actual runtime objects, setters and cleanup require valid provider/owner/lifetime conditions; pointer identity and final ownership are not inferred merely from the requested class. The selected runtime128/130 constants are separate from wire parameters118/120 and do not establish new stored fields.

STORE emits the selected pointer/alignment/loop-index/f64/handle/stop suffix without matching these old LOAD omission gates; word154 keeps its separate0x46CC gate. This is installed writer/reader asymmetry, not a claim every historical writer emitted the same older body. Special name/archive modes and inherited framing remain outside this local suffix matrix. Modern raw fields, current reader admission and all historical charges remain unchanged. No source/container/baseline replay, frozen-proof rerun, CAD/COM/native execution, production parser/accounting change, new population or byte/global coverage gain is claimed.

### 11.1763 Logical-base Derived legacy omission and explicit reset

*Status: canonical raw-word codecs, version gate and local reset verified; inherited admission separate · Evidence: C-3448*

The declared sgLogDim serializer4C595D70 first delegates to sgDim, then implements the common logical suffix. STORE emits low16 of current RefIndexA0 and low16 of current DerivedA4 without a matching old-version omission gate. On LOAD, RefIndex is always consumed as u16 and zero-extended into currentA0 before the effective-version query.

When the current archive helper effective version is below0x49 (73 decimal), LOAD consumes no Derived word and explicitly writes currentA4=0 before returning. At or above0x49 it consumes rawu16 Derived and zero-extends it into currentA4. Effective-version780 equal0 invokes the named version-manager fallback; the query is current metadata, not proof of a lifetime-constant archive version. A null provider reaches an error/int3 after the first word was already read and assigned, so no cursor/map/member rollback is implied.

This is an explicit default for this skipped field, unlike suffix branches that merely omit an assignment. It is not a general reset rule, enum/Boolean domain or geometry role. The selected LOAD suffix occupies two or four bytes after the inherited base; neither that local extent nor STORE asymmetry admits an older containing object or specifies its inherited size. Canonical declaring-class bindings, concrete descendants, archive overrides and object/provider lifetime remain separate. Raw fields and existing modern scope are retained; no parser/accounting change, source replay, native execution, population admission or coverage credit is claimed.

### 11.1765 Linear-pattern legacy suffix gates and runtime default matrix

*Status: canonical retained suffix and codecs verified; legacy/context admission separate - Evidence: C-3450*

The sgLinearPattern serializer0x4C5954C0 delegates to the distinct sgPattern base and obtains current archive-helper effective version780, with version-manager fallback when zero. It latches that version in nonvolatile EBX before STORE/LOAD dispatch. These gates share one latched value under normal ABI preservation, not separate queries or lifetime-constant metadata. STORE reuses RBX at0x4C595A03 after its last version gate; LOAD bypasses that STORE-only reuse, so no whole-body RBX liveness is implied. Missing provider reaches error/int3 after the base has run; no rollback is implied.

| Threshold | Selected LOAD suffix | Selected STORE policy |
|---|---|---|
| >=0x4EC | raw u16 A0, A4, zero-extended to runtime dwords | same gate; low16 of both members |
| >=0x4BA | handle reads copied to embedded20/48, then raw u16 at70 | same gate; ordinary and special-context routes distinct |
| >=0xCE8 | raw u16 D0/D4, array helpers E0/E8 | emitted without this LOAD threshold |
| >=0x1039 | requested-class object F8; nonnull output receives current owner10 at its88 | emitted without this LOAD threshold |
| >=0x131C | f64 StepF0 | emitted without this LOAD threshold |
| >=0x27AD | raw u16 D8 replaces runtime default | always low16 D8 |

Before the optional D8 read, LOAD explicitly assigns currentD8=0 for unsigned latched versions below0x2710, or1 otherwise. At>=0x27AD the stored rawu16 replaces that default. Runtime defaults consume no input and do not close the stored domain. Other skipped groups do not directly reset their members in the local omission branch; constructor, reused-object, override and callback state remain separate.

Handle LOAD selects current DB/ordinary codecs, copies results into embedded handles, may destroy nonnull temporaries and associates current owner. No fixed inline object/tag width, universal runtime-class identity, DB/MFC equivalence, callback preservation or final lifetime state is inferred. Array helpers and inherited base retain their independent grammars. STORE asymmetry does not establish a symmetric canonical historical writer. There is no expanded class/version/context admission, geometric enum/unit interpretation, source/sample replay, native execution, parser/accounting change, population or coverage credit. Historical charge and phase scopes remain unchanged.

### 11.1767 Circular-pattern latched-version legacy suffix matrix

*Status: canonical serializer, current version latch and exact codecs verified; legacy admission separate - Evidence: C-3452*

The canonical sgCircularPattern primary0x4D1B98E8 selects serializer0x4C58FDB0. Current effective version780, with version-manager fallback when zero, is latched in R14D before the distinct sgPattern base call. All local full/subregister writes preserve that value through the suffix gates under normal ABI behavior; this is not provider immutability or a global version-history assertion.

LOAD always consumes rawu16 at98/9C/A0. It reads A4 only from0x4EC. The centre point atA8 is restored on both paths. On the older path, versions below0x4BA then return without the later suffix. For versions admitted past that branch, the retained matrix is:

| Threshold | Selected LOAD fields | Selected STORE policy |
|---|---|---|
| >=0x4EC | A4 rawu16 before centre point | same gate |
| >=0x4BA | handles20/48 and rawu16 at70 | same gate |
| >=0x4C6 | handleC8 and rawu16 C0 | same gate |
| >=0x4EC | rawu16 F4 | same gate |
| >=0x509 | handleF8 and rawu16 at120 | same gate |
| >=0xCE8 | rawu16 at124 and typed moCircleRef pointer helper at128 | selected later suffix emitted without this LOAD gate |
| >=0x103D | requested-class object at130; nonnull result receives current owner10 at its88 | selected later suffix emitted without this LOAD gate |

Rawu16 inputs are zero-extended into current runtime words; STORE emits low16. No Boolean/index/enum domain or offline normalization follows. Omitted fields receive no inferred local replacement/default; constructor, reused/copied object and callback state remain separate. The old early return occurs after the centre-point read, not before it.

Current DB/ordinary object dispatch, handle copying, temporary destruction, typed reference helpers and owner association do not imply fixed tag widths, canonical runtime replacement policy, map equivalence, null safety, rollback or final lifetime state. The base and special archive contexts remain independently qualified. STORE asymmetry does not prove a symmetric canonical legacy writer. This matrix expands no parser class/version/context admission and claims no geometry/frame/unit interpretation, source/container/baseline replay, native execution, population, accounting change, coverage credit or phase advance.

### 11.1769 Shared pattern-constraint legacy optional words and current increment

*Status: canonical shared serializer, raw codecs and selected runtime operation verified; old admission separate - Evidence: C-3454*

The canonical primary tables for sgLinearPattCon and sgCircularPattCon both select serializer0x4C599730. It first delegates to the logical-G2 base, then implements the shared suffix. STORE and LOAD each obtain their own current effective-version result after the initial five words, using helper780 or the version-manager fallback when zero. The selected version value in ESI supplies the subsequent gates within that direction under normal ABI/provider preservation; no cross-operation or lifetime-constant version is inferred.

| Suffix order | STORE and LOAD presence | Wire fields |
|---|---|---|
| 1 | always after the inherited base | five rawu16 words at158/164/168/16C/170 |
| 2 | version>=0x4FF | sixth rawu16 word at174 |
| 3 | version>=0x12CA | signedlong32 pair at15C/160 |

LOAD zero-extends the rawu16 values into current dwords; STORE writes low16. The signedlong32 pair preserves all bits. Neither the save-key index/scale names nor the native gates close a Boolean, enum, index or scale domain. Omitted optional fields receive no implied local reset/default; constructor, reused/override object and callback state remain separate.

Before the later optional wire reads, LOAD with current version>=0x4BA and current pattern-constraint member98 nonzero invokes virtual0x4B0 on that same original pattern-constraint receiver and directly increments the dword at returned-pointer1C. There is no null-result guard before the INC32 and native32 overflow is possible. This is a current runtime operation, not an extra wire field, counter-type/ID meaning, ownership guarantee or final-state/persistence invariant. The inherited logical-G2 call may already have changed current state; member98 is not asserted equal to an unchanged archive value.

Missing helper reaches error/int3 after the initial suffix words have already been consumed/assigned; no rollback follows. Later providers, aliases or callbacks can change state. Canonical table binding leaves pending overrides, object maps and caller contexts separate. No older containing-class admission, geometry/unit interpretation, source/container/baseline replay, native execution, parser/accounting change, measured population, coverage credit or phase advance is claimed.

### 11.1770 Logical-group pre-base gate, stored count and set-reset policy

*Status: canonical body gate, independent STORE traversal and LOAD mutation order verified; containing admission separate · Evidence: C-3455*

The canonical sgLogGroupDim serializer obtains its current effective version before delegating to the logical base. On the selected version-below0x46CC branch it returns before the base and local suffix codecs. The helper/provider lookup and version-manager fallback remain opaque context machinery: this local codec omission is not a universal no-effect/no-wire assertion about those calls, nor admission of an older containing class. The missing-provider error/int3 path remains distinct.

At or above0x46CC, STORE emits low16 of the current full B0 count and then independently traverses the current A8 set through its sentinel, emitting each reached node1C as signed32. The emitted u16 count is not the native traversal bound. Count/container consistency, valid sentinel/tree structure, normal codecs and stable current iterator storage are prerequisites rather than automatic invariants. More than65535 current elements can therefore produce a count/traversal mismatch; this does not establish an offline truncation rule or a larger decoded wire count.

LOAD consumes the rawu16 count first, then clears the existing tree and resets its links and size before reading the first signed32 key. A zero count still clears that current tree and reaches the final receiver call. The loop consumes every indicated signed32 value; duplicate keys can be recognized and omitted from insertion while their wire occurrences remain consumed. This is a runtime set operation, not a dictionary-based loss of raw input order/duplicates in the file schema.

Read, allocation or provider failure after clearing or partial insertion can leave partial mutation. There is no transaction, cursor/map rollback, final-state or persistence guarantee. Selected positional lookup and runtime-cache branches of the reached final helper4C426760 are qualified in11.1776; its remaining effects and final cache state remain separate. Full signed key bits and current signed comparisons do not establish a relation-index range, enum, geometric group meaning or global uniqueness policy.

This clause refines the local body policy associated with11.44, without changing its direct collection framing or admitting unsupported legacy/base/override contexts. Provider callbacks, aliases, current object/tree identity and owner lifetimes remain qualified. No new tagged wrapper, whole-body extent, parser/accounting change, source/container/baseline replay, native execution, population, byte/global coverage credit or phase advance is claimed.
