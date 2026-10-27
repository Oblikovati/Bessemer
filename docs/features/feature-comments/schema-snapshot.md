### 11.193 Feature-comment lists, extra data and text items

*Status: B / ? — native direct-member types, time width and 13 populated containing-root comments/text items verified; kind/string roles and further item variants remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moNewFeatComment_c primary table `0x4CF88938`, complete-object locator `0x4D337D58` and serializer slot +28 select `0x4BBF86D0`. RTTI establishes moFeatComment_c ancestry, but this serializer starts with its direct list; the inherited runtime string/state is not an additional serialized prefix here. Constructor `0x4BBF73D0` establishes suObList at +30 and moCommentExtraData_c at +88. Runtime descriptor `0x4DAD9B70` selects factory `0x4BBF6B70`, which allocates 0xA0 runtime bytes and calls that constructor. The allocation is not a file length.

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | comments +30 | direct suObList body | always | Packed ReadCount followed by that many tagged item objects. Preserve repeated/null identities; no list class header, presence word or list map slot. | B |
| 2 | raw kind +28 | i32 | always in this scope | Native loading begins after `0x23CF`. Before `0x332C`, native maps stored 2 to runtime 1 and other values to 0; offline reading retains the original word. | V / ? |
| 3 | extra data +88 | direct moCommentExtraData_c body | invoked always; consumes bytes only `v >= 0x3B10` | Three i32 words in native member order +0C, +08, +10. Older versions have a zero-byte direct extra body. No extra-data pointer tag or map entry. | V / ? |
| 4 | feature +78 | tagged moCompFeature_c pointer | `v >= 0x3B10` | Preserve standard null/cached/new framing. | B |
| 5 | chain +80 | tagged moChainComponent_c pointer | `v >= 0x3B10` | Separate pointer occurrence after the feature. | B |

moCommentExtraData_c primary table `0x4CF88810` selects `0x4BBF80B0`. The constructor-installed table and serializer prove row 3's direct type and non-offset wire order. With an empty list, an older supported comment body occupies six bytes; at/after `0x3B10` its empty body occupies 22 bytes with null owner pointers. Tagged item bodies and nonnull new owner objects add their own framing and data.

moCommentTextItem_c primary table `0x4CD8DDA0` selects `0x4BBF8380`, which first calls moCommentItem_c helper `0x4BBF82E0`. Its complete ordinary modern body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | raw +08 | i32 | always | Preserve complete signed word. | V / ? |
| 2 | string +10 | CString | always | Stored string role is not assigned solely from class/member position. | B / ? |
| 3 | time +18 | su_CTime, u32 archive value | always | Four wire bytes despite the eight-byte runtime time member. Retain the raw archive value; exact application role/timezone remains open. | V / ? |
| 4 | strings +20/+28 | two CStrings | always | Two separate occurrences, including equal or empty strings. | B / ? |
| 5 | string +30 | CString | `v >= 0x3B10` | No placeholder on older supported versions. | B / ? |

The time call is bound through a bounded raw PE import lookup to swccu.dll's su_CTime extractor `0x31D4BC60`, with its existing four-byte word-reader evidence checked against the installed module. Convenience PE import enumeration truncates before this entry and is not used to infer its symbol. An empty-string text item is 11 bytes before `0x3B10`, or 12 bytes at/after that version, before outer class/object framing. Readers preserve all stored string occurrences and control/text content without merging fields or applying native kind migrations.

The audit retains **13 complete feature comments and 13 text items**. Each comment contains one moCommentTextItem_c; all store raw kind zero, three zero extra words and null feature/chain pointers. All text items store raw +08 equal to zero and include the newer +30 string. Stored time words range from 1,476,207,034 through 1,623,438,529. These observations do not establish complete kind domains, string roles or temporal semantics. Four focused tests cover the `0x332C`/`0x3B10` transitions, retention of stored kind 2, raw extra-word order, cached list/owner identity, four-byte time, string gates, direct-member map accounting, minimum tagged composition, context/truncation rejection and a source-hash-bound complete native containing root with stored text/time equality.

The frontier now has **23,606 complete roots**, **726 complete lists** and **180 explicit stops** across 906 complete modern contexts: 29 additional complete roots relative to Edition 0.199, with unchanged list counts. **661 lists reach exact EOF**, spanning **38,002,522 decoded bytes**; **65 complete lists retain four-byte unknown tails**. Every complete list passes recorded stream-size endpoint/tail invariants. `resolved-feature-list-frontier-0.199.json` preserves the preceding checkpoint. Further comment item types, legacy/special modes and whole part/assembly/drawing/all-installed-project bit coverage remain open.

Evidence: C-221.


### 11.29 Sketch point prefix and supporting references

*Status: Native-derived modern nonminimal-owner layout; installed point collections remain incomplete · Evidence: [C-44], F.32*

This clause applies to ordinary external-XML part archives with v > 0x27CB, a nonminimal sketch owner and neither archive condition 2 nor 4. It specifies a prefix, not the complete sgSketch body. Readers SHALL preserve the inherited object map and SHALL stop on an unsupported reference body, minimal-owner context or nonzero jet presence. Direct points do not introduce an MFC class tag or object-map entry.

The sketch begins with an i32 point count. Each entry is a u16 category followed by the direct point body below. Native category 1 selects an external-point prototype, 2 a frame-point prototype and other values the ordinary prototype; the same archive routine restores each. The next geometry collection is outside this clause.

| Order | Direct entity fields | Encoding |
|---|---|---|
| 1 | permanent mark; unknown_0c | i32; i32 |
| 2 | visual properties | direct moEntVisProp_c (11.12), nonminimal owner only |
| 3 | reference count; stored minimal flag | i32; u16 (0 in this scope) |
| 4 | color; constraint status; line type; width | three u16; direct utLineWidth_c restore type 2 |
| 5 | external entity | sgExtEnt_c pointer |
| 6 | flags_54; flags_56; unknown_50 | u16; u16; i32 |
| 7 | group; description presence; optional description | moGroup pointer; u8; CString when nonzero |
| 8 | unknown_68; unknown_74; unknown_78 | f64; u32; u32 |

The point appends a u16 mask, position, u16 point type and u16 geometric-reference count. Position contains two f64 values when mask bit 0x10 is set, otherwise three. Mask bit 0x20 denotes the unsupported minimal-owner branch. In the supported branch two sgEntHandle pointers follow, then u16 center-reference count, i32 unknown_108 and two further positions using the same coordinate width. A u16 jet-presence word follows; zero is followed by the u16 bidirectional external-reference count. Nonzero jet bodies remain unspecified. Other mask meanings are partial and all stored bits SHALL be retained.

The modern sgEntHandle body shared by sgLineHandle and sgArcHandle is u16 entity index, an i32 replacement index when the u16 equals 0x777F, then i32 reference ID and i32 DimOnCM. sgExtEnt_c contains four pointers (moRefWrapper_c, moDrView, moSkIntersectInfo_c, moSkIsoCurveInfo_c), then reference-axis CString in this scope.

moSketchExtRef_w contains the 11.16 wrapper followed by an i16 mode (-2, -1 or 0) and moBackedUpData_c pointer. moTempAxisRef_w contains that wrapper, two f64 values, a direct three-f64 point, three-f64 vector and u8 unknown_f0. For v > 0x371D it appends two three-f64 points and i32 unknown_128. These wrappers retain the existing component-reference restrictions; further component/feature classes are not implied supported.

