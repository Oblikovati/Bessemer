### 11.241 Weld-corner features and weld-body data

*Status: B / ? — four native bindings, 28 complete populated feature roots and three exact-EOF lists verified; raw field domains, historical contexts and later dependencies remain partial.*

`moWeldCornerFeat_c` derives from `moBodyFeature_c` and selects serializer `0x4C74AA00`. For the supported ordinary external-XML model context with version greater than `0x27CB`, the established body-feature base is followed by this wire sequence:

1. Direct `suObArray` at runtime `+0x800`, then independent direct `suObArray` at `+0x820`.
2. One i32 at `+0x8C8`, then one tagged generic object pointer at `+0x8C0`.
3. Direct `suObArray` at `+0x840`.
4. Three i32 words at `+0x8CC`, `+0x8D0`, `+0x8D4`, in that order.
5. One tagged `moPointRef_w` pointer at `+0x860`.
6. One i32 at `+0x868` only for file versions greater than `0x334D`. Earlier accepted versions load runtime value 1 without consuming a word.

Each direct object array has its own `ReadCount` and tagged elements; the array itself adds no tagged-object map slot. Counts are not inferred from another array. With empty arrays and null/mapped pointers this suffix occupies **26 bytes** through version `0x334D`, or **30 bytes** afterward. These sizes exclude the inherited feature and any newly introduced tagged-object bodies.

Constructor `0x4C74A6E0` proves all three serialized members are `suObArray`. It also creates runtime containers at `+0x870`, `+0x880` and `+0x8A0`; the selected serializer does not serialize those containers. Their presence in memory does not imply additional wire fields. The historical load gates for `+0x8C0` (`>=0x933`), array `+0x840` and word `+0x8CC` (`>=0x961`), word `+0x8D0` (`>=0xCBC`), word `+0x8D4` (`>=0xF92`) and point reference `+0x860` (`>0x12EB`) all precede the supported modern range. Historical conversion behavior remains outside this reader's scope.

`moWeldBodyData_c` selects `0x4C7527D0`. After the empty `su_CObject` base, it serializes one direct `suObArray` at runtime `+0x108`, then i32 words at `+0x78` and `+0x7C`. Loading `+0x7C` is gated by version greater than `0xFAA`, satisfied throughout the supported modern range. Constructor `0x4C751E50` proves the array type. Other constructor-created arrays, pointer arrays and the runtime transform are not fields of this serializer. The empty modern base body occupies **10 bytes**.

`moWeldTrimmedBodyData_c` derives from that base and selects `0x4C752970`: common weld-body data followed by one direct `suDWordArray` at runtime `+0x148`. Its independent count is followed by that many u32 words; there is no array object tag. Constructor `0x4C752230` proves its type. The empty modern derived body occupies **12 bytes**.

`moWeldTrimToolData_c` also derives from the common base and selects `0x4C7528B0`. It appends binary64 values at runtime `+0x148` and `+0x150`, giving **26 bytes** with an empty base array. Native loading can set runtime flag `+0x158` from a comparison of these values; no archive byte stores that flag. The decoder preserves the two raw binary64 values without inferring geometric domains or engineering units.

Five affected configurations retain **28 complete weld-corner roots / 41,631 bytes**, with **30 fresh trimmed-body records** and **45 fresh trimming-tool records** inside completed roots. The native fixtures exercise populated direct arrays, null and mapped references and several independent count combinations. Three lists reach exact EOF: beam_truss (75,506 decoded bytes), bikeframe Config-9 (650,399) and loop (96,707), totaling **822,612 bytes**. Both leg configurations advance to the explicit `moEllipseBackedUpData_c` stop; no partial containing root is accepted.

The 906-context audit now contains **25,307 complete roots**, **813 complete lists** and **93 explicit stops**, with every prior complete root record unchanged. This adds 162 roots and three exact-EOF lists. Totals are **735 exact-EOF lists / 58,770,901 decoded bytes** and **78 complete lists with four-byte unknown tails**. Checkpoints `resolved-feature-list-frontier-0.260.json` and `resolved-feature-list-frontier-0.261.json` preserve the comparison. These results establish selected framing; complete document/project semantic bit coverage remains open.

Three focused tests verify independent direct arrays, tagged null/shared identity, signed raw words, the exact `0x334D/0x334E` gate, direct DWORD ordering, binary64 bit preservation, absent runtime-only fields, truncation/context rejection and all 28 hash-bound native containing roots through three complete lists and the two later stops.

Validation: all **876 regression tests pass** (122.742 seconds); specification lint reports zero findings (465 headings, 96 anchors).

Evidence: C-283.
