# Shared projected-line data prefix

The existing projected-line data reader owns the required feature pointer and
nullable direct-array grammar. Extracting that prefix preserves the earlier
projected reader's field paths, pointer roles and byte order. All five existing
polyline tests passed with their unchanged four owned fixtures and primitive
oracles (`target/projected-line-prefix-reuse.log`). No earlier baseline changed.
The new C-377 suffixes are exercised separately.

The humanizer skill is unavailable; prose review remains a maintainer task.
