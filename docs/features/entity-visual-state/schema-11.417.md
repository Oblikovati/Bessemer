### 11.417 Entity visual state predicates and runtime override

*Status: exact native mask behavior; predicate names remain unresolved.*

Within the unresolved visual-properties group, stored u8 +4 corresponds to runtime integer state +4. Leaf accessor `0x4C160910..0x4C160917` returns `state & 2`, so its integer result is 0 or 2 rather than a normalized boolean. Accessor `0x4C160920..0x4C160933` first checks runtime dword +18: if nonzero it returns 1; otherwise it returns `state & 1`.

These operations establish two specific bit consumers and an override. They do not establish visibility, selection, printing or inheritance names. Runtime +18 must not be invented as an extra stored visual-property field, and the override must not replace the stored byte in an offline decoder. Preserve high bits and the unknown charge for +4. The constructor/reset state 3 observed in C-1808 enables both low masks, but does not restrict the stored domain to 0–3.

Evidence: **[C-1874]** `part-entity-visual-state-predicates-evidence.json` / `part_entity_visual_state_predicates_audit.py` retain both complete leaf ranges, native bytes/disassembly, source-module hash and mask/override assertions. Target CAD code was not executed. No parser behavior or semantic coverage totals change.
