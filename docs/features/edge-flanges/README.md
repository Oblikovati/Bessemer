# Edge flanges

The original C-285 support covers six native classes in section 11.243:
`moEdgeFlange_c`, `moEdgeFlangeData_c`, `moOneBend_c`,
`moMultiEdgeFlangeDataPerEdge_c`, `moEdgeGeom_c` and `moLinearEdgeGeom_c`.
The behavior version is `C285-edge-flanges-v1`.

Feature records retain their complete body-feature base. Edge-flange data is
serialized directly inside its feature, while standalone data has an object tag.
Single bends retain both conditional direct sketch handles without allocating
object-map slots for the handles. Both geometry classes have a 124-byte body.

The saved configuration-manager helper selects the sheet-metal layout. Six
complete helper-one documents cover the feature, standalone data and single bend,
including independent scalar edits. The ordinary fixture corpus contains 684
graphs across nineteen archive versions and six pointer styles, with 218,560
primitive observations and 42 complete document streams. Shared references,
signed extrema and exact floating-point payload bits are preserved.

Positive single-bend raw-body counts and nonzero raw-body presence fields return
explicit unsupported errors. [Conic bend data](../conic-bend/README.md) now has
the complete ordinary schema-1 72-byte codec from C-3579, preserving point/vector
bits and full signed state words. Populated native conic, DB and application
acceptance remain unproven. These scopes do not constitute complete file support.

Two fresh native-corpus processes retained the same 821 complete configurations
and 85 explicit failures. Their statuses and failure causes matched exactly.
Native sample 407 advanced from an unsupported edge flange to an unsupported
unfold feature. The independent reader completed that sample's 851,130-byte
stream and observed six edge flanges, twelve single bends, nine per-edge records
and four linear geometries. Bessemer still reports that file as incomplete.

Run `cargo test --test resolved_edge_flanges` for document reconstruction and
edit isolation, and `cargo test --lib edge_flange_tests` for field-level oracles,
version boundaries and exhaustive truncation rejection. Run
`python -B docs/features/edge-flanges/run_pinned_tests.py` to regenerate fixtures
using an immutable copy of the actual reference readers from
`../inventor-interface/solidworks`.

The humanizer skill is unavailable in this environment; prose review remains
with the maintainer.
