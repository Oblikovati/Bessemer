# Shared pattern body

The existing linear-pattern reader now calls a common-body helper before its
derived suffix. The helper preserves every field path, gate, count budget and
object-map callback. This extraction has an existing production caller and
changes no supported layout.

Eight existing linear-pattern tests passed. All 906 native corpus outcomes
and failure causes match the previous checkpoint, including all 419 accepted
cases. The complete mirror reader is a separate behavior change, supported by
independent full-object and owned feature-stream fixtures.

The humanizer skill is unavailable in this session; its prose review remains
with the maintainer.
