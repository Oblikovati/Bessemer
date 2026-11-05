# Mold parting-line archives

Issue #216 adds the documented ordinary modern parting-line feature, settings,
edge element and face-split-info bodies. The feature retains its body-feature
base, data pointer, signed body count, strict kernel frames and tail words.
Settings retain the owner, direction, component body, five direct collections,
raw Angle bits and signed flag words in native wire order.

The edge base has four generic CObject pointers. A valid stored owner alias in
one of those slots is preserved without imposing component-only classes. Its
split-info pointer and the split-info face pointer keep their declared types.
Nonpositive body counts retain their original signed bits and consume no body
frames. Native list reconstruction, post-load validity filtering, partition
moves and geometry callbacks are outside stored-archive decoding.

Seventeen independent graphs include twelve containing streams. Twelve private
and four public tests cover exact carrier reconstruction, eight independently
expected edits and fresh reversals, revision-bound history, source mutation,
eight-worker isolation, explicit kernel schema, raw-bit domains, malformed
counts and context/resource/type-role rejection. Twenty-six prior regressions
and two fixture tests passed; one native kernel test remained ignored.

Full local and exact-head remote gates remain required before integration.
Legacy/native-modern population, geometry and SolidWorks application acceptance
remain unproven. This feature is excluded from PR212 and the completed
core/cavity gate at0acde5cb. The humanizer skill is unavailable; its prose review
remains with the maintainer.
