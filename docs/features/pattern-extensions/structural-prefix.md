# Shared pattern prefix extraction

Issue #54. The existing linear, mirror and circular pattern readers retain their
stored scopes and bytes. The common prefix now returns the containing node,
and the original-dimension arrays have a separate reader. Existing standard
patterns call both helpers; neither creates a new wire field or map slot.

All 29 existing pattern tests passed. Every retained linear, mirror and circular
fixture hash matched the pre-change baseline. These structural changes precede
the table/sketch/dimension-pattern behavior additions. The humanizer skill was
unavailable; prose review is left to the maintainer.
