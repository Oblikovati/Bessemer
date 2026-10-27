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

