### 11.184 Helices, reference-curve wires and wire-edge identities

*Status: B / ? — native modern field order, seven populated helix boundaries and present kernel wires verified; raw controls, optional engineering semantics and full coverage remain open.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moHelix_c primary table `0x4D2E8ED0` selects serializer `0x4CB7EBE0`. RTTI establishes moReferenceCurve_c ancestry; the inherited serializer is `0x4C284E20`, selected by reference-curve primary table `0x4D1674A8`. That serializer first calls the shared reference-geometry helper `0x4C283A00` from 11.100, then restores the wire through `0x4C287C20` and reads the direct list below. This is a reference-geometry family, without the derived body-feature extrusion/revolution suffix.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common reference geometry | direct shared body | always | Existing versioned feature/atom/reference grammar and its dependencies. | B / ? |
| 2 | wire +710 | kernel archive body | always in this ordinary scope | Includes suppression byte and existing strict compressed-Parasolid framing. Present bodies are decoded, not skipped as opaque spans. | B / ? |
| 3 | wire node ID +718 | i32 | always in scope | Native load resolves a runtime edge using this stored ID; retain the signed word offline. | V / ? |
| 4 | attributes +728 | direct suObList body | always in scope | Packed ReadCount and tagged objects. No list class tag, pointer-presence word or extra list map slot. | B |

The reference-curve constructor `0x4C2CDA80` invokes the imported suObList constructor at member +728; the concrete helix constructor `0x4CB79740` calls that reference-curve constructor. This establishes the direct virtual list type. Native storage gathers/restores runtime body attributes around the wire; these actions do not add a second wire or list. Historical wire gates and the list gate at `0x12B2` all precede the supported minimum. Special archive conditions that suppress wire storage are outside this scope.

After the inherited reference-curve body, the helix suffix is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +778/+780 | i32; f64 | always | Retain the signed selector and double. | V / ? |
| 2 | data +788 | direct helix-data body | always | Two i32 at data +00/+04; six f64 at +10/+18/+20/+28/+30/+38 (NumRev at +20); two i32 at +40/+44; three f64 at +48/+50/+58; then u8 variable-pitch presence. There is no data class tag or pointer tag. | V / ? |
| 3 | variable-pitch +60 within data | direct variable-pitch body | presence byte nonzero | i32 at +00, followed by five direct dynamic f64 arrays at +08/+18/+28/+38/+48. Preserve the raw presence byte. | B / ? |
| 4 | dimensions +790/+798/+7A0 | three tagged moDisplayDim_c-family pointers | always | Separate pointer occurrences, including repeated or null values. | B |
| 5 | dimension +7A8 | tagged moDisplayDim_c-family pointer | `abs(data raw +28) > 1e-6` | Strict comparison against the native double at `0x4CDD72C8`; the sign-clear mask at `0x4CD9B130` proves absolute-value handling. Equality omits this pointer. | B |
| 6 | raw +7C8; dimension +7B0 | i32; tagged pointer | always | Preserve exact sequence after the conditional pointer. | V / ? |
| 7 | dimension-table presence | u8 | always | Only value **1** enters the table. Other stored values do not cause table reads in native loading. | V / ? |
| 8 | table +7C0 | direct dimension table | presence byte equals 1 | Signed i32 row count, then four tagged dimension pointers per row. When row count is positive, append one additional tagged dimension pointer at table +60. An empty table has no terminal pointer. | B |

Exact moHelix_c::getNumRev reads the binary64 value at current feature 0x788->data 0x20. This names the existing definition-data double NumRev; it does not impose an integer count, positivity, finiteness, units or a successful helix result. The getter requires valid current definition data and adds no null guard.

getDefData first writes the current raw feature 0x780 double to its optional output when nonnull, then separately reads/writes current feature 0x788 to its optional data-pointer output. The outputs are sequential and the returned data pointer aliases current storage. Aliasing with the receiver or between outputs can affect the later read/write; this is not an atomic snapshot or ownership transfer. The separate 0x780 double receives no new engineering label from this signature.

Exact data getVariablePitchData returns its current 0x60 pointer. Feature getDefVariablePitchData returns null when 0x788 is null, otherwise the current data 0x60. isHelixVariablePitch returns 1 exactly when both current pointers are nonnull. The selected writer derives the existing presence byte from data 0x60 nonnullness; a nonzero stored byte selects the existing variable-pitch body on loading. Current-pointer presence, raw stored presence and allocation outcomes remain distinct; no array-role, mathematical pitch-domain or current/file identity guarantee follows. Runtime TolTaperAngle at feature 0x7B8 is not added to the wire layout. Existing independent version queries and historical paths remain qualified. Evidence: C-3538.

Each dynamic f64 array uses helper `0x4BD15480`: u32 element width; zero ends the array, otherwise width 8, two u32 metadata words, u32 element count and count f64 values in the supported typed scope. Every array has its own frame; there is no shared count for the five arrays. Metadata, signed selectors and values are retained before native historical pitch reconstruction. Negative/impossible table counts and unsupported array widths stop explicitly. The pitch-load helper is `0x4CB999E0`; the dimension-table loader is `0x4CB99E70`. All of their selected modern load gates are already satisfied by `v > 0x27CB`.

The direct data body is 89 bytes when variable pitch is absent. The minimum latest-version suffix is 114 bytes with null dimensions and absent dimension table. With a 279-byte empty reference-geometry base, suppressed wire, zero wire-node ID and empty direct list, a synthetic tagged helix body before outer framing is 400 bytes. Present wires and nested dimensions add variable bodies. Optional synthetic fixtures establish wire framing and rejection behavior, not the engineering validity of arbitrary pitch/dimension combinations.

The inherited geometry dependencies require moWireEdgeIdRep_c. Primary table `0x4CF040E0` selects `0x4BBBDE70`, which calls shared entity-representation helper `0x4BBBBF50` and appends exactly one i32 at +38. The scoped reader reuses 11.18's common entity-reference body plus that signed suffix. It is not a direct kernel edge or an extra helix scalar. Pointer map identity is retained; no global class registration is added.

Native runtime descriptor `0x4DB083C0` selects factory `0x4CB79320`, which allocates 0x878 runtime bytes and calls helix constructor `0x4CB79740`. The allocation size is not a file length. Retained evidence verifies three primary RTTI/COL/type/ancestry bindings, ten native bodies, inherited/helper calls, the exact threshold/mask, the direct-list constructor import and the factory/constructor call.

The audit completes **seven native helix roots**. All seven contain present decoded kernel wires, wire-node ID 6 and empty attribute lists. Variable-pitch presence is zero, data +28 is positive or negative zero, the optional +7A8 dimension is absent and dimension-table presence is zero. The optional array/table branches have synthetic tests, not populated native corpus evidence. Five focused tests cover the strict threshold on both signs (including equality and NaN behavior), exact-one table presence, empty/populated tables, cached dimension identity, five independent dynamic arrays, raw signed fields, tagged minimum composition, truncation/context rejection, wire-edge identity framing and a source-hash-bound native containing-root span.

The frontier now records **23,220 complete roots**, **697 complete lists** and **209 explicit stops** across 906 complete modern contexts: 12 more complete roots than Edition 0.194, with unchanged list counts. **635 lists reach exact EOF**, spanning **35,257,972 decoded bytes**; **62 complete lists retain four-byte unknown tails**. `resolved-feature-list-frontier-0.194.json` preserves the preceding checkpoint. Later dependencies include component-reference curves and sketch-intersection records. Whole ResolvedFeatures, part, assembly, drawing and all installed-project bit coverage remain incomplete.

Evidence: C-212.

### 11.227 Sew-reference surfaces

*Status: B / ? — native-bound modern layout and three complete roots verified; scalar meanings, legacy variants and whole-document semantics remain partial.*

This clause covers `moSewRefSurface_c` in ordinary external-XML model archives with file version greater than `0x27CB`. The primary vtable in the installed `sldMoPartu.dll` selects serializer `0x180228C70`. It calls the body-feature base, then serializes the owned data addressed by runtime field `+0x800`. Runtime offsets below identify native fields; they are not archive offsets.

The payload is, in order:

1. Five direct `suObArray` bodies at owned-data offsets `+0x30`, `+0x50`, `+0x70`, `+0x90`, `+0xB0`. Each uses ReadCount followed by that many tagged object references in the inherited object map. There is no additional tagged wrapper for these direct arrays.
2. Six four-byte flag words. Retain their exact stored values. The loader derives runtime flags at `+0x00`, `+0x04`, `+0x08`, `+0x0C`, `+0x10`, `+0x14`: the first is true when word 0 is zero; each remaining flag is true when its corresponding word is nonzero. The third word overwrites the temporary `+0x08` value initially derived from word 0.
3. Three binary64 values in runtime-field order `+0x18`, **`+0x28`**, **`+0x20`**. Their named roles are KnitTolerance, MinGapRange and MaxGapRange respectively, under the qualified property/codec rules below. Preserve raw bytes, including signed zero and NaN representations.

Exact own moSewRefSurface_c methods operate through the current owned-data pointer at feature 0x800. They name the following existing runtime slots.

| Data member | Native property | Runtime access |
|---|---|---|
| 0x00 | MakeSolid / MakeSurface mode | Predicates test full mode equal to 0 / 1 respectively |
| 0x04 | DoNotIterateByIncreaseTol | Full32 integer getter/setter |
| 0x08 | TryToFormSolid | Full32 integer getter/setter |
| 0x0C | UseGapFilters | Full32 integer getter/setter |
| 0x10 | MergeEntities | Full32 integer getter/setter |
| 0x14 | KnitSingleBodies | Full32 integer getter/setter |
| 0x18 | KnitTolerance | Raw binary64 getter/setter |
| 0x28 | MinGapRange | Raw binary64 getter/setter |
| 0x20 | MaxGapRange | Raw binary64 getter/setter |

The archive does not store the full current mode. Its first signed32 word is STORE int(mode==0); At the selected first-word LOAD checkpoint, any nonzero stored word maps to runtime mode 0 and zero to mode 1. Each of the other five words is STORE int(current!=0); where its gated LOAD read is reached, nonzero likewise canonicalizes to 1. These writer projections do not restrict accepted stored words to 0/1. On the reached LOAD path whose independent current query is >=0xFB6, the third word overwrites the temporary TryToFormSolid assignment from the first word; older paths without that read retain their separate policy. Offline readers preserve all six raw words rather than applying native normalization.

The three named no-argument mode writers are also distinct. With nonnull current data, setMakeSolid selects mode 0 and writes feature 0x538=3 only when the mode changes; setMakeSurface selects mode 1 and writes 0x538=2 only when the mode changes. setMakeSolidIfPossible selects mode 2 without that side write. Each returns 1 for valid nonnull data, including an unchanged mode; null data returns 0 with no local write. Mode 2 is not preserved as 2 by the first-word archive conversion. No meaning for feature 0x538 or successful solid/surface creation is inferred.

The three double properties retain their exact saved binary64 values and native order 0x18,0x28,0x20. Their names establish no length unit, scale, positivity, finite bounds, min/max ordering or effective knitting tolerance. All property and archive operations are current-pointer checkpoints; they do not promise identity across calls, lifetime, final geometry or rollback. Existing historical/context gates remain separate. Evidence: C-3537.

The native owned-data constructor `0x1802281A0` constructs six arrays, including `+0xD0`, but the selected serializer writes only the first five. The sixth array must not be inserted into the archive layout. With five empty arrays the direct payload occupies 58 bytes. These named property associations leave the existing raw-field views and semantic coverage accounting unchanged.

Native historical gates are: the first two flag words after `0xF99`, the third after `0xFB5`, and the final three words plus doubles from `0x10B4`. These gates are all satisfied by this clause's supported archive context. The older reference-surface migration path below `0x8DB` is not implemented by this reader.

`moSewRefSurfIdKeeper_c` selects vtable serializer thunk `0x1803ACE78`. Its RIP-relative jump binds IAT slot `0x1803EB580` to `swccu.dll` export `?Serialize@su_CObject@@UEAAXAEAVsu_CArchive@@@Z`. This selected base serializer contributes zero body bytes. A keeper with the same apparent role in another surface class is not evidence for this layout.

Three complete native sew-surface roots total **19,704 bytes**: Mouse Bottom Housing, Mouse Main Housing Overmold and the API toaster sample. They reach explicit stops at `moSplit_c`, `moExtendRefSurface_c` and `moCompositeCurve_c`, respectively. The full 906-context audit now contains **24,632 complete roots**, **785 complete lists** and **121 explicit stops**. All prior complete root records remain identical; this adds 34 roots. Exact-EOF totals remain **712 lists / 48,774,033 decoded bytes**; 73 complete lists still retain four-byte unknown tails. The preceding and current checkpoints are retained as `resolved-feature-list-frontier-0.246.json` and `resolved-feature-list-frontier-0.247.json`. No additional whole-document semantic bit coverage is claimed.

Validation: all 835 regression tests pass (87.865 seconds); specification lint reports zero findings.

Evidence: C-269.

### 11.234 Split features, maps and split items

*Status: B / ? — native-bound modern parent layout, four populated roots and seven split items verified; option meanings, historical variants and alternate body contexts remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, `moSplit_c` selects serializer `0x180349850` in `sldMoPartu.dll`. It writes the body-feature base followed by selected members of owned `moSplitData_c` addressed by runtime `+0x868`, interleaved with parent members. There is no tagged wrapper for this owned data. Offsets below identify runtime fields, not archive offsets.

The modern suffix is:

1. Owned-data direct `suObArray` bodies at `+0x20`, `+0x48`.
2. Owned-data tagged `moProfileFeature_c` pointer at `+0x18`, tagged `moDirectionSpec_c` pointer at `+0x40`, then u32 at `+0x10`.
3. Parent direct `moBodyChooser_c` at `+0x870`, then direct `suObArray` at `+0x808`.
4. Parent i32 at `+0x910`.
5. Owned-data direct `suDWordArray` at `+0xC0`: ReadCount followed by u32 elements.
6. Parent i32 at `+0x804`, then direct `suObArray` at `+0x828`.
7. Parent integer-map count (i32), followed by that many `(i32 key, i32 value)` pairs. This is not ReadCount. The native load loop consumes no pairs for nonpositive counts. Retain the raw count and all consumed pairs. Native runtime insertion accepts only pairs with nonnegative key and value; a later accepted duplicate key replaces its earlier value. The diagnostic reader exposes both the raw pairs and this derived map.
8. Parent i32 at `+0x91C`.
9. Owned-data entity-reference keeper pointed to by `+0x78`, written **directly** as a `suObArray` body without a tagged object header or presence word.
10. Owned-data i32 at `+0x160`.
11. Parent i32 words in order `+0x924`, `+0x920`, `+0x928`, `+0x918`, `+0x914`, `+0x9D8`, `+0xA20`.

The following exact moSplit_c properties name existing parent signed32 fields; they add no wire fields or owned-data wrapper.

| Parent member | Native property | Current integer API |
|---|---|---|
| 0x910 | ConsumeCutBodies | Full32 getter/setter |
| 0x914 | DeriveResultingParts | Full32 getter/setter |
| 0x918 | OrigNumBodiesInPart | Full32 getter/setter |
| 0x91C | CopyCustomProperties | Full32 getter/setter |
| 0x920 | CutListPropLocation | Full32 getter/setter |
| 0x924 | PrevCutListPropLocation | Full32 getter/setter |
| 0x928 | CutlistPropsTransferDecision | Full32 getter only |
| 0x9D8 | SplitAllBodies | Full32 getter/setter |
| 0xA20 | PropagateVizProps | Full32 getter/setter |

These own-class accessors return or write the entire current integer without Boolean normalization. Their signed32 archive occurrences retain the existing wire order and individual unsigned LOAD gates below. Names do not establish Boolean, enum, count or index domains, actual property transfer, body consumption or successful geometry. In particular, OrigNumBodiesInPart is a named integer checkpoint, not proof that every stored value equals a current body count. The pre-0xCC0 ConsumeCutBodies load projection (value==2) is separate from the modern raw-word path. Runtime-only options and owned-data/map fields receive no transferred meanings. Current object lifetime, independent metadata queries and later native state remain qualified. Evidence: C-3536.

Constructor `0x180347BC0` establishes the parent arrays and body chooser; constructor `0x180347890` establishes the inherited trim-data layout and `suDWordArray`. The latter's vtable in `sldmfcu.dll` selects thunk `0x3CD6076B`, bound to `swccu.dll` export `?Serialize@su_CDWordArray@@UEAAXAEAVsu_CArchive@@@Z`. The entity keeper and body chooser use the established layouts from the trim-surface evidence. Other constructed storage, including parent array `+0x848` and inherited owned-data arrays `+0x80`/`+0xA0`, is not emitted by the selected parent serializer.

Both `moSplitData_c` and `moSplitBodyIdKeeper_c` select the empty `su_CObject::Serialize` import thunk `0x1803ACE78`. Their vtable serializers must not be substituted for the parent's explicit owned-data layout. The final `moNode_c::SerializeDBData` call operates on the separate DB/deferred store and adds no bytes to the ordinary archive stream described here.

Nested `moSplitItem_c` selects serializer `0x1803494B0`. Its modern layout is a counted CString name at runtime `+0x08`, a tagged visual-properties pointer at `+0x10`, and an i32 body-presence word. When absent, proceed directly to the transforms below. When present in the verified ordinary inline context, read the established kernel-body framing, followed by a direct `suObList` of attributes at runtime `+0x138`. The implemented positive path requires a fully validated compressed single-body Parasolid form; other kernel forms stop explicitly.

Finally read two direct archive-operator `mgXform_c` values at runtime `+0x20` and `+0xA8`. Each verified ordinary form is **30 u32 words**, not the compact `mgXform_c::restore` grammar. The exact imported operator is `sldmgu.dll` symbol `??5@YAAEAVsu_CArchive@@AEAV0@AEAVmgXform_c@@@Z`. Retain all 120 wire bytes per transform; a special high-nibble kind in word 28 remains an explicit stop. Runtime spacing between these fields is not their serialized size.

The split-item native branch under helper condition `0x40000` can read a runtime body handle before kernel data; this alternate context is not inferred or decoded here. A present body that yields no loaded kernel object can also return early before attributes/transforms; the diagnostic reader explicitly rejects its suppressed-body variant rather than guessing that boundary. Body naming/partition migration and pointer cleanup are runtime processing. The named parent scalar roles above do not change raw values or semantic coverage accounting. Unassigned owned-data scalars, DWORD values, map pairs and transform words retain their unresolved interpretations.

Historical split loading gates include the `+0x910` word from `0x7D4` (with a pre-`0xCC0` conversion), DWORD array after `0xCBF`, parent `+0x804`/`+0x828` after `0xE1A`, map after `0x10F8`, `+0x91C` after `0x10FF`, entity keeper after `0x1178`, owned `+0x160` after `0x12E5`, parent `+0x924`/`+0x920`/`+0x928` after `0x182B`, `+0x918` after `0x1845`, `+0x914` after `0x1BE8`, `+0x9D8` from `0x1C09`, and `+0xA20` after `0x1FBC`. Split-item transforms are loaded after `0x777` and from `0x77C` respectively. The modern scope satisfies all gates; older defaults and writer/load asymmetries remain outside this reader.

Four complete native split roots total **131,828 bytes**: Bottom Housing (27,659), Main Housing (12,649), simulation frame (87,656) and tutorial mousemultibody (3,864). The frame root includes seven split items with decoded compressed Parasolid bodies and paired fixed transforms; compressed/inflated hashes are retained. Parsing continues to `moPunch_c`, `moTemplateSheetMetal_c` or `moCutThin_c` stops. These results establish selected framing without claiming whole-document or all-project-format semantic bit coverage.

The frame items exercise six empty attribute lists and one populated list with six entries. The full 906-context audit now contains **24,944 complete roots**, adding 90 with every preceding complete root record unchanged. Totals remain **793 complete lists / 113 explicit stops**, including **720 exact-EOF lists / 53,189,313 decoded bytes** and 73 complete lists with four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.253.json` and `resolved-feature-list-frontier-0.254.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 856 regression tests pass (92.585 seconds); specification lint reports zero findings.

Evidence: C-276.
