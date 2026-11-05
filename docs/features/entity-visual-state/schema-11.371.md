### 11.371 Entity visual-property layer number distinguished from line style

*Status: native field-role bindings and parser names verified; complete flag/color semantics and accounting integration remain partial.*

Entity visual-property reader 0x4C1606C0 stores the first two wire words at native offsets +0x00 and +0x02, then a version-gated byte at +0x04. Its nested visual-property serializer 0x4C1607C0 reads the raw u32 at +0x08, a word at +0x0C and the line-width body at +0x10. Native evidence now distinguishes the two words that the prior reader labeling conflated [C-1808].

The leading word at +0x00 is a layer number. Entity update function 0x4C160940 passes that word to lookup 0x4BBFCA30, the same lookup invoked by the named exported moLayerMgr_c::GetLayerDataFromLayerNum. The lookup matches the layer table record's +0x30 word. This supplies type-specific lookup corroboration rather than an inference from small integer values.

The word at +0x02 receives the constructor argument typed swByLayer_e in the exported moEntVisProp_c constructor signature. Native update helper 0x4C160C00 tests its low bits to select overrides for the raw +0x08 value, +0x0C word and line width. Complete bit-domain semantics remain partial and the parser preserves every stored bit.

The later word at +0x0C supplies line style. Getter 0x4C160870 returns this word; consumer 0x4B88E320 places the getter result into the same output that its alternate path fills through named apLineStyleMgr_c::getLineIndex. Thus the historical parser key layer for that later word was misleading. `dl_light.read_moEntVisProp_c` now exposes layer_number from the leading word, by_layer_flags from the second word and line_style from the later word. Raw unknown_* keys remain, and deprecated layer retains its historical later-word value for API compatibility; new consumers SHALL use the explicit names.

Two focused tests use different layer/style values and preserve all-ones sentinels/unrecognized flag bits; nineteen existing display-dimension tests also pass. Wire reads, end offsets and unknown-byte accounting remain unchanged in this change. Color channel order, complete +0x04 state semantics and a corpus-based semantic accounting update remain open; no new byte credit is claimed here.

Evidence: **[C-1808]** `part-entity-visual-fields-native-evidence.json`, `part-entity-visual-fields-native.txt` and `part_entity_visual_fields_native_audit.py` retain eight complete selected functions plus the style getter, executable/prior/source hashes, typed constructor/layer-lookup exports and named line-style import bindings. `test_entity_visual_fields.py` verifies independent field labels, aliases and raw preservation. Target CAD code was not executed. Zero additional payload-byte coverage credit; complete part schema and full installed-format coverage remain incomplete.
