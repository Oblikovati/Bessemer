# Left/right trim-edge identifiers

Bessemer reads, edits and writes `moLeftTrimEdgeIdRep_c` and
`moRightTrimEdgeIdRep_c` in ordinary non-DB external-XML part archives newer
than `0x27CB`. Specification 11.224 / C-252 binds both primary tables to the
same serializer, proves entity-representation ancestry and identifies the direct
source entity through its constructor chain. Both finite names now select the
existing source-representation layout; no new grammar or name-based fallback
was added.

The complete body preserves the inherited feature and external-feature
references, a direct entity-ID body containing its tagged representation
pointer, and one signed word. The direct entity-ID member adds no object-map
slot. With null targets the body occupies ten bytes before outer framing. The
stored word is retained without assigning an enum, Boolean or index domain.

The independent reader consumed 60 complete graphs at five versions with
11,292 primitive observations. Fixtures cover both class names, signed extremes,
complete trim-feature parents, external references, sketch-derived and recursively
derived source representations, nulls, shared targets, distinct equal roots and
repeated aliases. Separate tagged entity-ID owners exercise fresh and cached
representation pointers through the finite declared role. Parsed identity checks
are independent from primitive partitions and slot counts. Fourteen containing
streams use the actual 97-slot document map, including separate left/right
signed-word edit goldens.

Six private Rust tests compare all fields, bytes and slots; reject every prefix,
trailing byte and old context; check the ten-byte minimum and direct-member map
accounting; reject unrelated cached source/external targets and entity-ID
representations; and verify that only the two exact native names select this
layout. Three public tests reconstruct all containing documents and check
successful/rejected edit isolation for both classes. Two Python tests regenerate
every fixture and parsed identity check.

Two fresh native processes agree on all 906 outcomes and causes with a stable
executable hash. All 747 previously complete cases remain complete, and both
files previously stopped at left-trim identifiers now complete, reaching 749.
The remaining 157 configurations retain explicit failures, so the broad corpus
assertion still fails. A verified immutable copy of 1,240 reference Python
modules independently consumed all accepted feature streams through exact EOF,
covering 46,924,581 bytes. All four input-stream hashes and each feature stream's
unknown-byte total are retained in the audit. Geometry and application acceptance
receive no additional credit from archive coverage.

Runtime topology operations, geometry and SolidWorks application acceptance,
historical/DB/special contexts and full documented-schema parity remain unproven.
No dependency, exported Rust API or package version changed. The humanizer skill
is unavailable; its prose pass remains with the maintainer.
