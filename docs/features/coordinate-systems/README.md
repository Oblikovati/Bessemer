# Coordinate systems

Bessemer reads, edits and writes the documented ordinary modern `moCoordSys_c`
body and the shared `moCSysAxisData_c`, `moLineCSysAxisData_c` and
`moPntCSysAxisData_c` axis layout. These require a non-DB external-XML context
with version above `0x27CB`. Coordinate systems call the shared reference-geometry
serializer; they do not add the extrusion/revolution body-feature suffix.

Both selectors retain their complete u16 values. An axis appears when either
selector equals its number, with occurrences ordered 1, 2, 3. Duplicate selectors
select one occurrence. Out-of-range selectors select no corresponding axis.
These load-framing cases are preserved for archive reconstruction; the native
writer separately requires exactly two active axes. They make no claim about
native writer validity or usable geometry.

Origin and selected-axis pointers retain explicit class families. Each tagged
axis object contains a direct packed-count pointer array and three f64 vector
components. The direct array adds no outer tag or object-map slot. Shared/null
references, nonunit values, signed zero and NaN payload bits remain intact.
Raw flags stay three separate bytes. The point and late scalar fields are
preserved without recomputing geometry; the two words and three doubles after
the point appear only above version `0x371A`. DB/deferred contents remain separate.

Independent readers consumed 304 complete cases to exact EOF and recorded
61,976 primitive observations. Eight versions cover inherited geometry, the
coordinate suffix and temporary-reference boundaries. Eight selector pairs
cover numeric order, duplicates, zero and out-of-range values. All three axis
classes have empty and populated direct-array cases. Four private tests compare
every primitive, reconstructed byte and slot, reject every truncated prefix and
extra byte, check numeric axis order and reject unrelated pointer targets.

Seven owned streams import the blank part's actual model/CMgr state with 97
slots. Public tests reconstruct complete documents and compare a point-component
edit against independently constructed expected bytes. Selected-axis aliases,
raw vectors/scalars, inherited owner references and source inputs remain intact.
Three Python tests regenerate all bytes/oracles and verify the matrix and
invalid constructor boundaries.

The source checkout changed during initial generation. Successful regeneration
and its tests use an immutable copy of 1,068 Python files, verified against a
stable before/after hash map of the approved checkout. A scoped import finder
keeps reused builders' reader imports in that copy. The copy's hashes remain
unchanged during regeneration, tests and the independent native audit. Its
identity and file hashes are retained; temporary tooling and the copy are
removed after verification.

Two fresh native processes have identical outcomes and failure causes. All
previous 630 successes remain; 638 of 906 configurations now reconstruct
exactly. Ten frontiers advance: eight complete and two stop at later gaps.
An independent audit verifies all 638 accepted streams to exact EOF, totaling
37,003,070 bytes, against four input-stream hashes per case. The broad parity
assertion still fails for 268 configurations. `frontier-checkpoint.json` and
`accepted-reference.json` retain the measurements.

`reference.json` binds generators, reused builders and readers.
`schema-snapshot.md` and `schema-reference.json` retain the relevant clauses and
hashes. Complete schema parity, native geometry/topology validation and native
application acceptance remain outside this checkpoint. No dependency or package
version changed. The humanizer skill is unavailable; its prose pass remains
with the maintainer.

The staged `cargo xtask pr` gate passed on Windows x64 with Rust 1.99.0:
494 library tests, four xtask tests, all integration suites and 265 doctests.
Three external corpus tests remain ignored by default; the native measurements
above come from separate explicit runs, repeated against the final selector
helper. `local-verification.json` records the gate-log and source hashes.
