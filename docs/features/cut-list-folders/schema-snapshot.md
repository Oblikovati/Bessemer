### 11.176 Cut-list folders and inherited property lists

*Status: B — native inheritance, field widths, direct property construction and populated corpus boundaries verified; raw folder meanings and runtime updates remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moCutListFolder_c primary table `0x4D0FFD80` selects `0x4C2308D0`. Its native ancestry is moSubWeldFolder_c, moSubAtomFolder_c, moAtomBodyFolder_c, moFolder_c and the common feature/node bases. The corresponding serializers call each other in that order. The cut-list suffix cannot be appended directly to a generic folder body without the intermediate layers.

moAtomBodyFolder_c selects shared serializer `0x4C224970`, already used by the documented surface/solid body folders: common moFolder_c body, i32 +374, i32 +378, tagged moMaterialFolder_c +380 and tagged moCutListSortOptions_c +3D0. All four fields occur throughout this modern scope. moSubAtomFolder_c primary table `0x4D0FC2A0` selects `0x4C238E70` and appends:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | list +3D8 | tagged suObList pointer | always | Ordinary MFC pointer identity, not a direct list body. | V |
| 2 | raw +3E0 | i32 | always | Preserve complete signed word. | V / ? |
| 3 | feature reference +3E8 | tagged moFeatureRefWrapper_c pointer | always in modern scope | Historical pre-0xA00 migration is excluded. | V |
| 4 | raw +3F4/+3E4 | two i32 | always in modern scope | Preserve this non-offset wire order; gates after 0x11B5 and 0x17F2 precede scope. | V / ? |

This suffix is **16 bytes** with null pointers. moSubWeldFolder_c primary `0x4D0FE010` selects `0x4C2390B0`, calls the sub-atom serializer and appends one **direct moCustomPropertyList body at +448** using the complete property layout of 11.168. Native load allocates 0x28 bytes and calls constructor `0x4BE70500`, which installs moCustomPropertyList primary `0x4D038618` and selects serializer `0x4BE70EB0`. The list and its direct property records allocate no MFC class/object-map slots. Do not insert a tagged list pointer or presence word. The list's signed i32 count and ordered records follow immediately after +3E4. Negative/impossible counts remain rejected.

The obsolete temporary records for versions `0x1BF3` through `0x1BF7` are absent in this scope. The modern direct property list is also distinct from the pre-`0xA00` migration path. The latest empty body-folder, sub-atom and sub-weld bodies occupy **159**, **175** and **179 bytes** respectively, including their inherited common folder fields.

After the complete sub-weld body, moCutListFolder_c appends:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | feature +490 | tagged moCompFeature_c pointer | always in modern scope | The historical +488 moAtomFeature_c alternative is excluded. | V |
| 2 | raw +49C/+498 | two i32 | always | Preserve order and signed values. | V / ? |
| 3 | raw +484 | u8 | always in modern scope | One byte on wire, widened into a four-byte runtime member. Preserve values above one. | V / ? |
| 4 | plane +460 | tagged moPlaneRef_w pointer | always in modern scope | No separate presence word. | V |
| 5 | raw +468/+470/+478 | three f64 | always in modern scope | Preserve three original scalar values without deriving geometry. | V / ? |
| 6 | raw +4A8/+4AC | two i32 | always in modern scope | Preserve complete signed words. | V / ? |

All selected gates precede the modern minimum: the feature pointer after 0xA51 uses moCompFeature_c from 0xA7F, +498 follows 0xE34, byte +484 follows 0x1214, plane/scalars follow 0x1441, +4A8 begins at 0x1BEB and +4AC at 0x1C49. The null-pointer cut-list suffix is **45 bytes**, giving **224 bytes** for the latest empty composed cut-list body. Its three doubles do not imply padding around the preceding byte. Native post-load update helper `0x4C136E80` receives the feature object rather than an archive argument; offline reading retains stored data without executing that update.

The scoped registry admits the four verified folder classes with their own serializer entry points. The body-folder base is reused only after RTTI and native helper-call bindings establish the exact shared layout. Direct custom-property strings remain the original wire values, before native localized/choice/expression rewrites described in 11.168.

The complete-root audit retains **45 cut-list folders** with **227 populated direct property records**. List sizes range from one to 16 records. Every feature pointer is moCompFeature_c and all plane pointers are null in these samples. Stored byte +484 is 3 in 26 folders, 1 in 16 and 2 in three; these observations establish raw values, not engineering option names. Native tests verify stream SHA, complete containing-root spans, full property records and all three flag values. Synthetic tests cover signed extremes, byte width, non-offset field order, repeated direct properties, tagged ancestor identity, context rejection and truncation. Additional nonnull plane/reference contexts and complete raw meanings remain open.

Edition 0.187 records **22,658 complete roots**, **685 list-complete configurations** and **221 explicit stops** across **906 complete modern contexts**. **623 lists reach exact EOF**, spanning **33,656,410 decoded bytes**; **62 complete lists retain four-byte unknown tails**. Sweep cuts, annotations, helices, sketch dimensions and other feature/chooser dependencies remain unresolved. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these record boundaries.

Evidence: C-204.
