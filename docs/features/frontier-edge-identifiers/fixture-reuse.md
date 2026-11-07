# Shared identifier fixture setup

The C-392 identifiers need the same saved-model context, primitive observation,
owned streams and equal-but-distinct root objects as the existing trim edges.
`ObjectFixtureFamily` supplies the concrete builder, identity check and local
source registration while retaining the old defaults.

The two existing trim-edge fixture tests passed after this extraction. All 60
object graphs, 14 owned streams, primitive partitions and bytes matched their
unchanged recorded expectations (`target/edge-family-reuse.log`). No production
reader or prior baseline changed. The new family is exercised separately.

The humanizer skill is unavailable in this session; prose review remains a
maintainer task.
