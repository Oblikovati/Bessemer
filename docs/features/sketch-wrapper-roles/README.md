# Sketch external-entity wrapper roles

Bug #46 records native case 351, `learn/Mouse/Mouse Master.SLDPRT`, failing at
`sgExtEnt_c.reference` with a complete `moSingleFaceRef_w`. The field declares
`moRefWrapper_c` in specification 11.29 / C-44. Its previous finite list covered
the base, sketch-external and temporary-axis variants but rejected other known
wrappers. The fresh/cached single-face regression fails before the fix with the
same native error.

The field now reuses `model_references::WRAPPER_CLASSES`, the existing reviewed
eighteen-class family already used at other model wrapper boundaries. No tagged
body reader or global assignability rule is added. View, intersection and
isocurve pointers retain their separate types. Unrelated fresh node-name objects
and cached arrays remain invalid. The shared display-list policy is unchanged.

The independent fixtures cover 1,188 graphs across eleven versions and six
reference styles, with 71,568 primitive observations and 126 owned streams.
They include nulls, fresh references, seeded cached references, distinct equal
targets, repeated owners, the temporary-axis suffix gate and wrapper-word gates.
Eighteen independently assembled edits change the first target's common signed
word while preserving the distinct second target.

The approved source parses complete class bodies and verifies actual identities,
EOF and primitive partitions. Its MFC reader deliberately ignores expected-class
mismatches, so it is **not a role oracle**. The source also parses the complete
unrelated node-name fixture; Rust rejects it at the typed field. Native field
declarations, supplied dispatch evidence and the existing reviewed wrapper family
provide the role contract. `native-binding-reference.json` records these limits
and binds the unchanged canonical family and installed model DLL.

Rust checks all eighteen classes through fresh and cached pointers, exact bytes
and map slots, every truncated prefix, appended bytes, old context and unrelated
pointer roles. Public tests cover document storage preservation and input
isolation after successful and rejected edits. Run
`cargo test --lib model_archives::sketch_wrapper_role_tests` and
`cargo test --test resolved_sketch_wrapper_roles`. The repository gate is
`cargo xtask pr`. Regenerate independently with
`python -B -X utf8 -m unittest discover -s docs/features/sketch-wrapper-roles`.

`schema-reference.json` binds clauses 11.16 and 11.29; `reference.json` records
the fixture observations. Native reconstruction is recorded separately from
synthetic success, geometry validity and SolidWorks application acceptance.
The bug affects the unreleased model runtime on the candidate branch; that
runtime is not present on the current `main` patch line. Bug #46 remains open
for the wider finite-family audit.

Two fresh native runs reproduce all 906 statuses and causes. Case 351 advances
to unsupported `moSewRefSurface_c`; it remains incomplete. Complete reconstruction
stays at 761 inputs, with 145 explicit stops and no lost successes. The accepted
index set is identical to the prior independent audit of 48,364,098 feature-stream
bytes; `accepted-reference.json` binds that report without claiming a fresh replay.

The humanizer skill is unavailable in this session; prose review remains a
maintainer task.
