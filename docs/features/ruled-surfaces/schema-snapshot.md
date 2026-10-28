### 11.229 Ruled surfaces from edges

*Status: B / ? — native-bound modern direct layout and one populated root verified; scalar meanings, historical variants and whole-document semantics remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, `moRuledSrfFromEdge_c` selects serializer `0x1801BC7A0` in `sldMoPartu.dll`. It writes the reference-surface base, one direct `suObArray` body at runtime `+0x7B8`, then the data body pointed to by runtime field `+0x7B0`. The data body is **direct**, without a tagged class/object header or presence byte. The load branch constructs `moRuledSrfFromEdgeData_c` before calling its serializer. The containing feature's separate embedded data object at `+0x7D8` is not emitted by this function.

The primary vtable's archive-backend selector at `+0xAB0` points to `0x1800E3930`, whose `33 C0 C3` body returns zero, establishing inline surface-base framing. Root constructor `0x1801BB670` proves the `+0x7B8` array's concrete type.

Data serializer `0x1801BC470` writes the following modern payload in order. Offsets identify runtime fields, not positions in the archive.

1. `moFeatureData_c` base: one tagged owning feature pointer at `+0x08`. The native loader requires a feature object. In the populated fixture this is a back-reference to the containing ruled-surface root, which has already entered the object map.
2. Direct `suObArray` body at `+0x30`: ReadCount, then that many tagged object pointers.
3. Direct `su_CWordArray` body at `+0xF0`: ReadCount, then that many **u16** elements. These element values remain unknown for semantic coverage accounting.
4. Direct `suObArray` body at `+0x50`.
5. Tagged `moDisplayDistanceDim_c` pointer at `+0x18`, then tagged `moDisplayAngularDim_c` pointer at `+0x20`.
6. One binary64 at `+0x90`, then four i32 values at `+0x98`, `+0x9C`, `+0xA0`, `+0xA8`.
7. Tagged `moDirectionSpec_c` pointer at `+0xE8`.
8. One binary64 at `+0xB0`, one i32 at `+0xB8`, one `mgVector_c` at `+0xC0` (three binary64 components), then one i32 at `+0xD8`.

The raw scalars and vector components retain their stored values without assigning unsupported option names or domains. Constructor `0x1801BB550` proves the direct array types and initializes other runtime containers, including an object array at `+0x70`, that are absent from this serializer. Do not infer wire fields from object size or constructor defaults.

Historical gates differ from the modern layout: the WORD array is serialized from `0x8FD`; the `+0x50` object array is serialized after `0x989`; the angular dimension is read after `0x8FD`; and the final `+0xD8` word is read from `0x8FE` (otherwise initialized to 1). The writer always emits the angular pointer and final word. These differences do not authorize extending the modern reader into historical contexts. The loader's final `0x1801C1060` call performs runtime processing without an archive argument; it does not add another wire field.

The Mouse Main Housing Overmold sample contains one complete **8,261-byte** ruled-surface root. Its data owner is the root object itself. Parsing proceeds through further roots before an explicit `moDelFace_c` stop. Native source hashes and exact root spans are retained; this result proves the selected record framing and does not establish whole-document semantic bit coverage.

The complete 906-context audit now contains **24,654 complete roots**, adding sixteen while preserving every preceding complete root record. Totals remain **785 complete lists / 121 explicit stops**, including **712 exact-EOF lists / 48,774,033 decoded bytes** and 73 lists with four-byte unknown tails. The previous and current checkpoints are retained as `resolved-feature-list-frontier-0.248.json` and `resolved-feature-list-frontier-0.249.json`. No additional whole-document semantic coverage is credited.

Validation: all 841 regression tests pass (86.422 seconds); specification lint reports zero findings.

Evidence: C-271.

