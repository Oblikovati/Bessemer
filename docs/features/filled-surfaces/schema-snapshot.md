### 11.231 Filled surfaces and curve contacts

*Status: B / ? — native-bound modern layout and five populated roots verified; scalar meanings, historical migrations and whole-document semantics remain partial.*

For ordinary external-XML model archives with file version greater than `0x27CB`, `moFillRefSurface_c` selects serializer `0x1802F43D0` in `sldMoPartu.dll`. It writes the body-feature base, then serializes selected members of owned data addressed by runtime field `+0x808`. There is no tagged wrapper for this owned data. Runtime offsets below are not archive offsets.

The modern owned-data payload is:

1. Three direct `suObArray` bodies at `+0x10`, `+0x30`, `+0x50`, each using ReadCount and tagged pointers in the inherited object map.
2. One i32 at `+0x70`.
3. Direct ID-keeper body pointed to by `+0x118`: one direct `suObArray` at keeper offset `+0x48`. No tagged keeper header or presence word occurs here.
4. Three i32 values at `+0x78`, **`+0x74`**, `+0x108`.
5. One **u8** at `+0x10C`. The native loader casts this byte through signed char to a runtime int; retain both the stored byte and its signed interpretation. This is not a four-byte wire flag.
6. Five i32 values at `+0x08`, `+0x148`, `+0x150`, `+0x154`, **`+0x14C`**.

With all four direct arrays empty, this payload occupies 45 bytes. The writer changes runtime `+0x74` from 2 to 1 before emitting it. The reader retains the original stored value instead of applying this writer mutation. Other scalar meanings remain unresolved.

Constructor `0x1802F3AB0` proves the three owned arrays' types. Root constructor `0x1802F3820` allocates `moFillRefSurfIdKeeper_c` at owned-data `+0x118`. Its primary vtable selects thunk `0x1803AC8FC`, bound to `SLDMODU.dll` export `?Serialize@moEdgeSurfIdKeeper_c@@UEAAXAEAVsu_CArchive@@@Z`. The target at `0x4BBBBF40` consists of `48 8B 41 48 48 83 C1 48 48 FF 60 28`: load the member vtable, advance this by `0x48`, and tail-call its serializer. Constructor `0x4B3F5CD0` proves that member is `suObArray`. This 12-byte leaf has no unwind-table entry. Its other runtime members and base storage do not imply additional wire fields.

The `moFillSurfData_c` primary vtable itself selects empty `su_CObject::Serialize`; the feature's owned-data serialization is performed explicitly by the parent function above. Calling the data vtable serializer in place of the parent layout would omit the payload.

Nested `moCurveContact_c` selects serializer `0x4CB6E990` in `sldmodu.dll`. After the empty object base, its modern payload is:

1. Direct `suObArray` at `+0x50` (established by inherited constructor `0x4CB6DBB0`).
2. Binary64 at `+0x90`, i32 at `+0x98`, tagged `moDisplayAngularDim_c` pointer at `+0xA0`.
3. Binary64 at `+0xA8`, then i32 values at `+0xB0`, `+0xB8`, `+0xC8`, `+0xCC`.
4. Tagged `moLoftSynchRef_c` pointer at `+0xE0`.
5. Three i32 words at `+0x40`, `+0x44`, `+0x48`, followed by binary64 at `+0xC0`.

The last three words are stored through unsigned writer overloads and loaded through signed overloads; retain all four-byte bit patterns. Other raw scalars and doubles likewise receive no unsupported semantic labels. With an empty array and two null pointers this direct contact body occupies 62 bytes.

`moCurveContactData_c` selects `0x4CB6EE80`: empty object base, tagged `moRefWrapper_c` pointer at `+0x18`, generic tagged object pointer at `+0x08`, then one u32 at `+0x10`. This wire order differs from runtime field order. It is not interchangeable with the shorter `moContactData_c` layout in 11.226.

Historical filled-surface loading below `0x919` uses a reference-surface proxy migration. The final owned-data `+0x148` word is read after `0xCCD`; the `+0x150`, `+0x154`, `+0x14C` words are read from `0x185C`. Contact historical gates include `+0x90` from `0xA6B`; `+0x98` and angular pointer from `0xAE9`; `+0xA8`, `+0xB0`, `+0xB8` from `0xB42`; `+0xC8` from `0xCAA`; `+0xCC` from `0xC72`; synchronization pointer from `0xC92`; three `+0x40` words after `0xCFF`; and final double from `0xD8A`. The implemented context satisfies all gates. Earlier defaults and sign-based migration are not reproduced by this modern reader.

Five complete native filled-surface roots total **9,158 bytes**: NAFEMS 3dnlg7 (1,656), NAFEMS ic9 (846), mousebase (2,316), mousemultibody (2,024) and mousesplit (2,316). Both simulation feature lists complete. The tutorial files continue to `moTrimExtEdgeIdRep_c` or `moEdgeRefIdRep_c` dependencies. Retained packets bind stream/root hashes, container counts and raw scalar values; this verifies framing without claiming full field meaning or whole-document bit coverage.

The 906-context audit contains **24,765 complete roots**, **788 complete lists** and **118 explicit stops**. All preceding complete root records remain identical. This adds 45 roots and two exact-EOF lists: 3dnlg7 (66,165 bytes) and ic9 (28,209 bytes), totaling 94,374 decoded bytes. Totals are **715 exact-EOF lists / 49,560,755 decoded bytes**; 73 complete lists retain four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.250.json` and `resolved-feature-list-frontier-0.251.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 847 regression tests pass (94.490 seconds); specification lint reports zero findings.

Evidence: C-273.
