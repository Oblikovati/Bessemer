### 11.250 Mitered flanges and sketch-associated handle arrays

*Status: B / ? — native serializer, factory/constructor and populated containers in two complete feature roots verified; engineering scalar domains and full document coverage remain partial.*

`moSMMiteredFlange_c` selects serializer `0x3DB67BD0` through primary vtable `0x3DC3EE50` and locator `0x3DC73EF0` in hash-bound installed `sldsmmu.dll`. Its ancestry includes `moSheetMetalBendGroup_c`, and it calls established serializer `0x3DB68EC0`. After the inherited body feature and bend group, its ordinary modern suffix stores these fields in wire order:

1. Tagged profile-reference pointer at runtime `+0x918`.
2. Four binary64 values `+0x948,+0xC20,+0xC28,+0xC30`.
3. Eight i32 values `+0x968,+0x96C,+0x970,+0x97C,+0x980,+0xB9C,+0x984,+0xB98`.
4. Two binary64 values `+0xC08,+0xC10`, then i32 values `+0xC18,+0xC1C`.
5. Tagged display-dimension pointers `+0xC38,+0xC40`, then i32 `+0x974`.
6. Direct `suObArray` `+0x988`, direct `su_CDWordArray` `+0xBA8`, direct `su_CDWordArray` `+0xBC8`, then direct `suObArray` `+0xBE8`.
7. Four independent sketch-associated handle arrays `+0x9F0,+0xA10,+0xA30,+0xA50`, using the helper below.
8. Direct `suObArray` `+0xC58`, then i32 `+0x978`.

The three direct object arrays each use ReadCount followed by tagged entries. Each direct DWORD array uses its own established count and u32 entries. None adds a tagged container header or container map slot. With null/mapped tagged references and all collections empty, the derived suffix occupies **128 bytes**. New pointed-object bodies and collection entries add their respective sizes. Raw scalar values and binary64 bits are retained; option names, units and valid engineering combinations remain unresolved.

Runtime descriptor `0x3DDAF530` binds creator `0x3DB56520` to default constructor `0x3D9BCE00`. The constructor proves the three object arrays and two DWORD arrays above, and identifies the four helper inputs as `suPtrArray`. It also constructs a list at `+0x9A8` and other pointer arrays/geometry members that this serializer does not append as separate bodies. Its 0xC78 runtime allocation is not file length. The native serializer copies `+0xC20,+0xC28,+0xC30` into runtime `+0x950,+0x958,+0x960`; these copies add no serialized doubles.

Helper `0x3DB6F720` writes one **signed i32 element count**, followed, only for a positive count, by **one tagged sketch pointer** and that many **tagged entity-handle pointers**. It does not write a sketch per element and does not use ReadCount. Native storing searches the handles for a nonnull associated sketch, which may remain null. Native loading reads the sketch once and assigns it to every loaded handle. The decoder preserves the stored sketch reference and handle objects, including shared identity. A zero-length array occupies exactly four bytes and has no sketch tag. Negative counts and counts impossible within the remaining bytes are rejected.

The helper has three adjacent native unwind ranges spanning `0x3DB6F720..0x3DB6F85A`; the retained byte packet includes all three, including both loops. The fourth caller argument is omitted by decompilation. Exact instructions starting at `0x3DB68038` load `R8` from the feature's `+0xA50`, confirming the fourth array independently of the decompiler's signature inference.

Historical gates precede the supported modern range: a temporary double `+0x930` occurs only below `0x678`; `+0x984` starts at `0x693`; `+0xB98` above `0x69D`; the two final doubles/words/dimension pointers above `0x718`; `+0x974` above `0x832`; direct array `+0xC58` above `0xA93`; and final word `+0x978` above `0xB45`. No gate within the supported modern range removes these fields.

`sm1-remove-edges.sldprt` supplies one **27,912-byte** containing root with object-array counts 1/1/1, DWORD counts 1/0, and handle-array counts 0/1/1/1. Its second handle array contains a sketch and an arc handle; the last two contain generic entity handles with null stored sketches. `cover.sldprt` supplies one **49,755-byte** root with object-array counts 7/4/7, DWORD counts 4/0, and handle-array counts 0/0/4/4. These last two arrays also have null stored sketches. Both roots total **77,667 bytes**; source-stream and root SHA-256 values are retained in `mitered-flange-targeted-corpus.json`. Both complete feature lists reach exact EOF, at **178,088** and **116,246 decoded bytes** respectively. These byte counts describe containing roots/streams, not semantic coverage of every byte.

The 906-context audit preserves every prior complete root record and adds **seven roots**, giving **25,512 complete roots**, **827 complete lists** and **79 explicit stops**. The two new exact-EOF lists add **294,334 decoded bytes**. Totals are **749 exact-EOF lists / 62,379,911 decoded bytes**, plus **78 complete lists with four-byte unknown tails**. Frozen 0.269/0.270 checkpoints retain the comparison. These framing measurements earn no whole-document semantic coverage credit.

Three focused tests verify scalar ordering and signed-zero bits, direct empty containers, signed handle counts, one-sketch-per-array framing, duplicate pointer identity, truncation/context rejection, both native root hashes, every populated container summary and exact list endpoints. The older Sketch Bend regression now checks completed mitered-flange roots at its former stops; historical research remains frozen to checkpoint 0.269. Complete part/assembly/drawing/all-installed-project semantic bit coverage remains open.

Validation: all **914 regression tests pass** (145.170 seconds); specification lint reports zero findings (474 headings, 96 anchors).

Evidence: C-292.
