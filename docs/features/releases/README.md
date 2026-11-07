# Releases

Bessemer 0.1.0 is the first GitHub source release. It does not claim complete
SolidWorks schema parity or native application acceptance. Individual feature
documents record supported layouts, validation coverage and exclusions.

Run `cargo xtask release --version 0.1.0` to check that the manifest and dated
changelog agree. Run `cargo xtask pr` before pushing. Integrate through a green
pull request, then dispatch `bessemer.yml` on the exact main commit with
`mode=release` and `version=0.1.0`. All PR, sanitizer and platform checks must
succeed on that commit before creating `v0.1.0` and its GitHub release.

The GitHub release provides the tagged source archives. Crates.io publication
remains disabled. After publication, verify the tag and release, then run
`cargo clean` for each build target used and remove completed worktrees.

Release tracking: #254. The humanizer skill was unavailable; prose awaits the
maintainer review pass.

Known follow-up #253 tracks duplicate stream-name admission and per-occurrence
no-op preservation. Reference builder issue #252 affects empty-file construction;
the owned empty-reader fixtures remain independent of that builder.

## Working branches

The maintainer requested consolidating every existing branch into main. After
this initial release, main is the integration branch and `codex/work` is the
only working branch. Reuse it for successive scoped pull requests, merge each
green pull request before starting another, and fast-forward it from main after
integration. Remove the superseded candidate and completed task branches once
their commits are verified on main. This replaces the standing candidate branch
workflow for subsequent work.
