# Combine and delete bodies

Bessemer reads, edits and writes ordinary non-DB external-XML
`moCombineBodies_c`, `moDeleteBody_c` and `moDeleteBodyData_c` records newer
than `0x27CB`. Specification 11.220 / C-248 binds both features to the imported
body-feature base and its ordinary node hooks. The delete payload has a
zero-byte ordinary base, a direct array and one signed word; its empty wire
body occupies six bytes, independent of the runtime allocation size.

Combine features retain a typed solid-body component, a separate direct array,
a signed option word and a raw byte only after version `0x3F07`. Stored byte
values remain unchanged, including values greater than one. Delete features
retain a nullable typed payload pointer with ordinary class/object identity.
Direct arrays preserve member order, nulls, duplicates and shared targets;
their element types are not restricted to the observed native sample classes.
Runtime Boolean normalization, cleanup and body operations do not run offline.

The independent reference reader consumed 90 complete graphs at five versions,
producing 16,911 primitive observations. Fixtures cover the option boundary,
signed extremes, raw byte values 0/1/2/128/254/255, independent direct arrays,
complete solid/surface components, complete absent-body solid references,
shared payloads, distinct equal roots and repeated aliases. Parsed identities
are checked separately from byte spans and map counts. Nineteen containing
streams start from the actual 97-slot document map, including an independent
single-word edit golden.

Six private tests compare every field, byte and map slot; reject every prefix,
trailing byte and old context; check wire minima, raw byte gates, declared
pointer roles and bounded escaped counts. Three public tests reconstruct every
containing document and verify successful/rejected edit isolation. Two Python
tests reproduce every fixture and parsed identity check.

Two fresh native processes agree on all 906 outcomes and causes with an unchanged
executable hash. All 740 previously complete cases remain complete; two combine
files now complete, reaching 742. The remaining 164 cases fail explicitly, so the
broad corpus assertion still fails. An immutable, verified copy of 1,225 reference
Python modules independently consumed all accepted feature streams through exact
EOF, covering 46,636,189 bytes. The report retains all four input-stream hashes
and each feature stream's unknown-byte total. Archive coverage is separate from
geometry and application acceptance.

This extends the existing solid-reference implementation from the same section.
Historical, DB and special contexts, geometric validity, SolidWorks application
acceptance and complete documented-schema parity remain unproven. No dependency,
exported Rust API or package version changed. The humanizer skill is unavailable;
its prose pass remains with the maintainer.
