### 11.223 Offset reference surfaces, data and ID keeper

*Status: B / ? — five populated surfaces/data objects verified inside complete native roots; raw option meanings, other data subclasses and full-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature context, version >0x27CB, with helper reuse state +834 clear and ordinary binary base mode. sldMoPartu.dll primary RTTI binds moOffsetRefSurface_c (0x180448A58 / COL 0x1804EC108) to 0x180218590, with moRefSurface_c ancestry. The serializer first calls imported moRefSurface_c::Serialize and then restores a tagged moOffsetSurfData_c pointer at +7C8. Null data causes a native archive exception; the scoped reader requires this supported nonnull class. Runtime ID-keeper allocation and post-load fixups add no wire fields.

The feature's virtual +AB0 returns zero (native bytes 33 C0 C3), proving the existing inline shared reference-surface framing instead of the backend-pointer shortcut. The reader therefore composes reference-geometry framing, the complete shared surface payload (kernel bodies/attributes and versioned word vector), and the required tagged offset-data pointer. Unsupported kernel contexts continue to stop through the strict existing parser. With empty surface fields and cached data, the complete body is 289 bytes at 0x27CC, 293 at 0x2BCD, and 297 from 0x2F86, reflecting inherited gates. New data objects contribute their own tagged framing/body/map slots.

moOffsetSurfData_c primary 0x1804489C8 / COL 0x1804EC208 selects 0x180218670 and has su_CObject ancestry. After its ordinary zero-byte base:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | array presence +08 | i32 | always in scope | suObArray::serializePtr presence; retain the full signed value. | B |
| 2 | pointed-to array | direct suObArray body | row 1 nonzero | ReadCount and tagged items; no tagged array object or allocation slot. Preserve duplicates/order. | B |
| 3 | dimension +10 | tagged moDisplayDistanceDim_c | always | Preserve nullable target identity. | B |
| 4 | raw +18 | i32 | always | Preserve raw signed option. | B / ? |

The pre-0x1CF single-surface-pointer compatibility branch is outside scope. An absent-array/null-dimension body is **10 bytes**; present-but-empty array adds the normal two-byte ReadCount, for **12 bytes**. A nonzero presence is not the array count and must not be normalized before retention.

The nested moOffsetRefSurfIdKeeper_c primary 0x1804199A8 / COL 0x1804E71D0 has moIdKeeper_c ancestry and selects thunk 0x1803ACE78. The thunk resolves to swccu.dll su_CObject::Serialize (0x31D3F030). In ordinary binary mode, archive flag 0x10 clear takes the verified zero-byte return. The keeper therefore contributes zero body bytes in this context; its normal tagged occurrence still allocates class/object map slots. Other archive modes are excluded. The feature and keeper use the previously verified ordinary inherited behavior; the two new offset readers and keeper binding remain scoped to the copied feature-context map.

Four focused tests cover signed nullable-array presence, duplicate/cached pointers, raw signed extremes, absent-array versus present-array framing, reference/surface version boundaries, required-data null/wrong-class stops, empty tagged keeper and exact map/self identity, following-byte alignment, every truncated empty composed-feature prefix, context rejection and source-hash-bound native complete containing-root spans and fields for both surface and data classes.

Five native offset surfaces have zero sheet counts and empty raw word vectors; two have present extra kernel bodies, parsed by the existing shared grammar. All five data objects have presence 1, arrays of 6/1/1/1/1 moSingleFaceRef_w targets and moDisplayDistanceDim_c pointers. Raw +18 is zero in three and one in two; these observations do not define an enum or allowed array-item domain.

Across 906 complete modern contexts there are **24,492 complete roots**, **775 complete lists** and **131 explicit stops**: 93 additional roots and one additional list relative to Edition 0.228. **702 lists reach exact EOF**, spanning **47,949,325 decoded bytes**, an increase of 1,205,331 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. Further affected parsing reaches later reference-curve/net-blend dependencies rather than skipping them. `resolved-feature-list-frontier-0.228.json` preserves the preceding checkpoint. Whole-document coverage credit is unchanged; complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-251.
