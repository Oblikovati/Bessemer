# Sheet-metal corners and hems

Bessemer reads and edits the nine native classes documented in section 11.247,
C-289: corner features, base and planar face geometry, hem features and data,
chain data, chains, chain elements and relief data. The behavior version is
`C289-corner-hems-v1`.

Corners and hems retain the complete body-feature and bend-group prefix.
Corners preserve both tagged lists and the final direct list. Their stored
`raw_9a0` value is retained even when zero; native loading's zero-to-two
normalization does not overwrite the wire value.

A hem owns one directly serialized data body without an additional object tag
or map slot. Its feature array and two data arrays have independent counts.
The saved configuration-manager helper selects whether the data's sheet-metal
pointer is present. Six complete alternate-state documents verify this choice
through the actual manager parser, including independent scalar edits.

Face geometry has a 48-byte body. Chain data has ten bytes with null pointers,
an empty chain has two bytes, an element with a null edge has two bytes, and
relief data has 36 bytes. Tagged pointed objects add their own complete bodies.
Engineering option domains and geometric validity remain unresolved.

Independent fixtures contain 378 complete graphs across seven archive versions
and six pointer styles, with 66,711 primitive observations. There are 61 ordinary
complete document streams and six alternate-state streams. Scalar-bearing
classes have independent edit goldens; chain and element fixtures retain pointer
identity, nulls and typed reference checks.

Two fresh native-corpus processes retained all 821 completed configurations and
85 explicit failures, with identical statuses and failure causes. Native sample
407 advanced from an unsupported hem to an unsupported UI bend; it still counts
as incomplete. The independent reader's native regression test verified one
corner root and three hem roots, totaling 354,505 bytes, against stored hashes,
field values, owner identities and containing-list boundaries.

Run `cargo test --lib corner_hem_tests` for exact primitive oracles, body widths,
array counts, owner and reference roles, and exhaustive truncation rejection.
Run `cargo test --test resolved_corner_hems` for reconstruction, independent edits
and input isolation. Run
`python -B docs/features/corner-hems/run_pinned_tests.py` to regenerate fixtures
with frozen copies of the actual `../inventor-interface/solidworks` readers.

The humanizer skill is unavailable in this environment; prose review remains
with the maintainer.
