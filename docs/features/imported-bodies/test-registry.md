# Model test declaration registry

The model module root reached 495 lines before imported/base-body support. Its
private test declarations now live in `src/model_archives/test_modules.rs`,
included only for test builds. The declarations retain their original names,
paths and order. They remain siblings of the production modules because the
private suites share parser helpers through that namespace.

The root now has 193 lines and the declaration registry has 259. No non-test
tokens changed. All 445 library tests passed, with the same three external
corpus gates ignored by default. A fresh native corpus run produced identical
outcomes and failure causes for all 906 configurations, retaining 516 accepted
cases. `test-registry-verification.json` records the comparisons and source/log
hashes. This structural change is committed separately from the new imported
and base-body runtime behavior.

The humanizer skill is unavailable; its prose pass remains with the maintainer.
