### 11.237 Populated sketch intersections and loop references

*Status: B / ? — ordinary tagged-reference zero-marker bodies and ten populated native records verified; raw field meanings, identity-address modes and historical variants remain partial.*

This clause completes the zero-marker path introduced in 11.66. It applies to ordinary external-XML model archives with file version greater than `0x27CB`, ordinary archive condition flags zero and helper field `+0x834` zero. `sgIntersection_c` selects serializer `0x4C593CC0` in `sldmodu.dll`. In the main sketch collection it is a **direct** body, without an additional tagged object header or object-map slot.

Read the initial u16 invalid marker at runtime `+0x68`. Any nonzero value ends this body immediately, preserving the original value. When zero, the modern tagged-reference payload continues:

1. u32 flags at `+0x6C`, i32 base-entity/handle count, i32 last-edge count at `+0x58`.
2. Four tagged `moRefWrapper_c` pointers at `+0x10`, `+0x18`, `+0x20`, `+0x28`.
3. Two tagged `bentRefArr` pointers at `+0x30`, `+0x38`.
4. The stated number of tagged `sgEntHandle` objects, associated with runtime array `+0x40`. A nonpositive signed count consumes zero entries.
5. Direct dynamic DWORD-word groups at runtime pointer `+0x50`, using helper `0x4BB1DA60`: u32 element width; width zero ends the array, otherwise two u32 allocator metadata words, u32 group count, and `width × count` bytes. Nonzero width is divisible by four. This array's group count is independent of the earlier handle count.
6. One **direct** `moIntersectionEdge_c` body per positive handle count, associated with runtime array `+0x48`. These embedded edges are not separately tagged or mapped.
7. One u8 transform-presence value; if nonzero, direct compact `mgXform_c::restore` data at runtime `+0x60`, using the existing optional-matrix/translation/scaling/flags grammar. This is not the 30-word archive operator used by split items.
8. If version is greater than `0x2B71`, two u16 words at `+0x70`, `+0x74`, a tagged `suObList` pointer at `+0x78`, then tagged `sgArcHandle` pointer at `+0x80`.
9. If version is greater than `0x2B98`, four u16 words at `+0x88`, `+0x8C`, `+0x90`, **`+0xA0`**, a tagged `moLineRef_w` pointer at `+0x98`, then an i32 point count and that many direct `mgPoint_c` values (three binary64 components each). A nonpositive count consumes zero points. Retain the raw sequence rather than substituting native runtime uniqueness processing.
10. If version is at least `0x2BE0`, one binary64 at `+0xB0`.

The native handle reader constructs temporary handles and assigns their owning sketch before copying them into runtime storage. Sketch-owner updates, reference-use increments and temporary destruction do not add wire fields. The diagnostic view preserves the consumed tagged objects and shared-map identities. Raw flags, option words, allocator metadata, edge fields, point values and final double remain unresolved for semantic coverage accounting.

`bentRefArr` selects `0x4C58CA60`. In this tagged-reference context it delegates to `suObArray::Serialize`: ReadCount followed by tagged object pointers. Operator `0x4C585750` independently confirms that an enclosing `bentRefArr*` is an ordinary tagged `ReadObject` pointer, not an inline nullable-array marker.

`moIntersectionEdge_c` selects `0x4C58CE80`. After the empty object base it writes five i32 values at `+0x158`, `+0x15C`, `+0x160`, `+0x164`, `+0x168`, then four direct reference groups in runtime-field order `+0x38`, `+0x80`, `+0xC8`, `+0x110`. Group helper `0x4C58CC20` writes a u16 at relative `+0x00`, three i32 words at relative `+0x20`, `+0x24`, `+0x28`, then tagged `moLoopRef_c`, `moEdgeRef_c`, `moEdgeRef_c` pointers at relative `+0x08`, `+0x10`, `+0x18`. Runtime group size and the edge's `0x180` allocation size are not wire lengths.

`moLoopRef_c` selects `0x4BBBD230`: the established boundary-reference serializer `0x4BBB6F40`, followed by one u32 at runtime `+0xF0`. Native ancestry includes `moBEntRef_c`. The modern scope satisfies this suffix's historical version gate after `0x65C`.

Archive identity-address modes are distinct: the same native functions can write raw 64-bit reference values in place of tagged pointers, and `bentRefArr` then contains counted 64-bit entries. The selector depends on helper conditions 2, 1 (or the zero-condition predicate when helper `+0x834` is nonzero), `0x20`, `0x200000` and `0x8000000`. `hasCondition(0)` tests whether all archive condition flags are zero; it is not a bitwise test with mask zero. The recorded ordinary flags-zero/helper-zero evaluation selects tagged references. The decoder rejects nondefault sketch helper mode and does not infer identity-address framing from plausible-looking bytes. Condition-4 processing and historical default/owner migrations remain outside this scope.

Historical gates below the implemented modern minimum include flags after `0xDB7`, third/fourth wrapper references after `0x4F5`, explicit transform presence after `0xAE4`, the first edge word after `0x73A` and group u16 word after `0x736`. The later tail gates above are implemented at their exact boundaries.

The five affected configurations contain **ten complete zero-marker intersection bodies totaling 22,044 bytes**, inside five complete profile roots. The fixtures exercise one- and two-handle cases, corresponding direct edge records, independent dynamic word groups, shared references and present compact transforms. Exact body and containing-root spans/hashes are retained. Both side-part feature lists complete; the Mouse housings and bike frame continue to newly exposed identifier, plane-constraint or weld-corner dependencies. These are framing results, not a claim of whole-document or all-installed-project bit coverage.

The 906-context audit now contains **25,105 complete roots**, **799 complete lists** and **107 explicit stops**, with every preceding complete root record unchanged. This adds 85 roots and two exact-EOF lists totaling **537,432 decoded bytes**: the simulation FloWorks side part (268,856) and tutorial ball-valve side part (268,576). Totals are **726 exact-EOF lists / 56,402,792 decoded bytes**; 73 complete lists retain four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.256.json` and `resolved-feature-list-frontier-0.257.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 865 regression tests pass (111.705 seconds); specification lint reports zero findings.

Evidence: C-279.

