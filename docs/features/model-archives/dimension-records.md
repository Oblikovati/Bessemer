# Ordinary dimensions and handles

Feature #44 now reads base and geometric dimensions, angle and revolved-radius
suffixes, mixed-width tolerance records, integer parameters and ordinary part
attachment points. The documented scaler alias uses the geometric layout.
Integer and floating parameters share the existing node/name prefix reader;
their value widths and alternate-value presence bytes remain distinct.

Model/reference dimension handles and four sketch-handle variants now retain
their component prefix, optional transforms, four drawing-tag pointers,
reference-dimension branch and internal/external target branch. Internal targets
must be nonnull and belong to the declared finite class set. Unknown classes
still fail. These readers do not implement sketch bodies, external-reference
bodies or the separate reference-dimension suffixes.

The owned packet contains 661 cases and 35,708 independent scalar expectations.
It covers the modern string boundary, tolerance string/mask boundaries, every
combination of optional geometric vectors, transform and point bits, high mask
bits, noncanonical presence values, signed integer extremes and negative zero.
Both byte reconstruction and scalar/transform expectations match the approved
reference reader. Every truncated prefix and trailing byte fails.

Handle cases explicitly declare already-loaded `moGFR_c` or `sgSketch` targets
where required. This tests the pointer/context contract, not those targets'
unimplemented bodies. Missing or unrelated loaded targets fail. Ninety-six
internal-target cases also reject null pointers. New records in the full native
corpus continue through normal dispatch and stop at unsupported bodies.

An owned base-dimension edit changes its signed reference count from -7 to
12345. It matches a separately assembled, reference-validated byte fixture,
reparses to the edited fields and leaves the original bytes unchanged. Source
overwrite checks cover all 661 cases. No dimensional evaluation, geometry
validation or native application acceptance is claimed.

Native exact reconstruction increased from 851 to 866 of 906 configurations.
The remaining first failures are 38 `sgSketch` bodies and two existing role
validation failures tracked by #46. The full corpus acceptance assertion still
fails. Earlier measurements remain in `native-corpus-checkpoint.json`.

All six focused tests, all 170 model tests, all-target Clippy and all 50 Python
tests passed. The broader run explicitly ignored its two external-input tests;
the native corpus was executed separately and repeated with identical outcomes.
The exact checks are bound in `dimension-verification.json`. The full PR gate
belongs to the earlier geometric/kernel checkpoint and must rerun before push.
Public model APIs, operation-result contracts and complete corpus acceptance
remain pending. Humanizer was unavailable; prose remains for maintainer review.
