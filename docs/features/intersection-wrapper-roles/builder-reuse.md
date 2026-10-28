# Intersection fixture builder reuse

Bug #46 needs complete reviewed wrapper bodies at all four intersection fields,
with allocation order matching the wire. The existing complete intersection
builder now accepts a wrapper-pointer callback. Its containing sketch-tail
builders accept one complete-intersection callback, preserving their existing
styled curve, nonzero-marker intersection and remaining sketch fields.

Before the hooks were added, 448 original cases were pinned by length, SHA-256
and final map-slot count. They cross sixteen versions, rich/minimal bases,
empty/imported maps and nonpositive handle counts. The baseline test verifies
that every original default remains unchanged. Existing spline-control sketch
baselines independently retain their earlier bytes and identities.

The hooks allow the new regressions to reuse complete inherited records and
retain object identities without copying their serializers. This preparation
changes no production behavior. Humanizer is unavailable; prose review remains
with the maintainer.
