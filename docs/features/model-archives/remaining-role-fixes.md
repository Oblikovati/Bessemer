# Remaining role boundaries

Bug #46's remaining observed first stops now accept the documented spline
segment handle, dimension-handle family and plane component reference. Increment
records also admit the documented derived display dimensions while retaining
both stored pointers regardless of their absence bytes. The plane component
reference has a complete typed body; unsupported nested reference geometry fails
the containing operation. Display-list role validation is unchanged.

Eight independent cases across two versions have 211 scalar expectations.
They cover new section-handle and plane-component objects, a containing
annotation view and increment records with explicitly preloaded handle/dimension
identities. Those scoped identities do not claim new target-body coverage.
The tests failed before the fix. Exact reconstruction, every truncated prefix
and trailing byte, source overwrite and unrelated-class rejection now pass.

Two native runs agree at 891/906, up from 890. All three earlier role first stops
advance: one case completes, one reaches `moSilhEdgeRef_c` and one reaches
`moRefPlane_c`. The remaining thirteen cases stop at extrusion, boss and
revolution feature bodies. No observed first stop is currently a role mismatch;
later dependencies and the wider class audit can still expose more. Bug #46
remains open through integration. The complete corpus gate still fails.

All 253 model tests, all-target Clippy and all 51 Python tests passed. The
verification manifest retains source, schema, fixture and test hashes.
Public operation contracts, selected-configuration diagnostics and native
application/platform gates remain required. Humanizer was unavailable; prose
remains for maintainer review.
