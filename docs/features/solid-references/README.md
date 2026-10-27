# Solid references

Bessemer reads, edits and writes `moSolidRef_w` in ordinary non-DB external-XML
part archives newer than `0x27CB`. Specification 11.220 / C-248 independently
binds the solid-reference primary serializer, its atom-geometry/reference-wrapper
ancestry and the wrapper/raw-word helper. The retained evidence includes the
component-solid-body ancestry used by the wrapper pointer. Its installed-module
hash was checked. This feature implements the solid reference from that section;
the neighboring combine/delete feature bodies remain separate dependencies.

The complete versioned wrapper is followed by a raw u32 and signed i32 body
presence. Zero ends the reference. Any nonzero stored value selects the strict
kernel-body grammar and then a direct attribute list, including when the chosen
body is suppressed. The presence value and suppression byte are independent.
The list adds no class/object-map slot. No body count, terminal pointer or native
ownership/partition fixup is synthesized.

The independent reference reader consumed 243 complete graphs through exact EOF,
producing 25,938 primitive observations. Nine versions cover the inherited
wrapper boundaries and the maximum stored version. The corpus covers null and
complete shared solid-body components; presence 0, -1, 1, minimum and maximum
i32; absent, suppressed and serialized bodies; complete shared attributes;
distinct equal references; and repeated aliases. Parsed target identities are
checked separately from primitive partitions and map counts. Ten containing
streams begin with the actual document-derived map of 97 slots, including an
independent one-u32 edit golden.

Six private tests compare every field span, byte and map count; reject every
prefix and extra byte; check the old-context boundary and declared wrapper and
component roles; verify the version-qualified absent minima; and prove the
attribute-list and explicit-kernel-context conditions. Four public tests
reconstruct all containing documents, match an isolated edit with independent
bytes, preserve the second equal reference and shared targets, and verify input
isolation after source mutation, rejected field-type edits and missing kernel
context. Two Python tests regenerate every fixture and target-identity check.

The documented native solid references have absent bodies. Present branches
are independently exercised by synthetic fixtures without claiming native
sample evidence or valid solid topology. Runtime partition/attribute fixups,
geometric validity, application acceptance, historical/DB/special contexts and
complete documented-schema parity remain unproven. No dependency, exported Rust
API or package version changed. The humanizer skill is unavailable; its prose
pass remains with the maintainer.

Two fresh native processes agree on every outcome and cause across all 906
configurations, with a stable executable hash. All earlier 734 complete cases
remain complete and two more complete, reaching 736. Three other cases advance
to later explicit dependencies and receive no completion credit. The remaining
170 cases retain failures, so the broad complete-corpus assertion still fails.
An independent immutable snapshot of 1,218 reference Python modules verifies
all 736 accepted streams through exact EOF: 45,480,011 feature bytes, with all
four original input hashes checked per configuration. The temporary source
copy and audit script were removed after their process terminated.

The local `cargo xtask pr` gate passed: 617 library tests, four xtask tests,
all integration tests and 265 doctests. Three native opt-in tests remain ignored
in the default gate; both complete-corpus replays above ran explicitly.
