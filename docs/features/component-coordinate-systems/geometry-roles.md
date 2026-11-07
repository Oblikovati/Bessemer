# Shared component geometry roles

Bug #46 also affected the implemented component-axis and component-point
readers. Their shared reference-geometry field rejected `moCoordSys_c` after
its complete codec was added. Specification 11.170 / C-198 establishes its
`moRefGeom_c` ancestry and shared serializer. Section 11.219 / C-247 identifies
the component-feature-plus-reference-geometry helper that consumes this field.
The fix admits that exact implemented coordinate-system class.

Independent readers verify 24 complete cases with 3,428 primitive observations.
Six versions cover inherited component/geometry gates. Fresh targets are direct
component children; cached targets share a coordinate object introduced earlier
in a genuine direct array. Source assertions check its concrete class and exact
identity. Capturing the target slot before nested name allocation prevents a
cached pointer from accidentally selecting that name object.

Against the pre-fix role, the corrected fixtures produce one passing negative
test and two failing positive tests. The error identifies `moCoordSys_c` and the
four admitted classes. With the fix, all three tests pass. They compare every
primitive, reconstructed byte and slot, reject every truncated prefix/extra
byte/old context, and retain unrelated component-target rejection. Three Python
tests reproduce the packet, pin the matrix and independently check cached
coordinate identity. `geometry-role-verification.json` records the logs and
source hashes; `geometry-roles-reference.json` binds the readers and builders.

The Bug ticket records this dependent fix on the integration candidate because
the runtime library is absent from the documentation-only main tree. Native
geometry/application acceptance and complete schema parity remain unproven.
The humanizer skill is unavailable; its prose pass remains with the maintainer.

The staged `cargo xtask pr` gate passed on Windows x64 with Rust 1.99.0:
497 library tests, four xtask tests, all integration suites and 265 doctests.
Three external corpus gates remain ignored by default. Explicit native replays
retain all 906 outcomes and failure causes, with 638 accepted streams. The
previous independent exact-EOF audit of those same inputs is hash-bound in the
verification record.
