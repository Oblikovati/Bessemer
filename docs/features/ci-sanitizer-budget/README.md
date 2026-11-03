# Complete ThreadSanitizer workload within hosted job limits

[Bug #183](https://github.com/Oblikovati/Bessemer/issues/183) records the six-hour
cancellation of Linux job 126055056145 in workflow 41675547743. The ordinary
full gate passed; the instrumented suite was still executing dialog tests when
GitHub cancelled it. No test failure was reported before cancellation.

CI partitions complete test targets across eight independent Linux jobs. The
library target comes first, followed by sorted integration targets discovered
from Cargo's default file and directory layouts. Target ordinal modulo eight
selects its partition. Every target appears exactly once; no individual test,
assertion, tolerance, instrumented dependency or standard-library check is
removed. Existing ignored native-input tests retain their original behavior.

All full platform jobs depend on every sanitizer partition. The existing five
required check names stay unchanged. `cargo xtask tsan` keeps the original full
local run; `cargo xtask tsan --shard INDEX/TOTAL` selects the matching partition.
Empty partitions finish without accidentally invoking Cargo's default targets.
Custom target manifests and binary targets fail explicitly until their coverage
is added to the planner.

The correction stays on the existing unreleased publication branch. `main`
contains only the initial README and has no Rust/CI implementation to patch.
The existing PR must pass fresh local and remote gates before merge. No version
bump, tag or release is part of this fix. The humanizer skill is unavailable;
its prose pass remains for maintainer review.
