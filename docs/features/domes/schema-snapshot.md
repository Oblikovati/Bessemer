### 11.159 Ordinary modern dome features

*Status: B — native modern framing verified; raw integer meanings and historical conversions remain partial.*

Scope: ordinary non-DB external-XML model archives with version greater than `27CB` hexadecimal. `moDome_c` uses the complete inherited body-feature layer before the following suffix. Native primary vtable `180472F88`, locator `1804EE0B0` and serializer slot +28 bind to `180264600` in the hash-qualified `sldMoPartu.dll` recorded in C-187. Offsets below name native members, not byte positions in the stream. Values are unaligned little-endian.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited body feature | body-feature record | always | Complete existing body-feature layout; no additional class tag for this layer. | Known |
| 2 | raw +858, +85C | 2 × i32 | always | Preserve full signed words; meanings unresolved. | Partial |
| 3 | dimension +850 | tagged object pointer | always | Native expected type `moDisplayDistanceDim_c`; null and shared identities follow the inherited MFC map. | Known |
| 4 | faces +838 | tagged object pointer | always | Native expected type `suObList`; this is a pointer-tagged list, not an inline count. | Known |
| 5 | references +840, +848 | 2 tagged object pointers | always | Native expected type `moRefWrapper_c`; preserve nulls and shared identities. | Known |
| 6 | raw +860 | i32 | always | Preserve full word; meaning unresolved. | Partial |

An all-null suffix occupies 20 bytes: 12 raw integer bytes and four two-byte null tags. The modern load order matches store order. Historical load branches use a single face pointer before `912`, omit +840 before `912`, omit +848 before `915`, and omit +860 through `30D`; these branches and their runtime conversions are outside this reader's modern scope. A final virtual postload call is runtime behavior, not an additional serialized field. Neither pointer expectations nor raw field offsets establish engineering semantics.

The reader frames 14 native dome features. The new reader advances the corpus to **21,177 complete roots**, **542 list-complete configurations** and **364 explicit stops** across **906 modern model contexts**. Of these, **503 lists reach exact EOF**, totaling **24,006,914 decoded bytes**; the other 39 complete lists retain four-byte tails. Remaining dependencies include sketch regions, component axes, 3D profiles and text handles. Full part, assembly, drawing and all installed-project coverage remains incomplete; this addition grants no whole-document coverage credit.

Evidence: C-187.

