# Component-axis and component-point interoperability

Bessemer reads, reconstructs and edits complete ordinary component-axis and
component-point records through the existing model and resolved-feature APIs.
This advances Feature #54 and parity goal #2. Specification sections 11.160 and
11.163 establish that both classes use the same native serializer: the complete
component-reference base, signed feature ID, unsigned feature time and a tagged
reference-geometry pointer. They add no plane-only `+0x100` word.

The new reader reuses the existing component-feature body. It admits the finite
documented base/plane/axis/point geometry family, retains null tags and requires
complete supported bodies for fresh objects. Reference-point feature bodies
remain a separate explicit stop. Existing geometric wrappers already admit
both component classes; the complete new bodies now make those callers usable.
No class-name substitution or prefix matching is added.

Independent source readers consume 152 complete objects to exact EOF, recording
21,264 primitive observations. Twelve versions cover both sides of the inherited
component and reference-geometry gates. Cases retain both class identities,
empty and populated bases, null/plane/axis geometry and enclosing wrappers.
Seven owned streams import the blank part's actual model and CMgr state with
97 slots. Public tests reconstruct whole documents and match a signed-ID edit
against separately built expected bytes, preserving geometry, history and inputs.
Four private tests cover primitives, bytes, final slots, every truncated prefix,
trailing bytes, wrong geometry classes and absence of the plane-only word.
Three Python tests verify regeneration, the matrix and invalid construction.

The populated fixtures exposed the existing reference-axis owner bug tracked
in #46. Its isolated fix and regressions are documented in
`../reference-axis-owners/README.md` and committed separately. The component
fixtures retain their populated owners. Reusing that feature's geometry builder
preserves all 152 case facts, all seven owned-stream facts and the entire packet
hash exactly; only builder provenance changes.

Two fresh native processes produce identical outcomes and failure causes.
All previous 588 successes remain, and 593 of 906 configurations now reconstruct
exactly. Twenty-five frontiers advance; twenty encounter later explicit stops.
The independent reference reader verifies all 593 accepted streams to exact EOF,
totaling 34,598,692 bytes, against four input-stream hashes per case. The broad
parity gate still fails for 313 configurations. `frontier-checkpoint.json` and
`accepted-reference.json` retain these measurements.

The staged `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 478 library
tests, four xtask tests, all integration suites and 265 doctests. Three external
corpus gates remain ignored by default. `local-verification.json` records the
executed checks, exclusions and source hashes.

The reference audit uses an immutable workspace copy of 1,042 Python files.
Its bytes match a stable before/after hash map of the approved source checkout,
and the copy remains unchanged throughout the audit. Its identity and original
location are recorded; the temporary copy is removed after verification.
`reference.json` binds generators, reused builders and source readers.
`schema-snapshot.md` and `schema-reference.json` retain the schema clauses and
hashes. Full documented schema parity, native geometry, topology and application
acceptance remain outside this checkpoint. No dependency or package version
changed. The humanizer skill is unavailable; its prose pass remains with the
maintainer.
