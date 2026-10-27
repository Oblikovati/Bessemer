### 11.175 Revolved reference surfaces

*Status: B — native inheritance, inline surface dispatch, data order and populated corpus boundaries verified; raw mode meanings and runtime geometry remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`, binary archive flag `0x10` clear and shared surface helper +834 zero. moRevolvRefSurf_c primary table `0x1804518A8` in sldMoPartu.dll selects serializer `0x1802234D0`. Its verified ancestor is moRefSurface_c. The serializer calls shared moRefSurface_c::Serialize `0x4BB31450`, which begins with the common reference-geometry body `0x4C283A00` and then the existing shared surface payload.

The revolved class's +AB0 virtual slot selects leaf `0x1800E3930`, whose exact instructions are xor eax,eax; ret. It therefore returns zero, establishing the same inline sheet/body framing used by the documented planar surface path. A nonzero stored archive-backend marker does not substitute a pointer or deferred-body shortcut for these inline fields. Existing strict embedded-kernel framing/decoder limits remain in force; unsupported bodies still stop rather than being skipped. After the complete shared surface body:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | revolve data +7B0 | tagged moRevolvSurfData_c pointer | always | Preserve null/new/cached identity. Native loading permits a null data pointer. | V |
| 2 | axis +7B8 | tagged moLineRef_w pointer | always in modern scope | Historical gate after 0x8DE precedes this scope. Null is permitted; nonnull must have the supported line-reference type. Additional derived line types remain explicit dependencies. | V |

No extrusion/revolution body-feature end-spec suffix follows these two pointers; this feature inherits reference-surface framing. Native loading allocates a moRevolvRefSurfIdKeeper_c runtime object when the inherited ID keeper is absent. That allocation introduces no pointer/class header or additional serialized body at this position. Post-load update calls and deletion of some runtime data references likewise do not add wire fields. The latest null/optional-empty feature body is **299 bytes**, including the shared reference-geometry and empty inline surface payload; this is a synthetic boundary fixture, not native engineering validity.

moRevolvSurfData_c primary table `0x180451818` selects `0x180223680`. After its zero-byte ordinary binary su_CObject layer:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | angular dimension +08 | tagged moDisplayAngularDim_c pointer | always | Null/new/cached framing, using the existing angular display-dimension reader. | V |
| 2 | raw +1C | i32 | always | Preserve the full signed word. | V / ? |
| 3 | angular dimension +10 | tagged moDisplayAngularDim_c pointer | always in modern scope | Gate at 0x15E precedes scope. The stored pointer is read regardless of later runtime mode cleanup. | V |
| 4 | raw +18 | i32 | always in modern scope | Retain this word after the second dimension pointer, not beside +1C. | V / ? |
| 5 | end specification +20 | tagged moRevEndSpec_c pointer | always in modern scope | Gate at 0x118B precedes scope. Uses the existing revolution end-spec reader. | V |

The null-pointer data body is **14 bytes**. The two angular dimensions may share a cached object. Offline reading retains both pointer occurrences even though native feature loading can delete the second runtime dimension for modes other than its two-direction case. Earlier angular-value corrections before version `0x26C` are excluded. Raw +1C/+18 values and complete mode meanings remain partial.

moRevolvRefSurfIdKeeper_c primary table `0x180451780` selects thunk `0x1803ACE78`, which jumps through IAT `0x1803EB580` to swccu.dll su_CObject::Serialize export `0x31D3F030`. Raw PE descriptor/IAT/ILT evidence establishes the import beyond the convenience parser's truncated import inventory. The target tests archive flag `0x10` and skips its XML metadata path when clear. The keeper therefore consumes **zero ordinary binary body bytes**, with no class-specific fields. Tagged instances still allocate ordinary MFC class/object-map entries; no general zero-byte rule is inferred for unrelated keeper classes or special archive contexts.

The complete-root audit retains **eight revolved surfaces** and **eight moRevolvSurfData_c objects**. Every surface has backend marker one, zero sheets, no extra body, a data pointer and a moLineRef_w axis. Both angular dimension pointers are null in these samples; every end-spec pointer is moRevEndSpec_c. Raw +1C/+18 pairs are `(0,0)` in four records, `(1,0)` in three and `(0,2)` in one. These observations establish the saved-record boundaries without assigning engineering mode names or crediting a populated native angular-dimension branch. Synthetic tests cover populated/cached dimension identity, raw signed values, minimum bodies, tagged keeper identity, axis-class rejection, context and truncation. Native tests verify stream SHA and complete containing-root spans.

Edition 0.186 records **22,529 complete roots**, **683 list-complete configurations** and **223 explicit stops** across **906 complete modern contexts**. **621 lists reach exact EOF**, spanning **33,289,354 decoded bytes**; **62 complete lists retain four-byte unknown tails**. Cut lists, sweep cuts, annotations, helices and other feature/chooser dependencies remain unresolved. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; no whole-document coverage credit is assigned to these boundaries.

Evidence: C-203.
