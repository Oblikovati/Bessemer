# Public resolved-feature read/write checkpoint

`PartResolvedFeaturesArchive` now connects complete CMgr parsing, configuration
graph validation, inherited model maps and feature reconstruction to a public
library caller. It owns immutable state and retains explicit limits and kernel
context. `to_bytes` performs an exact typed reconstruction and reparse.
Coverage and diagnostics distinguish stored-schema checks from geometry and
external-document validation. The 20 previously reported dead-code warnings
are gone without suppressions.

Four public integration tests cover the owned complete stream, 17 roots,
97 inherited and 161 final slots, input-buffer isolation, concurrent writes,
Send/Sync, incomplete inputs, completed empty results and required helper tails.
The example test preserves the complete document byte for byte. Running the
actual example CLI also produced the owned fixture's original SHA-256,
`b52e080ba2cc64d1d07ed28306f27281b07521aef550bc0c7fea8cd54be2b276`.

The 906-case probe now requires both the private parser and public API to
reconstruct accepted streams exactly. Two fresh processes produced identical
outcomes: 388 accepted and 518 explicit failures, preserving the prior accepted
set. The independent reader completed all accepted cases against matching
four-stream hashes, covering 15,305,243 feature bytes. This does not establish
native application acceptance or complete schema parity.

`cargo xtask pr` passed Clippy with warnings denied, build, the test suites and
255 doctests; its library suite passed 413 tests with three external gates
ignored. A test-only primitive helper was updated to the current compiler's
array-chunk API without changing its decoding. Additional public empty-result
and corpus-call assertions were added after the gate started; the final four
integration tests, example test, corpus unit test and final all-target Clippy
run passed on those changes. The explicit sample gate was run separately twice
and still failed as reported above.

Issue #54 also requires validated editing and revision-scoped history before
integration. Those remain pending, along with the full CI matrix and wider
schema parity. The README, public docs, example and CHANGELOG describe the
implemented read/write scope. No PR, push, dependency, version or release
changed. The humanizer skill is unavailable in this session; its prose review
remains with the maintainer.
