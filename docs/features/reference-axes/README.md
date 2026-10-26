# Reference-axis interoperability

The ordinary part resolved-feature reader supports `moRefAxis_c` and eight
axis-data classes: the base, surface, face-and-point, plane-and-point,
plane-intersection, line, conical-face and two-point layouts. This advances
issue #54 using specification sections 11.146, 11.240 and 11.341 from the
approved `../inventor-interface` reference.

An axis contains the existing complete reference-geometry body followed by a
required tagged axis-data object. The common data payload is exactly 112 bytes:
two points, two raw doubles, a vector and a final point. The runtime owner gap
does not consume wire bytes. Each derived class reads its declared suffix
pointers in order, retaining inherited object identities and shared references.
Null suffix pointers are permitted; the axis data pointer itself is required.
Supported surface wrappers use a finite class set, and unrelated classes fail
with typed errors. Unsupported nested bodies remain explicit failures.

Nine independently constructed full-object fixtures reach exact EOF in the
reference reader. Their primitive inventories check every stored field against
Bessemer, including negative zero and a NaN payload. Tests cover every truncated
prefix, trailing bytes, old contexts, null required data, pointer roles and
aliases, unknown class names, byte reconstruction and isolated raw-field edits.
The ninth fixture contains a complete temporary-axis reference through the
line-axis data pointer, as documented by the native serializer delegation
in C-44 and the installed examples in 11.146. This finite role case is tracked
under Bug #46; unrelated existing role gaps remain separate follow-up fixes.
Two Python tests check deterministic regeneration and the base's raw bits.

`schema-snapshot.md` retains the relevant specification sections with raw-byte
hashes in `schema-reference.json`. `reference.json` binds the fixture generator
and reference sources. Geometry, topology, database paths, application naming
and native application acceptance remain outside these stored-schema checks.
Other axis-derived classes, including hole-wizard slot-center data, require
their own complete documented bodies and remain unsupported here.

## Verification

The final `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 424 library
tests, all integration suites and 265 doctests. Three external corpus gates
remain ignored by default. Five focused Rust tests and two Python generator
tests passed. Two fresh explicit corpus runs reconstructed 418/906 samples
exactly, retaining all previous 404 accepted cases; the full corpus gate still
fails because 488 cases remain unsupported or encounter unresolved roles.
The independent reader verified all 418 accepted feature streams, totaling
16,877,233 bytes, against four-stream input hashes. The checkpoint and accepted
reference reports retain those results and remaining first failures.

The humanizer skill is unavailable in this session; its prose review remains
with the maintainer. No dependency or package version changed.
