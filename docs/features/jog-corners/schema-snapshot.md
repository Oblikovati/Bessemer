### 11.248 Sheet-metal jogs and break corners

*Status: B / ? — native serializer/factory bindings and two populated feature roots verified; special archive modes, option domains and complete document coverage remain partial.*

`moJog_c` selects serializer `0x3DB62D80` in the hash-bound installed `sldsmmu.dll`. After the established body-feature and bend-group base (`0x3DB68EC0`), its ordinary modern derived body stores the following fields in wire order. Member offsets identify native runtime members, not file offsets.

1. Tagged profile pointer `+0x8D0`, binary64 `+0x8E8`, i32 `+0x958`, binary64 `+0x950`, then i32 `+0x8F4`.
2. Tagged boundary-reference pointer `+0x920`, i32 `+0x8F0`, two tagged display-dimension pointers `+0x978,+0x980`, then i32 `+0x95C`.
3. One u8 reference-presence byte. Any nonzero value introduces two tagged pointers: point reference `+0x970`, then reference wrapper `+0x960`. Either pointer may itself be null.
4. Direct point `+0x900` (three binary64 components), followed by direct `suObList` `+0x880` with its own ReadCount and tagged entries.
5. i32 values `+0x8F8,+0x988,+0x8E4`, tagged component-feature pointer `+0x9A0`, i32 `+0x930`, then direct vector `+0x938` (three binary64 components).
6. Tagged boundary-reference pointer `+0x928` for versions `>=0x3AFE`.

With mapped/null pointers, an empty direct list and zero reference-presence byte, this suffix occupies **109 bytes** before the final-pointer gate and **111 bytes** afterwards. A nonzero presence byte adds at least four bytes. Fresh pointed-object bodies add their own sizes. The list has no tagged list-object header or separate map slot. Historical field gates other than the final boundary pointer precede the supported modern range.

Native load maps stored `+0x8F4` values 0 and 2 to runtime 2, value 1 to 1 and value 3 to 3. Other values leave the existing runtime member unchanged. Descriptor `0x3DDB01C0` binds creator `0x3DB577A0` to default constructor `0x3D98A190`, which initializes this member to 1 and constructs direct list `+0x880`. The reader therefore retains `raw_8f4` and separately exposes `fresh_loaded_8f4`; its default of 1 describes a freshly created object. The 0x9A8 allocation is not wire length. Engineering enum names remain unresolved.

On native storing, the presence byte is zero only when conditions `0x20` and `0x200000` are clear, condition 1 is set and helper `+0x834` is zero; otherwise it is one. Reading follows the actual stored byte. Native storing also transforms a copy of point `+0x900` using the profile transform. The decoder preserves the serialized coordinates and binary64 bits without applying a second transform.

`moBreakCornerFeat_c` selects `0x3DB5D010` and uses the same body-feature/bend-group composition. Its ordinary modern suffix stores:

1. Two tagged `suObList` pointers `+0x8B0,+0x8B8`, i32 `+0x8A0`, binary64 `+0x8A8`, then tagged display dimension `+0x890`.
2. One **direct `moFilletSIdKeeper_c` body** owned through pointer `+0x888`.
3. Tagged component-edge pointer `+0x8C0`, tagged display dimension `+0x898`, then i32 `+0x8C8`.
4. Tagged `suObList` pointer `+0x8D0`, i32 `+0x8D8`, binary64 `+0x8E0`, then tagged display dimension `+0x8F8`.
5. i32 values `+0x8E8,+0x908`, binary64 `+0x910`, then **tagged `suObArray` pointer** `+0x900`.
6. i32 `+0x918` for versions `>=0x46CB`.

The keeper uses established serializer `0x4BBBD030` in hash-bound `sldmodu.dll`: nonnegative i32 surface-ID count, that many entity-ID records, then a direct ReadCount object list of references. An empty keeper occupies six bytes and adds no tagged keeper header. Constructor `0x3D856760`, selected through descriptor `0x3DDAFB30` and creator `0x3DB56D10`, allocates this keeper and an array at `+0x900`. Despite constructor ownership, the array is serialized as a tagged pointer. Minimum suffix sizes are **66 bytes** before the final-word gate and **70 bytes** afterwards, with empty keeper/lists/array and mapped/null tagged pointers.

The native keeper predicate is `!condition(1) || condition(0x200000) || helper_834 != 0`. The decoder's ordinary context has condition 1 clear and includes the direct keeper. Special contexts that omit it remain unsupported; they are not represented by a null keeper tag. Older field gates precede the supported modern range. Scalar option meanings remain open, and raw values are retained.

The complex sheet-metal sample supplies one **27,268-byte Jog root** and one **12,034-byte Break Corner root**, totaling **39,302 bytes**. Jog has reference-presence byte 1 with both optional pointers null; its direct list is empty. Break Corner contains **14 surface IDs and 13 keeper references**, a tagged list of **12 component edges**, and an empty tagged array. Source/stream/root hashes and native instruction bytes are retained in the evidence packet.

The 906-context audit now has **25,484 complete roots**, **824 complete lists** and **82 explicit stops**. All prior complete root records remain unchanged. Six additional roots complete the complex sample exactly at its **570,840-byte** decoded stream end. Totals are **746 exact-EOF lists / 61,994,914 decoded bytes**, plus **78 complete lists with four-byte unknown tails**. Checkpoints `resolved-feature-list-frontier-0.267.json` and `resolved-feature-list-frontier-0.268.json` retain this comparison. These are framing measurements and earn no whole-document semantic coverage credit.

Four focused tests cover both version gates, all presence branches, fresh-object normalization, direct/tagged identity and ordering, binary64 bit retention, truncation/context rejection and both complete hash-bound native roots. All **909 regression tests pass** (95.340 seconds). Special archive modes, additional native value combinations, raw engineering domains and complete part/assembly/drawing/all-installed-project semantic bit coverage remain open.

Evidence: C-290.
