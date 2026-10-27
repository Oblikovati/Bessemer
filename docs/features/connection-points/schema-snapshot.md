### 11.165 Connection points, route points and configurable parameters

*Status: B — native modern layouts, direct parameter types and populated corpus boundaries verified; raw values and runtime routing semantics remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moConnectionPoint_c primary table `0x4D194010` selects `0x4C3BCFB0`; moRoutePoint_c primary `0x4D18FC48` selects `0x4C3BD8A0`. Both serializers call moSketchPntFeat_c helper `0x4C1072E0`, which calls common reference geometry `0x4C283A00` (11.100) and then reads one tagged moPointRef_w pointer at +6F0. This inherited point-feature layer contains no reference-point data object from 11.163. Native member offsets below identify fields, not byte positions or alignment requirements.

After the inherited layer, the connection-point suffix is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | line reference +710 | tagged moLineRef_w pointer | always | Preserve cached/new/null framing. | V |
| 2 | raw +73C; vector +6F8 | i32; three f64 | always | Preserve complete signed word and vector coordinates. | V / ? |
| 3 | dimension +740; raw +790 | tagged moDisplayDim_c pointer; i32 | always | Pointer can introduce a derived radial display dimension. | V / ? |
| 4 | four configurable parameters | four direct bodies | always | Native member order on wire is +798, +7F8, +7D8, +7B8. Each body contains two CStrings; no class/pointer tags or map entries. | V / ? |
| 5 | compatibility doubles; raw +818; compatibility word | two f64; i32; i32 | always | Retain both temporary doubles and the temporary word even though native loading does not preserve all of them in the feature object. | V / ? |
| 6 | string +820; compatibility string; temporary configurable parameter | CString; CString; direct two-CString body | always | Consume and retain all four string occurrences, including empty values. No parameter presence word or tag. | V / ? |
| 7 | dimensions +748/+750; raw +760; vector +768 | two tagged pointers; i32; three f64 | always | Preserve null/repeated pointers, raw word and coordinates. | V / ? |
| 8 | line reference +718; raw +780; dimension +788 | tagged pointer; i32; tagged pointer | always | Exact sequence follows row 7 without a +758 dimension in this modern scope. | V / ? |
| 9 | raw +81C; string +828 | i32; CString | always | Preserve stored values. | V / ? |

The all-null, empty-string connection suffix is 117 bytes: six pointer tags, 92 numeric bytes and 13 empty CString encodings. The inherited +6F0 point pointer adds two bytes when null, so 119 bytes follow the common reference-geometry body in that fixture. There is no outer length, padding or count for the four direct parameter members.

moConfigurableParam_c primary table `0x4CECB3F0` selects `0x4B8D6940`. After its zero-byte ordinary binary su_CObject layer it reads exactly two CStrings at native +10 and +18, retained as raw fields. New tagged instances use normal MFC map entries; the direct embedded/temporary bodies do not. Connection runtime descriptor `0x4DAF20A0` selects factory `0x4C3BB690`, which allocates `0x838` bytes and calls constructor `0x4C3BB960`. The constructor installs this same configurable-parameter vtable in all four embedded members, establishing the virtual dispatch type. Native store writes zero/empty defaults for the temporary compatibility values; offline load retains arbitrary stored values without asserting their semantic validity.

The connection fixtures require ThreeDRadialOrDiameterDim_c. Primary table `0x4D1B6790` selects `0x4C833200`, which calls common geometric-dimension helper `0x4C83D2C0` and appends one i32 at +F8. This field is four bytes; the angle-dimension u16 suffix is a different layout. The scoped reader and proven dimension-handle type lists admit this class. Its latest-version null/optional-empty geometric body is 111 bytes, including the four-byte suffix. Raw +F8 meanings remain partial.

The route-point suffix is only one tagged moLineRef_w pointer at +710 followed by the three-f64 vector at +6F8, totaling 26 bytes with a null line pointer. It follows the same inherited +6F0 point pointer, rather than the connection-point suffix. Native load can resolve an axis, update feature ordering, issue a warning and discard a runtime reference; offline reading retains the original stored pointer and vector. Historical route fields begin after `0x987`. All selected connection suffix gates precede the modern minimum, including the final +828 string after `0x1C00`; old conversions and the pre-`0xF64` +758 dimension are excluded.

The complete-root audit retains **23 connection points**, **11 route points** and **46 radial-or-diameter dimensions**. All four parameter names have the observed wire order Specification, MinStraightLength, EndLengthAdjustment, EndType. The temporary doubles/word are zero and temporary strings/parameter strings empty in all 23 connection points. These observations do not establish all engineering meanings or permitted values. Per-root summaries retain every compatibility field, parameter pair, reference/dimension class and raw vector/scalar.

Edition 0.176 records **22,129 complete roots**, **631 list-complete configurations** and **275 explicit stops** across **906 complete modern model contexts**. **577 lists reach exact EOF**, spanning **30,705,172 decoded bytes**; **54 complete lists retain four-byte unknown tails**. Flat patterns, weldment features, coordinate systems, drafts and reference-dimension handles remain leading dependencies. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; these boundaries receive no whole-document coverage credit.

Evidence: C-193.


### 11.29 Sketch point prefix and supporting references

*Status: Native-derived modern nonminimal-owner layout; installed point collections remain incomplete · Evidence: [C-44], F.32*

This clause applies to ordinary external-XML part archives with v > 0x27CB, a nonminimal sketch owner and neither archive condition 2 nor 4. It specifies a prefix, not the complete sgSketch body. Readers SHALL preserve the inherited object map and SHALL stop on an unsupported reference body, minimal-owner context or nonzero jet presence. Direct points do not introduce an MFC class tag or object-map entry.

The sketch begins with an i32 point count. Each entry is a u16 category followed by the direct point body below. Native category 1 selects an external-point prototype, 2 a frame-point prototype and other values the ordinary prototype; the same archive routine restores each. The next geometry collection is outside this clause.

| Order | Direct entity fields | Encoding |
|---|---|---|
| 1 | permanent mark; unknown_0c | i32; i32 |
| 2 | visual properties | direct moEntVisProp_c (11.12), nonminimal owner only |
| 3 | reference count; stored minimal flag | i32; u16 (0 in this scope) |
| 4 | color; constraint status; line type; width | three u16; direct utLineWidth_c restore type 2 |
| 5 | external entity | sgExtEnt_c pointer |
| 6 | flags_54; flags_56; unknown_50 | u16; u16; i32 |
| 7 | group; description presence; optional description | moGroup pointer; u8; CString when nonzero |
| 8 | unknown_68; unknown_74; unknown_78 | f64; u32; u32 |

The point appends a u16 mask, position, u16 point type and u16 geometric-reference count. Position contains two f64 values when mask bit 0x10 is set, otherwise three. Mask bit 0x20 denotes the unsupported minimal-owner branch. In the supported branch two sgEntHandle pointers follow, then u16 center-reference count, i32 unknown_108 and two further positions using the same coordinate width. A u16 jet-presence word follows; zero is followed by the u16 bidirectional external-reference count. Nonzero jet bodies remain unspecified. Other mask meanings are partial and all stored bits SHALL be retained.

The modern sgEntHandle body shared by sgLineHandle and sgArcHandle is u16 entity index, an i32 replacement index when the u16 equals 0x777F, then i32 reference ID and i32 DimOnCM. sgExtEnt_c contains four pointers (moRefWrapper_c, moDrView, moSkIntersectInfo_c, moSkIsoCurveInfo_c), then reference-axis CString in this scope.

moSketchExtRef_w contains the 11.16 wrapper followed by an i16 mode (-2, -1 or 0) and moBackedUpData_c pointer. moTempAxisRef_w contains that wrapper, two f64 values, a direct three-f64 point, three-f64 vector and u8 unknown_f0. For v > 0x371D it appends two three-f64 points and i32 unknown_128. These wrappers retain the existing component-reference restrictions; further component/feature classes are not implied supported.

