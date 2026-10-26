# Circular-pattern interoperability

Bessemer reads, reconstructs and edits complete ordinary-part circular patterns
through the existing resolved-feature API. This advances Feature #54 and goal
#2. The reader reuses the complete common pattern body, then consumes the suffix
documented in section 11.148. No linear- or mirror-pattern suffix intervenes.

The first four dimension pointers retain the supported display-dimension
family. Dimensions +1198 and +11A0 require angular dimensions. At least one of
+1138 or +1198 must be nonnull; the reader checks this before consuming the
integer collections. Shared dimensions retain their archive identity. The
generic object and line-reference pointers remain distinct, and line fields
accept complete ordinary or temporary-axis references.

Map +11D0 and vector +1200 use fixed signed i32 counts. Negative or oversized
counts fail explicitly. Every serialized map entry and its order are preserved,
including duplicate integer keys that native runtime insertion would replace.
Keys, values and all seven raw bytes retain their stored values without inferred
semantic meaning. The direct point/vector pair appears only above model wire
version `0x3EFA`.

Independent fixtures contain 22 complete objects at `0x27CC`, `0x3EFA`,
`0x3EFB` and 19000. They cover both required-dimension alternatives, shared
dimensions, empty and populated integer collections, duplicate keys, signed
extrema, noncanonical bytes and complete ordinary/temporary line references.
The reference reader reaches exact EOF and records final map slots. Its 21,306
primitive observations partition every byte. Tests compare those observations,
bytes and slots, reject every truncated prefix and trailing byte, reject old
contexts, and check missing dimensions, malformed counts and wrong pointer
families. Populated integer collections are synthetic; native examples have
empty collections.

Five owned streams use the blank part's actual model and CMgr state, including
97 inherited slots. Public tests reconstruct entire documents byte for byte.
An isolated flag edit from 255 to 7 matches a separately constructed expected
stream, retains shared dimensions and duplicate map entries, records one
field-history change and leaves input archives and documents unchanged. Three
Python tests check deterministic regeneration, variant coverage, invalid
construction options, exact observation boundaries and local reader registration.

The local `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 439 library
tests, all integration suites and 265 doctests. Three external corpus gates
remain ignored by default. Five private circular reader tests, two public
operation tests and three Python fixture tests passed. `local-verification.json`
records the executed scope and runtime/test source hashes.

Two fresh native corpus processes reconstruct 483 of 906 feature configurations
exactly with identical outcomes and failure causes, retaining all previous 463
successes. The broad parity gate still fails for 423 explicit stops. The
independent reader verifies all 483 accepted streams, totaling 23,218,570 bytes,
against four-stream input hashes. Full documented schema parity remains
incomplete. `frontier-checkpoint.json` and `accepted-reference.json` retain the
observations.

`shared-observer.md` records the separate fixture-observer extraction.
`reference.json` binds fixture generators, existing builders and reference
readers. `schema-snapshot.md` preserves the relevant specification sections;
`schema-reference.json` records their raw-byte hashes. Runtime file version
19000 remains distinct from model wire version. Database paths, archive
condition `0x800`, geometry, topology and native application acceptance remain
outside stored-schema validation. No dependency or package version changed.
The humanizer skill is unavailable; its prose review remains with the maintainer.
