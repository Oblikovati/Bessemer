# Existing contracts reused by split features

The C-276 split item uses the same fixed 30-word transform operator as C-247
component coordinate systems. The shared reader preserves the earlier field
paths, word-28 rejection boundary, error entity and message. The existing
coordinate-system private and public tests passed, including all 256 flag bytes;
the three source fixture tests regenerated their unchanged baselines.

The split parent reuses the existing direct trim-data prefix. The document test
contract now accepts an explicit optional kernel context while preserving its
old no-kernel wrappers. All three existing copy-edge document tests passed after
this extension. Logs are `target/split-transform-reuse.log`,
`target/split-transform-source-reuse.log` and `target/split-document-reuse.log`.
No prior fixture baseline or stored grammar changed.

The humanizer skill is unavailable; prose review remains a maintainer task.
