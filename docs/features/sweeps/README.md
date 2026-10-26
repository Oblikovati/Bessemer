# Ordinary-part sweep interoperability

Bessemer reads, reconstructs and edits complete ordinary-part sweeps and sweep cuts through
the existing resolved-feature API. This advances Feature #54 and parity goal #2.
The reader preserves the complete body-feature base, direct sweep data, five
direct arrays, two direct lists, raw scalar values and conditional dimensions.
Direct collections allocate no MFC object slots; tagged objects retain shared
class and object identities.

The two direction predicates independently test equality with 2. A nonzero
enablement byte controls the first generic dimension; the second also requires
mode 3. Angular modes 8 and 9 require both angular pointers. A nonzero radial
word enables the radial pointer. The final sweep-data word appears only after
wire version `0x27d9`. Runtime file version and model wire version stay distinct.

Complete wire-contact, component-profile, endpoint and loft-synchronization
records support these graphs. Endpoint and synchronization records inherit the
existing geometric wrapper with its version gates. Pointer families are finite;
unrelated classes fail. Fresh bidirectional-twist bodies remain unsupported
because their complete schema is undocumented. Section 11.177 establishes that
modern sweep cuts inherit the complete sweep body and add zero suffix bytes.
The historical cut-only fields fall below the supported version range.

Independent reference readers consumed all 160 object fixtures to exact EOF,
recording 102,024 primitive observations. The matrix covers 64 sweeps and 64 sweep cuts across four
versions and eight option combinations, plus 32 complete helper objects across
four wrapper versions. Tests compare every primitive, reconstructed byte and
final object slot; they reject truncation, trailing bytes, incorrect pointer
families, unsupported context and fresh undocumented twist bodies.

Seven owned streams import the blank part's actual model and CMgr context with
97 inherited slots. They cover empty and populated collections, enabled and
disabled dimensions, shared pointers and a separately constructed raw-word edit.
Two public tests reconstruct whole documents byte for byte, compare the edit
with independent expected bytes, check field history and preserve input
documents. Three Python tests verify deterministic fixture regeneration,
the version matrix and invalid constructor inputs.

Two fresh native corpus runs produced identical outcomes and failure causes:
570 of 906 configurations reconstructed exactly, preserving all previous 548
successes. The independent reference reader consumed all accepted feature
streams to exact EOF, totaling 31,327,801 bytes, after checking four input-stream
hashes for each case. The broad parity gate still fails for 336 explicit stops.
`frontier-checkpoint.json` and `accepted-reference.json` retain this evidence.

The staged `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 463 library
tests, four xtask tests, all integration suites and 265 doctests. Three external
corpus gates remain ignored by default. `local-verification.json` records the
executed checks, exclusions and source hashes.

`reference.json` binds the fixtures and generators to reference-reader hashes.
`schema-snapshot.md` preserves specification sections 11.156, 11.164, 11.177 and 11.204;
`schema-reference.json` records their hashes. Native geometry, topology and
application acceptance remain outside archive-schema validation. No dependency
or package version changed. The humanizer skill is unavailable; its prose pass
remains with the maintainer.
