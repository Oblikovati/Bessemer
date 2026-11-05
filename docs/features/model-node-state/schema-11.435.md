### 11.435 Node stored update state and ID-assignment flag

*Status: complete-export native bit bindings, decoder aliases and retained replay.*

The node +0C word is the **update state**, established by exact named `moNode_c::setUpdateState` (`0x4B2B15E0`), which stores its signed integer argument at +0C. The decoder exposes `update_state`, retaining signed `unknown_0c` unchanged. Named predicates bind seven bits of its unsigned representation:

| Mask | Native predicate | Stored decoder boolean |
|---|---|---|
| 0x00000001 | `needsUpdate` | `stored_needs_update` |
| 0x00000002 | `isUpdateError` | `stored_update_error` |
| 0x00000004 | `isUpdateWarning` | `stored_update_warning` |
| 0x00000100 | `isSatNotSolved` | `stored_sat_not_solved` |
| 0x00004000 | `isUpdateLocked` | `stored_update_locked` |
| 0x00008000 | `isUpdatePending` | `stored_update_pending` |
| 0x00020000 | `isUpdateSecurity` | `stored_update_security` |

`needsUpdate` at `0x4C04E180` returns true immediately when bit 0 is set; with that bit clear and argument one, it can consult virtual +168. Therefore `stored_needs_update` represents only the stored bit, not the full context-dependent native predicate. SAT and security names are retained without inferring additional meanings. `update_state_unresolved_bits` retains all remaining bits via mask **0xFFFD3EF8**.

Exact `SetNeedsIDToBeSet` (`0x4C04CD60`) conditionally sets or clears bit 30 of node +28, binding **`stored_needs_id_to_be_set`** to `flags_28 & 0x40000000`. The serialized +28 word is preserved. The native loader's forced bit-30 setting and later ID assignment are not performed by parsing. Exact `clearFlag2` and `setFlagOn2` establish +2C as a second flags word; **`flags_2c`** is an equal unsigned alias for `unknown_2c`, present only above its existing version gate 0x143A. Its individual bit roles remain unresolved.

Native state setters can clear neighboring bits. For example, `setNeedsUpdate` uses masks that clear error/warning/pending/security state, and `setUpdateError` can clear other states before setting error. The reader does not normalize those states or reject coexisting flags. Unit coverage exercises all 128 combinations of the seven known state bits with signed and unexplained-bit patterns and multiple ID-flag words; it also checks the exact second-word version boundary, raw aliases, adjacent ID, unchanged accounting and stream alignment.

Evidence: **[C-1918]** `part-node-state-fields-native-evidence.json` / `part_node_state_fields_native_audit.py` retain exact complete-export bindings, accessor ranges, native masks, module hashes and read-only decompilation provenance. **[C-1919]** `part-node-state-fields-replay-evidence.json` / `part_node_state_fields_replay_audit.py` retain **906** source-bound part streams at exact EOF and **89** passing related tests. Unknown-byte accounting is unchanged because remaining bits are unresolved: the retained part scope remains **18,235,871 decoded unknown bytes**. Target CAD code was not executed; part-document decoding continues before assembly fields.
