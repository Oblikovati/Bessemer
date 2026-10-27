# Constraint-plane families

Feature #54 implements seventeen exact classes documented in clauses 11.238 /
C-280, 11.332 / C-374 and 11.336 / C-378. The scope is ordinary external-XML
parts above version `0x27CB`. The C-374 contribution is limited to its parallel
plane data and geometry aliases. Other objects described in that clause have
separate acceptance requirements.

Nine concrete plane-data bodies reuse the existing complete common plane body.
Point/surface planes require complete surface and point references. Constraint
planes retain a nullable tagged interface. The two word-suffix variants retain
one signed word; the angle variant retains its own word and typed angular
dimension, without serializing a runtime-only inherited word. Default-plane
display points are not inserted into these bodies.

The interface contains exactly three geometry tags, including null and shared
targets, with no count prefix. Seven geometry classes retain the common wrapper,
two surface parameters and three point coordinates. Midplane, perpendicular and
tangent geometry add one raw flag byte. Angle geometry adds a raw double and
its own flag byte. `plane_geometry_flags()` returns immutable owned observations
with raw bytes and the native `raw == 1` interpretation. Noncanonical bytes are
preserved during editing and writing.

Bug #46 covers the finite plane-data role and the common-wrapper role. Native
bindings and ancestry support the nine new data classes. The common wrapper
family also admits the complete `moSurfaceRef_w` body introduced in C-377.
The existing external-sketch caller exercises that addition. Its original
eighteen-class fixture corpus remains unchanged; the combined coverage test
requires all nineteen classes. Unrelated cached arrays remain rejected.

Independent fixtures cover 1,620 complete graphs across fifteen versions,
188,867 primitive observations and 126 owned streams. They cover complete plane
parents, bare data, nullable interfaces, required references, shared targets,
distinct equal roots, exact floating-point payloads, all six flag bytes
`0, 1, 2, 128, 254, 255` and eighteen isolated edits. Source identity, EOF,
primitive partitions and reconstruction are checked against stable source
hashes. The source reader ignores expected-class mismatches; native evidence
and negative Rust tests establish the pointer-role contract.

Run `cargo test --lib model_archives::constraint_plane_tests`,
`cargo test --test resolved_constraint_planes` and `cargo xtask pr`.
Regenerate independent fixtures with
`python -B -X utf8 -m unittest discover -s docs/features/constraint-plane-families`.

Two fresh native processes agree on all 906 outcomes and causes. Eleven files
now complete, raising reconstruction from 775 to 786 with no prior success lost.
The aggregate corpus assertion still fails explicitly because 120 cases remain
unsupported. The pinned independent decoder verifies EOF and four original
stream hashes for all 786 accepted inputs, covering 50,415,460 feature bytes.
Under that same source snapshot, eleven original native configurations reproduce
every recorded root interval and class, all recorded root hashes, eight
constraint-plane roots' interface/geometry observations and five remaining-plane
objects' raw fields. Partial source prefixes receive no Bessemer completion
credit. The checkpoint and source reports are retained beside this document.

Historical contexts, geometric meaning and validity, application acceptance and
full schema parity remain open. The humanizer skill is unavailable; prose review
remains a maintainer task.
