### 11.192 PLine silhouette identity representations

*Status: B / ? — native embedded identity type, six populated containing-root records and fixed signed suffix verified; index/geometry meanings remain partial.*

Scope is ordinary non-DB external-XML archives with version greater than `0x27CB`. moPLineSilhIdRep_c primary table `0x4CF087D0`, locator `0x4D32A690` and serializer slot +28 select `0x4BBBD460`. RTTI establishes moRef2EdgeIdRep_c, moRefEdgeIdRep_c and moEntityIdRep_c ancestry. The serializer first calls common entity-representation helper `0x4BBBBF50`, then reads:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | common entity representation | direct common body | always | Existing feature/reference grammar from 11.18, preserving tagged identities. | B / ? |
| 2 | source identity +38 | direct moEntityId_c body | always | Its body is one tagged representation pointer. No extra entity-ID class header, pointer tag or entity-ID map slot. | B |
| 3 | raw +48/+4C | two i32 | always | Preserve both complete signed words; no index-domain or direction semantics are inferred. | V / ? |

Native runtime descriptor `0x4DAD6D90` selects factory `0x4BBA8750`, which allocates 0x50 runtime bytes, calls reference-edge constructor `0x4BA954F0` and installs the silhouette primary table. The inherited constructor calls entity-ID constructor `0x4C6CB040` at member +38. That constructor installs moEntityId_c table `0x4D1FB380`, whose serializer slot +28 selects existing entity-ID helper `0x4BBBC390`. This proves the direct body type; the runtime allocation size is not a file length.

With null/cached tags in the common representation and direct identity, the synthetic body is **14 bytes**: three two-byte pointer occurrences plus two i32 words. The direct identity body adds no map slot; newly tagged representation objects do. The scoped reader composes the existing derived-identity grammar with one further signed word and preserves moPLineSilhIdRep_c as its distinct runtime class.

The audit retains **six completed silhouette identity records**. Two source identities reference moFromSktEntSurfIdRep_c and four reference moPLineSurfIdRep_c. Raw +48 and +4C are one in all six. These observations do not establish universal index/option domains. Five combined focused tests for 11.191–11.192 cover thin-spec scalar order, repeated dimension pointers, duplicate/high-bit bend triples, positive 0xFFFF u16 count framing, impossible-count rejection, complete extrusion composition, nonnull-spec/context requirements, direct entity-ID map accounting, signed suffix extrema, truncation and source-hash-bound native containing-root spans with stored-value equality.

Native verification retains four primary RTTI/COL/type/ancestry bindings, ten native bodies, inherited/helper calls, silhouette factory metadata and the direct entity-ID constructor/vtable/serializer binding. The frontier now has **23,577 complete roots**, **726 complete lists** and **180 explicit stops** across 906 complete modern contexts. **661 lists reach exact EOF**, spanning **38,002,522 decoded bytes**; **65 complete lists retain four-byte unknown tails**. All complete lists pass the recorded stream-size endpoint/tail invariants. Relative to Edition 0.198 this adds 32 complete roots and seven exact-EOF lists. `resolved-feature-list-frontier-0.198.json` preserves the preceding checkpoint. Full part, assembly, drawing and all installed-project bit coverage remains incomplete; whole ResolvedFeatures coverage is not credited.

Evidence: C-220.

