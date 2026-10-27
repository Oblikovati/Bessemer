# Approved schema snapshot

### 11.174 Draft features and edge and face identifier keepers

*Status: B — native modern field order, helper conditions and populated draft boundaries verified; raw options, populated dynamic arrays and runtime geometry remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`, excluding archive-condition 1. moDraft_c primary table `0x180483540` in sldMoPartu.dll selects `0x1802DA600`. The serializer first calls the existing body-feature base, then reads:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | dimension +800; raw +830; direction +868; word array +828 | tagged moDisplayDim_c; i32; tagged moDirectionSpec_c; tagged su_CWordArray | always | Preserve pointer identities and full signed word. | V / ? |
| 2 | array +818 | i32 presence; direct suObArray when nonzero | always | suObArray::serializePtr framing: packed count and tagged elements after presence, without an outer array class tag or map slot. | V |
| 3 | raw +860/+840/+844 | three i32 | always | Retain original +844 before native option conversion using +840. | V / ? |
| 4 | array +820; raw +848; surface body +850; plane +858 | nullable direct array; i32; tagged moCompSurfaceBody_c; tagged moPlaneRef_w | always | Array uses the same presence/body framing as row 2. | V / ? |
| 5 | raw +870; dynamic array +878 | i32; conditional direct array body | array only when +870 nonzero | Original signed word is retained. Dynamic framing is described below. | V / ? |
| 6 | raw +880; face-group presence | i32; i32 | always in this scope | Group helper is true for the modern ordinary context; the stored presence word still determines whether row 7 occurs. | V / ? |
| 7 | raw +72C; face groups +890 | i32; i32 outer count and counted groups | group presence nonzero | Each group begins with i32 face count followed by tagged moCompFace_c pointers. Preserve empty groups and repeated faces. | V / ? |
| 8 | points +8C8 | i32 count; count × three f64 | always | Direct point values, with no class/pointer tags. Preserve coordinates. | V / ? |
| 9 | dimension +808; raw +810/+814 | tagged moDisplayDim_c; two i32 | version > 0x3746 | Older modern versions omit all ten bytes. | V / ? |
| 10 | keeper +838 | direct moDraftSIdKeeper_c body | always in modern scope | No keeper presence marker or MFC tag. Its constructor-proven type selects the inherited edge/face serializer below. | V |

All preceding historical field gates, including the dynamic array and point collection, precede the documented minimum. Reject negative or impossible group/face/point counts. Native archive-condition `0x800` and helper +834 can clear an existing runtime +818 array before loading; they do not replace its wire presence/count framing. The reader preserves stored references and raw values without executing this cleanup or native +844 remapping. Native load-time update calls after the late dimension fields add no payload fields.

Face-group helper `0x1802B4B00` consults environment setting `0x4F0`, file version, archive-condition 1 and helper +834. In this scope, version exceeds `0xC73` and condition 1 is absent, so it returns true regardless of the environment setting. Therefore the group-presence i32 is read in every supported modern draft. A nonzero value, including a noncanonical signed value, introduces raw +72C, outer count and groups. The writer's environment-selected presence value does not justify omitting that word in an ordinary reader.

Dynamic helper `0x1802DA5A0` delegates to load `0x1802D9910`/`0x1802D9830` and store `0x1802D9940`. Framing is u32 element size; zero ends the array body. Otherwise it is u32 capacity, u32 growth/allocation word, u32 element count, then exactly element-size × count raw bytes. Nonzero element size must be divisible by eight. The existing dynamic-array reader preserves both header words and raw payload and counts payload bytes as unknown meanings. This is not packed ReadCount or a tagged double-array object; +870 presence is separate from the array's own zero-size form.

Runtime descriptor `0x1806052D0` selects draft factory `0x1802D9800`, which calls constructor `0x1802D9A30`. That constructor allocates the +838 keeper and invokes `0x1802679A0`, installing moDraftSIdKeeper_c primary table `0x180475608`. Its serializer slot is six-byte thunk `0x1803AC9DA`, which jumps through IAT `0x1803E2AB0` to sldmodu.dll export moEdgeFaceSIdKeeper_c::Serialize at `0x4C6E0850`. The import name, full export-table ordinal/address binding and inherited RTTI chain are verified. There is no additional draft-keeper suffix.

moEdgeFaceSIdKeeper_c primary table `0x4D1FB718` selects that same serializer. Its body, used both directly and for scoped tagged keeper instances, is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | surface IDs +18 | i32 count; count × direct moEntityId_c body | always | Existing entity-ID body contains a tagged representation pointer; no outer entity class tag. | V |
| 2 | references +50 | direct suObArray body | always | Packed count and tagged reference objects. No outer tag/map slot. | V |
| 3 | surface IDs +28 | i32 count; count × direct moEntityId_c body | version > 0x3746 | Entire second array, including its count, is absent in older modern files. | V |

Keeper constructor `0x4C6E0170` installs suObArray's vtable at +50. Shared mo_dynSurfIdArr helper `0x4BBBE2E0` establishes the signed count and direct entity-ID bodies. Runtime external-object replacement and edge-reference use-count updates do not add ordinary archive fields and are not executed offline. Latest empty keeper size is **10 bytes**, versus **six bytes** through `0x3746`. The null/optional-empty draft suffix is **74 bytes** latest and **60 bytes** through `0x3746`; including the latest empty body-feature base gives **325 bytes**. These synthetic minimums establish framing, not engineering validity for arbitrary values.

The complete-root audit verifies **10 drafts**. All have +818 array presence one, +820 absence, dynamic-array +870 zero, group presence one and empty direct keepers. Six have one face and one point, two have four faces and four points, one has three faces and three points, and one has two single-face groups with two points. Native tests verify stream SHA and complete containing-root spans for these group shapes. Synthetic tests cover populated dynamic arrays, empty/repeated face groups, raw values before normalization, direct/tagged keeper identity, entity-ID representations, version gates, malformed counts/stride, truncation and context rejection. Populated native dynamic arrays and nonempty native draft keepers remain unverified.

Edition 0.185 records **22,506 complete roots**, **680 list-complete configurations** and **226 explicit stops** across **906 complete modern contexts**. **619 lists reach exact EOF**, spanning **33,244,902 decoded bytes**; **61 complete lists retain four-byte unknown tails**. Revolved surfaces, cut lists, sweep cuts, annotations and other feature/chooser dependencies remain unresolved. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these boundaries.

Evidence: C-202.
