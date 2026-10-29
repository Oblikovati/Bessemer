### 11.213 Sketch bitmap feature and data framing

*Status: B / ? — native primary ancestry/base hooks and four populated feature/data records verified; image-byte interpretation and full document coverage remain partial.*

moSketchBitmap_c primary table `0x4D0F54B0`, locator `0x4D350740` selects serializer `0x4C0FABC0` and has moModelFeature_c ancestry. Its local base call is verified against that class's primary runtime table; inherited node hooks +4B8/+4C0 select `0x4C189B60`/`0x4C18A920`. moSketchBitmapData_c primary `0x4D0F7448`, locator `0x4D350820` selects `0x4C0FC200` and has su_CObject ancestry, but its serializer begins directly with its point fields rather than adding another serialized base prefix.

For ordinary non-DB external-XML archives with version greater than `0x27CB`, the complete feature body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited model feature | direct moModelFeature_c body | always in scope | Historical gate `v > 0x76F`; existing complete feature fields and raw +354 word apply. | B |
| 2 | bitmap data +360 | tagged moSketchBitmapData_c pointer | always | Preserve null/cached/new identity. Native reference-count updates add no archive bytes. | B |
| 3 | transform presence | i32 | always in scope | Historical gate `v >= 0x76C`. This is four wire bytes, not a u8 bool. Preserve nonzero signed words. | B |
| 4 | transform +378 | mgXform_c::restore body | transform presence nonzero | Existing presence/matrix, translation, scaling and flags representation. | B / ? |

The tested latest empty feature with null data/absent transform occupies **149 bytes** before outer tagged framing. A historical profile pointer before version `0x770` is outside this modern scope and contributes no placeholder.

With archive condition `0x20` clear, its data body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | point +08 | three f64 | always | Native point archive operator. | B / ? |
| 2 | raw +28/+20/+30 | three f64 | always | Preserve this wire order rather than sorting runtime offsets. | B / ? |
| 3 | raw +48/+4C/+50 | three i32 | always in scope | The first two words later determine byte-plane extent; +50 historical gate is `v > 0x723`. | B / ? |
| 4 | string +58 | CString | always in scope | Historical gate `v > 0x76B`. Preserve before native identifier parsing/counter updates. | B / ? |
| 5 | reference presence | u32 | always in scope | Historical gate `v > 0x1822`. Preserve all stored bits; any nonzero word selects row 6. | B |
| 6 | references +B8/+C0 | tagged moLineRef_w, tagged moDimRefWrapper_c | reference presence nonzero | Modern scope includes the later dimension-reference gate `v > 0x182A`. Preserve both pointer occurrences. | B |
| 7 | raw +70/+60/+64/+68/+6C | i32, i32, i32, u32, i32 | always in scope | +70 gate `v > 0xB8B`; remaining words gate `v > 0xBB0`. Exact wire order and widths are retained. | B / ? |
| 8 | byte-plane presence | u8 | always in scope | Historical gate `v > 0xBF3`. Nonzero selects row 9; retain the original byte. | B |
| 9 | byte plane +40 | raw u8 array | byte-plane presence nonzero | Exactly raw +48 × raw +4C bytes. Reader rejects negative dimensions or an extent exceeding remaining data before allocation. Pixel/alpha/channel interpretation remains unresolved; payload bytes are counted as unknown semantics. | B / ? |
| 10 | raw +80 | i32 | `v >= 0x2BD3` | No placeholder before this version. | V / ? |

Native archive condition `0x20` introduces another presence/byte-array branch for runtime +38 before row 8; that mode is excluded here. The pre-`0x76C` always-present byte-array variant is also outside the supported scope. Data bodies with no references/plane and empty string occupy **86 bytes** before the +80 gate, or **90 bytes** after it, before outer tagged framing. Raw metadata/byte-plane framing does not establish the representation of an external or separate primary image resource, and no image decoding credit is assigned from this reader.

`sketchbitmaps.read` and `sketchbitmaps.data` are installed only in copied scoped maps. Four focused tests cover conditional reference identity, offset-specific double/word order, full-width transform presence, optional restore transform, raw high-bit metadata, populated six-byte planes, old/new +80 gates, negative extent rejection, truncation/context rejection, and source-hash-bound complete native containing-root spans with metadata/point/string/plane-hash/transform-presence equality.

The audit retains **four complete bitmap features and four data objects**. All store reference presence 1, plane presence 0 and feature-transform presence 0. Three dimension-word pairs are (200,200), one is (679,277). Thus optional populated byte planes and transforms are native-branch/synthetic verified, not present in this native set. Stored strings and metadata remain preserved without assigning unresolved image semantics.

`sketch_bitmap_evidence.py` binds both primary COL/type/ancestry records and native serializer bodies, the local moModelFeature_c base dispatch and inherited hooks. Across 906 complete modern contexts the frontier now has **24,251 complete roots**, **763 complete lists** and **143 explicit stops**: ten additional roots and three additional complete lists relative to Edition 0.218. **690 lists reach exact EOF**, spanning **46,116,176 decoded bytes**; **73 complete lists retain four-byte unknown tails**. All three newly complete lists retain such tails, so exact-EOF totals are unchanged. Completed-list endpoint/tail invariants pass. `resolved-feature-list-frontier-0.218.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-241.
