# Model test declaration registry

The model module root reached 495 lines before imported/base-body support. Its
private test declarations now live in `src/model_archives/test_modules.rs`,
included only for test builds. The declarations retain their original names
and paths. They remain siblings of the production modules because the
private suites share parser helpers through that namespace.

At the extraction checkpoint, the root had 193 lines and the registry had 259. No non-test
tokens changed. All 445 library tests passed, with the same three external
corpus gates ignored by default. A fresh native corpus run produced identical
outcomes and failure causes for all 906 configurations, retaining 516 accepted
cases. `test-registry-verification.json` records the comparisons and source/log
hashes. This structural change is committed separately from the new imported
and base-body runtime behavior.

`cargo fmt` does not follow `include!`. Bug #69 adds the registry explicitly to
the formatter command used by both `cargo xtask pr` and `cargo xtask full`.
From the repository root, format all sources with:

```text
cargo fmt --all -- src/model_archives/test_modules.rs
```

The language formatter may order declarations, while their sibling namespace
and paths remain unchanged. A temporary fixture proves that the old command
misses an unformatted private test, that the corrected gate rejects it before
later steps, and that formatting the registry's children makes the gate pass.

The humanizer skill is unavailable; its prose pass remains with the maintainer.
