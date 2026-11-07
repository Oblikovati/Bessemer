### 11.404 Component-edge signed count and composite-pointer declarations

*Status: modern signed-loop behavior and declared pair type; edge point/flag meanings remain partial.*

The component-edge serializer at `0x4BBB8CA0` restores the signed reference count and enters its object-pointer loop only when the count is positive [C-1856]. The decoder now retains a negative stored count and returns an empty references list, matching that load behavior. Positive counts remain bounded by the available minimum pointer-tag bytes. The change does not claim that native producers normally emit negative counts or extend the supported historical version range.

After the reference loop, the native load path returns early when both flag bits 0x2 and 0x4 are set. Otherwise flag bit 0x8 selects two pointers declared as moCompCompositeEdge_c at members +0x190 and +0x198, in that order. The decoder supplies that declaration to the existing object reader and retains null/mapped/derived object identities rather than reading inline bodies. Raw signed flags, the two three-f64 point values and other words remain unchanged and charged under their existing rules. Negative count skipping does not bypass the optional pair.

Nine component tests pass. New cases combine counts -1 and -2147483648 with flags 0, 8, 14 and a signed word containing the sign bit plus bit 8. They check pair absence/presence, mapped pair order/identity, raw count/flags, exact boundary and truncation. An impossible positive count still fails.

All 906 retained modern part resolved streams preserve source hashes, exact EOF and unknown totals [C-1857]. All 16,586 captured component-edge bodies have reference_count zero. Thus negative-count behavior has native/synthetic evidence only; this change claims no installed negative-count example, new semantic bytes or global coverage increase.

Evidence: **[C-1856]** `part-component-edge-native-evidence.json` / `part_component_edge_native_audit.py` retain complete native serializer bytes/disassembly, small direct callee ranges, the inlined composite-edge runtime descriptor and signed-loop/flag decompilation anchors. **[C-1857]** `part-component-edge-replay-evidence.json` / `part_component_edge_replay_audit.py` retain all 906 comparisons, target-stream count frequencies, nine test results and current parser/test source hashes. Target CAD code was not executed.
