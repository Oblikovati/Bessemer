### 11.234 Split features, maps and split items

*Status: B / ? — native-bound modern parent layout, four populated roots and seven split items verified; option meanings, historical variants and alternate body contexts remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, `moSplit_c` selects serializer `0x180349850` in `sldMoPartu.dll`. It writes the body-feature base followed by selected members of owned `moSplitData_c` addressed by runtime `+0x868`, interleaved with parent members. There is no tagged wrapper for this owned data. Offsets below identify runtime fields, not archive offsets.

The modern suffix is:

1. Owned-data direct `suObArray` bodies at `+0x20`, `+0x48`.
2. Owned-data tagged `moProfileFeature_c` pointer at `+0x18`, tagged `moDirectionSpec_c` pointer at `+0x40`, then u32 at `+0x10`.
3. Parent direct `moBodyChooser_c` at `+0x870`, then direct `suObArray` at `+0x808`.
4. Parent i32 at `+0x910`.
5. Owned-data direct `suDWordArray` at `+0xC0`: ReadCount followed by u32 elements.
6. Parent i32 at `+0x804`, then direct `suObArray` at `+0x828`.
7. Parent integer-map count (i32), followed by that many `(i32 key, i32 value)` pairs. This is not ReadCount. The native load loop consumes no pairs for nonpositive counts. Retain the raw count and all consumed pairs. Native runtime insertion accepts only pairs with nonnegative key and value; a later accepted duplicate key replaces its earlier value. The diagnostic reader exposes both the raw pairs and this derived map.
8. Parent i32 at `+0x91C`.
9. Owned-data entity-reference keeper pointed to by `+0x78`, written **directly** as a `suObArray` body without a tagged object header or presence word.
10. Owned-data i32 at `+0x160`.
11. Parent i32 words in order `+0x924`, `+0x920`, `+0x928`, `+0x918`, `+0x914`, `+0x9D8`, `+0xA20`.

Constructor `0x180347BC0` establishes the parent arrays and body chooser; constructor `0x180347890` establishes the inherited trim-data layout and `suDWordArray`. The latter's vtable in `sldmfcu.dll` selects thunk `0x3CD6076B`, bound to `swccu.dll` export `?Serialize@su_CDWordArray@@UEAAXAEAVsu_CArchive@@@Z`. The entity keeper and body chooser use the established layouts from the trim-surface evidence. Other constructed storage, including parent array `+0x848` and inherited owned-data arrays `+0x80`/`+0xA0`, is not emitted by the selected parent serializer.

Both `moSplitData_c` and `moSplitBodyIdKeeper_c` select the empty `su_CObject::Serialize` import thunk `0x1803ACE78`. Their vtable serializers must not be substituted for the parent's explicit owned-data layout. The final `moNode_c::SerializeDBData` call operates on the separate DB/deferred store and adds no bytes to the ordinary archive stream described here.

Nested `moSplitItem_c` selects serializer `0x1803494B0`. Its modern layout is a counted CString name at runtime `+0x08`, a tagged visual-properties pointer at `+0x10`, and an i32 body-presence word. When absent, proceed directly to the transforms below. When present in the verified ordinary inline context, read the established kernel-body framing, followed by a direct `suObList` of attributes at runtime `+0x138`. The implemented positive path requires a fully validated compressed single-body Parasolid form; other kernel forms stop explicitly.

Finally read two direct archive-operator `mgXform_c` values at runtime `+0x20` and `+0xA8`. Each verified ordinary form is **30 u32 words**, not the compact `mgXform_c::restore` grammar. The exact imported operator is `sldmgu.dll` symbol `??5@YAAEAVsu_CArchive@@AEAV0@AEAVmgXform_c@@@Z`. Retain all 120 wire bytes per transform; a special high-nibble kind in word 28 remains an explicit stop. Runtime spacing between these fields is not their serialized size.

The split-item native branch under helper condition `0x40000` can read a runtime body handle before kernel data; this alternate context is not inferred or decoded here. A present body that yields no loaded kernel object can also return early before attributes/transforms; the diagnostic reader explicitly rejects its suppressed-body variant rather than guessing that boundary. Body naming/partition migration and pointer cleanup are runtime processing. Raw split scalar words, DWORD values, map pairs and transform words remain unresolved for semantic coverage accounting.

Historical split loading gates include the `+0x910` word from `0x7D4` (with a pre-`0xCC0` conversion), DWORD array after `0xCBF`, parent `+0x804`/`+0x828` after `0xE1A`, map after `0x10F8`, `+0x91C` after `0x10FF`, entity keeper after `0x1178`, owned `+0x160` after `0x12E5`, parent `+0x924`/`+0x920`/`+0x928` after `0x182B`, `+0x918` after `0x1845`, `+0x914` after `0x1BE8`, `+0x9D8` from `0x1C09`, and `+0xA20` after `0x1FBC`. Split-item transforms are loaded after `0x777` and from `0x77C` respectively. The modern scope satisfies all gates; older defaults and writer/load asymmetries remain outside this reader.

Four complete native split roots total **131,828 bytes**: Bottom Housing (27,659), Main Housing (12,649), simulation frame (87,656) and tutorial mousemultibody (3,864). The frame root includes seven split items with decoded compressed Parasolid bodies and paired fixed transforms; compressed/inflated hashes are retained. Parsing continues to `moPunch_c`, `moTemplateSheetMetal_c` or `moCutThin_c` stops. These results establish selected framing without claiming whole-document or all-project-format semantic bit coverage.

The frame items exercise six empty attribute lists and one populated list with six entries. The full 906-context audit now contains **24,944 complete roots**, adding 90 with every preceding complete root record unchanged. Totals remain **793 complete lists / 113 explicit stops**, including **720 exact-EOF lists / 53,189,313 decoded bytes** and 73 complete lists with four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.253.json` and `resolved-feature-list-frontier-0.254.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 856 regression tests pass (92.585 seconds); specification lint reports zero findings.

Evidence: C-276.

