# Public resolved-feature edit checkpoint

`edit_fields` now returns `PartResolvedFeaturesEdit` with the validated immutable
archive, changed/unchanged completion, exact coverage, stored-field fidelity,
diagnostics and revision-bound structural history. Changed results get distinct
revisions. Clones and unchanged edits retain handle ownership; foreign and stale
handles fail explicitly. The existing configuration history helpers were shared
in behavior-preserving commit `d9d4147`, verified by all ten configuration API
tests before the resolved-feature caller was added.

Nine public integration tests cover the independent expected plane-token edit,
derived/deleted/created field relations and their parents, empty completed
results, failure isolation, input ownership, concurrent edits, Send/Sync,
whole-document reopening, unrelated-stream preservation and exact document
revert using the retained builder. Both examples passed their tests. Two fresh
CLI processes produced identical 31,691-byte edited documents. The independent
reader verified the 5,568-byte feature stream against the existing expected
fixture through EOF and confirmed the three selected context streams stayed
unchanged. This raw token edit does not apply the application's header naming
policy; the separate native naming matrix remains the evidence for that policy.

`cargo xtask pr` passed without warnings: Clippy, build, 413 library tests,
integration suites and 265 doctests. Three external gates remained ignored by
default. The explicit 906-case gate ran twice and still failed: 388 accepted
and 518 rejected, with identical fresh-process outcomes and no lost accepted
cases. Independent reference reads completed all accepted cases against matching
four-stream hashes, covering 15,305,243 feature bytes. These checks do not prove
full schema parity, native application or geometric acceptance.

The actual CLI outputs and corpus evidence are bound in `public-edit-reference`
and `public-edit-*` reports. No dependency, version or release changed. The full
CI matrix remains required before integration. The humanizer skill is unavailable
in this session; its prose review remains with the maintainer.
