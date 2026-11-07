### 11.216 Derived-feature and surface-body references

*Status: B / ? — native serializer framing and 20 references in complete containing roots verified; stored-word meanings and whole-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature archive, version >0x27CB. Runtime object offsets below identify native members, not wire offsets. Native primary RTTI binds moDerivedFeatRef_w to moRefWrapper_c and serializer 0x4BB22870. Its factory/constructor establishes a direct suObArray at +E0. The wire sequence follows the shared wrapper body:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | feature +A8 | tagged moFR_c | always | External feature reference. | B |
| 2 | raw +B8 | u32 | always | Preserve stored word. | B / ? |
| 3 | plane +C0 | tagged moRefPlnData_c | always in scope | Plane reference data. | B |
| 4 | axis +C8 | tagged moRefAxisData_c | always in scope | Axis reference data. | B |
| 5 | coordinates +D0 | tagged moRefCoordSysData_c | always in scope | Coordinate-system data. | B |
| 6 | array +E0 | direct suObArray | always | ReadCount followed by tagged objects; retain order and identity. | B |
| 7 | raw +100 | i32 | always in scope | Preserve signed word. | B / ? |
| 8 | raw +104 | i32 | always in scope | Preserve signed word. | B / ? |
| 9 | point presence | i32 | version >0x2F85 | Any nonzero value selects row 10; retain raw signed value. | B |
| 10 | point +D8 | 3 × f64 | row 9 nonzero | Stored point coordinates. | B |

Earlier native gates for rows 1–8 precede the supported context. The complete empty body with null/cached pointers is 72 bytes at 0x27CC, 76 at 0x2F85, 80 at 0x2F86, 84 at 0x2FB0 and 88 at version 19000. A present point adds 24 bytes. Tagged new objects contribute their own bodies and map slots. Constructor defaults and runtime ownership fixups add no wire fields.

The nested moSurfaceBodyRef_w primary serializer 0x4BB36470 calls the moBodyRef_w serializer 0x4BB212A0 without an additional archive suffix. It therefore uses the body-reference grammar already documented, including its explicit kernel-body/context restrictions. Both bindings are confined to the copied modern feature reader map.

The native evidence packet retains two primary ancestry bindings, six native bodies, the derived factory/constructor, direct-array construction, native bytes and complete-module hash. Pdata may describe fragments; native code lengths are not serialized lengths. Three focused tests cover cached identity, direct-array duplicates, signed-word limits, point presence, version boundaries, exact following-byte alignment, truncation/context rejection, and source-hash-bound native containing-root spans and field equality.

The audit now records **20 derived-feature references in completed containing roots** and **24,299 complete roots**, three more than Edition 0.221. Complete-list and explicit-stop totals remain **765** and **141**. Exact-EOF totals remain **692 lists / 46,159,654 decoded bytes**; **73 complete lists retain four-byte unknown tails**. The next affected dependency is moMoveCopyBody_c. The previous frontier is preserved in resolved-feature-list-frontier-0.221.json. These diagnostic completions earn no whole-document coverage credit. Part, assembly, drawing and all installed project-format bit coverage remain unfinished.

Evidence: C-244.
