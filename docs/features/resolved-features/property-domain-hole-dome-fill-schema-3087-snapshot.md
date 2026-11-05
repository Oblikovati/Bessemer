### 11.151 Ordinary modern hole-wizard feature, slot-axis data and fixed plane data

*Status: B — native-derived complete scoped suffix framing and populated native parameter pairs; raw meanings and positive native slot branches remain partial.*

moHoleWzd_c primary table `4D0A5030`, locator `4D34B468`, selects serializer `4BB27330` in sldmodu.dll. Runtime descriptor `4DAD3EA0` selects factory `4BB17260`, which calls constructor `4BF80EF0`. The constructor calls base-hole constructor `4B3E7A10`, derived from moSketchedFeature_c. Memory allocation size 0xC28 is not wire size. Serializer `4B3E7CA0` establishes the complete body-feature inherited layout and one compatibility exception: **0x2F46 <= v < 0x2F68 reads an additional i32 immediately after the inherited body**, before the following suffix. Preserve this word as unknown; do not apply it outside that interval.

The following load grammar is scoped to ordinary non-DB external-XML archives with v > 0x27CB. A list of offsets in a row denotes separate fields in the listed wire order; offsets are native member identifiers, not stream positions. Each raw value is retained without native normalization. CString means the length-prefixed archive string grammar in clause 11.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +80C; object +810; raw +818; raw +820 | i32; tagged CObject pointer; f64; i32 | Always in scope | Pointer may refer to a supported reference body; preserve identity | B |
| 2 | raw +828, +830 | Two CString | Always in scope | Serialized radius/depth label storage; retain complete strings | B |
| 3 | raw +844, +848, +84C | Three i32 | Always in scope | Preserve signed values, including unknown type codes | B |
| 4 | raw +850, +858, +860 | Three CString | Always in scope | No fixed-size string buffers on wire | B |
| 5 | raw +870, +878, +880, +888, +890 | Five f64 | Always in scope | Preserve wire values | B |
| 6 | raw +868; raw +898, +8C8 | CString; two i32 | Always in scope | Ordering differs from member ordering | B |
| 7 | raw +8A0, +8B0; raw +910, +914; raw +8B8 | Two CString; two i32; CString | Always in scope | Retain values before runtime compatibility repair | B |
| 8 | raw +840 | i32 | Always in scope | Native may normalize this value after reading; offline reader preserves it | B |
| 9 | pairs +958 | i32 count; repeated (i32, f64) | Always in scope | Fixed signed count, not ReadCount; nonnegative and bounded by remaining 12-byte pairs | B |
| 10 | raw +8CC, +8D0, +8D4, +8D8, +8DC, +8E0, +8E8, +8EC | Eight i32 | Always in scope | Preserve this non-contiguous order | B |
| 11 | raw +8F0; raw +8F8, +8FC, +8E4; raw +8C0 | f64; three i32; CString | Always in scope | Raw meanings remain unresolved | B |
| 12 | raw +980, +900, +904, +908, +90C, +918, +A2C, +A28 | Eight i32 | Always in scope | Separate fixed-width words | B |
| 13 | raw +A30, +A38; raw +A40; raw +A48; raw +A58, +A5C, +A60 | Two CString; i32; f64; three i32 | Always in scope | Nonzero +A40 enables the slot-specific block below | B |
| 14 | raw +998 | i32 | v > 0x2F47 | Independent of slot enablement | B |
| 15 | raw +BB0 | i32 | v > 0x332E | Independent of slot enablement | B |
| 16 | dimension +A68 | Tagged moDisplayDistanceDim_c pointer | +A40 nonzero | Derived dimension body uses its normal tagged grammar | B |
| 17 | slot axes +BD8 | Direct moSlotCenterAxisData_c bodies | +A40 nonzero | Before v 0x2B5E, body count equals field 9 count; otherwise a separate nonnegative i32 count precedes the bodies | B |
| 18 | raw +A70, +A74, +A78; dimension +A80 | Three i32; tagged moDisplayDim_c pointer | +A40 nonzero | These follow the slot-axis bodies | B |
| 19 | raw +83C | i32 | +A40 nonzero and v < 0x42EA | Older placement; do not also read field 24 | B |
| 20 | vector +AA8 | Three f64 | +A40 nonzero | Direct vector, following field 19 when present | B |
| 21 | raw +BA8 | f64 | v > 0x3355 | Outside the slot-specific block | B |
| 22 | raw +BF8; raw +BFC, +C00 | u8; two i32 | v > 0x3F02 | Preserve the original byte even though native converts it to boolean | B |
| 23 | array +C08 | Direct suObArray body | v > 0x3F02 | ReadCount and tagged items as in 11.149; constructor proves the embedded type | B |
| 24 | raw +83C | i32 | v >= 0x42EA | New unconditional placement, including when +A40 is zero | B |

The following exact own moHoleWzd_c methods name nineteen existing saved scalar fields. Availability identifies the methods proved here; it does not imply other getters or setters.

| Member | Native property | Proved API |
|---|---|---|
| 0x80C | ExitType | Getter, declared gcEndSpecType_e |
| 0x840 | CosmeticThreadsOpt | Getter/setter, declared cos_threadsOptions_e |
| 0x848 | HoleType | Setter, declared moHoleWzdHoleType |
| 0x898 | HoleDirectionFlipped | Integer getter |
| 0x8C8 | UserChangedName | Integer getter/setter |
| 0x914 | FastenerTypeEnum | Integer getter; selected body does not use its cosmetic-thread argument |
| 0x980 | HoleSeriesStatus | Integer getter/setter |
| 0x998 | UseTipPointForHoleDepth | Integer setter |
| 0xA40 | IsSlot | Integer setter |
| 0xA48 | SlotLength | Binary64 getter/setter |
| 0xA58 | SlotDimScheme | Getter/setter, declared moSlotDimScheme_e |
| 0xA5C | SlotStartPosition | Getter, declared moSlotStartPosition_e |
| 0xA60 | SlotOrientation | Getter, declared moSlotOrientation_e |
| 0xA70 | SlotLineIndex | Integer getter/setter |
| 0xBB0 | SurfIntersectionType | Getter/setter, declared upToSurfaceType_e |
| 0xBF8 | useExistingSketch | Byte getter/setter with native bool signatures |
| 0xBFC | IncludeSketchGeomChecked | Integer getter/setter |
| 0xC00 | IncludeConstructionGeomChecked | Integer getter/setter |
| 0x83C | HoleSeriesHole | Integer getter |

The integer accessors read or write the whole current32-bit member; their archive occurrences use signed32 codecs. Enum declarations establish type names, not numeric engineering labels or closed domains. SlotLength uses raw binary64, with no units, finiteness or geometric-validity restriction inferred. The useExistingSketch getter machine code zero-extends the current raw byte and its setter writes DL; its archive STORE and LOAD separately normalize zero/nonzero. Offline decoding retains the stored byte.

setSlotDimScheme writes the raw 0xA58 argument first, then conditionally tail-transfers to an opaque update when its third argument R8D is nonzero. This is not a final-state guarantee. SlotLineIndex at 0xA70 remains inside the reached IsSlot block. HoleSeriesHole at 0x83C remains inside that block below 0x42EA, but occupies the unconditional later suffix from 0x42EA. Existing individual version queries and 0x80C/0x840/0x914 compatibility migrations remain separate; a raw archive value need not equal the final current property. No Boolean, index, count, enum or geometry domain is closed by these names. Runtime-only similarly named fields are excluded. Evidence: C-3539.

The empty fixed suffix through field 13 is 214 bytes when its pointer uses a short null tag, all strings are empty and the pair count is zero. Later gates and the enabled slot block extend that boundary. Slot counts SHALL fit the remaining minimum 72-byte direct bodies. Negative or impossible counts fail rather than being used as opaque lengths. The pre-0x2B5E branch has **no independent slot count on wire**. Direct slot bodies do not insert MFC class/object-map entries; a separately tagged moSlotCenterAxisData_c object uses the same body with its normal map insertion.

moSlotCenterAxisData_c primary table `4CF53050` selects serializer `4BB335F0`. Its body is exactly point +08, point +20 and vector +38, each three f64: **72 bytes**. No object tags occur between these direct geometric fields. The hole-wizard constructor initializes the +BD8 runtime pointer array, while each body is allocated and serialized directly by the hole-wizard load path. The +C08 suObArray is a distinct collection with independent framing.

Runtime hole-profile/sketch owner updates and final post-load dispatch consume no extra archive fields. Native compatibility handling can overwrite +910 when +848 equals 5, normalize +840, change legacy selection/type values and convert +BF8 to boolean. The offline reader preserves their serialized values. Earlier historical branches, including reference conversion before v 0x30F and old string/type repairs, are not folded into the modern reader. Raw numeric values, strings and pair elements receive no semantic coverage credit until their meanings are established.

The dependency moFixedRefPlnData_c has primary table `4D13AA60`, locator `4D3575C0`, selecting serializer `4C283D30`. It calls su_CObject::Serialize and virtual +0x150, bound to common plane-data serializer `4C2880A0`. RTTI confirms it derives from moRefPlnData_c and is not a moDefaultRefPlnData_c. Therefore its modern body is precisely the common plane-data body in 11.100, without the default plane's inserted display-root point and without an additional derived suffix. Fixed-plane border/numeric values are retained before native clamping and normalization.

The frontier contains **48 freshly serialized hole-wizard features** in completed roots. Pair counts are 1 in 35 cases, 2 in eight, 3 in three, 4 in one and 0 in one. All have slot enablement zero and empty +C08 arrays. Native +848 values include 0, 1, 2, 3 and 5. Populated slot-axis bodies, noncanonical enablement/bytes, independent dimensions and array identity are supported by synthetic tests, not claimed as positive native slot fixtures. Tests exercise each relevant gate boundary, the compatibility interval, the +83C relocation, malformed counts, truncation, fixed-plane dispatch and a hash-bound native containing-root end with multiple pairs.

Edition 0.162 records **20,584 complete roots**, **463 list-complete configurations** and **443 explicit stops** across all **906 complete modern model contexts**. **429 lists reach exact EOF**, spanning **19,975,017 decoded bytes**; **34 list-complete streams retain four-byte unknown tails**. Leading stops are base bodies (22), sheet-metal features (21), extruded reference surfaces (21), sweeps (17), blend cuts (15) and component reference axes (14). Full part, assembly, drawing and all installed project-format semantic bit coverage remains incomplete; these framed roots add no whole-document coverage credit.

Evidence: C-179.

### 11.159 Ordinary modern dome features

*Status: B — native modern framing verified; raw integer meanings and historical conversions remain partial.*

Scope: ordinary non-DB external-XML model archives with version greater than `27CB` hexadecimal. `moDome_c` uses the complete inherited body-feature layer before the following suffix. Native primary vtable `180472F88`, locator `1804EE0B0` and serializer slot +28 bind to `180264600` in the hash-qualified `sldMoPartu.dll` recorded in C-187. Offsets below name native members, not byte positions in the stream. Values are unaligned little-endian.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited body feature | body-feature record | always | Complete existing body-feature layout; no additional class tag for this layer. | Known |
| 2 | ReverseDir +858, DoEllipticSurf +85C | 2 × i32 | always | Preserve full signed words; named native integer properties below. | Partial |
| 3 | dimension +850 | tagged object pointer | always | Native expected type `moDisplayDistanceDim_c`; null and shared identities follow the inherited MFC map. | Known |
| 4 | faces +838 | tagged object pointer | always | Native expected type `suObList`; this is a pointer-tagged list, not an inline count. | Known |
| 5 | references +840, +848 | 2 tagged object pointers | always | Native expected type `moRefWrapper_c`; preserve nulls and shared identities. | Known |
| 6 | Tangency-projected word +860 | i32 | always | Preserve full word; nibble query and destructive setter below. | Partial |

Exact own moDome_c methods bind ReverseDir to current 0x858 and DoEllipticSurf to 0x85C. Both getter/setter pairs transfer the full32-bit integer, matching the existing signed32 wire words without Boolean normalization.

Tangency is a projection of current 0x860: the getter returns 1 iff (word&0xF0)!=0. Its setter is destructive: zero input writes (old|0xF0)&0x0F; nonzero input writes (old|0xF0)&0xFF. Both branches clear the upper24 bits, while preserving only the low nibble. It is not a toggle that preserves all unrelated bits, and neither the getter nor setter closes the stored word domain.

The tagged references at 0x840 and 0x848 are named DomeApex and DomeDir. Each setter calls the old nonnull reference virtual slot 0x08 with argument 1 before installing the incoming pointer. An incoming alias may be affected by that call; no lifetime, cloning or final-validity guarantee follows. DomeFaces names the current suObList pointer at 0x838. Its two typed setter overloads perform qualified clear/allocation/append operations; the list-input path appends results of each reached nonnull item virtual slot 0x108. These provider calls do not prove atomic replacement, deep-copy semantics, safe aliasing, closed item classes or final contents.

Named setHeight forwards the unchanged input double in XMM1 through current dimension 0x850 virtual slot 0x860, with R8D=1 and R9D=0. It has no null guard. This identifies a dimension-backed setter checkpoint, not an extra saved height double or a guarantee of the final dimension value. Geometry, coordinate frames, units, pointer identity across calls and successful dome creation remain unproved. The independent historical LOAD gates and later postload dispatch remain qualified below. Evidence: C-3540.

An all-null suffix occupies 20 bytes: 12 raw integer bytes and four two-byte null tags. The modern load order matches store order. Historical load branches use a single face pointer before `912`, omit +840 before `912`, omit +848 before `915`, and omit +860 through `30D`; these branches and their runtime conversions are outside this reader's modern scope. A final virtual postload call is runtime behavior, not an additional serialized field. Neither pointer expectations nor raw field offsets establish engineering semantics.

The reader frames 14 native dome features. The new reader advances the corpus to **21,177 complete roots**, **542 list-complete configurations** and **364 explicit stops** across **906 modern model contexts**. Of these, **503 lists reach exact EOF**, totaling **24,006,914 decoded bytes**; the other 39 complete lists retain four-byte tails. Remaining dependencies include sketch regions, component axes, 3D profiles and text handles. Full part, assembly, drawing and all installed-project coverage remains incomplete; this addition grants no whole-document coverage credit.

Evidence: C-187.

### 11.231 Filled surfaces and curve contacts

*Status: B / ? — native-bound modern layout and five populated roots verified; scalar meanings, historical migrations and whole-document semantics remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, `moFillRefSurface_c` selects serializer `0x1802F43D0` in `sldMoPartu.dll`. It writes the body-feature base, then serializes selected members of owned data addressed by runtime field `+0x808`. There is no tagged wrapper for this owned data. Runtime offsets below are not archive offsets.

The modern owned-data payload is:

1. Three direct `suObArray` bodies at `+0x10`, `+0x30`, `+0x50`, each using ReadCount and tagged pointers in the inherited object map.
2. One i32 at `+0x70`.
3. Direct ID-keeper body pointed to by `+0x118`: one direct `suObArray` at keeper offset `+0x48`. No tagged keeper header or presence word occurs here.
4. Three i32 values at `+0x78`, **`+0x74`**, `+0x108`.
5. One **u8** at `+0x10C`. The native loader casts this byte through signed char to a runtime int; retain both the stored byte and its signed interpretation. This is not a four-byte wire flag.
6. Five i32 values at `+0x08`, `+0x148`, `+0x150`, `+0x154`, **`+0x14C`**.

With all four direct arrays empty, this payload occupies 45 bytes. The writer changes runtime `+0x74` from 2 to 1 before emitting it. The reader retains the original stored value instead of applying this writer mutation. Other scalar meanings remain unresolved except for the scoped native property joins below.

Exact own feature getData and its property forwarders bind the following names to the current moFillSurfData_c reached through feature 0x808. The parent serializer above performs these wire operations; the empty data virtual serializer does not replace it.

| Saved data member | Native property association | Qualified current operation |
|---|---|---|
| 0x70 bits 3..5 | NumSpanFactor | Getter=(word>>3)&7; setter accepts unsigned input<=3, then replaces mask 0x38 with input<<3 |
| 0x70 bits 0..2 | Vendor | Getter=word&7; setter accepts only 1 or 2, then replaces the low three bits |
| 0x70 mask 0x1C0 | setIdVersion | Replaces that fragment with input&0x1C0, without shifting the input |
| 0x70 bits 15..17 | setContinuity | Replaces mask 0x38000 with the input low three bits shifted left15 |
| 0x70 bits 9..14 | setVendorVersion | Replaces mask 0x7E00 with the input low six bits shifted left9 |
| 0x78 | CreatedByCoonsPatch | Raw full32 integer getter/setter, signed32 archive word |
| 0x74 | TryCoonsPatch | Raw full32 integer getter/setter; writer current2->1 mutation remains separate |
| 0x10C | AlwaysMakeSurface / MergedWithParents shared status | Distinct predicates and conditional transitions below; one stored byte |

Out-of-range NumSpanFactor setter inputs and Vendor inputs other than 1/2 leave the current word unchanged. These local producer restrictions do not restrict all stored values. The named setCurrentIdVersion forwards the already positioned fragment 0x40 to setIdVersion. The latter preserves bits outside 0x1C0.

The Continuity/VendorVersion arithmetic is additionally closed through the exact shared helper: for the two constant nonzero masks it returns their lowest set-bit powers 0x8000 and 0x200 while preserving the incoming receiver and input registers. Both setters therefore write (old&~mask)|((input32*power)&mask), modulo32-bit arithmetic, including zero input clearing the selected fragment. All bits outside each selected mask are preserved at that checkpoint. This proves packed-bit placement, not numeric continuity/vendor enum labels or geometry results.

At current status 0x10C, AlwaysMakeSurface tests unsigned(status)<=1, while MergedWithParents tests status!=0. They are not independent stored Booleans. STORE writes the low byte; LOAD sign-extends the signed-char interpretation to the runtime integer. Negative and other noncanonical stored bytes retain their original offline representation.

The two status setters use an opaque helper called with the current embedded array at data+0x10, whose returned integer is denoted h below. Incoming requests are tested as zero/nonzero. Conditions referring to current status after the helper use its re-read checkpoint, not an immutable entry snapshot.

| Setter and reached condition | Local status/result policy |
|---|---|
| AlwaysMakeSurface, entry status 0 or 4 | Bypass helper and return0 without a local status write |
| AlwaysMakeSurface, helper h not1/2 | Return0; helper effects remain opaque |
| AlwaysMakeSurface, h1/2 and request nonzero | Store1, return1 |
| AlwaysMakeSurface, h1 and request0 | Store1, return0 |
| AlwaysMakeSurface, h2 and request0 | If current status1 store3, otherwise preserve it; return1 |
| MergedWithParents, request0 | Store0, return1 without helper |
| MergedWithParents, request nonzero and current status nonzero | h nonzero: preserve and return1; h0: store0 and return0 |
| MergedWithParents, request nonzero and current status0 | h1/2: store1/return1; h3: store4/return1; otherwise store0/return0 |

These are normal-path native transitions, not success guarantees for merging, surface creation or final persistence. Original receiver/storage validity, helper effects, aliases, lifetime and independent version queries remain qualified. Other saved slots and packed residual bits remain unassigned.

The static evidence includes one qualified private-setter overscan: only the entry-reachable 16 bytes at 0x1802F91A0..0x1802F91B0 are used for that setter, and adjacent retained bytes are excluded from its body claim. The additive helper proof likewise uses its exact23-byte body and excludes the nine remaining ceiling bytes. No extra wire fields or offline normalization are introduced. Evidence: C-3541.

Constructor `0x1802F3AB0` proves the three owned arrays' types. Root constructor `0x1802F3820` allocates `moFillRefSurfIdKeeper_c` at owned-data `+0x118`. Its primary vtable selects thunk `0x1803AC8FC`, bound to `SLDMODU.dll` export `?Serialize@moEdgeSurfIdKeeper_c@@UEAAXAEAVsu_CArchive@@@Z`. The target at `0x4BBBBF40` consists of `48 8B 41 48 48 83 C1 48 48 FF 60 28`: load the member vtable, advance this by `0x48`, and tail-call its serializer. Constructor `0x4B3F5CD0` proves that member is `suObArray`. This 12-byte leaf has no unwind-table entry. Its other runtime members and base storage do not imply additional wire fields.

The `moFillSurfData_c` primary vtable itself selects empty `su_CObject::Serialize`; the feature's owned-data serialization is performed explicitly by the parent function above. Calling the data vtable serializer in place of the parent layout would omit the payload.

Nested `moCurveContact_c` selects serializer `0x4CB6E990` in `sldmodu.dll`. After the empty object base, its modern payload is:

1. Direct `suObArray` at `+0x50` (established by inherited constructor `0x4CB6DBB0`).
2. Binary64 at `+0x90`, i32 at `+0x98`, tagged `moDisplayAngularDim_c` pointer at `+0xA0`.
3. Binary64 at `+0xA8`, then i32 values at `+0xB0`, `+0xB8`, `+0xC8`, `+0xCC`.
4. Tagged `moLoftSynchRef_c` pointer at `+0xE0`.
5. Three i32 words at `+0x40`, `+0x44`, `+0x48`, followed by binary64 at `+0xC0`.

The last three words are stored through unsigned writer overloads and loaded through signed overloads; retain all four-byte bit patterns. Other raw scalars and doubles likewise receive no unsupported semantic labels. With an empty array and two null pointers this direct contact body occupies 62 bytes.

`moCurveContactData_c` selects `0x4CB6EE80`: empty object base, tagged `moRefWrapper_c` pointer at `+0x18`, generic tagged object pointer at `+0x08`, then one u32 at `+0x10`. This wire order differs from runtime field order. It is not interchangeable with the shorter `moContactData_c` layout in 11.226.

Historical filled-surface loading below `0x919` uses a reference-surface proxy migration. The final owned-data `+0x148` word is read after `0xCCD`; the `+0x150`, `+0x154`, `+0x14C` words are read from `0x185C`. Contact historical gates include `+0x90` from `0xA6B`; `+0x98` and angular pointer from `0xAE9`; `+0xA8`, `+0xB0`, `+0xB8` from `0xB42`; `+0xC8` from `0xCAA`; `+0xCC` from `0xC72`; synchronization pointer from `0xC92`; three `+0x40` words after `0xCFF`; and final double from `0xD8A`. The implemented context satisfies all gates. Earlier defaults and sign-based migration are not reproduced by this modern reader.

Five complete native filled-surface roots total **9,158 bytes**: NAFEMS 3dnlg7 (1,656), NAFEMS ic9 (846), mousebase (2,316), mousemultibody (2,024) and mousesplit (2,316). Both simulation feature lists complete. The tutorial files continue to `moTrimExtEdgeIdRep_c` or `moEdgeRefIdRep_c` dependencies. Retained packets bind stream/root hashes, container counts and raw scalar values; this verifies framing without claiming full field meaning or whole-document bit coverage.

The 906-context audit contains **24,765 complete roots**, **788 complete lists** and **118 explicit stops**. All preceding complete root records remain identical. This adds 45 roots and two exact-EOF lists: 3dnlg7 (66,165 bytes) and ic9 (28,209 bytes), totaling 94,374 decoded bytes. Totals are **715 exact-EOF lists / 49,560,755 decoded bytes**; 73 complete lists retain four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.250.json` and `resolved-feature-list-frontier-0.251.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 847 regression tests pass (94.490 seconds); specification lint reports zero findings.

Evidence: C-273.
