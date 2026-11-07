# Shared line family

The existing two-class line family moved from linear patterns to geometric
references so other declared line fields can reuse the same definition.
Linear patterns and reference-axis data retain their existing accepted types,
field paths and wire behavior. Six focused tests passed. All 906 native corpus
outcomes and failure causes match the preceding checkpoint: 418 accepted and
488 explicitly rejected. `structural-verification.json` records the log hashes.

This structural step prepares the separately tested correction for Bug #46.
It adds no public API or supported layout. The humanizer skill is unavailable
in this session; its prose review remains with the maintainer.
