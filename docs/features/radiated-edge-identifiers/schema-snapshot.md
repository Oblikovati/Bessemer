# Approved schema snapshot

### 11.215 Radiated-edge entity identifier representations

*Status: B / ? — native primary ancestry/base call and 14 populated records in complete containing roots verified; signed-word meanings and full document/project coverage remain partial.*

moRadiateEdgeIdRep_c primary table `0x4CF04380`, complete-object locator `0x4D3295D0` and serializer slot +28 select `0x4BBBBEC0` in sldmodu.dll. RTTI establishes moEntityIdRep_c ancestry. The serializer first calls shared entity-representation helper `0x4BBBBF50`, then reads two signed words. In ordinary non-DB external-XML archives with version greater than `0x27CB`, the complete body is:

| # | Field | Type | Presence | Description and rule | St. |
|---|---|---|---|---|---|
| 1 | inherited representation | direct entity-representation body | always | Existing feature pointer and tagged moFR_c external-feature-reference framing, preserving both identities. No additional tagged base object or entity-ID container is inserted here. | B |
| 2 | raw +38 | i32 | always | Preserve the complete signed word without assigning an enum/index meaning. | B / ? |
| 3 | raw +3C | i32 | always | Separate second word immediately following row 2. | B / ? |

No version gate or additional derived suffix appears in this native serializer. Its suffix is eight bytes, and the complete body is **12 bytes** with null/cached inherited pointer occurrences, before outer tagged framing. New feature/FR objects contribute their normal tagged bodies and map allocations.

`bodyfeatures.install_scoped` binds this class to the existing two-word entity-representation reader (`modelboundary.end_face`) only in the copied modern feature-context map. Native ancestry and explicit helper dispatch establish reuse of that grammar; the edge/surface class spelling is not used to infer an archive layout. `radiate_edge_evidence.py` retains primary COL/type/ancestry, the derived native body, shared helper bytes/decompilation and their complete-module hash. Pdata ranges may be fragments; no native function-range size is interpreted as serialized length.

Two focused tests cover a complete tagged representation with cached feature/FR pointers, both signed-word limits, cached self identity, exact map accounting and following-byte alignment, truncation/context rejection, and a source-hash-bound complete native containing root with word/target-class equality.

The audit retains **14 complete radiated-edge representations**. All observed raw +38/+3C pairs are (1,16), with null feature pointers and nonnull moFR_c external references. These values do not establish valid option domains, field roles or restrictions on other pointer combinations.

Across 906 complete modern contexts the frontier now has **24,296 complete roots**, **765 complete lists** and **141 explicit stops**: 14 additional complete roots relative to Edition 0.220, with unchanged list counts. **692 lists reach exact EOF**, spanning **46,159,654 decoded bytes**; **73 complete lists retain four-byte unknown tails**. Completed-list endpoint/tail invariants pass. Further affected parsing reaches unsupported moDerivedFeatRef_w rather than skipping its body. `resolved-feature-list-frontier-0.220.json` preserves the preceding checkpoint. Whole part/assembly/drawing and all-installed-project bit coverage remains unfinished.

Evidence: C-243.
