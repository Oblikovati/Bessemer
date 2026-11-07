### 11.243 Edge flanges and nested sheet-metal records

*Status: B / ? — seven native type bindings and 11 populated containing edge-flange roots verified; raw engineering domains, positive single-bend raw-kernel branches and other sheet-metal classes remain partial.*

The supported context is an ordinary external-XML modern model archive, file version greater than `0x27CB`, with sheet-metal helper state `+0x850` equal to 0 or 1. Unsupported helper states stop. The following runtime offsets describe object ownership; they are not file offsets.

`moEdgeFlange_c` selects `0x3DB5F3D0` in the hash-bound installed `sldsmmu.dll`. It calls established bend-group serializer `0x3DB68EC0`: body-feature base, common bend data, three i32 words at `+0x840,+0x844,+0x848`, direct object array `+0x858`, and u32 `+0x878`. It then serializes one **direct owned `moEdgeFlangeData_c` body**, associated with pointer member `+0x880`, followed by direct `suObArray` `+0xAF8`. A final i32 at `+0xB38` loads only for version `>=0x372F`; older accepted versions initialize it to 0 without consuming a word. The array's older gate (`>0xB3A`) is satisfied throughout the supported range.

Constructor `0x3D8BF0E0` proves the `+0xAF8` array type. It also constructs an embedded data copy at `+0x888` and another array at `+0xB18`; the selected serializer does not append either of these as another body. Loading allocates the owned `+0x880` data object and calls its virtual serializer directly, so no tagged data-object header or map slot appears at that point. Feature flag updates, legacy sketch-constraint removal and data flags `+0xB0,+0xB4` are runtime processing, not extra fields.

`moEdgeFlangeData_c` selects `0x3DB5E4A0`. Its inherited `moFeatureData_c` serializer `0x4B50C220` stores a tagged feature-owner pointer at `+0x08`; native loading checks feature ancestry. The modern direct data body then follows this order:

1. Tagged `moSheetMetal_c` pointer at `+0x18` only when helper `+0x850` is 0.
2. Tagged distance dimension `+0xB8`, i32 `+0xC0`, then binary64 values `+0x28,+0x38,+0x40,+0x48`.
3. i32 words `+0x64,+0x6C,+0x70,+0x74,+0x7C`; tagged wrapper `+0x88`; tagged point reference `+0x98`; i32 `+0x80`; tagged wrapper `+0x90`; tagged point reference `+0xA0`.
4. i32 words `+0xAC,+0xA8,+0xC4,+0xC8,+0x68`; tagged distance dimension `+0x138`; tagged wrapper `+0x140`; i32 words `+0x148,+0x60,+0x50`.
5. i32 `+0x78` only for version `>0x2F5F` (otherwise loaded runtime value 0), then direct `suObArray` `+0x170`.
6. Tagged distance dimension `+0x198`; i32 words `+0x168,+0x58,+0x54,+0x204`; i32 `+0x240` only for version `>0x3356`.
7. Tagged wrapper `+0xD0`; i32 words `+0xD8,+0xE8`; binary64 `+0xE0`; i32 words `+0x190,+0x268`; finally i32 `+0x26C` only for version `>0x42CB`.

All lower historical gates and temporary compatibility arrays in this serializer precede the supported modern range. Its post-load updates do not consume additional fields. Constructor `0x3D8BEE10` proves the `+0x170` array type; other constructor-created arrays/maps are not additional modern wire fields. With empty direct arrays and null/mapped pointers, the data body is 156 bytes before the three late i32 gates, plus four bytes for each satisfied gate; helper state 1 omits the two-byte sheet-metal pointer. At version 19,000 this gives 168 bytes for helper state 0 or 166 for state 1. Native feature validity and engineering option domains remain incompletely modeled; the decoder retains raw values.

`moMultiEdgeFlangeDataPerEdge_c` selects `0x3DB64240`. After the empty object base it stores tagged profile `+0x08`, tagged component edge `+0x10`, i32 `+0x20` and tagged `moEdgeGeom_c` pointer `+0x70`. For version `>=0x42CC`, it also stores i32 `+0xC8`; any nonzero value adds tagged distance dimension `+0xD0`. The older geometry gate (`>0xBAD`) is satisfied throughout the supported context. With mapped/null pointers this body occupies 10 bytes before the late gate, 14 bytes with a zero late flag, or 16 with a nonzero flag and mapped/null dimension.

`moEdgeGeom_c` selects `0x3DB5F760`: i32 `+0x08`, direct vectors `+0x10,+0x28`, then direct points `+0x40,+0x58,+0x70`. Each vector or point has three binary64 components. The body is exactly **124 bytes**, with no count or presence word. `moLinearEdgeGeom_c` selects the five-byte leaf `0x3DB63370`, whose exact bytes `E9 EB C3 FF FF` jump to `0x3DB5F760`; it adds no suffix. The leaf has no unwind-table entry. Raw components and scalar bits are preserved without inferring units or geometric validity.

`moAbsoluteRefPlnData_c` selects existing plane serializer `0x4C283D30`. Its native `+0x150` data hook is `0x4C2880A0`, and its verified ancestry excludes `moDefaultRefPlnData_c`; therefore the default-plane-only display-root point is absent. The existing ordinary `moRefPlnData_c` layout and its version gates apply without an absolute-plane suffix.

`moOneBend_c` selects `0x3DB64540`, calling established common sheet-metal serializer `0x3DB68FB0` (body-feature base, common bend data and three i32 words `+0x840,+0x844,+0x848`). Its supported modern body continues with:

1. Generic tagged sketch associated with runtime handle `+0x878`, generic tagged boundary reference `+0x860`, tagged angular dimension `+0x868`, tagged dimension `+0x870`, u16 `+0x858`, i32 `+0x8F8`, binary64 `+0x8B8`.
2. i32 words `+0x8C0,+0x8C8,+0x8CC,+0x8C4,+0x8D0`; tagged dimensions `+0x8D8,+0x8E0`; i32 `+0x8E8`; u8 `+0xA10`.
3. Only for helper state 0: a generic tagged sketch associated with handle `+0x880`; if nonnull, its direct line-handle body; then tagged `suObList` `+0x888`. The direct handle uses the established entity-handle serializer and adds no object-map slot.
4. Generic tagged reference `+0xA90`, tagged component-feature references `+0xAA0,+0xAA8`, i32 `+0xAC8`, tagged conic-bend data `+0xAD0`, two direct compact `mgXform_c` bodies at `+0x900,+0x988`, then i32 words `+0xAD8,+0xADC,+0xA30,+0xB64,+0xA34`.
5. Generic tagged reference `+0xA98`, tagged component-feature references `+0xAB0,+0xAB8`, then the direct line-handle body associated with `+0x878` if its earlier sketch pointer was nonnull.
6. i32 declared raw-body count for `+0xAE0`; a second i32 count for `+0xB00` only for version `>0x335A`. Native loops read bodies only for positive counts; negative counts are retained as nonpositive declarations. Positive counts enter `mo_loadBodyFromArchiveRaw` and currently stop explicitly before any raw body is accepted.
7. i32 optional raw-body presence at `+0xB40` for version `>=0x334C`, then at `+0xB50` for version `>=0x334F`. Any nonzero presence requires a raw kernel body and currently stops explicitly.
8. i32 `+0xA88` only for version `>0x3734`, then i32 words `+0xA58,+0xA5C,+0xD54`. Lower legacy compatibility words and the helper-specific pre-`0x141E` conversion are outside the supported range.

Eleven complete containing edge-flange roots total **367,477 bytes** across five configurations. Within these completed roots, fresh-object counts are **19 each** for single bends, per-edge data, linear geometry and absolute-plane data. Every observed single-bend raw-body count and presence is zero. Exact source/root hashes, independent data/feature array counts, raw data fields and nested counts are retained. Native positive raw-kernel branches and nonnull conic-bend payloads remain open.

The 906-context audit now contains **25,413 complete roots**, **816 complete lists** and **90 explicit stops**, with all earlier complete root records unchanged. This adds 34 roots across six configurations, including one standalone single-bend root in sheet.sldprt. No additional list reaches EOF; totals remain **738 exact-EOF lists / 60,010,312 decoded bytes**, with **78 complete lists retaining four-byte unknown tails**. Next stops include `moUiBend_c`, `moSM3dBend_c`, `moCornerFeat_c` and `moFlattenBends_c`. Checkpoints `resolved-feature-list-frontier-0.262.json` and `resolved-feature-list-frontier-0.263.json` retain the comparison. These framing results do not establish complete semantic document/project bit coverage.

Six focused tests verify geometry order and binary64 bits, per-edge flag/version boundaries, direct-data minimum sizes and helper states, all single-bend raw-tail gate boundaries and unsupported-body stops, direct feature composition and the final-word gate, plus all 11 hash-bound native containing roots and next stops. Complete engineering semantics, historical/DB contexts, further sheet-metal feature types and full document/project coverage remain unresolved.

Validation: all **886 regression tests pass** (104.144 seconds); specification lint reports zero findings (467 headings, 96 anchors).

Evidence: C-285.
