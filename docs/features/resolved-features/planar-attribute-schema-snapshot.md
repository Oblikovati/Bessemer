# Planar surface and attribute schema snapshot

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
| 9 | planar member 7B0 | Direct suObArray | Always | ReadCount and tagged object pointers, no outer tag or presence word | B |
| 10 | raw member 7B8 | i32 | Always | Modern scope; original gate v > 0x10D1 | B |

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


### 11.140 Attributes, definitions and scalar/string parameters

*Status: B — ordinary modern framing; units, flags and application-specific attribute/value meanings remain partial.*

moAttribute_c and moAttributeDef_c primary tables `4CFF5080` and `4CFF6F98` both select serializer `4C186D60` and the common concept lightweight/resolved hooks. Their factory constructors are respectively `4BD0FCD0` and `4BD0F9F0`. Both call moModelFeature_c(owner = null, has-list = 0). That constructor passes the flag through moFeature_c to helper `4BDB67C0`, which leaves member +0x58 null when the flag is zero. moConcept_c::SerializeResolvedData tests the member before serializing it. Consequently these classes omit the **entire direct child-feature-list body**, including ReadCount and final i32. They still have the ordinary node/concept lightweight fields and visual-properties pointer. This absence is selected by the class construction path, not by detecting zero-valued bytes. Existing feature classes retain their list-bearing grammar by default.

The following applies to ordinary non-DB external-XML loading, with special condition 0x80 clear and v > 0x27CB. Definition-class constructor state at +0x3A8/+0x3B0 does not add a derived serialized suffix.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | model-feature base | Common feature body; i32 +354 | Always | Child-feature-list container is absent as above; other inherited fields retain their existing grammar | B |
| 2 | parameters +360 | Direct moParameterList_c | Always | ReadCount then tagged parameter objects; no outer class tag or map slot | B |
| 3 | reference +380 | Tagged object pointer | Always | Native typed constructor accepts a component reference; retain actual class and nulls | B |
| 4 | definition +388 | Tagged moAttributeDef_c pointer | Always | Shared archive identity, including recursive references | B |
| 5 | reference +390 | Tagged object pointer | Always in modern scope | Original gate v > 0x65B | B |
| 6 | raw +3A4 | i32 | Always in modern scope | Original gate v >= 0x1086; meaning unresolved | B |

The moParameterList_c table `4CFF4FF0` and suObArray table `4CD8D720` select the same serializer thunk `4CBA123E`, importing SLDMFCU.dll's suObArray::Serialize. The attribute constructors establish the direct embedded parameter-list member; no presence word, extra node-list suffix or MFC wrapper slot is added. Counts SHALL fit the remaining pointer-record minimum. Parameter order, null entries, duplicates and inherited references are retained. Native session-level definition interning may replace an in-memory definition with an existing equal definition after reading; it consumes no extra bytes. The offline reader retains the serialized graph identity rather than merging definitions by name.

The parameter family uses the node base rather than the common model-feature base. Exact primary bindings are moParameter_c table `4D124378` to serializer `4C1896E0`, moScalerParameter_c table `4D124DE0` to `4C187370`, and moStringParameter_c table `4CFF8EB0` to `4C189A20`. **Scaler** is the native class spelling. All three select moNode_c::SerializeLWData at virtual +0x4B8 and a no-byte return at +0x4C0. Their supported base body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | node lightweight | Existing 11.11 body | Always | Includes its own moNodeName_c pointer and existing flags/ID fields | B |
| 2 | name +40 | Tagged moNodeName_c pointer | Always in modern scope | A second name pointer; can share identity with field 1's name | B |

Historical unit-pointer and old flag words in parameter serializer `4C1896E0` precede the supported version range. Its append to the archive helper's runtime parameter-collection array does not serialize a count or another pointer. The base moParameter_c has no additional modern suffix.

moScalerParameter_c appends f64 raw value at +0x50, then u8 override-presence at +0x60. Any nonzero presence adds f64 raw override at +0x58. The presence/value fields occur because condition 0x80 is clear and all modern versions exceed the original 0x3DA gate. Native code assigns the override to the runtime value and clears its runtime flag after consuming it; readers preserve the original value, raw presence byte and optional override separately. Its virtual +0x528 returns **one**, so the generic negative-value normalization branch is not taken for this class. Negative scalar wire values SHALL be retained. Native DB-key text identifies the stored field as Value, but units and application meaning remain unresolved; raw values receive no additional semantic bit credit.

moStringParameter_c appends one CString at +0x50 after the same parameter base. It is not a tagged string object, and it has no scalar override byte. Its contents retain the existing CString encoding, including the UTF-16 marker and encoded length.

The installed frontier retains **113 freshly serialized attributes and 67 definitions** in completed roots, with **677 scalar parameters and 259 string parameters**. One scalar is negative. All observed scalar override-presence bytes are zero; 125 string values are nonempty. Attribute parameter lists include populated mixed scalar/string arrays. Observed +380 references include moCompFace_c, moCompEdge_c and moCompBodyFeature_c; definition pointers share the archive graph and observed +390 pointers are null. Synthetic tests cover nonzero override presence, distinct raw/override negatives, duplicate inherited parameters, second-name identity, UTF-16 values, extended ReadCount, absent-child-list boundaries for both feature classes, truncation and a hash-bound native negative-parameter root boundary.

The edition 0.151 frontier records **18,720 complete roots** and **246 list-complete configurations**, of which **242 reach exact EOF**, spanning **7,069,660 decoded bytes**. Four retain four-byte unknown tails, including the newly reached Optimization/disc configuration. All **906 modern model contexts** complete; **660 lists** still stop explicitly. No attribute/definition/scalar/string-parameter stop remains at the audited frontier. The leading dependencies are moICE_c (44), chamfers (43), context-chain components (40), mirror patterns (36) and moPLine_c (35). The full part, assembly, drawing and installed-project semantic bit-coverage goal remains open.

Evidence: C-168.

