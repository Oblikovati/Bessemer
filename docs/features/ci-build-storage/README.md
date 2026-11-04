# CI build storage budget

Bug #196 records the Linux full-gate runner failing at PR #191 head
`425f7034dfa70a587abda229adbcaed4d007a5fa` with `No space left on device`.
The runner could not write its own diagnostic log or publish the job log. Its
check-run failure annotation is retained beside this document. All eight
ThreadSanitizer partitions and the PR Linux gate passed before this failure.

`cargo xtask pr` and `cargo xtask full` apply two settings to their Cargo child
commands: `CARGO_PROFILE_TEST_DEBUG=0` and
`CARGO_PROFILE_TEST_INCREMENTAL=false`. This limits transient test artifact
storage while preserving the complete command arguments, optimization level,
debug assertions, overflow checks, warnings-denied lint and fixture corpus.
Cargo documents [debug information and runtime checks as separate settings](https://doc.rust-lang.org/cargo/reference/profiles.html).

The policy changes only child test-profile settings. Ordinary contributor
`cargo test` defaults and the running xtask executable's dev profile remain
unchanged. Keeping the bootstrap profile stable also avoids rebuilding the
running executable under a different profile on Windows. The separate
ThreadSanitizer command and its instrumentation settings are unchanged.

The related Windows working-tree artifact cache's `debug/deps` directory held
19,994,028,848 bytes when inspected. That cache includes broader and earlier
working-tree builds; it is supporting storage-pressure evidence, not a matched
before/after Linux measurement. The fix still needs its exact-head complete
local gate, artifact measurements and the full remote matrix before integration.
