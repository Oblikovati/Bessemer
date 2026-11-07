# Complete paired Simulation images

Tracking: [#112](https://github.com/Oblikovati/Bessemer/issues/112).
Behavior profile: `simulation-paired-images-v1`.

The implementation follows clauses 14.100–14.105 in the approved
`../inventor-interface` specification. `schema-snapshot.md` preserves those
clauses; `source-snapshot.json` records the actual copied Python readers and
their import origins. No reader functions or master contexts are substituted.

MAS support covers the observed 28,420-byte image: nine header words,
81 field definitions, a permutation of 70 physical records, all 5,670 field
patterns and every padding byte. The directory selects logical row 21, field 3
for the signed version. Logical cell IDs bind coordinates to an immutable
snapshot. Construction requires every retained region.

GEN support requires an actual decoded MAS and versions 800050 through 890103.
It checks complete transfer framing, fourteen nonnegative count/scalar words,
all fixed blocks, 52-byte nodes, 140-byte elements, auxiliary records, extra
144-byte records, optional vectors, the mixed 48-byte trailer and exact EOF.
Nonzero delegated counts at indices 0, 1, 2, 5, 6 or 7 are unsupported.
The auxiliary width follows the documented 288-byte trial and range predicate;
failure selects 284 bytes for every record. Neither version nor EOF guesses
the width. An absent vector differs from a present empty vector.

Node views retain activity, the signed group tag and exact binary32 coordinate
patterns. Element views retain activity, accessor count, eight ELIS slots and
eleven signed node slots. Only the selected prefix supplies node references.
Invalid selected references remain visible; unused negative slots are retained.
An accessor count above eleven fails the element view, while the complete raw
image remains available. These are stored-image APIs, not accepted geometry.

`SimulationStudy` composes the paired images with the compound-file codec.
Coordinate replacement reopens the complete CWR, preserves unrelated streams
and returns carrier derivation plus GEN node derivation. Exact pattern error
is zero. Physical allocation may change under the compound codec's documented
rules. No-edit serialization preserves original bytes.

## Verification

Nine independently built owned pairs cover both auxiliary widths, vectors,
absent and empty vectors, an empty mesh, reversed MAS directory order, a second
version and an invalid selected reference. Independent byte patches define
cell and coordinate edit goldens; independently built CWR files define full
carrier expectations. The frozen actual readers supply every field and block
projection, rather than only mesh totals.

The installed corpus contains three complete CWR pairs, 45,246 nodes and 25,106
elements. The GEN images total 5,872,644 bytes. Native inputs stay outside the
repository; their paths and hashes appear in `native-reference.json`.

```powershell
cargo test --test simulation_images
python -B docs/features/simulation-images/freeze_images.py --export
cargo test --test simulation_images -- --include-ignored
python -B docs/features/simulation-images/verify_study_edits.py
cargo xtask pr
```

Reference baseline updates require explicit `--write` and review. Run the Python
scripts sequentially because the source snapshot recorder uses one temporary
report name. Tests include truncated and malformed inputs, foreign revision IDs,
source-buffer mutation and concurrent edit/revert operations. Public types have
compile-time Send/Sync assertions. ThreadSanitizer and native-application checks
are separate acceptance work; they are not implied by a successful Rust test.

The Windows PR gate passed 2,919 tests with 12 corpus-dependent tests ignored.
All seven focused tests passed when the installed Simulation corpus was enabled.
Two fresh processes produced identical 6,037,524-byte native edit bundles,
compared byte for byte. `complete-verification.json` records hashes, options,
observed time and memory, and acceptance exclusions. There is no previous
published implementation for a release comparison.

## Limits

Unknown field roles, solver behavior, units and coordinate frames remain
unassigned. Geometry, topology, self-intersection and tolerance consistency are
not established. Native SolidWorks acceptance is not established. The API
reports these exclusions explicitly. Other versions, nonempty delegated
collections, solver results and project-level operations require separate
documented profiles and fixtures.

No dependencies were added. The maintainer humanizer skill was unavailable;
the prose pass remains for review.
