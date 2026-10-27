# Reference geometry object dispatch

Five existing match arms moved into `reference_geometry_objects.rs`. They cover
thirteen component, plane, axis and axis-data runtime classes. The axis-data arm
uses the passed class name instead of reaching through its old `ArchiveClass`.
All reader calls and class names remain the same. Unrecognized names return
without reading bytes or changing the object map.

The extraction reduces `object_body.rs` from 497 to 486 lines before further
schema work. Source comparison against the base revision in
`dispatch-verification.json` verifies every moved arm and the remaining root.
Existing owned fixtures pin these bodies. All 486 library tests passed, with
three external corpus tests ignored by default.

Two explicit native runs match the preceding connection-point checkpoint:
all 906 outcomes and failure causes are identical, with 630 accepted streams.
The full parity assertion still fails for the remaining 276 configurations.
This commit changes routing structure only. The humanizer skill is unavailable;
its prose pass remains with the maintainer.
