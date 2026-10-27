# Ordinary-part reference-point interoperability

Bessemer reads, reconstructs and edits complete ordinary reference-point
features and all seven documented point-data classes through the existing
resolved-feature API. This advances Feature #54 and parity goal #2. The point
inherits the complete reference-geometry base, then stores required nonnull
data, an optional profile and an optional surface reference. Their tag order
and actual runtime identities remain intact.

The modern `moRefPointData_c` base has zero body bytes after its MFC tag. It
still allocates class/object slots and requires the explicit ordinary context.
The six derived bodies preserve face-center, center-edge, face-edge,
face/vertex-projection, two-edge and along-curve layouts. Along-curve fields
follow wire order rather than sorting native offsets: curve, two doubles, two
signed words, dimension, then two signed words. No point/vector payload is
inferred for the empty base. Historical conversions below the modern minimum
remain unsupported.

Declared pointer families retain complete face, curve, point, plane, profile
and surface records. Curve fixtures include general curves, circles, lines and
temporary axes with their actual distinct bodies. Projection fixtures include
point, endpoint and loft-synchronization references. Native delegate evidence
and the relevant wrapper clauses are retained in `schema-snapshot.md`. Unknown
fresh bodies and unrelated classes remain explicit errors.

Independent readers consumed 132 complete object cases to exact EOF, recording
21,968 primitive observations. The matrix covers point roots, every standalone
data class, all curve variants, point-wrapper extensions and four real component
geometry callers. Four versions cover the inherited geometry gate. Semantic
checks retain empty-base structure, exact profile classes, repeated two-edge
curves and face/surface aliases. Profile-feature identity keys remain separate
from component-profile keys used by nested endpoint helpers.

Six owned streams import the blank part's actual model and CMgr state with 97
slots. Public tests reconstruct whole documents, include a component-point
caller, and compare an along-curve word edit with separately constructed
expected bytes. They retain profile/surface/curve identities, field history and
source inputs. Four private tests cover every primitive, byte and final slot,
all truncated prefixes, trailing bytes, required data, declared pointer roles,
old versions and unspecified contexts. Three Python tests verify deterministic
regeneration, matrix coverage and invalid options.

Two fresh native processes produce identical outcomes and failure causes.
All previous 593 successes remain, and 611 of 906 configurations reconstruct
exactly. Twenty-one frontiers advance; three encounter later explicit stops.
The independent reader verifies all 611 accepted streams to exact EOF, totaling
35,194,696 bytes, against four input-stream hashes per case. The broad parity
gate still fails for 295 configurations. `frontier-checkpoint.json` and
`accepted-reference.json` retain these measurements.

The staged `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 482 library
tests, four xtask tests, all integration suites and 265 doctests. Three external
corpus gates remain ignored by default. `local-verification.json` records the
executed checks, exclusions and source hashes.

The audit uses an immutable workspace copy of 1,047 Python files, verified
against a stable before/after source hash map before execution. The copy stays
unchanged throughout the audit, and its temporary files are removed afterward.
Its identity, original location and hashes are recorded. `reference.json` binds
fixture generators, reused builders and source readers; `schema-reference.json`
binds the preserved schema clauses. Full documented schema parity, native
geometry, topology and application acceptance remain outside this checkpoint.
No dependency or package version changed. The humanizer skill is unavailable;
its prose pass remains with the maintainer.
