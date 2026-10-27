# Radiated-edge entity identifiers

Bessemer reads, edits and writes `moRadiateEdgeIdRep_c` in ordinary non-DB,
external-XML part archives above version `0x27CB`. Specification 11.215 / C-243
proves its entity-representation ancestry, direct common-helper call and two
signed-word suffix. The complete existing representation codec handles this
class and admits it through the established finite representation role.

The body preserves a feature pointer, external `moFR_c` pointer and distinct
i32 words at native offsets +38 and +3C. It adds no tagged base, direct entity-ID
wrapper, derived count or map allocation. Null and cached pointers keep their
original framing. No prefix-based class matching is used.

Independent readers verify 18 complete graphs across three versions, both null
and populated inherited references, standalone objects and declared-role owners.
The two-owner graph uses a fresh representation in the first direct entity ID,
a cached representation in the second and a final cached array entry. Four
owned streams import 97 actual configuration-map slots and finish with 99, 103,
107 and 107 slots. An independent golden changes only the first signed word.
Tests compare every primitive, byte and slot; reject truncated and extended
records, old contexts and unrelated cached classes; reconstruct full documents;
and verify isolation after edits, source mutation and rejected edits.

`schema-reference.json` records the approved schema section and hashes.
`reference.json` records the complete independent observations and reader hashes.
Raw word meanings, general native validity of synthetic pointer combinations,
native application acceptance, older and DB layouts, and full documented file
schema parity remain unproven. No dependencies or package versions changed.
The humanizer skill is unavailable; its prose pass remains with the maintainer.
