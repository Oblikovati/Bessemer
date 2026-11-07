# Polyline schema snapshot

### 11.145 PLine features, surface-ID keepers and project data

*Status: B — ordinary modern feature/helper framing with populated native arrays; raw values and application meanings remain partial.*

moPLine_c is implemented in sldMoPartu.dll. Primary table `1804D6048`, locator `1804F33E8`, selects serializer `18037AE70`. Its concept hooks import the common lightweight/resolved serializers. Runtime descriptor `180606CB0` selects factory `180379B70`, whose constructor call selects `18037A130`. The constructor calls moBodyFeature_c and allocates a moPLineSIdKeeper_c at +800 through constructor `18037A050`. Memory sizes 0x820 for the feature and 0x140 for the keeper are not serialized sizes.

In ordinary non-DB external-XML loading with v > 0x27CB, the body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | body-feature base | Existing complete inherited body | Always | Includes atom stages and body suffix; excludes extrusion/revolution/rib suffixes | B |
| 2 | keeper +800 | Direct moPLineSIdKeeper_c body | Always in modern scope | No object tag, class header or wrapper map slot | B |
| 3 | raw +814 | i32 | Always in modern scope | Original gate v >= 0x2E5 | B |
| 4 | data +808 | Tagged moPLineData_c-family pointer | Always | Required nonnull; native runtime-class check admits derived project data | B |

The direct keeper's primary table `1804D8AD8` selects serializer `180381E10`. Its constructor proves the collection types; the serializer proves **file order**, which differs from member-offset/constructor order:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | surface IDs +18 | i32 count; direct moEntityId_c[count] | Always | Nonnegative fixed count; each direct ID contains a tagged representation pointer | B |
| 2 | array +68 | Direct suObArray | Always | ReadCount and tagged object pointers | B |
| 3 | array +88 | Direct suObArray | Always | Same count/pointer grammar | B |
| 4 | raw WORD array +C8 | Direct su_CWordArray | Always | ReadCount then u16[count] | B |
| 5 | array +48 | Direct suObArray | Always in modern scope | Follows +C8, despite its lower memory offset | B |
| 6 | raw DWORD array +A8 | Direct su_CDWordArray | Always in modern scope | ReadCount then u32[count] | B |
| 7 | raw +114, +118 | i32[2] | Always in modern scope | Original gate v > 0x10FD; retain signed wire values | B |

The historical +F0 array branch is confined to versions above 0x3C3 and below 0x425 and is absent here. Native surface-ID/name/configuration updates and reference-use adjustments operate on objects after reading and add no modern fields. Counts SHALL fit the remaining minimum record sizes. Readers preserve order, null entries where permitted, duplicate inherited pointers, high-bit numeric values and shared map identity; direct collections allocate no wrapper object slots.

The keeper's numeric arrays are distinct from **tagged numeric-array objects** nested in feature graphs. SWCCU primary su_CDWordArray table `31D50F18` selects serializer `31D3C120`; su_CWordArray table `31D512C8` selects `31D3E4F0`. Both use packed ReadCount. The native load paths then read exactly count times four or two bytes respectively. The WORD count is not restricted to a fixed u16 count: the normal 0xFFFF extension introduces u32 count before the u16 values. Local dispatch supports both tagged forms while retaining the same primitive body grammar. Raw array values remain semantically unresolved.

moPLineData_c primary table `1804D86B8` selects thunk `1803AC890`, importing moFeatureData_c::Serialize at `4B50C220` in sldmodu.dll. That body is a **single tagged nonnull owner-feature pointer**; no additional derived data suffix is consumed for the base class. The native loader checks that the target is a feature. Recursive references to the currently loading PLine feature use its already allocated MFC object-map slot.

moPLineProject_c primary table `1804D88C8` selects serializer `18037ADE0`. Its native RTTI base hierarchy includes moPLineData_c, moFeatureData_c, su_CObject and CObject. It therefore satisfies the PLine serializer's data-family check. Its body is the feature-data owner pointer, followed by i32 array-pointer presence at +10, a **direct suObArray** when that value is nonzero, then i32 raw +20 and raw +24. This uses suObArray::serializePtr presence/body framing, not a tagged array pointer. The reader preserves the presence word and both raw words. Unsupported data subclasses and null required data/owner pointers retain explicit stops. This project class is nested part-feature data, not an installed project-file carrier format.

The installed audit retains **62 freshly serialized PLine features**. Each has populated surface-ID, WORD and DWORD arrays with matching counts ranging from 4 to 32. All data pointers identify moPLineProject_c, all data owners reference the containing feature, and all project-array presence words are 1; project arrays contain 1 through 8 entries. A hash-bound native regression replays a populated keeper/project case through its exact root end. Synthetic tests cover mixed count widths, extended WORD/DWORD counts, duplicate inherited references, high-bit words, negative raw values, tagged numeric arrays, absent project arrays, both base and project data classes, recursive owner identity, truncation and required-pointer/context rejection. Base-only PLineData and absent project arrays are synthetic variants, not positive native PLine fixtures.

The edition 0.156 frontier records **19,569 complete roots**, **361 list-complete configurations** and **545 explicit stops**, with all **906 modern model contexts** complete. **355 lists reach exact EOF**, spanning **14,251,075 decoded bytes**; six retain four-byte unknown tails. No PLine, PLineProject or numeric-array stop remains at this frontier. Leading dependencies are shells (37), reference axes (37), moOneContour_c (33), hole-wizard features (27) and scale features (23). Full part, assembly, drawing and installed-project bit coverage remains incomplete.

Evidence: C-173.

