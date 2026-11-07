# Simulation element projections

Tracking: [#118](https://github.com/Oblikovati/Bessemer/issues/118).
Behavior profile: `simulation-elements-v1`.

The implementation follows approved `../inventor-interface` clauses
14.112–14.113, retained in `schema-snapshot.md`. Complete copied actual
`simulationelementkind` and `simulationtopology` readers provide reference
projections. Installed records are located through actual copied CFB, MAS and
GEN readers with complete paired context. Reader functions and contexts are
not substituted.

## Stored records and classification

`SimulationElementRecord` retains all 140 bytes and an immutable snapshot
identity. `SimulationGenArchive::element_record` supplies a snapshot from the
complete paired GEN image. Classification accepts every unsigned count byte;
the existing mesh accessor continues to reject counts above its eleven slots.
Reading a classification therefore does not weaken the mesh accessor's guard.

The stored family, count, variant and u16 code remain separate from the full
derived u32 selector and its signed view. The all-ones sentinel stays u32::MAX,
with signed value -1. Family-8 explicit tables precede ordered divisibility
fallbacks, including zero-count cases. No engineering names or conversion to
the separate Simulation API enum domain are assigned.

`SimulationElementLegacyContext` requires an explicit version in
800050..=820024 and a supplied signed analysis code. `project_legacy` assumes
this record was visited by the native loop: count two overrides the selector
with 28 for analysis four and 37 otherwise; other counts store the selector's
low sixteen bits. Only bytes 26..28 can change. The outcome includes complete
record bytes, parent/result identity, no-op completion, exact replacement
coverage, requested-word error and runtime/geometry exclusions. This projection
does not establish native loop visitation or execute a GEN file rewrite.
It is never automatically applied to stored current-version records.

## Named topology and first-record predicate

`solid_ten_node` requires nonzero activity, family four and count ten. It
preserves the first four corner IDs and the six ordered midside associations
from the named averaging consumer. Invalid IDs retain duplicates and order.
An optional supplied node-table count enables the upper-bound check; without
it only positivity is checked. The unused eleventh slot remains in raw bytes.
The result carries its complete original record, check mode and exclusions.
Associations do not impose coordinate-midpoint constraints or execute result
averaging, accumulation or runtime selection gates.

`beam_first_record_predicate` tests only a caller-supplied first record after
the native positive-count/non-null-container guards. It checks family two and
stored code unequal to 37; activity, node count and derived classification do
not control this predicate. It does not scan a container or prove that every
record is a beam. True and stored-code exclusion cases are synthetic coverage.

## Evidence and reproduction

An independent closed-form oracle and the actual reader agree for all
16,777,216 unsigned family/count/variant combinations. Owned fixtures also
cover 1,200 literal legacy replacement goldens, sixteen topology cases with
optional bounds and extreme/duplicate IDs, and 216 beam predicates. Tests cover
all short record prefixes, oversized records, version boundaries, source and
returned-byte mutation, no-op identity, Send/Sync and eight concurrent workers.

All 25,106 installed records across three complete paired images retain their
stored code. Exactly 25,094 stored codes differ from the selector; the twelve
matches do not justify rewriting the other historical values. All 14,642
benchmark records pass the named topology gate, yielding 87,852 associations
with no invalid IDs against their paired node count. Both composite images
have no eligible topology records. All three first-record beam tests are false.

```powershell
python -B docs/features/simulation-elements/freeze_elements.py --export
cargo test --test simulation_elements -- --include-ignored
python -B docs/features/simulation-elements/freeze_elements.py --export --verify-rust
cargo xtask pr
```

The second source run also checks the independently emitted Rust projection
packet byte for byte. Baseline replacement requires explicit `--write` and
review. Native input stays in ignored `target/`; normal tests use owned bytes.
Run source snapshot scripts sequentially because their recorder shares one
temporary report name.

## Completed verification

The final local PR gate passed 2,957 tests with zero failures and seventeen
explicitly ignored corpus checks. All eight focused tests passed with corpus
checks enabled. Two fresh processes each ran the complete focused workload and
produced byte-identical 3,062,404-byte native projection packets. The complete
packet also matched the actual frozen readers. Revisions, runtime/test/reader
hashes, check scopes and exclusions are in `complete-verification.json`.

The two processes took 1.149 and 1.422 seconds including startup. Observed
working-set peaks were 59,588,608 and 52,613,120 bytes; one-millisecond sampling
can miss the actual peak. There is no prior published implementation for
comparison, and these observations make no statistical speed claim. Required
Linux ThreadSanitizer CI remains separate from the Windows host checks.

## Acceptance limits

Complete engineering enums, historical producer behavior, native loop
visitation, pointer/count gates, postprocessing arithmetic and application
acceptance remain unproven. These projections do not establish geometry,
topology consistency, self-intersection or tolerance validity. The legacy
projection is not a whole-file migration or a new GEN loader profile.

No dependencies, package versions or release tags changed. The maintainer
humanizer skill was unavailable; the prose pass remains for review.
