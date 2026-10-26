# Sketch role regressions

Bug #46 now admits the documented extension-line indicator in sketch user-object
lists and both concrete help-array names. Four independent fixtures cover new
objects, aliases and a sketch self-reference, with 346 scalar expectations.
They failed before the fix. Unrelated classes, truncated prefixes and trailing
bytes still reject; overwritten source buffers do not change owned results.
Production display-list role validation is unchanged.

Native reconstruction advances from 886 to 890/906. The remaining sixteen stops
are thirteen feature bodies and three role mismatches: a spline handle in a
segment-handle field, the older dimension handle and the older component
reference. The complete corpus gate still fails, and Bug #46 stays open.

The earlier tail fixture generator reproduces its original wire packet exactly.
Its report's source-inventory metadata refreshed during revalidation; the old
report hash remains in the historical verification manifest, and both hashes
and the reason are recorded in `sketch-role-verification.json`. No wire baseline
or expected scalar value was changed. Clippy and all 51 Python tests pass.
All 250 model tests passed, including the three new regressions. Public operation
contracts and the remaining feature bodies remain required under #44.
Humanizer was unavailable; prose remains for maintainer review.
