### 11.432 Entity visual ByLayer flag roles

*Status: complete-export native binding, decoder aliases and retained replay.*

Five low bits of the stored `moEntVisProp_c` +2 word have named **ByLayer** roles. The complete installed-module export table binds each `use...` getter and matching `setFlag...ByLayer` / `unsetFlag...ByLayer` pair. Every getter reads +2 and masks the indicated bit; setters OR that mask into +2, and unsetters AND its complement. This resolves the names directly, beyond the update-routine behavior in C-1825.

| Mask | Native role | Decoder boolean |
|---|---|---|
| 0x0001 | Color | `by_layer_color` |
| 0x0002 | Font | `by_layer_font` |
| 0x0004 | Width | `by_layer_width` |
| 0x0008 | FontSP | `by_layer_font_sp` |
| 0x0010 | WidthSP | `by_layer_width_sp` |

The **SP** suffix is retained literally; its expansion and additional context are not inferred. `read_moEntVisProp_c` preserves the original u16 in `by_layer_flags` and `unknown_2`, and exposes `by_layer_unresolved_bits = by_layer_flags & 0xFFE0`. Set bits establish the named stored flags, not effective resolved layer color, style or width. Runtime layer lookup and invalid-value fallback are not simulated.

The upper eleven bits remain unresolved, so the complete word retains its existing unknown-byte charge. No semantic byte credit is claimed. Entity visual properties remain the largest unresolved aggregate (**894,411 decoded bytes**) in the retained part scope; the raw state and color domains also remain partial. The exhaustive unit test covers all **65,536** flag words, raw preservation, each boolean, upper-bit preservation, adjacent style, unchanged accounting and the next stream sentinel.

Evidence: **[C-1910]** `part-entity-visual-by-layer-bits-native-evidence.json` / `part_entity_visual_by_layer_bits_native_audit.py` retain 15 named native accessor ranges, full bytes, module hashes and complete export-table bindings. **[C-1911]** `part-entity-visual-by-layer-bits-replay-evidence.json` / `part_entity_visual_by_layer_bits_replay_audit.py` retain all **906** source-bound part streams with exact EOF, unchanged accounting and **83** passing related tests. Target CAD code was not executed. Scoped unknown bytes remain **18,520,455**; historical global coverage checkpoints remain unchanged.
