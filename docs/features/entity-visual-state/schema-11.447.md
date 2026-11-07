### 11.447 Stored visual-color high-byte preservation

*Status: native constructor, copy and setter behavior; high-byte purpose remains unresolved.*

The explicit exported `moEntVisProp_c` constructor (`0x4C1603E0`) saves its complete unsigned-long color argument at runtime +08. The default constructor (`0x4C1604A0`) initializes that field to the all-one missing sentinel. Copy construction (`0x4B299740`) copies the complete u32. The explicit `setLayer(short, swLayerMask_e, int, int, utLineWidth_c)` overload (`0x4C160AA0`) writes its complete color argument on the branch for a non-sentinel layer; its absent-layer branch does not replace the color field.

These paths preserve the high byte rather than normalizing it to zero. Together with the getter-to-renderer projection in 11.446, they establish a distinction between the stored 32-bit value and displayed RGB. They do **not** establish alpha semantics, reserved status or a complete valid domain for bits 24–31. The raw high byte, exact missing sentinel, decoder output and unknown accounting remain unchanged.

**[C-1960]** `part-entity-visual-color-preservation-evidence.json` / `part_entity_visual_color_preservation_audit.py` retain the module hash, exact native ranges, named exported bindings, full instruction listings, argument-to-field assertions and read-only decompiler provenance. Target CAD code was not executed. This evidence adds no coverage credit.
