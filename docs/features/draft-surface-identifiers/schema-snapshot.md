# Approved schema snapshot

### 11.225 Draft and back-draft surface identifiers

*Status: B / ? — 25 draft and 41 back-draft identifiers verified inside complete native roots; signed-word meanings and full-document/project coverage remain partial.*

Scope is the ordinary non-DB external-XML feature context, version >0x27CB. sldmodu.dll moDraftSurfIdRep_c primary 0x4CF6B120 / COL 0x4D333EF0 and moBackDraftSurfIdRep_c primary 0x4CF6C3B8 / COL 0x4D334388 both select serializer **0x4BBBDE70**. Native RTTI establishes moEntityIdRep_c ancestry. The serializer calls shared helper 0x4BBBBF50, then reads one signed word:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited representation | direct moEntityIdRep_c body | always | Feature pointer and tagged moFR_c external-feature reference, preserving normal archive identities. | B |
| 2 | raw +38 | i32 | always | Exact signed stored word, without assigning an index/enum domain. | B / ? |

No derived version gate or additional suffix appears in the native body. With null/cached inherited pointer occurrences, the complete body is **eight bytes** before outer tagged framing. New feature/FR targets contribute normal tagged bodies and class/object map slots. The class names do not imply another draft-feature body or kernel-surface record here.

Both names are bound to the existing one-word entity-representation reader only in the copied modern feature-context map. Native primary dispatch, ancestry and the explicit common-helper call establish grammar reuse independently of their spelling. The evidence packet retains both COL/type/ancestry bindings, shared serializer/common-helper native bytes and decompilation, and the complete module hash. Pdata may describe fragments; native code sizes are not wire sizes.

Three focused tests cover both classes with cached feature/FR identity, both signed-word limits, every truncated minimal body prefix, tagged minima/self-map accounting at 0x27CC and version 19000, exact following-byte alignment, scoped-context/version rejection and source-hash-bound complete native containing-root spans/raw words/target-class equality for both types.

The audit retains **25 draft identifiers**, with raw +38 values 0–9, feature pointers null or moDraft_c, and external references moFR_c. **41 back-draft identifiers** have raw values 1,2,3,5,6, feature pointers null or moExtrusion_c, and moFR_c external references. These observed ranges do not establish valid domains or field roles.

Across 906 complete modern contexts there are **24,569 complete roots**, **780 complete lists** and **126 explicit stops**: 63 additional roots and three additional lists relative to Edition 0.230. **707 lists reach exact EOF**, spanning **48,314,752 decoded bytes**, an increase of 313,435 bytes; **73 complete lists retain four-byte unknown tails**. Endpoint/tail invariants pass. Later affected parsing reaches explicit sketch-intersection, delete-face or reference-plane-data dependencies. `resolved-feature-list-frontier-0.230.json` preserves the preceding checkpoint. Whole-document coverage credit is unchanged; complete part/assembly/drawing/all-installed-project bit coverage remains unfinished.

Evidence: C-253.
