### 11.154 Ordinary modern sheet-metal, bend-allowance and fixed-face bodies

*Status: B — native-derived layered framing with native allowance/fixed-face boundaries; populated native numeric grids, raw meanings and additional sheet-metal classes remain partial.*

moSheetMetal_c is implemented in sldsmmu.dll, SHA-256 `ae20d533b36f105f22de4ed5984fb2245bbf03fb91a84ae473e0941a4b9e3c54`, image base 0x3D630000. Primary table `3DC63F38`, locator `3DC77398`, selects serializer `3DB692E0`. Runtime descriptor `3DDAFC50` selects factory `3DB56E50`, calling constructor `3DB50F00`. That constructor calls moBodyFeature_c and initializes the moSheetMetalCommon_c layer with direct moCommonBendData_c +800. Memory size 0x980 is not wire length.

The grammar below applies to ordinary non-DB external-XML model archives with v > 0x27CB and clear special archive conditions. An additional **archive-helper state at +850** controls several fields. The moArchiveHelperObject_c constructor in sldmfcu.dll (`3CA561B0`) zeros nonvolatile RBP and stores it to +850 at `3CA5658D`, proving the default state is zero. The reader uses this default unless the caller explicitly supplies sheet_metal_helper_850=1; it carries that state across the inherited-map resolved-stream handoff. No file-version heuristic or byte probe selects this state. Other values are rejected. 11.254 binds nondefault setup to configuration-manager member +0x650; decoding that persisted field and broader archive contexts remain incomplete.

After the complete body-feature base, read this common/derived sequence. Lists of offsets denote separate fields in wire order.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common bend +800 | Direct moCommonBendData_c body | Always | Defined below; no tagged object header | B |
| 2 | raw +840, +844 | Two i32 | Always in modern scope | Direct pair helper `3DB6BC90` | B |
| 3 | raw +848 | i32 | Always in modern scope | Original gate v >= 0x10C7 | B |
| 4 | raw +8F0; dimension +8F8; raw +900; dimension +908 | f64; tagged moDisplayDim_c pointer; i32; tagged moDisplayDim_c pointer | Always in modern scope | Preserve raw numeric values and actual dimension classes | B |
| 5 | fixed face +910 | Tagged moFixedFaceInFlat_c pointer | Always in modern scope | Native operator delegates to ReadObject | B |
| 6 | raw +930, +950; raw +958 | Two i32; CString | Always in modern scope | Preserve complete signed/string values | B |
| 7 | raw +960, +964, +968, +96C | Four i32 | Always in modern scope | Not pointer tags or array counts | B |
| 8 | raw +978, +974 | Two i32 | Helper +850 nonzero | Preserve this reversed member-offset order | B |
| 9 | raw +97C | i32 | Helper +850 nonzero and v >= 0x2BC5 | Independent version gate within the helper-controlled tail | B |

Exact own moSheetMetal_c getter/setter pairs bind existing member 0x950 to UseSMEnvTable and 0x97C to UsesBodyMaterial. Both APIs move the full runtime 32-bit word, and the selected archive operators STORE/LOAD full signed32 patterns. The names do not establish Boolean-only domains, table selection behavior, applied material identity or geometry success.

Selected STORE writes 0x950 without a local version test; LOAD reads it only when its independently queried current unsigned effective version is at least 0xB46. Selected STORE writes 0x97C only when archive-helper state 0x850 is nonzero, without a corresponding local 0x2BC5 test. LOAD requires that helper state and a later independent unsigned effective-version query at least 0x2BC5. The intermediate 0x974 read has its separate 0x1436 gate. These LOAD gates do not imply symmetric old-writer behavior or admit additional reader contexts. Earlier skipped reads do not prove a reset or preservation guarantee.

The same exact own-class review binds UseReliefRatio to runtime 0x938 and FirstSheetMetal to runtime 0x970, but establishes no direct member codec for either in the selected canonical serializer. They are not additional saved fields here; this is not a universal nonpersistence claim across other paths. Archive-helper state is separate from the feature words. Store preparation, inherited calls and subsequent providers remain qualified; current runtime values need not equal an earlier file state. Evidence: C-3555.

With helper state zero, null short pointers, no allowance body and an empty CString, the common/derived payload is **76 bytes**. Constructor-only points, pointer arrays and other runtime members between +850 and +938 are not implicit payload fields. Store preparation `3DB549A0` and final post-load processing do not add load fields. Native versions below 0x1BF reject this feature; early fixed-face/reference and dimension compatibility paths remain outside the supported modern scope.

The direct common bend body is serialized by `3DB5D540`, called from common-layer serializer `3DB68FB0`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | dimension +10; feature +18 | Tagged moDisplayDim_c pointer; tagged moCompFeature_c pointer | Always in modern scope | Two independent references | B |
| 2 | allowance presence | u8 | Always | Any nonzero byte selects the following direct body | B |
| 3 | allowance +08 | Direct moBendAllowData_c body | Presence nonzero | Not a tagged CObject pointer or i32 presence pair | B |
| 4 | raw +00, +04; discarded words 1, 2 | Four i32 | Always in modern scope | Native loads two words into locals; offline decoder preserves all four | B |
| 5 | sheet-feature reference +20 | Tagged moCompSheetmetalFeat_c pointer | Helper +850 nonzero | No bytes for this field with default helper state | B |

Store-side reference filtering and insertion-map updates under condition 0x200000 do not remove field 1 on ordinary load or add extra wire fields. Earlier loads before v 0xBB5 convert a generic feature pointer to a component reference; modern scope uses the component-feature grammar directly.

Exact own moCommonBendData_c methods give the two saved full signed32 words qualified current-source roles. Word 0x00 selects the getAllowanceData request source: zero, or a null ancestor result, uses local pointer 0x08; nonzero with a nonnull first ancestor result re-queries the ancestor and delegates. The repeated queries need not return the same object. This is not an unconditional local-pointer getter.

Word 0x04 selects the numeric BendRadius source: zero uses local dimension 0x10 when nonnull, otherwise positive zero; nonzero requests the ancestor's BendRadius, or returns positive zero when that ancestor result is null. The separate BendRadiusDim pointer getter suppresses local 0x10 only for the exact value 1; other values return it. Thus arbitrary nonzero raw words do not behave alike across these APIs. The BendRadius setter always addresses local 0x10 and forwards the supplied double to virtual slot 0x860 with arguments 1,0, without a local null guard. The local numeric getter uses slot 0x868(0); these requests add no raw radius double.

KFactorDim returns local allowance member 0x08 only when common word 0x00 is zero, local allowance pointer 0x08 is nonnull and that allowance's word 0x00 equals 1. Otherwise it returns null. This literal test does not establish a global TableType enum label. The separate 3DBendRadiusDim request returns local 0x10 only after an opaque ancestor/type-predicate sequence; no complete 3D-class list follows. The direct common-data body is joined through the actual moSheetMetalCommon_c layer and inline member 0x800; no invented data-object RTTI or tag is needed. Ancestor calls, native lifetime, aliases and final geometry remain qualified. Evidence: C-3557.

The optional direct moBendAllowData_c body is serialized by `3DB5BF00`:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | arrays +50, +60, +70 | Three direct dynamic arrays | Always | Independently framed as described below | B |
| 2 | raw +38; raw +40; raw +00 | CString; i32; u8 | Always in modern scope | Runtime +00 is wider, but archive field is exactly one byte | B |
| 3 | dimensions +08, +10 | Tagged moDisplayDim_c pointer; tagged moDisplayDistanceDim_c pointer | Always in modern scope | Preserve nulls and shared identity | B |
| 4 | raw +30, +34 | Two i32 | Always in modern scope | Original version compatibility repairs are not applied | B |
| 5 | numeric table +78 | Product of array counts × f64 | Always | Shape is (count +50, count +70, count +60), with +60 varying fastest | B |
| 6 | raw +110, +100, +F8, +108 | Four CString | Always in modern scope | Preserve this ordering | B |
| 7 | discarded material-compatibility word | i32 | 0x2BAF <= v < 0x2BBA | Separate from the following tagged pointer | B |
| 8 | material parameters +128 | Tagged moSMParamsFromMat_c pointer | v > 0x2BAE | Static nested grammar below; production-reader admission unchanged | B |
| 9 | string arrays +D0, +A8, +80 | Three direct su_CStringArray bodies | Always in modern scope | Independent ReadCount followed by CString items | B |

Exact own moBendAllowData_c methods bind seven groups in this direct body. The typed common-data local allowance route and its existing nonnull direct serializer call establish the owner association; an ancestor-returned allowance need not be that local object. This data serializer has no newly inferred MFC object tag or vtable.

| Member | Native property | Qualified association |
|---|---|---|
| 0x08 | Kfactor | Modern tagged display-dimension pointer; getter null gives positive zero, otherwise slot 0x868(0); setter slot 0x860(input,1,0) without null guard |
| 0x10 | Allowance | Modern tagged distance-dimension pointer; getter slot 0x868(0) without null guard; setter uses the gate below |
| 0x34 | TableType | Full32 integer getter/setter and selected signed32 archive word |
| 0x40 | TableVersion | Full32 integer getter/setter and selected signed32 archive word |
| 0x30 | isOldFormat / setTableFormat | Differently named APIs read/write the same full32 word; the getter is not a Boolean projection |
| 0x38 | TableName | Declared CString output/input APIs and existing string codec, with opaque native callbacks |
| 0x128 | SMParamsFromMat / UseMaterialSMParams | Typed material-parameter pointer getter/direct setter and existing tagged codec; UseMaterialSMParams projects current pointer nonnull to 0/1, adding no wire flag |

Allowance's setter compares its original input with the authenticated readonly binary64 constant 1e-5. Ordered input at least that threshold is forwarded unchanged; lower or unordered input becomes positive zero on nontrapping execution. It then calls current dimension slot 0x860 with arguments 1,0 without a local null guard. This is not replacement by the threshold, a finite-value test, or a proved physical constraint.

Selected STORE emits the words and pointer without the corresponding historical LOAD tests. LOAD reads TableVersion at its independent unsigned version query at least 0x1D7. TableFormat reads from 0x654, otherwise its reached older branch assigns 1. TableType assigns 0 below 0x66B; from 0x66B until 0x7DD it maps a read word equal to 1 into 0/1; from 0x7DD it reads the full raw word. Other compatibility callbacks may change values. Material pointer 0x128 is read from 0x2BAF; the separate intermediate compatibility-word test remains distinct. Dimension pointers in the modern branch differ from the allocated eight-byte scalar compatibility below 0x1DA. A reached LOAD path with allowance word 0x00 equal to 2 and dimension 0x10 nonnull invokes the Allowance getter/setter migration.

TableName's getter writes through an unnamed MFC ordinal then returns a current string-header nonempty test. Its setter can skip on an ordinal comparison result; otherwise it calls opaque preparation, assignment and conditionally further callbacks when current 0x30 is nonzero. No case-comparison rule, stable copy identity, rollback or final table binding is established. Raw 0x00 remains unnamed. No closed enums, physical units, valid-file Boolean domains or geometric success follow from these property names. Evidence: C-3558.

Each dynamic array begins with element_size u32. Zero terminates that array immediately and yields no count/header payload. A nonzero size SHALL be divisible by eight, matching native load helper `3DB580B0`; it is followed by three u32 metadata words (capacity, growth, count), then exactly element_size × count bytes. Reader wrappers `3DB58190`/`3DB581C0` establish this layout. Store-side growth metadata is limited to 1 or 2; the loader preserves the serialized metadata and bounds payload reads. The offline reader retains raw element bytes rather than assuming every element is a single f64. Array counts, not raw payload byte lengths, determine field 5 dimensions. The table product SHALL fit remaining eight-byte entries before allocation; zero dimensions produce no table bytes. Grid values, dynamic payload bytes, strings and unresolved numeric fields receive no semantic coverage credit.

The nested moSMParamsFromMat_c body now has a static native grammar. Actual zero-offset primary table 0x3DBFA258 and locator 0x3DC6DA30 select the complete 542-byte native serializer routine 0x3DB684E0. Its selected body is i32 raw member 0x08, i32 SelectionType at 0x0C, wide CString SMTableName at 0x10, then i32 count and that many direct canonical thickness-range bodies. SelectionType has the native enum type moMaterialSMType_e, without recovered numeric labels. The table-name methods bind the same string member; their unnamed import behavior remains opaque.

The parent LOAD constructs moDevLengthCalThicknessRange_c records with primary table 0x3DBFA1C8, locator 0x3DC6DBC8, and complete 271-byte native serializer routine 0x3DB5E390. Each canonical direct record has this exact wire order, with no nested object/schema header:

| Member | Native property | Wire representation |
|---|---|---|
| 0x08 | FromOperatorIndex | i32 |
| 0x10 | FromThickness | f64 |
| 0x18 | ToOperatorIndex | i32 |
| 0x20 | ToThickness | f64 |
| 0x28 | UnitIndex | i32 |
| 0x2C | DevLengthCalMethodIndex | Low unsigned8 on STORE; LOAD zero-extends that byte into the runtime32 word |
| 0x30 | DevLengthFactorValue | f64 |

The canonical record consumes 37 wire bytes, distinct from its 56-byte runtime vector stride. Neither selected serializer queries an archive-helper version or emits an additional base-object body. Parent STORE derives its count from the current vector and invokes direct record serializers. LOAD zero-extends the read count bits, may reserve, constructs records and appends to its existing vector; it does not first clear that vector. A zero count therefore need not empty existing native state. Reserve/append helpers and arbitrary mutated virtual targets remain qualified. No negative-count rejection, safe allocation, interval validation, interpolation rule, operator/unit/method enum labels or material-application result is inferred.

The named moSMParamsFromMat_c::findThicknessRange request supplies a current thickness and output references for modevLengthCalMethod_e and a factor. Its reached private record helper gives the two saved operator words concrete comparison roles, using the stored bounds directly:

| Operator word | Zero | Any nonzero32 value |
|---|---|---|
| FromOperatorIndex 0x08 | input >= FromThickness 0x10 | input > FromThickness 0x10 |
| ToOperatorIndex 0x18 | input < ToThickness 0x20 | input <= ToThickness 0x20 |

These are literal COMISD/SETA(E) gates on nontrapping floating-point completion. Unordered comparisons fail the match; no finite-value check, bound-order validation, unit conversion or narrower permitted operator-code domain follows.

The parent walks its current 56-byte runtime records in order and stops on the first nonzero helper result. After a failed record it reloads current backing/end pointers and recomputes the count; this is not a stable vector snapshot or a uniqueness, sorting or non-overlap guarantee. The helper returns 0 on failed comparisons. On success it first stores the current FULL32 method word through the method output, then reads the current factor's 64 bits and stores them through the factor output, then returns 1. Outputs may alias records or each other, so the factor need not be an original pre-write snapshot. No local pointer/output validity checks, atomic output update or rollback is implied. The archived method field remains low unsigned8 with zero-extending LOAD, as documented above.

The separate methodTypeToString formatter passes numeric arguments 0x3B1D for method 0, 0x341E for method 1 and 0x341F for method 2 to an opaque imported provider; other values skip that selected call. The actual localized strings, current resource handle and import behavior remain opaque. This does not recover closed method names or forbid other raw codes. Evidence: C-3564.

These static bodies document a previously missing nested grammar without admitting new production-reader contexts or parsing fresh files. Raw parent member 0x08 remains unnamed, and historical outer version gates retain their caller context. Evidence: C-3559.

moFixedFaceInFlat_c primary table `3DC02670`, locator `3DC6E868`, selects serializer `3DB5F8E0`. In modern scope, read either a tagged moOneBend_c pointer at +08 when helper +850 is zero, or an i32 bend identifier at +10 when helper +850 is nonzero. Then read raw +14 i32, tagged moCompFace_c +18, raw +20/+28 f64 and tagged moCompEdge_c +30. The helper-dependent identifier is **not an MFC tag**. The original nondefault-state identifier gate is v >= 0x1454, always satisfied in modern scope. Native store-side bend filtering and runtime ID repair are not applied to the retained wire representation.

Sheet-metal dimensions expose the dependency moScalerDim_c. Its sldmodu.dll primary table `4D241DD8`, locator `4D3736C0`, selects serializer `4C83D2C0`; RTTI includes Dimension_c. Native modern order is the Dimension_c base, three points, optional pair of vectors, optional compact transform and optional points selected by a u16 mask. This is the existing geometric-dimension grammar in 11.28. Scoped dispatch reuses that reader, and dimension-handle validation now accepts this proven class without weakening unknown-class stops.

The frontier contains **19 fresh sheet-metal features**, all under default helper state zero and all with direct allowance bodies whose grid shape is (0,0,0). Eighteen fixed-face pointers are null; one points to a native moFixedFaceInFlat_c helper. No positive native populated grids or nondefault helper state are claimed. Five focused tests cover populated synthetic grids, dynamic-array widths, material-compatibility boundaries, preserved strings/raw bytes, both helper branches, tail gates, pointer-versus-ID fixed-face framing, scalar-dimension dispatch, helper-state handoff, malformed inputs and a hash-bound native feature/fixed-face root end.

Edition 0.165 records **20,752 complete roots**, with **492 list-complete configurations** and **414 explicit stops** across the **906 complete modern model contexts**. Exact-EOF results remain **453 lists / 20,512,540 decoded bytes**, with **39 complete lists retaining four-byte unknown tails**. Eighteen newly reached paths stop at moSMBaseFlange_c; sweeps (17), face/point reference-plane data (16) and blend cuts (15) also remain leading dependencies. Full part, assembly, drawing and all installed project-format bit coverage remains incomplete. Completing these roots does not establish all field semantics or add whole-document coverage credit.

Evidence: C-182.
