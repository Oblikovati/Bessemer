### 11.228 Extended reference surfaces

*Status: B / ? — native-bound modern serializer and one populated root verified; enum domains, historical variants and whole-document semantics remain partial.*

In the ordinary external-XML model archive context with file version greater than `0x27CB`, `moExtendRefSurface_c` selects serializer `0x1801FE070` in `sldMoPartu.dll`. It writes the reference-surface base followed by one tagged `moExtendRefSurfaceData_c` pointer at runtime field `+0x7B0`. The base uses the reference-geometry and surface payload layouts already described for planar and offset surfaces. The extended-surface primary vtable's `+0xAB0` selector points to `0x1800E3930`, whose `33 C0 C3` body returns zero, establishing inline surface-body framing in this context.

The tagged data object's serializer is `0x180216DE0`. After the empty `su_CObject` base, its modern payload is:

1. Three direct `suObList` bodies at runtime offsets `+0x30`, **`+0xC0`**, **`+0x78`**, in that wire order. Each uses ReadCount followed by tagged object pointers in the shared map. Constructor `0x1802168B0` independently establishes all three concrete list types.
2. One i32 at `+0x08`, supplied by a `uoExtendRefSurfType_e` constructor parameter; its stored value domain remains unresolved.
3. One i32 end-condition selector at `+0x0C`, supplied by a `uoEndCondition_e` constructor parameter.
4. If the selector is 0, one tagged `moDisplayDistanceDim_c` pointer at `+0x28`; if 1, one tagged `moRefWrapper_c` pointer at `+0x20`; if 2, one tagged `moRefWrapper_c` pointer at `+0x18`. All other selector values consume no pointer. Preserve arbitrary stored selector values rather than inventing a payload for them.
5. One i32 at `+0x10`, supplied by a `uoExtendMethod_e` constructor parameter. Its value domain remains unresolved.

The historical loader includes `+0xC0` from version `0x4E8`, includes `+0x78` after `0x4A5`, and reads the final method word from `0x10D4`; otherwise it initializes that runtime field to 1. The historical writer includes `+0xC0` unconditionally. These differences lie below the implemented modern context and do not authorize a universal older-version reader.

`moExtendRefSurfIdKeeper_c` selects the same empty `su_CObject::Serialize` import thunk `0x1803ACE78` bound in 11.227. The root loader constructs this keeper if its runtime pointer is null; that allocation does not consume an additional tagged object. Its post-load virtual call is runtime processing, not another wire field.

The Mouse Main Housing Overmold fixture contains one complete **12,893-byte** extended-surface root. Its lists contain 5, 0 and 5 pointers respectively; the stored type is 0, end condition is 0 and method is 2. Parsing continues through additional profile, extruded, trimmed, planar and sew-surface roots before an explicit `moRuledSrfFromEdge_c` stop. This verifies the populated record's framing, without establishing enum meanings or whole-document bit coverage.

The 906-context audit contains **24,638 complete roots**, adding six without changing any previously complete root record. Totals remain **785 complete lists / 121 explicit stops**, including **712 exact-EOF lists / 48,774,033 decoded bytes** and 73 lists with four-byte unknown tails. Checkpoints `resolved-feature-list-frontier-0.247.json` and `resolved-feature-list-frontier-0.248.json` retain the comparison. No additional whole-document semantic coverage is credited.

Validation: all 838 regression tests pass (115.640 seconds); specification lint reports zero findings.

Evidence: C-270.
