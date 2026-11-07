# Publication requirements

The folder/configuration checkpoint reported 20 dead-code warnings because the
private CMgr and resolved-feature parsers had no production caller. The public
`PartResolvedFeaturesArchive` and reconstruction example now connect graph
validation, inherited map snapshots and reconstruction. Their focused build
reported no warnings. Private implementation types remain private; no warning
suppression was added. Passing parser tests alone does not prove integration
readiness.

Publication follows the existing `PartConfigurationArchive` conventions:

- Implemented: resolve the configuration ID, model version, selected header and kernel
  context once. Retain the complete inherited map from that configuration.
- Implemented: read CMgr with its own map, validate the configuration graph and helper
  state, then read the separate feature stream through EOF with the model map.
- Implemented: publish immutable owned state with exact stored-archive coverage and explicit
  geometry/external-document exclusions. Preserve typed unsupported failures.
- Implemented: reconstruct and reparse edits against the same context. Return completion,
  diagnostics, validation coverage and revision-bound history together, reusing
  the existing field-change rules where their contracts match.
- Implemented: owned public no-op write, reopening, input isolation, preservation
  of unrelated document streams and an independent expected edit through
  a validated public edit result. Keep application naming
  policy explicit, as established by the native plane-name matrix.
- Implemented: an actual example caller, public doc examples, README and CHANGELOG
  references to issue #54, Send/Sync assertions and deterministic concurrent reads.
- Implemented: connect the unused production paths and pass `cargo xtask pr`.
  Pending: pass the required CI matrix before merging; passing
  `cargo test` alone does not establish a green PR.

Complete schema parity still requires the explicit corpus failures and wider
document classes to be resolved. A public API for supported complete streams
must not claim complete SolidWorks coverage. The humanizer skill is unavailable
in this session; its prose review remains with the maintainer.
