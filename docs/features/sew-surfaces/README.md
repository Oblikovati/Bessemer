# Sew-reference surfaces and empty keepers

Issue #54 implements `moSewRefSurface_c` and `moSewRefSurfIdKeeper_c` in the
ordinary external-XML part context above version `0x27CB`, following clause
11.227 / C-269. The feature uses the existing complete body-feature base and a
direct owned payload. The keeper's exact native thunk imports the zero-byte
`su_CObject::Serialize` body from swccu.dll.

The payload contains five direct object arrays at +30, +50, +70, +90 and +B0,
then six signed four-byte flag words and three doubles in order +18, +28, +20.
The constructor's sixth array at +D0 is not serialized. Empty arrays produce a
58-byte payload. Flag words retain their stored values rather than the native
loader's Booleanized state; doubles retain their IEEE bits. Earlier historical
gates lie outside this modern context and do not extend supported legacy layouts.

The independent fixture set contains 108 complete graphs across nine versions,
23,619 primitive observations and 13 owned streams. It covers empty, null,
populated and wide-count arrays, shared complete face/plane references, signed
extremes, distinct double bit patterns, equal-but-distinct roots and repeated
aliases. Keeper fixtures include actual nonnull body-header callers with fresh
and cached empty keepers. A separate golden stream changes only the first
surface's first flag word while preserving its distinct second surface.

Rust checks complete primitive partitions, exact map slots, all truncated
prefixes, appended bytes, old context, impossible array extents, raw flag and
double values, and rejection of null/unrelated keepers when the header declares
one present. Public document tests check unchanged storage and input isolation
after successful and rejected edits. Run
`cargo test --lib model_archives::sew_surface_tests` and
`cargo test --test resolved_sew_surfaces`; `cargo xtask pr` runs the repository
gate. Run `python -B -X utf8 -m unittest discover -s docs/features/sew-surfaces`
for independent regeneration with stable source hashes.

`schema-reference.json`, `native-binding-reference.json` and `reference.json`
bind the supplied specification, installed part/core DLLs and independent byte
observations. Native reconstruction is recorded separately from synthetic
success, geometry validity and SolidWorks application acceptance. Scalar meanings
remain unresolved, and unsupported later objects receive no prefix credit.

Both fresh native processes reproduce all 906 outcomes and failure causes.
Complete reconstruction remains 761 inputs with 145 explicit stops; cases 351
and 611 advance to filled-surface and composite-curve readers respectively.
No prior success is lost and neither partial input receives completion credit.
The unchanged accepted index set is bound to the prior independent audit of
48,364,098 feature-stream bytes. Separately, a pinned-source replay reproduces
the three original C-269 sew-root spans and all 19,704 bytes exactly. Those
records are in `accepted-reference.json` and `native-sentinel-reference.json`.

The humanizer skill is unavailable in this session; prose review remains a
maintainer task.
