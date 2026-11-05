### 11.148 Ordinary modern circular-pattern suffix

*Status: B — native-derived framing with native dimension alternatives; raw fields and populated integer collections remain partial.*

moCirPattern_c primary table `4D023470`, locator `4D340CD8`, selects serializer `4BB219E0` in sldmodu.dll. Runtime descriptor `4DAD3720` selects factory `4BB168A0` and constructor `4BDA52B0`. The constructor calls the shared pattern constructor. Allocation size 0x1210 is not wire size. Common concept hooks select `4C189B60` and `4C18A920`; virtual +0x870 selects `4B29C5A0`, whose bytes return zero, establishing the shared variation layout used here.

The following grammar applies to ordinary non-DB external-XML loading with v > 0x27CB and archive condition 0x800 clear. It follows the complete shared pattern base (11.94–11.96, 11.144), including its variation and versioned tail. No linear- or mirror-pattern derived suffix intervenes.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | dimensions +1130, +1138, +1140, +1148 | Four tagged moDisplayDim_c pointers | Always in scope | Load uses generic dimension declarations; store declares +1138/+1148 angular. Preserve actual derived classes and shared identity | B |
| 2 | object +1150; line +1158 | Tagged CObject pointer; tagged moLineRef_w pointer | Always | Condition 0x800 would insert an owner i32 before +1150 and is outside this scope | B |
| 3 | raw +1160 | u8 | Always | Preserve all bits; meaning unknown | B |
| 4 | dimension +1198; raw +11A8 | Tagged moDisplayAngularDim_c pointer; u8 | Always | Raw byte has no inferred boolean meaning | B |
| 5 | dimension +11A0 | Tagged moDisplayAngularDim_c pointer | Always | Preserve null/reference/new-object distinction | B |
| 6 | raw +11AC, +11B0, +11B4, +11C0, +11C4 | Five u8 | Always | Preserve bytes in this order | B |
| 7 | map +11D0 | i32 count, repeated i32 key and integer vector | Always after dimension validation | Each vector is i32 count followed by that many i32 values; counts SHALL be nonnegative and fit available bytes | B |
| 8 | vector +1200 | i32 count, repeated i32 values | Always | Independent count; same bounds rule | B |
| 9 | point +1168; vector +1180 | Three f64; three f64 | v > 0x3EFA | Direct geometry bodies without object tags | B |

The canonical moCirPattern_c body has the following independently bound own-class property associations:

| Runtime member | Native property | Saved representation |
|---|---|---|
| 1160 | FlipDirection | u8 |
| 11C4 | Dir2Check | u8 |
| 11C0 | Symmetric | u8 |
| 1138 | Spacing | tagged display dimension |
| 1148 | Spacing2 | tagged display dimension |
| 1198 | TotalSpacing | tagged display dimension |
| 11A0 | TotalSpacing2 | tagged display dimension |

FlipDirection and Dir2Check getters/setters move raw full 32-bit values. The Symmetric getter instead projects the whole current DWORD's zero/nonzero state to a C++ bool in AL, while its setter stores the full 32-bit input. All three archive fields STORE the low byte and reached LOAD zeroextends the byte to full 32-bit. The Boolean getter projection is not the wire rule or a closed 0/1 file domain. Member 0x1160's LOAD gate begins at 0x90; older branches and later callbacks retain their separate policies.

The four spacing names identify current tagged-dimension slots, not new raw-double fields. Each getter returns positive zero for a null current dimension; otherwise getModelDimH(0) followed by getValue(1) supplies the result. Spacing/Spacing2 setters check the initial dimension pointer; TotalSpacing/TotalSpacing2 setters call getModelDimH without that initial null check. Dynamic provider handling and final values remain unproved. The companion typed-codec join authenticates the selected angular-dimension pointer writer; existing generic LOAD and older compatibility branches remain unchanged. Units, angle conventions, numeric domains and physical spacing/symmetry behavior are not inferred. Other bytes, maps and vector values remain partial. Evidence: C-3526.

The native post-load callback `4BDB1620` repairs dimension owners without synthesizing missing dimensions. At least one of dimension +1138 or +1198 SHALL be nonnull: both null lead to AfxThrowFileException, not a shorter valid suffix. The scoped reader rejects that state before reading the integer collections.

Map and vector counts are fixed signed i32, not ReadCount. Despite a misleading pointer type in decompilation, the map key is serialized as a 32-bit integer, not an MFC pointer tag. Native map insertion replaces duplicate keys; the offline reader preserves every serialized entry and its order. Integer keys/values and the seven raw bytes receive no semantic coverage credit. Native post-tail adjustment of the second instance count when raw +11C4 is zero consumes no additional archive bytes.

The frontier contains **43 freshly serialized circular patterns** in completed roots. Twenty-six have dimension +1138 and seventeen instead have +1198; all have +1130 and no +1140/+1148/+11A0 dimension. All have empty map/vector collections and the final geometry pair. Their raw-byte tuples are (0,0,0,1,1,0,0) in 26 cases and (0,1,0,1,1,0,0) in 17. Populated collections, signed extrema, duplicate keys and noncanonical bytes are covered synthetically, not claimed as populated native fixtures. Tests also cover the 0x3EFA/0x3EFB gate, inherited identity, malformed/truncated inputs, missing dimensions, unsupported contexts and a hash-bound native root boundary.

The edition 0.159 frontier records **20,199 complete roots**, **406 list-complete configurations** and **500 explicit stops**, with all **906 modern model contexts** complete. **400 lists reach exact EOF**, spanning **16,919,494 decoded bytes**; six list-complete streams retain four-byte unknown tails. Leading dependencies are hole-wizard features (35), moOneContour_c (33), scale features (23), base bodies (22), sheet-metal features (21) and extruded reference surfaces (21). This traversal adds no whole-document semantic coverage credit. Full part, assembly, drawing and all installed project-format bit coverage remains incomplete.

Evidence: C-176.

### 11.158 Ordinary modern blend and blend-cut feature data

*Status: B — shared native serializer and populated numeric-column framing verified; raw meanings, optional native dimensions and older conversions remain partial.*

moBlendCut_c primary table `4D2D29F0`, locator `4D37FCC8`, and moBlend_c primary table `4D2D0348`, locator `4D37FBD0`, both select serializer `4CADA8F0` in sldmodu.dll. Both therefore use the same scoped wire reader. The serializer calls moBodyFeature_c::Serialize, then direct moBlendData_c::serialize at member +810. The full export name binds that data serializer to `4CAECAA0`; constructor `4CAE9250` establishes its embedded collection types. This is a separate data layout from moSweepData_c despite several similar control offsets.

For ordinary non-DB external-XML loading with v > 0x27CB, begin with the complete body-feature base, followed by the direct blend-data body below. Data offsets are relative to moBlendData_c, whose containing feature offset is +810. The temporary profile-feature array passed by the native caller is runtime context, not a separately serialized list inserted before this data.

| # | Data field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | declared count +418 | i32 | Always in modern scope | Preserve independently; negative values rejected | B |
| 2 | arrays +330, +310, +350 | Three direct suObArray bodies | Always in modern scope | Independent ReadCount/tagged items; this order is intentional | B |
| 3 | words +2F0 | n × u32 | Always | n is the loaded item count of array +330 | B |
| 4 | words +2F8 | n × u32 | Always | Complete first column precedes this second column | B |
| 5 | points +2D8 | n × three f64 | Always | Complete point column follows both integer columns | B |
| 6 | words +2E8 | n × i32 | Always | Final numeric column; no count or tag before it | B |
| 7 | raw +110, +114; raw +118, +120 | Two i32; two f64 | Always in modern scope | Preserve signed/numeric wire values | B |
| 8 | raw +300, +374, +378; raw +58 | Three u8; f64 | Always in modern scope | Runtime members can be wider than the byte fields | B |
| 9 | raw +60, +64 | Two i32 | Always in modern scope | Conditional pointer tests use only each word's low byte | B |
| 10 | direction +68, +70 | Tagged moDirectionSpec_c pointers | Respectively (raw +60 & 0xFF) == 2, (raw +64 & 0xFF) == 2 | Other word bits remain preserved; omit the full tag when false | B |
| 11 | lists +80, +C8 | Two direct suObList bodies | Always in modern scope | Independent ReadCount and tagged objects | B |
| 12 | raw +230; raw +420, +428 | i32; two f64 | Always in modern scope | Direct values with no wrapper object | B |
| 13 | raw +234; raw +238, +240; raw +248, +24C | u8; two f64; two i32 | Always in modern scope | +234/+248 control feature dimensions below | B |
| 14 | arrays +380, +3A0, +3C0, +3E0 | Four direct suObArray bodies | Always in modern scope | Constructor helper `4B3B9830` proves the last two types | B |
| 15 | raw +78, +7C, +304, +308 | Four i32 | Always in modern scope | Final data fields in this order | B |

The modern load path sets its numeric-column extent from **array +330's item count**, even though the first field remains a separate declared/allocation count. It reads all +2F0 words, all +2F8 words, all point triples and finally all +2E8 words. These four columns are **not interleaved per record**. Their total is 36 × n bytes, which SHALL fit the remaining archive before reading. The reader retains both counts without allocating from the separate declared value or silently rewriting it. Native store loops assume consistent declared/source counts; mismatched-count synthetic tests establish framing retention, not semantic validity or native round-trip safety.

Every direct collection uses its own ReadCount and normal object-pointer grammar. Direct arrays/lists do not occupy MFC map slots, while newly serialized pointed-to objects do. Original byte and numeric values, integer-column bit patterns, point values and item ordering are preserved. Unresolved numeric semantics receive no coverage credit. With empty collections and n = 0, the modern direct data body is **126 bytes**.

The direction predicates differ from the sweep predicates: native load instructions at `4CAED595` and `4CAED5A7` compare **byte ptr** through addresses +60 and +64. Thus i32 values 0x102 or -254 select a pointer because their low byte is 2. The reader does not replace those words with 2 or treat every nonzero word as presence. This behavior is retained in exact machine-code evidence.

Exact moBlendData_c native APIs bind the following saved members. The data is embedded at feature 0x810 in both moBlend_c and moBlendCut_c; it is not interchangeable with SweepData despite similar offsets.

| Runtime members | Native association | Wire |
|---|---|---|
| 110 / 114; 118 / 120 | indexed DraftAngle returned status and double output/input | two i32, then two f64 |
| 420 / 428 | EndTangentFactor / TangentFactor | two f64 |
| 60 / 64 | indexed EndMatchingType, EndFaceSet and EndTangentSense controls | two i32 |
| 378 | ForceNonRational and GuideInfluence components | u8 |
| 300 | BlendClosed, getter-only association | u8 |
| 374 | KeepTangency, getter-only association | u8 |
| 304 / 308 | StartApplyAll / EndApplyAll | two i32 |

Indexed APIs signextend EDX without an index bounds check; supplied indices 0/1 select the declared pairs. DraftAngle's getter writes the double through R8 before reading its EAX status result, without an output-null guard. Its setter takes status in R8D and double in XMM3. Output aliasing can change the later status read, so this is not an atomic status/value snapshot. No status-code domain or angle units follow.

EndMatchingType reads the low 8 bits of each control word; its setter replaces that low byte from R8B while preserving the upper 24 bits. EndFaceSet is logical NOT bit 0x200, and EndTangentSense logical NOT bit 0x100. Their setters clear the selected bit for nonzero requests and set it for zero, preserving all other bits. Existing tagged directions 0x68/0x70 remain controlled solely by low 8 bits==2; that structural code does not receive an inferred engineering enum name.

ForceNonRational reads bit 0 and its setter ORs the canonical nonzero-request bit into the old word. GuideInfluence returns mask0xF0 unshifted and its setter ORs (input&0xF0). Neither setter clears old components when supplied zero. Member 0x378 and getter-only 0x300/0x374 have full 32-bit current access but STORE only low bytes and reached LOAD zeroextends. ApplyAll 0x304/0x308 remain raw full 32-bit. Other bits and numeric domains remain unassigned.

The tangent-factor setter takes signed EDX index and double XMM2. Under normal nontrapping completion it compares input x with current upper U using COMISD, computes MAXSD(current lower L,x), then stores U only for ordered x>U; the JBE path stores the MAXSD result. Unordered comparison takes that path, and MAXSD's second-operand rule allows a NaN input to be stored. Unmasked invalid-operation exceptions can prevent the store. The PE bytes 0.1/10 at 4DB06F40/4DB06F48 are initializers in writable storage, not observed live values or immutable bounds. This policy is not an offline validation or universal finite clamp. Architectural instruction rules are described in Intel's [COMISD reference](https://www.intel.com/content/dam/support/us/en/documents/processors/pentium4/sb/25366621.pdf) and [MAXSD reference](https://cdrdv2-public.intel.com/774492/325383-sdm-vol-2abcd.pdf).

A bounded native value-flow trace follows the matching-type bytes beyond the BlendData accessors. On the reached valid-object path, convertToGCData zeroextends current words 0x60/0x64's low bytes into a local two-element int32 pair. It passes that pair as declared formal 17 to gcLoftData_c::initialize, which forwards it as formal 15 to gcSweepLoftCommonData_c::initialize. The latter copies the nonnull pair unchanged into runtime backend 0xC0/0xC4. Those offsets are backend memory state, not additional serialized BlendData fields; all bits of the original two wire words remain preserved offline.

The same backend's exact setEndMatchingType writes full R8D to 0xC0+4*signextendedEDX. This confirms a direct runtime setter checkpoint, not numeric enum labels or a universal backend domain. The captured caller's formal 16 gciJoinBspline argument is literal 0; its separate formal 18 H argument receives GuideInfluence's mask 0xF0. Neither adjacent declaration justifies assigning the matching bytes to gciJoinBspline or calling the guide integer that enum.

The tempting getMatchFlag alias instead reads 0x5C+4*index. Named getNeedsStartMatching/getNeedsEndMatching directly test runtime members 0xE0/0x128 and vector routes 0x90/0xA8 rather than directly reading 0xC0/0xC4; that bounded rejection does not prove absence of indirect effects. A value-dependent 0xC0/0xC4 geometry consumer or actual API translation is still needed to label the matching codes. Allocation, opaque calls, normal ABI/lifetime and intervening mutation remain prerequisites; this is not proof of final geometry or source-file/current-value identity. Evidence: C-3532.

Existing codecs, historical gates, raw keys and native later callbacks remain separate. No physical tangent units, successful loft geometry, old-class admission or current/saved identity is inferred. Runtime 0x400 MicronLoftTol is excluded. Evidence: C-3528.

After the data body, the shared feature suffix is:

| # | Feature field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +C80 | i32 | Always in modern scope | Preserve the full signed value | B |
| 2 | raw +C84, +C88, +C8C, +C90 | Four u8 | Always in modern scope | Independent raw byte fields | B |
| 3 | dimension +C70 | Tagged moDisplayDim_c pointer | Data raw +234 nonzero | Feature control at +A44 | B |
| 4 | dimension +C78 | Tagged moDisplayDim_c pointer | Data raw +234 nonzero and raw +248 == 3 | Feature control at +A58; full-word equality here | B |
| 5 | raw +C94 | u8 | Always in modern scope | Original native gate v >= 0x70E | B |

The minimum data/feature payload is **135 bytes** with n = 0, empty direct collections and absent conditional dimensions. Unsupported newly serialized nested classes remain explicit stops. Runtime profile collection, dimension-owner/post-load handling and the store-side transfer of +C10 state into a node flag add no extra load fields after this suffix.

Earlier layouts use generic/tagged profile arrays, profile-count reconstruction, alternative numeric-column formats and discarded temporary geometric collections. In particular, the modern numeric layout begins at v 0xA51, while temporary data in versions 0xA16–0xA1E has a separate format. All are below the supported modern context. These branches are retained as native evidence rather than folded into one guessed layout. Native parameter conversions and runtime cached arrays do not replace the serialized values recorded here.

The frontier contains **26 fresh native features: 15 moBlendCut_c and 11 moBlend_c**. Every declared count is 2 and every +330 array contains two items. All numeric columns therefore contain two entries; the other six object arrays and both direct lists are empty. Controls +60/+64/+234/+248 are zero in every observed native feature. Positive direction/dimension branches, populated auxiliary collections, signed/unsigned extrema and count mismatch have synthetic evidence only. Four focused tests cover column ordering, low-byte conditions, shared identity, raw bytes, optional dimensions, both composed class dispatches, malformed/context rejection and hash-bound native root ends for both classes.

Edition 0.169 records **21,151 complete roots**, **540 list-complete configurations** and **366 explicit stops** across all **906 complete modern model contexts**. **501 lists reach exact EOF**, spanning **23,976,363 decoded bytes**; **39 complete lists retain four-byte unknown tails**. Leading dependencies include domes/sketch regions/component axes (14 each), 3D profiles (13), reference points/flat patterns (12 each) and spline tangency controls (11). Full part, assembly, drawing and installed-project bit coverage remains incomplete. Object/list completion does not establish unknown semantics or grant whole-document coverage credit.

Evidence: C-186.
