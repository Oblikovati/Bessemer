### 11.383 Entity visual color word and native low-24-bit projection

*Status: color role and native projection established; channel order and stored high-byte meaning remain partial.*

The visual-property u32 at member `+0x08` supplies a native color getter at `0x4C160850`. If the leading layer word is 0xFFFF, the getter returns zero without writing its output. Otherwise it writes member +0x08 masked with 0x00FFFFFF to the output and returns one [C-1826]. This projection does not establish that the stored high byte is padding, alpha or a flag, and readers SHALL retain it.

Consumer `0x4C9C0350` passes a stack output at +0x44 to that getter, then loads it into ESI and forwards it as R9D to `0x4BBFBB50`. That routine saves and forwards the same argument to `0x4BBFBC90`, whose layer creation path stores it at the new layer's `+0x20` member. The retained typed moLayer_c::setColor(unsigned long) implementation at `0x4BBFCE00` writes the same layer member, independently establishing its color role (11.114). This dataflow binds the visual u32 to color without assuming an RGB channel order.

The decoder exposes `color_word` as an alias of the retained raw `unknown_8` value. It does not mask the stored value or replace it with the native getter result. In particular, 0xFFFFFFFF remains distinct from 0x00FFFFFF, and a layer sentinel does not erase the raw color word. No new wire bytes or keys are introduced. Unknown accounting stays conservative pending a separately scoped regrade; global coverage is unchanged. The separate +0x04 byte and complete ByLayer enum remain partial.

Evidence: **[C-1826]** `part-entity-visual-color-native-evidence.json` / `part_entity_visual_color_native_audit.py` retain complete getter/consumer/layer creation bytes and disassembly, fifteen exact dataflow assertions, the typed setter decompilation hash and its matching native store. `test_entity_visual_fields.py` checks the raw alias with a nonzero high byte and with 0xFFFFFFFF, while retaining existing boundary/accounting assertions. Target CAD code was not executed; semantic byte-coverage credit is zero.
