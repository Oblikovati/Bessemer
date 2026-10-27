# Spline tangency, polygon and curvature controls

Bessemer reads, edits and writes `moSplineTangencyControl_c`,
`moSplineControlPolygonControl_c` and `moSplineCurvatureControl_c` in ordinary
non-DB external-XML part archives above version `0x27CB`. The scope excludes
special sketch-owner archive conditions. Specification 11.164 / C-192 establishes
tangency's complete derivative/sketch helper chain and direct handle types;
11.209 / C-237 establishes polygon and curvature construction and serializers.

Every control begins with a nullable tagged sketch pointer. Polygon appends one
direct spline handle. Tangency and curvature share the derivative body: a direct
spline handle followed by a separate direct point handle. Tangency then retains
one point and one vector. Curvature retains a point, two vectors and its final
raw double. The existing handle reader preserves short and sentinel-escaped
indices, signed reference IDs and dimension-on-CM words without adding object
tags or map slots. Native index lookup, watcher creation and coordinate updates
are runtime behavior and consume no offline wire fields.

The three classes also join the finite declared sketch-user-object role. The
existing base and extension indicator retain their earlier behavior. Unrelated
classes remain rejected; no name-prefix matching or generic wrapper fallback
was added.

`native-ancestry-reference.json` retains all three primary bindings and parent
chains from the reference's native evidence. Each chain includes the declared
sketch-user-object base. The installed module's SHA-256 was checked against that
evidence before accepting the three finite role additions.

Independent readers observe 72 complete graphs and 12,297 primitive reads across
three classes, six versions and four owner/handle styles. They include null and
fresh sketches, earlier cached sketches, and complete sketches containing fresh,
null and repeated control references. Embedded controls refer back to their real
sketch owner. Short and escaped indices vary independently between the two
handle records. Signed extrema, negative zero, infinity and NaN payload bits
remain unchanged.

Nine owned resolved streams import the actual 97-slot configuration map and end
with 99 or 101 slots. Three independent goldens edit only the first direct handle's
reference ID. Public tests reconstruct full documents, compare those edits and
verify isolation after source mutation and rejected count edits. Four private
tests compare complete fields/bytes/slots, reject every prefix and extra byte,
check old contexts and unrelated cached pointer roles, and prove direct handles
do not allocate map entries. Three Python tests regenerate all observations and
check the original sketch builder against 16 owned pre-change byte/slot baselines.

The shared sketch fixture builder gained an optional typed user-object callback;
its original defaults remain identical. Source sections and hashes are retained
in `schema-reference.json` and `schema-snapshot.md`. `reference.json` binds the
independent observations and reader-source hashes. Native geometric validity,
arbitrary synthetic index validity, native application acceptance, special/DB
contexts, historical layouts and complete file-schema parity remain unproven.
No dependency or package version changed. The humanizer skill is unavailable;
its prose pass remains with the maintainer.

The full local PR gate passes 555 library tests, four xtask tests, every integration
suite and 265 doctests; three explicit external-corpus tests remain ignored.
Two fresh native processes preserve all 708 previous complete configurations
and reach 709 of 906. Nine other cases advance to later explicit stops. All
outcomes and causes agree between the runs, with a stable executable hash.
An independent immutable snapshot of 1,143 Python modules verifies every accepted
stream through exact EOF: 42,468,950 feature bytes, with all four original input
hashes checked per configuration. Temporary source copies and audit code were
removed after verification. The remaining 197 configurations stay explicit
failures, so the broad full-corpus assertion continues to fail as intended.
