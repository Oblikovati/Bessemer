### 11.214 Mirrored solids, matrix-index helpers and unsigned arrays

*Status: B / ? — native ancestry/constructor/helper bindings and four mirrored-solid roots with six matrix helpers verified; raw option meanings and full document/project coverage remain partial.*

moMirrorSolid_c primary table `0x4CFF0048`, locator `0x4D33CFA0` selects serializer `0x4BB2C2D0` and has moBodyFeature_c ancestry. Its direct base call and inherited node hooks are bound to the supported ordinary body-feature layout. Runtime descriptor `0x4DAD3D50` records allocation size `0x9F8`, creator `0x4BB170E0` and constructor `0x4BCF6540`. The constructor establishes direct suObArray members +820/+840 and suObList +880. Other constructed runtime containers are not assumed to be serialized here.

In ordinary non-DB external-XML archives with version greater than `0x27CB`, and archive reuse state +834 clear, the complete body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited body feature | complete direct moBodyFeature_c body | always | Existing inherited framing, without another tagged base object. | B |
| 2 | face +800 | tagged moCompFace_c pointer | always | Modern native saving writes null here; offline loading preserves any stored supported pointer. | B |
| 3 | raw +808 | i32 | always in scope | Historical gate `v >= 0x253`. | B / ? |
| 4 | first plane +810 | i32 presence; optional point and normal | always in scope | Historical gate `v > 0x53B`. Any nonzero presence adds two three-f64 records: root point then normal vector. Preserve the full presence word and stored geometry. | B / ? |
| 5 | arrays +820/+840 | two direct suObArray bodies | always in scope | Packed ReadCount and tagged objects, separately framed. The +820 body is omitted in native reuse mode; that mode is excluded here. | B |
| 6 | raw +9B8 | i32 | always in scope | Follows +840; historical gate for both is `v >= 0x70B`. | B / ? |
| 7 | surface +9A0; helper +9C8 | tagged moSurfaceRef_w, tagged moMatrixIndexHelper_c | always in scope | Historical gate `v >= 0x7F7`. Native pre-gate wrapper construction is runtime behavior outside this scope. | B |
| 8 | raw +9B4/+9BC/+9F0 | u8, i32, u8 | always in scope | Keep independent widths and the stored values before runtime updates. | B / ? |
| 9 | raw +9C0/+9C4 | two u8 | always in scope | Historical gates `v > 0xB71` and `v > 0xBA7`. | B / ? |
| 10 | list +880 | direct suObList | `v > 0x2F86` | Packed count and tagged members; no outer container map slot. | V |
| 11 | words +8C8 | u32 count, count × i32 | `v > 0x2F93` | Helper `0x4BB17E10` reads fixed-width words, not packed ReadCount. Preserve duplicates/order and signed entry bits. | V / ? |
| 12 | raw +9B0; surface +9A8 | u8, tagged moSurfaceRef_w | `v > 0x3745` | Two distinct occurrences under equivalent native gates. | V / ? |
| 13 | second plane +818 | i32 presence; optional point and normal | `v > 0x3760` | Same framing as row 4, independently retained. Native final cleanup consumes no further archive values. | V / ? |

The latest empty complete body occupies **294 bytes** before outer tagged framing. The suffix alone occupies 30 bytes at `0x27CC`, 32 after `0x2F86`, 36 after `0x2F93`, 39 after `0x3745`, and 43 after `0x3760`, using null pointers/empty containers/absent planes. Each populated plane adds 48 bytes. These are tested minima, not populated-feature fixed lengths. Native runtime filtering/removal of array entries does not alter offline stored-order retention.

moMatrixIndexHelper_c primary `0x4CE87EF8`, locator `0x4D316460` selects serializer `0x4C27F890`. RTTI includes moBaseBodyChooser_c, but this serializer calls su_CObject's ordinary empty base rather than serializing a chooser prefix. Descriptor `0x4DAEDE20`, creator `0x4C26FDC0` and constructor `0x4B775250` prove two direct suObArray members:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | array +10 | direct suObArray | always | Packed count and tagged objects. | B |
| 2 | array +30 | direct suObArray | always | Separate array; preserve duplicate/cached identities. | B |
| 3 | raw +08 | i32 | always | Follows both arrays. Minimum helper body is eight bytes. | B / ? |

Observed nested suUIntArray objects are defined in sldmfcu.dll, SHA-256 `9ab6733de9a90a1564f671f83b9298c3ed15f205a359849d6b2fd68c90e95290`. Primary `0x3CF2EE20`, locator `0x3CF82F60` selects serializer `0x3CB7A630`. Its body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | element count | i32 | always | Fixed-width signed count. Reject negative or out-of-bounds extents before reading values. | B |
| 2 | growth value | i32 | always | Native SetSize's growth argument; retained even for an empty array. | B / ? |
| 3 | elements | count × u32 | always | Native bulk-reads four bytes per element. Preserve high bits, duplicates and order. No packed-count escape or typed-object tag per element. | B / ? |

The unsigned-array header is eight bytes, including for an empty array. Its outer tagged object participates in ordinary identity; the direct arrays inside the helper do not add container map slots. All three readers are installed only in copied scoped maps.

Six focused tests cover both plane bodies and full-width presence words, raw byte/word retention, duplicate array entries, all suffix gates, bounded fixed-width vector/array counts, latest tagged minimum/self identity, unsigned high-bit values/growth metadata, truncation/context rejection, and source-hash-bound complete native root spans with raw/plane/helper/nested-unsigned-array field equality.

The audit retains **four mirrored solids** and **six matrix helpers** in complete roots. All four mirrored solids include the first plane and omit the second, with direct array counts 0/1. Four helpers store raw +08 = 2 and array counts 1/1; the other observed helpers store 12 with 1/1 and 7 with 2/2. These observations do not define option domains or general array-length constraints.

`mirror_solid_evidence.py` retains two model primary/ancestry bindings, two factory descriptors, seven model native bodies and the separately hash-bound unsigned-array serializer. Across 906 complete modern contexts the frontier now has **24,282 complete roots**, **765 complete lists** and **141 explicit stops**: 31 additional roots and two additional complete lists relative to Edition 0.219. **692 lists reach exact EOF**, spanning **46,159,654 decoded bytes**; **73 complete lists retain four-byte unknown tails**. Two additional lists reach EOF, adding 43,478 exact-EOF bytes. Completed-list endpoint/tail invariants pass. `resolved-feature-list-frontier-0.219.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-242.
