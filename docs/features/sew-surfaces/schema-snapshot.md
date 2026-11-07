### 11.227 Sew-reference surfaces

*Status: B / ? — native-bound modern layout and three complete roots verified; scalar meanings, legacy variants and whole-document semantics remain partial.*

This clause covers `moSewRefSurface_c` in ordinary external-XML model archives with file version greater than `0x27CB`. The primary vtable in the installed `sldMoPartu.dll` selects serializer `0x180228C70`. It calls the body-feature base, then serializes the owned data addressed by runtime field `+0x800`. Runtime offsets below identify native fields; they are not archive offsets.

The payload is, in order:

1. Five direct `suObArray` bodies at owned-data offsets `+0x30`, `+0x50`, `+0x70`, `+0x90`, `+0xB0`. Each uses ReadCount followed by that many tagged object references in the inherited object map. There is no additional tagged wrapper for these direct arrays.
2. Six four-byte flag words. Retain their exact stored values. The loader derives runtime flags at `+0x00`, `+0x04`, `+0x08`, `+0x0C`, `+0x10`, `+0x14`: the first is true when word 0 is zero; each remaining flag is true when its corresponding word is nonzero. The third word overwrites the temporary `+0x08` value initially derived from word 0.
3. Three binary64 values in runtime-field order `+0x18`, **`+0x28`**, **`+0x20`**. Their meanings remain unresolved; preserve raw bytes, including signed zero and NaN representations.

The native owned-data constructor `0x1802281A0` constructs six arrays, including `+0xD0`, but the selected serializer writes only the first five. The sixth array must not be inserted into the archive layout. With five empty arrays the direct payload occupies 58 bytes. The six flag words and three doubles remain unknown for semantic coverage accounting.

Native historical gates are: the first two flag words after `0xF99`, the third after `0xFB5`, and the final three words plus doubles from `0x10B4`. These gates are all satisfied by this clause's supported archive context. The older reference-surface migration path below `0x8DB` is not implemented by this reader.

`moSewRefSurfIdKeeper_c` selects vtable serializer thunk `0x1803ACE78`. Its RIP-relative jump binds IAT slot `0x1803EB580` to `swccu.dll` export `?Serialize@su_CObject@@UEAAXAEAVsu_CArchive@@@Z`. This selected base serializer contributes zero body bytes. A keeper with the same apparent role in another surface class is not evidence for this layout.

Three complete native sew-surface roots total **19,704 bytes**: Mouse Bottom Housing, Mouse Main Housing Overmold and the API toaster sample. They reach explicit stops at `moSplit_c`, `moExtendRefSurface_c` and `moCompositeCurve_c`, respectively. The full 906-context audit now contains **24,632 complete roots**, **785 complete lists** and **121 explicit stops**. All prior complete root records remain identical; this adds 34 roots. Exact-EOF totals remain **712 lists / 48,774,033 decoded bytes**; 73 complete lists still retain four-byte unknown tails. The preceding and current checkpoints are retained as `resolved-feature-list-frontier-0.246.json` and `resolved-feature-list-frontier-0.247.json`. No additional whole-document semantic bit coverage is claimed.

Validation: all 835 regression tests pass (87.865 seconds); specification lint reports zero findings.

Evidence: C-269.
