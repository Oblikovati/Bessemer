### 11.168 Weldment features and direct custom properties

*Status: B — native modern serialization and empty-list corpus boundaries verified; populated property order tested synthetically; field meanings remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moWeldmentFeature_c primary table `0x4D212028` selects serializer `0x4C77B610`. This calls moModelFeature_c::Serialize `0x4BB2CDE0`, the common feature body followed by word +354 (11.81). It does not call the atom-feature or body-feature base and does not inherit their direct arrays, ID keeper or suffixes. After the model-feature base:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | properties +360 | direct moCustomPropertyList body | always | No pointer/class tag, presence marker or object-map slot. Native loading allocates the object and calls its serializer directly. | V |
| 2 | raw +370 | i32 | always in modern scope | Preserve the complete signed word. Historical native load derived this field before version 0x10AD; that branch is outside scope. | V / ? |

The suffix with an empty list occupies eight bytes: four-byte list count and four-byte raw word. moCustomPropertyList primary table `0x4D038618` selects `0x4BE70EB0`. After its zero-byte ordinary binary su_CObject layer:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | property count | i32 | always | Signed count; reject negative counts and counts exceeding the remaining minimum-size records. This is not packed ReadCount. | V |
| 2 | properties | count × direct moCustomProperty body | always | Native loading allocates 0x40-byte objects, invokes constructor 0x4BE70110 and directly calls each serializer. Preserve order and duplicate records; no tags/map entries for these direct bodies. | V |

Weldment loading allocates a 0x28-byte list and calls constructor `0x4BE70500`, which installs the moCustomPropertyList vtable. Property constructor `0x4BE70110` installs moCustomProperty's vtable after its moBaseCusProp_c base. These constructor bindings establish the dynamic types used by the direct virtual calls; a pointer-shaped runtime member does not imply an archive pointer tag.

moCustomProperty primary table `0x4D038500` selects `0x4BE70980`. Its modern wire order is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | strings +08/+10/+18 | three CStrings | always | Retain the original stored strings, including newlines and at-sign expressions. | V / ? |
| 2 | raw +2C; raw +28 | i32; u32 | always | Preserve signed and unsigned widths and this non-offset order. | V / ? |
| 3 | string +20 | CString | always in modern scope | Follows both words, rather than the other strings. | V / ? |
| 4 | raw +34/+38/+3C | three i32 | always in modern scope | Preserve all signed values. | V / ? |

An empty-string property body occupies **24 bytes**: four one-byte empty CString encodings and five four-byte words. Relevant historical load gates are +2C at `0x8F5`, +28 at `0xADF`, +20 after `0x12C6`, +34 at `0x17F2`, +38 at `0x1822` and +3C after `0x186D`; all precede the documented modern minimum. No modern extra version gate or trailing alignment is inferred.

Native loading can replace localized text inside the third stored string and can derive a choice label from the second string using the raw +28 index. The offline reader preserves wire strings before these runtime rewrites. These operations do not establish complete engineering meanings or permitted values for the raw fields. Tagged moCustomPropertyList and moCustomProperty instances use ordinary MFC class/object-map framing; only the direct list/member bodies omit it. Scoped readers support both contexts without global registrations.

The edition 0.179 audit retained **10 weldment records**, each with an empty property list. Raw +370 is zero in nine and one in one. Hash-bound native tests verify both values and complete containing-root spans. Populated synthetic tests cover repeated direct records, original newline/at-sign strings, signed/unsigned extremes, direct versus tagged map identity, invalid counts, truncation and context rejection. Edition 0.187 additionally verifies 227 populated native property records inside 45 cut-list folders (11.176); this extends native evidence for the shared property layout without changing the observed empty weldment lists.

Edition 0.179 records **22,163 complete roots**, **643 list-complete configurations** and **263 explicit stops** across **906 complete modern contexts**. Exact-EOF counts and decoded bytes remain **589 lists** and **31,023,044 bytes**, with **54 complete lists retaining four-byte unknown tails**. The ten former weldment stops now reach nine moWeldMemberFeat_c dependencies and one reference-dimension-handle dependency. Coordinate systems, drafts, cut lists, additional weldment features, raw meanings, old/special contexts and all installed-project formats still require work. Full part, assembly, drawing and project bit coverage remains incomplete; no whole-document coverage credit is assigned.

Evidence: C-196.

