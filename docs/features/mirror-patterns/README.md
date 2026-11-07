# Mirror-pattern interoperability

Bessemer reads, reconstructs and edits complete ordinary-part mirror patterns
through the existing resolved-feature API. The reader reuses the complete
common pattern body already used by linear patterns, then reads a tagged
plane reference when the model wire version exceeds `0x3745` and the always
present direct plane-reference body. This advances Feature #54 and goal #2.

The tagged plane allocates its normal MFC class/object slots. The direct
plane allocates no slot of its own; its nested tagged references still do.
Both planes can share one face component while retaining independent point,
normal and transform fields. `moPlaneRef_w` has a complete tagged-body reader
and occupies the documented wrapper and surface reference families. Mirror
objects also occupy their inherited atom-feature owner role, allowing the
common pattern's cached self references. Unknown dependencies remain explicit
failures, and name-prefix matches do not substitute for declared types.

The common-body extraction first passed eight existing linear-pattern tests.
All 906 corpus outcomes and failure causes remained identical at that
structural checkpoint: 419 accepted. Linear-pattern field paths and expected
bytes were unchanged. `shared-pattern-verification.json` records the evidence.

The independent fixture generator builds 26 complete mirror objects across
12 model-version boundaries and two standalone plane references. Their 7,724
primitive observations partition every wire byte. The source reader reaches
exact EOF and records every object-map slot. Tests check those slots and
primitive spans against Bessemer, reject all truncated prefixes and trailing
bytes, reject old contexts and enforce finite plane, wrapper and surface roles.
MFC schema words are retained independently of the model version.

Four owned feature streams use the blank part's actual model and CMgr state,
with 97 inherited slots. Public tests reconstruct whole documents byte for
byte. A direct-plane coordinate edit matches a separately constructed expected
stream, leaves the tagged plane unchanged, preserves the shared face identity,
retains one field-history change and leaves source inputs unchanged. The source
reader independently checks both plane coordinates and component identity.
Three Python tests check deterministic regeneration, versioned case identity
and invalid construction options.

The local `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 431 library
tests, all integration suites and 265 doctests. Three external corpus gates
remain ignored by default. Four focused body tests and two public mirror
operation tests passed. `local-verification.json` records the executed scope.

Two fresh 906-case runs reconstruct 451 feature configurations exactly,
retaining every previous success. The broad parity gate still fails for 455
explicit stops. The independent reader verifies all 451 accepted feature
streams, totaling 19,653,899 bytes, against four-stream input hashes.
`frontier-checkpoint.json` and `accepted-reference.json` retain those findings.

`schema-snapshot.md` contains the relevant sections of the approved reference
specification with raw-byte hashes. `reference.json` binds the generators,
existing fixture builders and reference readers. Runtime file version 19000
remains distinct from the model wire version. Database paths, geometry,
topology and native application acceptance remain outside stored-schema
validation. No dependency or package version changed. The humanizer skill is
unavailable in this session; its prose review remains with the maintainer.
