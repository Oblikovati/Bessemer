### 11.436 Dimension bend-angle enforcement and fillet change-number mask

*Status: complete-export native bindings, decoder aliases and retained replay.*

The dimension-base `mask` is the stored **+54 word**, not +58. Named `Dimension_c::Serialize` at `0x4C832980` reads and writes +54 followed by the separate +5C integer. Named accessors establish four bits of +54:

| Mask | Role | Decoder field |
|---|---|---|
| 0x00000002 | Bend-angle range enforcement | `enforce_bend_angle_range` |
| 0x0000001C | Fillet reference-point change number (shift right 2) | `fillet_ref_point_change_number` |

Exact `isApplicableToBendAngleRange` (`0x4B2A75D0`) tests mask 2; `setEnforceBendAngleRange` (`0x4BE8BC60`) sets or clears that same bit according to its argument. The reader exposes the stored flag without enforcing a range or inferring angle units or limits. Exact `getFilletRefPointChangeNumber` (`0x4C82A610`) computes `(word >> 2) & 7`. Exact `incFilletRefPointChangeNumber` (`0x4C82A930`) increments modulo eight and changes a zero result to one, preserving neighboring bits. This native runtime transition does not restrict stored input: the reader preserves extracted zero through seven and never advances the counter.

The original signed `mask` remains unchanged. `mask_unresolved_bits` preserves all remaining bits with **0xFFFFFFE1**. The neighboring `unknown_5c` remains a separate unresolved i32; no counter role is transferred to it. The supported modern reader already includes these fields; legacy serializer gates are not expanded by this change.

Evidence: **[C-1920]** `part-dimension-base-mask-native-evidence.json` / `part_dimension_base_mask_native_audit.py` retain exact complete-export bindings, native getter/setter/incrementer/serializer ranges, full bytes, installed-module hashes and read-only provenance. **[C-1921]** `part-dimension-base-mask-replay-evidence.json` / `part_dimension_base_mask_replay_audit.py` retain **906** source-bound part streams at exact EOF and **90** passing related tests. The new mask test covers all sixteen combinations of these four bits with unexplained low/high and signed patterns, preserving adjacent +5C, accounting and the next stream sentinel. All mask bits retain their unknown-byte charge because the complete domain remains unresolved. The retained part scope remains **18,235,871 decoded unknown bytes**; historical global coverage checkpoints remain unchanged. Target CAD code was not executed. Part decoding continues before assembly fields.
