# Shell-feature interoperability

Bessemer reads, reconstructs and edits complete ordinary-part shell features
through the existing resolved-feature API. This advances Feature #54 and goal
#2. The reader consumes the complete body-feature base, followed by six tagged
pointers and one raw byte in the order documented in section 11.147.

The list pointers retain nulls, lengths, item order and shared identity. Native
post-load code can trim list +828 to match list +830; Bessemer preserves the
serialized lists. The raw +818 field retains all eight bits. The dimension
pointer admits the complete supported display-dimension family, the edge
pointer admits ordinary and silhouette component edges, and the solid-body
pointer requires its complete component body. Unsupported nested classes remain
explicit failures. No opaque suffix is used to claim a complete read.

The independent fixture generator builds 18 complete shell objects at model
versions `0x27CC` and 19000. It covers null pointers, shared lists and faces,
unequal lists, concrete distance dimensions, ordinary and silhouette edges,
solid bodies and noncanonical flag bytes. The reference reader reaches exact
EOF and records final object-map slots. Its 5,557 primitive observations
partition every byte. Tests compare those spans, bytes and slots with Bessemer,
reject every truncated prefix and trailing byte, reject old contexts and check
all six declared pointer roles against cached unrelated objects.

Five owned streams use the blank part's actual model and CMgr state with 97
inherited slots. Public tests reconstruct entire documents byte for byte. An
isolated flag edit from 255 to 7 matches a separately constructed expected
stream, preserves shared references and records one field-history change. Both
the source document and source archive remain unchanged. Three Python tests
check deterministic fixture regeneration, variant coverage and invalid builder
options. Positive solid-body and silhouette-edge dependencies are synthetic;
they are not claimed as native shell fixtures.

The local `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 434 library
tests, all integration suites and 265 doctests. Three external corpus gates
remain ignored by default. The focused suite passed three private reader tests,
two public operation tests and three Python fixture tests. `local-verification.json`
records the executed scope.

The native corpus currently reconstructs 463 of 906 feature configurations
exactly in two fresh processes with identical outcomes and failure causes,
preserving all previous 451 successes. The broad parity gate still fails for
443 explicit stops. The independent reader verifies
all 463 accepted streams, totaling 20,357,947 bytes, against four-stream input
hashes. Full schema parity remains incomplete. `frontier-checkpoint.json` and
`accepted-reference.json` retain
the independent observations; `reference.json` binds fixtures, builders and
reference readers. `schema-snapshot.md` preserves the relevant specification
sections with raw-byte hashes in `schema-reference.json`.

Runtime file version 19000 remains distinct from model wire version. Database
paths, geometry, topology and native application acceptance remain outside
stored-schema validation. No dependency or package version changed. The
humanizer skill is unavailable in this session; its prose review remains with
the maintainer.
