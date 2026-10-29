### 11.245 Flatten-bends and process-bends features

*Status: B / ? — two native type bindings, four complete containing roots and two populated raw-kernel frames verified; alternate archive modes, option domains and later sheet-metal classes remain partial.*

`moFlattenBends_c` selects serializer `0x3DB60830` in the installed hash-bound `sldsmmu.dll`. It calls established bend-group serializer `0x3DB68EC0`, covering the body-feature base, common bend data, three i32 words, direct object array `+0x858` and u32 `+0x878`. In the supported ordinary external-XML modern context (version `>0x27CB`, helper `+0x834` clear, archive condition `0x40000` clear), the derived body is:

1. One i32 raw value at runtime `+0x8C8`.
2. **Only when inherited node flags `+0x28` contain bit `0x800`:** one i32 body-presence value associated with runtime body pointer `+0x8C0`.
3. **Only when that presence value is nonzero:** one `mo_loadBodyFromArchiveRaw` body, using the selected strict compressed form from 11.244. No suppression u8 is inserted.

A flag-clear feature omits the presence word entirely; it does not store a zero placeholder. An unrelated nonzero node flag does not select the body path. The raw i32 at `+0x8C8` loads only from version `>=0xFD1`; all supported modern versions satisfy this gate. Historical create-version migrations below that gate and native rejection of versions below `0x1BF` are outside this reader's scope.

Native storing sets node bit `0x800` before writing the inherited base. With helper `+0x834` clear and condition `0x40000` clear, it emits presence 1 and a raw body when the runtime body and feature state allow it; otherwise it emits 0. Nonzero helper `+0x834` omits that stage. Under condition `0x40000`, the following i32 instead participates in runtime body-handle/partition-transfer reuse, not the ordinary raw-body interpretation. These special archive paths are not inferred from bytes. The current reader rejects the nondefault helper state and implements the established ordinary context only.

The ordinary suffix therefore occupies four bytes when node bit `0x800` is clear, eight bytes with the bit set and absent body, or eight bytes followed by a raw body when present. Signed presence values are retained before the native nonzero test. Raw `+0x8C8` engineering meaning and the remaining node-flag domains remain unresolved.

`moProcessBends_c` selects `0x3DB659E0` and uses the same inherited bend-group serializer. Its ordinary modern suffix stores a tagged `moFixedFaceInFlat_c` pointer at runtime `+0x888`, then one i32 at `+0x890`. The native load gates (`>0x25C` and `>0x26D`) precede the supported range. With a null/mapped face pointer the suffix occupies six bytes. Only when archive conditions 1 **and** `0x200000` are both set does native serialization add a tagged `moFlattenBends_c` pointer at `+0x880`; ordinary context leaves both clear, so that pointer is absent here. Native storing and loading agree on this conjunction.

Two complete `moFlattenBends_c` roots occupy **8,305 bytes** in Right Blade and **13,049 bytes** in sheet.sldprt. Both retain node flags `0x40000800`, raw `+0x8C8` value 0 and body-presence 1. Their raw frames are independently bounded and decoded using the existing strict single-body Parasolid validator:

- Right Blade: source bytes **67,131–72,168**, a **5,037-byte** raw frame with length 5,033, inflating to **11,145 bytes**.
- sheet.sldprt: source bytes **51,161–52,324**, a **1,163-byte** raw frame with length 1,159, inflating to **2,036 bytes**.

Exact source-stream, containing-root, serialized-frame, payload and inflated SHA-256 values are retained. Both frames terminate at their containing flatten-bends root endpoint. Two complete process-bends roots occupy **3,241 bytes** and **25,808 bytes** respectively. The four feature roots total **50,403 bytes**; neither raw-body framing nor full inflated parsing establishes complete engineering semantics for the body contents.

The 906-context audit now contains **25,431 complete roots**, **819 complete lists** and **87 explicit stops**, with all prior complete root records unchanged. This adds 15 roots across two configurations. No new list reaches EOF: totals remain **741 exact-EOF lists / 60,491,872 decoded bytes**, with **78 complete lists retaining four-byte unknown tails**. Right Blade advances to `moSM3dBend_c`; sheet.sldprt reaches `moMidRefSurface_c`. Checkpoints `resolved-feature-list-frontier-0.264.json` and `resolved-feature-list-frontier-0.265.json` preserve the comparison. No partial next root is accepted and no whole-document semantic coverage credit is assigned.

Three focused tests verify exact node-bit gating, omitted presence versus stored zero, signed option/presence values, raw-body composition without suppression, special-helper/context rejection, process pointer order/identity, truncation and all four hash-bound native containing roots. Both native frames are re-parsed independently and compared with their retained compressed and inflated hashes. Flag-clear native fixtures, alternative raw-kernel/archive forms, historical migrations and full part/assembly/drawing/all-installed-project bit coverage remain open.

Validation: all **894 regression tests pass** (119.717 seconds); specification lint reports zero findings (469 headings, 96 anchors).

Evidence: C-287.
