### 11.382 Entity visual mask low-bit update behavior

*Status: partial mask semantics from a native consumer; complete ByLayer and color domains remain unresolved.*

The typed swByLayer_e word at visual-property member `+0x02` (11.372) controls the initial assignments in native update routine `0x4C160C00` [C-1825]. The routine loads the word and tests its low three bits independently:

| Bit | Member / role | Initial assignment when bit is clear | Behavior when bit is set |
|---|---|---|---|
| 0x0001 | raw u32 at +0x08 | copy incoming EDX | skip this initial copy |
| 0x0002 | line style at +0x0C | copy incoming R8W | skip this initial copy |
| 0x0004 | line width at +0x10 | assign incoming utLineWidth_c from R9 | skip this initial assignment |

Later fallback logic overrides the initial skip behavior: raw +0x08 equal to 0xFFFFFFFF is replaced with the incoming value and bit 0x0001 is cleared; a negative signed line-style word is replaced and bit 0x0002 is cleared; line width for which named utLineWidth_c::isSet returns false is assigned from the incoming width and bit 0x0004 is cleared. Thus a set bit does not guarantee that the member remains unchanged throughout this routine. This establishes behavior in this consumer, not general inheritance policy or an exhaustive swByLayer_e enum definition.

The decoder continues retaining the full raw u16 as `by_layer_flags` / `unknown_2`. High bits, color interpretation of +0x08 and the separate byte at +0x04 remain partial. No bits are discarded or synthesized, no defaults are applied to stored records, and unknown-byte accounting remains unchanged. This native consumer does not add bytes or named keys to the wire grammar.

Evidence: **[C-1825]** `part-entity-visual-update-mask-evidence.json` / `part_entity_visual_update_mask_audit.py` retain the module hash, complete routine bytes/disassembly matched against C-1808, sixteen exact branch/member/mask assertions and named utLineWidth copy/assignment/isSet imports. Static assertions pass. Target CAD code was not executed; semantic byte-coverage credit is zero.
