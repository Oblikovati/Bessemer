### 11.184 Helices, reference-curve wires and wire-edge identities

*Status: B / ? — native modern field order, seven populated helix boundaries and present kernel wires verified; raw controls, optional engineering semantics and full coverage remain open.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moHelix_c primary table `0x4D2E8ED0` selects serializer `0x4CB7EBE0`. RTTI establishes moReferenceCurve_c ancestry; the inherited serializer is `0x4C284E20`, selected by reference-curve primary table `0x4D1674A8`. That serializer first calls the shared reference-geometry helper `0x4C283A00` from 11.100, then restores the wire through `0x4C287C20` and reads the direct list below. This is a reference-geometry family, without the derived body-feature extrusion/revolution suffix.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common reference geometry | direct shared body | always | Existing versioned feature/atom/reference grammar and its dependencies. | B / ? |
| 2 | wire +710 | kernel archive body | always in this ordinary scope | Includes suppression byte and existing strict compressed-Parasolid framing. Present bodies are decoded, not skipped as opaque spans. | B / ? |
| 3 | wire node ID +718 | i32 | always in scope | Native load resolves a runtime edge using this stored ID; retain the signed word offline. | V / ? |
| 4 | attributes +728 | direct suObList body | always in scope | Packed ReadCount and tagged objects. No list class tag, pointer-presence word or extra list map slot. | B |

The reference-curve constructor `0x4C2CDA80` invokes the imported suObList constructor at member +728; the concrete helix constructor `0x4CB79740` calls that reference-curve constructor. This establishes the direct virtual list type. Native storage gathers/restores runtime body attributes around the wire; these actions do not add a second wire or list. Historical wire gates and the list gate at `0x12B2` all precede the supported minimum. Special archive conditions that suppress wire storage are outside this scope.

After the inherited reference-curve body, the helix suffix is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +778/+780 | i32; f64 | always | Retain the signed selector and double. | V / ? |
| 2 | data +788 | direct helix-data body | always | Two i32 at data +00/+04; six f64 at +10/+18/+20/+28/+30/+38; two i32 at +40/+44; three f64 at +48/+50/+58; then u8 variable-pitch presence. There is no data class tag or pointer tag. | V / ? |
| 3 | variable-pitch +60 within data | direct variable-pitch body | presence byte nonzero | i32 at +00, followed by five direct dynamic f64 arrays at +08/+18/+28/+38/+48. Preserve the raw presence byte. | B / ? |
| 4 | dimensions +790/+798/+7A0 | three tagged moDisplayDim_c-family pointers | always | Separate pointer occurrences, including repeated or null values. | B |
| 5 | dimension +7A8 | tagged moDisplayDim_c-family pointer | `abs(data raw +28) > 1e-6` | Strict comparison against the native double at `0x4CDD72C8`; the sign-clear mask at `0x4CD9B130` proves absolute-value handling. Equality omits this pointer. | B |
| 6 | raw +7C8; dimension +7B0 | i32; tagged pointer | always | Preserve exact sequence after the conditional pointer. | V / ? |
| 7 | dimension-table presence | u8 | always | Only value **1** enters the table. Other stored values do not cause table reads in native loading. | V / ? |
| 8 | table +7C0 | direct dimension table | presence byte equals 1 | Signed i32 row count, then four tagged dimension pointers per row. When row count is positive, append one additional tagged dimension pointer at table +60. An empty table has no terminal pointer. | B |

Each dynamic f64 array uses helper `0x4BD15480`: u32 element width; zero ends the array, otherwise width 8, two u32 metadata words, u32 element count and count f64 values in the supported typed scope. Every array has its own frame; there is no shared count for the five arrays. Metadata, signed selectors and values are retained before native historical pitch reconstruction. Negative/impossible table counts and unsupported array widths stop explicitly. The pitch-load helper is `0x4CB999E0`; the dimension-table loader is `0x4CB99E70`. All of their selected modern load gates are already satisfied by `v > 0x27CB`.

The direct data body is 89 bytes when variable pitch is absent. The minimum latest-version suffix is 114 bytes with null dimensions and absent dimension table. With a 279-byte empty reference-geometry base, suppressed wire, zero wire-node ID and empty direct list, a synthetic tagged helix body before outer framing is 400 bytes. Present wires and nested dimensions add variable bodies. Optional synthetic fixtures establish wire framing and rejection behavior, not the engineering validity of arbitrary pitch/dimension combinations.

The inherited geometry dependencies require moWireEdgeIdRep_c. Primary table `0x4CF040E0` selects `0x4BBBDE70`, which calls shared entity-representation helper `0x4BBBBF50` and appends exactly one i32 at +38. The scoped reader reuses 11.18's common entity-reference body plus that signed suffix. It is not a direct kernel edge or an extra helix scalar. Pointer map identity is retained; no global class registration is added.

Native runtime descriptor `0x4DB083C0` selects factory `0x4CB79320`, which allocates 0x878 runtime bytes and calls helix constructor `0x4CB79740`. The allocation size is not a file length. Retained evidence verifies three primary RTTI/COL/type/ancestry bindings, ten native bodies, inherited/helper calls, the exact threshold/mask, the direct-list constructor import and the factory/constructor call.

The audit completes **seven native helix roots**. All seven contain present decoded kernel wires, wire-node ID 6 and empty attribute lists. Variable-pitch presence is zero, data +28 is positive or negative zero, the optional +7A8 dimension is absent and dimension-table presence is zero. The optional array/table branches have synthetic tests, not populated native corpus evidence. Five focused tests cover the strict threshold on both signs (including equality and NaN behavior), exact-one table presence, empty/populated tables, cached dimension identity, five independent dynamic arrays, raw signed fields, tagged minimum composition, truncation/context rejection, wire-edge identity framing and a source-hash-bound native containing-root span.

The frontier now records **23,220 complete roots**, **697 complete lists** and **209 explicit stops** across 906 complete modern contexts: 12 more complete roots than Edition 0.194, with unchanged list counts. **635 lists reach exact EOF**, spanning **35,257,972 decoded bytes**; **62 complete lists retain four-byte unknown tails**. `resolved-feature-list-frontier-0.194.json` preserves the preceding checkpoint. Later dependencies include component-reference curves and sketch-intersection records. Whole ResolvedFeatures, part, assembly, drawing and all installed-project bit coverage remain incomplete.

Evidence: C-212.
