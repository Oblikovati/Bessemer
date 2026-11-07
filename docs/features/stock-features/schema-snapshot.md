### 11.194 Mirrored-stock suffix and unresolved inherited stock body

*Status: B / ? — native suffix and inheritance chain verified; complete stock/derived-feature grammar, decoder and populated boundaries remain unfinished.*

In ordinary non-DB external-XML archives with version greater than `0x27CB`, moMirrorStock_c primary table `0x4CF58CB0`, locator `0x4D32FFD0` and serializer slot +28 select `0x4BB2CD20`. Native RTTI establishes moStock_c ancestry. The serializer first calls moStock_c helper `0x4BB344D0`, then appends:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | full stock base | direct inherited moStock_c body | always | Required before the suffix. Its complete grammar is not yet established here; no fixed length or opaque skip is assigned (OI-003). | ? |
| 2 | mirror plane +F20 | tagged moPlaneRef_w pointer | always in this modern scope | Historical load gate is `v > 199` decimal. Preserve normal pointer framing; a null/cached pointer occupies two bytes. | B |

moStock_c primary table `0x4D1F1E40` selects `0x4BB344D0` and inherits moDerivedFeat_c. Its serializer calls derived-feature helper `0x4B3F0760`, which calls the body-feature serializer before further direct members and collections. Thus the stock base cannot be replaced by the ordinary body-feature body alone. Retained native bodies show additional derived-feature helper data, collections, stock body/reference handling and transforms whose full versioned framing remains to be verified. The +F20 member offset is a runtime location, not a stream offset or stock-base length.

No new stock/mirrored-stock reader is installed, no populated stock boundary is claimed, and no stock bytes receive coverage credit from this provisional suffix. The native evidence preserves both stock runtime bindings, derived-feature ancestry/dispatch and complete-module hashes so this work can continue from the verified inheritance chain. Complete stock parsing and all installed project formats remain part of the active unfinished objective.

Evidence: C-222.

### 11.195 Derived-feature direct collection suffix

*Status: B / ? — native load/store framing and synthetic boundary checks verified; populated containing-root proof and complete stock composition remain open.*

The moDerivedFeat_c serializer `0x4B3F0760` calls moBodyFeature_c::Serialize before this suffix. In ordinary non-DB external-XML archives with version greater than `0x27CB`, the load branch consumes the following fields. These are direct collections without class headers or object-map slots.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | pairs +800 | u32 count, count × (i32, i32) | `v > 0x2B68` | Loader `0x4B3EF610` reads two words per entry; matching store helper `0x4B3EF7A0` writes the count and pair words. Preserve stored order and duplicates; native insertion may normalize the runtime collection. | B / ? |
| 2 | pairs +880 | same direct pair collection | `v > 0x2B68` | Separate invocation of the same helper, immediately after row 1. | B / ? |
| 3 | words +900 | u32 count, count × i32 | `v > 0x2B68` | Native inserts into a runtime set; offline decoding preserves all serialized words and their order. Runtime set size is not an extra wire field. | B / ? |
| 4 | records +950 | i32 count, count × (i32, i32, tagged moHoleWizardInfo_c pointer) | `v >= 0x373C` | Two raw words precede each independently framed pointer. Null/cached pointers each occupy two bytes; new objects add their own bodies. Collection meanings remain unassigned. | B / ? |

No suffix bytes occur in this scope through `0x2B68`. Empty collections occupy 12 bytes from `0x2B69` through `0x373B`, and 16 bytes at/after `0x373C`. Native loading reads the last count as signed and iterates only for positive values; the offline reader rejects negative counts instead of accepting corrupt collection framing. All counts are bounded against remaining bytes before iteration. The first two loaders' floating-point calculations size runtime hash storage and consume no extra archive values.

`derivedfeatures.payload` implements only this suffix and requires the inherited body to have been consumed by its caller. It does not register a complete derived-feature or stock reader. Two focused tests cover exact gate transitions, duplicate and signed-word retention, repeated cached pointer identity, direct-member map accounting, following-byte alignment, truncation, invalid counts and context rejection. No populated native root is newly decoded, and the frontier/whole-document coverage totals remain unchanged. Full stock transforms, references and later fields remain unresolved under OI-003.

Evidence: C-223.

### 11.196 Stock loading prefix and save-time transfer guard

*Status: B / ? — native stock load prefix and temporary guard identity verified; complete stock body and populated boundary proof remain open.*

The retained moStock_c serializer `0x4BB344D0` first invokes the inherited derived-feature body described in 11.195. In ordinary non-DB external-XML archives with version greater than `0x27CB`, the first stock-specific archive fields are:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | source body reference +A00 | tagged moBodyRef_w pointer | always in this scope | Native historical gate is `v > 0x148`. Older external-object +A50 framing does not occur in the supported modern scope. | B |
| 2 | raw mode +968 | u32 | always in this scope | Native historical gate is `v > 0x89`. Preserve the complete word; the modern body branch occurs exactly for values 1 or 2. | B / ? |
| 3 | embedded body | kernel body archive | mode equals 1 or 2 | Native calls mo_loadBodyFromArchive with archive/body mode arguments zero. The modern branch uses a temporary body and subsequently transfers it into stock runtime state. No additional chain-component pointer is consumed in this scope; that pointer belongs to versions before `0x149`. | B / ? |
| 4 | referenced configuration +A70 | direct virtual serializer body | always in this scope | Native historical gate is `v >= 0x4F4`. This is a direct member invocation, not an extra tagged pointer. Complete composition and subsequent direct members remain unresolved (OI-003). | B / ? |

Raw mode zero or another value omits row 3; it does not supply an empty-body placeholder. Native's mode-1-to-2 migration applies only before version 300 decimal and therefore does not alter this modern scope. Kernel suppression, compression and Parasolid framing still require the existing exact kernel-body grammar; no opaque body skip or guessed stock endpoint is introduced.

The save branch's call `0x4BB18420` before row 1 is **not a transform serialization call**. Its retained body constructs su_CObject, installs the moTransferBodiesIfPossible vtable, constructs two runtime suPtrArray members, and records archive/stock pointers. Its paired `0x4BB18B40` body destroys the same guard after the direct +A30 member operation. Native decompilation incorrectly labels their shared stack storage as mgXform_c in the parent function; stack-variable type alone does not establish serialized transform bytes. The actual explicit +C30 transform archive operation occurs later in the stock serializer and remains separate from this guard.

`stock_comment_evidence.py` now retains thirteen native bodies and validates both guard call targets in the stock serializer. The full module hash binds those native ranges and decompilations. No complete stock reader is installed from this prefix, no populated root boundary is claimed, and frontier/whole-document coverage totals remain unchanged. Constructor binding for remaining direct members and the full later stock suffix remain required before complete decoding.

Evidence: C-224.

### 11.197 Stock direct member types and dynamic word array

*Status: B / ? — factory/constructor binding, direct member types and dynamic-array framing verified; complete populated stock composition remains open.*

Runtime descriptor `0x4DAD2DC0` names moStock_c, records allocation size `0xF20`, and selects factory `0x4BB15B80`. The factory calls constructor `0x4C6AD5C0`, which installs the primary stock vtable `0x4D1F1E40`. The allocation size is runtime storage, not a serialized byte count. This constructor establishes the following direct members used by the stock serializer:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | referenced configuration +A70 | direct moReferencedConfiguration_c | modern scope of 11.196 | Constructor explicitly calls the referenced-configuration constructor at this member. Its serializer follows the body reference/mode/conditional body prefix. | B |
| 2 | objects +A10 | direct suObArray | modern scope | Constructor installs suObArray::vftable after su_CObArray construction. Native load gate is `v > 0x711`. | B |
| 3 | objects +A88 | direct suObArray | modern scope | Same verified construction, separate array; native load gate `v > 0x7E7`. | B |
| 4 | raw +96C | i32 | modern scope | Load gate `v > 0xDC4`. Retain stored bits before runtime flag migrations. | B / ? |
| 5 | objects +A30 | direct suObArray | modern scope | Third separately constructed array, after the raw word. Native loading may transfer its first object into runtime +A08; offline data must retain all stored entries. | B / ? |
| 6 | objects +CF8 | direct suObList | modern scope, later in body | Constructor explicitly creates suObList here; native load gate `v > 0xF73`. Intervening point/map data must precede this member; it is not adjacent to row 5. | B |

These direct containers use their existing count-and-tagged-object bodies without outer class tags or container map slots. Their element objects still participate in normal archive identity. Member offsets neither supply file offsets nor establish adjacent wire layout beyond the sequence expressly stated above.

The later +DE8 member is serialized by helper `0x4BB1DA60`, after raw +DDC and +DE0 words under native gate `v > 0x1047`. Its exact ordinary binary framing is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | element width | u32 | always | Zero terminates an empty array immediately. Nonzero width must be divisible by four; no fixed width is inferred from a single sample. | B |
| 2 | metadata 1 | u32 | nonzero width | Stored allocator metadata retained separately. | B / ? |
| 3 | metadata 2 | u32 | nonzero width | Native saving permits values 1 or 2, but the load branch reads the word without this restriction. Offline reading preserves it. | B / ? |
| 4 | element count | u32 | nonzero width | Bound count against remaining bytes divided by width before reading. A zero count still retains both metadata words. | B |
| 5 | elements | count × width bytes | nonzero width | Native performs one raw byte read. `stockfeatures.dynamic_words` retains each element as width/4 unsigned words, including all high bits and stored order. Application interpretation remains unresolved. | B / ? |

An absent array occupies four bytes. A nonzero-width array occupies `16 + count × width` bytes. This grammar differs from the f64-only display parameter decoder: widths 4, 8, 12 and 20 are exercised synthetically without assigning floating-point or identifier roles. Two focused tests cover metadata, high-bit words, zero-count framing including a large width, direct-member map accounting, following-byte alignment, truncation, malformed widths/counts and context rejection.

The native verifier now retains sixteen bodies and checks factory metadata, constructor dispatch/vtable and the stock-to-array helper call. The direct-member identities and standalone array decoder do not establish a complete stock endpoint. No new stock reader registration, populated-root credit or whole-document coverage increase is claimed; the remaining composed body is still OI-003.

Evidence: C-225.

### 11.198 Stock final collections and historical hole-info ownership

*Status: B / ? — native final-field framing and synthetic composition verified; complete populated stock body boundaries remain open.*

After the direct +C30 transform, moStock_c serializer `0x4BB344D0` consumes the following suffix in ordinary non-DB external-XML archives with version greater than `0x27CB`. All historical gates in rows 1–10 precede this supported scope; row 11 is the remaining version-dependent field. Direct counts below are signed i32 unless stated otherwise.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +DDC/+DE0 | u32, i32 | always in scope | Native historical gate `v > 0x1047`. Preserve the separate words. | B / ? |
| 2 | array +DE8 | direct dynamic word array | always in scope | 11.197 framing, including a single u32 zero for an absent array. | B |
| 3 | map +DF8 | i32 count, count × (i32, i32) | always in scope | Historical gate `v > 0x10F7`. Native may discard keys outside a runtime feature bound; offline reading retains all serialized pairs and duplicates. | B / ? |
| 4 | map +E20 | i32 count, count × (tagged moCompEdge_c, tagged moMirrorHWHelper_c) | always in scope | Historical gate `v > 0x1176`. Each pair consists of two independent pointer occurrences. | B / ? |
| 5 | raw +DD8 | i32 | always in scope | Separate word after row 4. | B / ? |
| 6 | nested sets +E60 | i32 count, records | always in scope | Each record is a tagged moCompEdge_c key, an i32 member count, then that many tagged moCompEdge_c pointers. Retain repeated keys/members and null pointers, without native set normalization. | B / ? |
| 7 | raw +EB0 | i32 | always in scope | Historical gate `v > 0x183A`. | B / ? |
| 8 | map +EB8 | i32 count, count × (i32, i32) | always in scope | Historical gate `v > 0x1854`. | B / ? |
| 9 | map +E90 | i32 count, count × (i32, i32) | always in scope | Historical gate `v > 0x1C47`; follows row 8 regardless of their runtime member order. | B / ? |
| 10 | retired transform collection | no bytes in this scope | absent | Native branch exists only for `0x23A0 < v < 0x23B6`. It does not add a count or placeholder in modern stock archives. | B |
| 11 | historical hole info +950 | i32 count, count × (i32, i32, tagged moHoleWizardInfo_c) | `0x3335 < v < 0x373C` | This list is serialized here only before it moves into the inherited derived-feature body at `v >= 0x373C` (11.195). Do not consume it twice. | V / ? |

Rows 4–6 share historical native gate `v > 0x1176`; it is already satisfied in this scope. After row 11, the native load branch performs runtime cleanup and returns without further archive fields. The minimal suffix is 40 bytes when row 11 is absent, or 44 bytes with its empty count present. These lengths exclude every earlier stock field and its inherited body, and do not establish a complete stock root length.

`stockfeatures.late_payload` implements this final suffix with bounded signed collection counts and raw-value preservation. Negative counts are rejected rather than treated as empty collections by native positive-loop conditions. New nonnull objects require their own supported serializers; cached identities and null pointers retain standard framing. Five combined stock tests cover the standalone dynamic array plus populated pair/nested-set composition, duplicates, cached identity, high-bit/signed words, exact following-byte alignment, every truncated empty-suffix boundary, negative counts and the `0x3335`/`0x373C` ownership transitions.

The retained parent native body and matching store branch supply these field sequences. No complete stock registration, populated stock endpoint or whole-document coverage increase is claimed. Composing the earlier inherited/body/reference/container/point/map/transform stages and verifying against native documents remains required under OI-003.

Evidence: C-226.

### 11.199 Composed stock and mirrored-stock readers

*Status: B / ? — native stage composition and synthetic complete-body boundaries verified; native populated completion blocked by an unsupported nested folder.*

`stockfeatures.read` composes the inherited body-feature body, the versioned derived-feature suffix (11.195), the stock prefix (11.196), the direct members of 11.197, and final collections (11.198). `stockfeatures.mirrored` appends the tagged +F20 moPlaneRef_w pointer verified in 11.194. Both readers are installed only in the copied modern feature-context reader map; they are not global registrations. Every stage requires ordinary non-DB external-XML version greater than `0x27CB`.

The middle segment between the direct +A30 array and final suffix is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | point +C10 | three f64 | always in scope | Native mgPoint_c::restore historical gate `v > 0xB77`; preserve all coordinate values. | B / ? |
| 2 | raw +CC4 | i32 | always in scope | Historical gate `v > 0xDBD`. | B / ? |
| 3 | map +CC8 | i32 count, count × (i32, i32) | always in scope | Historical gate `v > 0xF58`. Retain serialized order and duplicate keys instead of applying native SetAt replacement. | B / ? |
| 4 | list +CF8 | direct suObList | always in scope | Count-and-tagged-object body, constructor-bound in 11.197; historical gate `v > 0xF73`. | B |
| 5 | transform +C30 | 30 u32 archive words, optional perspective suffix | always in scope | Historical gate `v > 0xF7B`. Exact operator import is bound below. Decoder retains all words and explicitly rejects nonzero `words[28] & 0xF0` before any unsupported perspective payload. | B / ? |

At `0x4BB35840`, the native stock load branch calls IAT `0x4CD3C4B0`, bound by bounded raw PE import lookup to sldmgu.dll symbol `??5@YAAEAVsu_CArchive@@AEAV0@AEAVmgXform_c@@@Z`. This is the fixed archive operator described in 11.55, distinct from mgXform_c::restore's presence/matrix representation. The native evidence packet retains the call bytes, IAT lookup and symbol. No transform is silently substituted or skipped.

With an empty latest inherited body, empty containers/strings, null pointers, raw mode zero and zero transform words, the complete stock body occupies **482 bytes** and mirrored-stock body **484 bytes**, before outer tagged framing. A sixth stock test verifies both complete tagged compositions, cached self identity, scoped-only registration, following-byte alignment and truncation. These are synthetic minima, not lengths assigned to populated native features.

The installed 906-context audit now proceeds into the stock readers, but encounters unsupported `moFeatSurfaceBodyFolder_c` nested objects. Its retained primary table `0x4D058618` selects `0x4C224970`, whose decompilation begins with moFolder_c::Serialize before further words and material/cut-list pointers. Complete folder inheritance remains unresolved; no folder bytes are skipped. The frontier remains **23,606 complete roots**, **726 complete lists**, **180 explicit stops**, **661 exact-EOF lists** and **65 four-byte tails**, with unchanged **38,002,522 exact-EOF decoded bytes**. `resolved-feature-list-frontier-0.204.json` preserves the prior checkpoint. No complete native stock endpoint or increased whole-document coverage is claimed.

Evidence: C-227.

### 11.202 Body-reference wrapper and conditional body collection

*Status: B / ? — primary ancestry/base call and ordinary native load/store framing verified; complete populated native references remain blocked by nested component bodies.*

moBodyRef_w primary table `0x4CE8B570`, complete-object locator `0x4D316BB0` and serializer slot +28 select `0x4BB212A0`. RTTI establishes moRefWrapper_c ancestry; the serializer first calls shared wrapper helper `0x4C2842D0`. In ordinary non-DB external-XML archives with version greater than `0x27CB`, excluding smart-copy condition `0x40000`, its body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited wrapper | direct moRefWrapper_c body | always | Existing versioned wrapper fields, tagged components and optional transforms; no extra base class/map entry. | B |
| 2 | raw control +A8 | i32 | always in scope | Native historical gate `v >= 0x3CE`. Preserve stored value; any nonzero value selects the following fields. Do not normalize negative values to a different stored word. | B / ? |
| 3 | body count | i32 | control nonzero | Native historical gate `v >= 0x7D1`; modern scope always has this count when selected. Offline rejects negative or impossible counts. | B |
| 4 | bodies and attributes | count × (kernel body, direct suObList) | control nonzero | Each kernel-body archive is immediately followed by its own count-and-tagged-object attribute list, even for a suppressed body. No outer tagged container or body-reference object is inserted between records. | B / ? |
| 5 | external object +E0 | tagged moExtObject_c pointer | control nonzero | Follows the complete collection, including when body count is zero. Historical gate `v > 1999` decimal. A zero control omits this pointer entirely. | B |

The native ordinary body loop calls mo_loadBodyFromArchive with a kernel-dependent first mode, zero second mode, then directly invokes suObList::Serialize. Installed Parasolid framing is handled by the existing kernel archive reader; unsupported noncompressed forms still stop explicitly. Native runtime attribute reassignment, filtering of invalid bodies and replacement of +B0 do not erase stored occurrences in the offline output. Smart-copy condition `0x40000` adds a preceding control word and can replace body payloads with runtime handle words; that separate archive mode is not inferred from document bytes or accepted by this ordinary reader.

`stockfeatures.body_reference` is installed only in the copied feature-context map. Three focused tests cover wrapper version-size transitions, zero/nonzero raw controls, two suppressed bodies with separate direct attribute lists, repeated cached external/attribute identity, signed control values, tagged self identity, following-byte alignment, truncation, invalid counts and explicit context rejection. The empty latest body is **66 bytes** before tagged framing; nonzero control with an empty collection and null external pointer adds six bytes. Earlier supported wrapper gates change only the inherited length, not the suffix rule.

The native verifier retains eight primary bindings and seventeen bodies, including the body-reference-to-wrapper call. The 906-context audit now reaches unsupported nested `moCompBody_c` in six contexts. Its primary table `0x4CE938F0` selects `0x4BBB8570`; RTTI and a targeted decompilation are retained for the next stage. Complete-root/list/EOF totals remain **23,606 / 726 / 661**, with **180 explicit stops**, **65 four-byte tails** and **38,002,522 exact-EOF bytes**. `resolved-feature-list-frontier-0.207.json` preserves the preceding checkpoint. No complete native stock/body-reference endpoint or whole-document coverage increase is claimed; nested component-body framing remains OI-003.

Evidence: C-230.

### 11.203 Component-body references and native stock completion

*Status: B / ? — native primary ancestry/base dispatch, six populated containing-root component bodies and six stock roots verified; field meanings and full document/project coverage remain partial.*

moCompBody_c primary table `0x4CE938F0`, complete-object locator `0x4D3191F8` and serializer slot +28 select `0x4BBB8570`. RTTI establishes moCompRef_c ancestry; the serializer calls moCompRef_c::Serialize at `0x4BBBA690`. Its complete ordinary non-DB external-XML body, for versions greater than `0x27CB`, is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | component reference | direct moCompRef_c body | always | Existing versioned reference fields, including independent inherited optional transform/string fields. No extra tagged base object. | B |
| 2 | alias presence | u8 | always in scope | Any nonzero byte selects row 3. Retain original byte. Historical gate `v > 0x176`. | B |
| 3 | alias +08 | CString | alias presence nonzero | Native alias lookup supplies the stored string during saving. Preserve the serialized string. | B / ? |
| 4 | raw +F4 | i32 | always in scope | Historical gate `v > 0x648`. | B / ? |
| 5 | transform +F8 presence | u8 | always in scope | Historical gate `v > 0x650`. Nonzero selects direct mgXform_c::restore body immediately after the byte. | B |
| 6 | transform +F8 | mgXform_c::restore body | row 5 nonzero | Presence/matrix, translation, scaling and flags encoding; not the fixed 30-word archive operator used by stock +C30. | B / ? |
| 7 | raw +158 | u32 | always in scope | Historical gate `v > 0x655`. Retain high bits. | B / ? |
| 8 | raw +15C/+160/+164 | three i32 | always in scope | Historical gates respectively `v > 0x664`, `v > 0x66D`, `v > 0x697`; all precede this scope. | B / ? |
| 9 | box +100 | mgBBox_c, ten f64 | always in scope | Historical gate `v > 0x825`. Preserve the full native box body. | B / ? |
| 10 | transform +168 presence | u8 | always in scope | Historical gate `v > 0x12E9`; keep independently from the +F8 presence byte. | B |
| 11 | transform +168 | mgXform_c::restore body | row 10 nonzero | Native saving may omit identity transforms, but offline reading consumes exactly what the presence byte selects. | B / ? |

`stockfeatures.component_body` is installed only in the copied scoped map. The wrapper component whitelist admits moCompBody_c based on verified moCompRef_c ancestry. The empty latest body occupies **185 bytes**, before outer tagged framing. Three tests verify tagged minimum boundaries, cached self identity and wrapper acceptance; noncanonical nonzero presence bytes, alias text, two independent restore transforms, signed/high-bit words, bounding-box framing, truncation and context rejection; and source-hash-bound complete native stock-root boundaries with retained component/stock field equality.

The audit retains **six complete component bodies**: all store +F4 = -1, +158 = 100, and +15C/+160/+164 = -1. All omit aliases and +168 transforms; five include +F8 transforms and one omits it. These observed values do not establish full option domains. All six accompanying stock dynamic arrays have element width 4, metadata 16/1 and one element; no broader array-domain restriction is inferred.

With these nested bodies supported, the audit completes **five moMirrorStock_c roots and one moStock_c root**. Across 906 complete modern contexts it now has **23,614 complete roots**, **732 complete lists** and **174 explicit stops**: eight additional roots and six additional complete lists relative to Edition 0.208. **666 lists reach exact EOF**, spanning **38,163,331 decoded bytes**; **66 complete lists retain four-byte unknown tails**. Thus five additional lists reach EOF, adding 160,809 exact-EOF bytes. All completed-list endpoints and tail lengths pass the recorded stream-size invariants. `resolved-feature-list-frontier-0.208.json` preserves the preceding checkpoint.

These completed native stock roots supersede the earlier provisional stock-boundary limitation for the observed ordinary forms. Other stock modes, perspective archive transforms, special archive contexts and unobserved component/body variants remain open. Exact feature-list EOF is not whole-document bit coverage; unresolved part/assembly/drawing streams and all installed project formats remain part of the active objective.

Evidence: C-231.
