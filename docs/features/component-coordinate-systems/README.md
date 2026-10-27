# Component coordinate-system references

Bessemer reads, edits and writes the documented ordinary modern `moCompCoordSys_c`
body in non-DB external-XML contexts above version `0x27CB`. It reuses the complete
component-feature/reference-geometry helper. The signed feature ID, u32 feature
time and separate tagged geometry occurrence are retained. The supported geometry
family is explicit: reference geometry, planes, axes, points and coordinate
systems. The preceding [role fix](geometry-roles.md) covers coordinate targets
through this shared helper.

Above version `0x3B27`, the body appends exactly 30 u32 transform words. This is
the fixed archive operator, separate from the variable restore encoding. Every
word is preserved without float conversion, normalization or application
ownership fixups. If word 28 has any `0xF0` bits, reading or editing returns a
typed unsupported error containing the complete offending word and expected mask.
These special kinds remain explicit stops in the reference schema.

Independent readers consumed 160 complete cases to exact EOF with 29,844 primitive
observations. Eighteen versions cover inherited component/geometry boundaries
and both sides of the fixed-transform gate. Null, fresh, cached and wrapped
references preserve exact class/object-map identities. Source assertions check
the concrete geometry class and cached target identity before recording oracles.
Six owned streams import the blank part's actual model and CMgr state with 97
slots; the fixed words allocate no objects.

Four private tests compare every primitive, reconstructed byte and slot, reject
every truncated prefix/extra byte/old context, preserve high flag bits, check all
256 low-byte patterns against the documented kind mask, and reject unrelated
geometry pointers. Three public tests reconstruct whole documents, compare a
fixed-word edit against independently constructed bytes, and prove a rejected
special-kind edit leaves its source archive/document intact. Three Python tests
regenerate the complete fixtures, pin the matrix and verify invalid constructor
inputs plus the reference reader's special-kind stops. The directory's three
shared-role Python tests also pass.

`reference.json` binds the generators, reused builders and stable before/after
reader hashes. `schema-snapshot.md` and `schema-reference.json` retain the source
clauses and hashes. Geometry/application semantics, DB/deferred storage, native
application acceptance and complete documented schema parity remain separate
work. No dependency or package version changed. The humanizer skill is unavailable;
its prose pass remains with the maintainer.

The staged Windows x64 `cargo xtask pr` gate passed with Rust 1.99.0:
501 library tests, four xtask tests, all integration suites and 265 doctests.
The three external corpus gates remain ignored by default. Two explicit native
replays retain all 906 outcomes and causes, with 638 accepted and 268 rejected.
This codec causes no native frontier change yet; later missing containing bodies
still precede the documented component reference. The previously audited same
accepted inputs are hash-bound in `local-verification.json`. The relevant schema
clauses still match the prerequisite fix's retained snapshot.
