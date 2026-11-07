# Ordinary-part dome interoperability

Bessemer reads, reconstructs and edits complete ordinary-part dome archives
through the existing resolved-feature API. This advances Feature #54 and parity
goal #2. Section 11.159 establishes the complete body-feature base followed by
two signed words, a distance-dimension pointer, a tagged object-list pointer,
two geometric-wrapper pointers and a final signed word. All four pointer tags
are present, and each can independently be null. Historical conversion branches
fall below the supported ordinary context above wire version `0x27cb`.

The list remains a tagged `suObList`, with its own class/object slots and direct
ReadCount body. It is not an inline list substituted for a pointer. Declared
pointer families reject unrelated classes. Raw signed words, nulls, shared
wrappers, root self references and noncanonical count framing remain intact.
The reader assigns no engineering meaning to the raw words and adds no fields
for the native virtual postload call.

Independent reference readers consumed 128 complete object cases to exact EOF,
recording 42,006 primitive observations. Four versions and both inherited-body
variants exercise every combination of the four nullable pointers. Reference
checks verify list/wrapper aliases, self references, empty and populated tagged
lists and final slots. Four private Rust tests compare all primitives, bytes
and slots, check the independent null matrix and signed extrema, reject every
truncated prefix and trailing byte, and reject wrong pointer families and old
contexts.

Four owned streams import the blank part's actual model and CMgr context with
97 inherited slots. They cover null pointers, an empty tagged list, populated
shared graphs and a separately constructed final-word edit. Two public tests
reconstruct whole documents byte for byte, match the expected edit, retain one
field-history change and preserve source documents and archives. Three Python
tests verify deterministic regeneration, matrix coverage and invalid inputs.

Two fresh native processes produce identical outcomes and failure causes:
588 of 906 configurations reconstruct exactly, preserving all previous 574
successes. All fourteen cases previously stopped at a dome now complete.
An independent reference audit verifies all 588 accepted streams to exact EOF,
totaling 33,922,124 bytes, after checking four input-stream hashes per case.
The broad parity gate still fails for 318 explicit stops.

The live reference checkout changed during two audit attempts. The successful
audit used a workspace copy of all 1,018 Python files, verified byte for byte
against a stable before/after source hash map before execution. The copy's hash
map also remained unchanged throughout the audit. `accepted-reference.json`
records its original location, snapshot identity and file hashes;
`frontier-checkpoint.json` retains the native measurements. The temporary reader
copy is removed after verification.

The staged `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 471 library
tests, four xtask tests, all integration suites and 265 doctests. Three external
corpus gates remain ignored by default. `local-verification.json` records the
executed checks, exclusions and source hashes.

`reference.json` binds fixture generators, reused builders and reference readers.
`schema-snapshot.md` and `schema-reference.json` retain the schema clause and
hashes. Full documented schema parity, native geometry, topology and application
acceptance remain outside this checkpoint. No dependency or package version
changed. The humanizer skill is unavailable; its prose pass remains with the
maintainer.
