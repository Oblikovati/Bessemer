# Surface identifier variants

Feature #54 adds the exact `moFromSktTopSurfIdRep_c` and
`moEndCapChamferSurfIdRep_c` bodies from clauses 11.320 / C-362,
11.332 / C-374 and 11.348 / C-390. Both reuse the existing complete entity-ID
representation prefix and its feature/external-reference identities.

The top-surface primary table `0x4CF6BE68` selects direct serializer
`0x4BBBDE70`, which calls prefix `0x4BBBBF50` and reads one signed i32 +38.
It does not consult the virtual suffix slot. The end-cap primary table
`0x4CF73E98` selects dispatcher `0x4BBB79B0`, with suffix target
`0x4BBBF3F0` reading signed i32 +38 and +3C. No direct source entity,
additional object header, owner word or mapped object is inserted. Raw signed
words retain their full range; no topological index meaning is inferred.

Independent fixtures cover 108 graphs, 17,637 primitive observations and fourteen
owned streams across nine inherited version boundaries and six caller styles.
They retain actual shared feature and FR objects, distinct representations,
repeated roots, cached entity pointers and stored nulls. Two edit goldens change
only the first signed suffix word. Tests compare all fields, map slots and
bytes, reject every prefix and extra byte, retain the modern context guard,
check minimum body extents and reject unrelated cached representation/FR roles.

Run `cargo test --lib surface_identifier_variant_tests`,
`cargo test --test resolved_surface_identifier_variants` and `cargo xtask pr`.
Regenerate with `python -B -X utf8 -m unittest discover -s docs/features/surface-identifier-variants`.
Historical/internal contexts, scalar meanings, geometry validity, application
acceptance and whole-schema parity remain incomplete. Humanizer is unavailable;
maintainer prose review remains open.

The final local PR gate passed 852 library tests, all integrations and 274
doctests. Two fresh native processes matched all 906 outcomes and causes,
retaining all 818 complete reconstructions. Cases 407 and 615 advance to
`moEdgeFlange_c` and `moDisplayWeld_c`; both remain explicit failures and receive
no completion credit. All 88 remaining inputs failed explicitly, so both
aggregate corpus commands exited 101. Independent readers verified EOF for
all 818 accepted streams, 307 recorded prefix roots across five native source
graphs, eight top-surface IDs and eight end-cap chamfer IDs. Historical partial
source roots remain recorded separately from complete Bessemer operations.
