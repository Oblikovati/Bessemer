# Reference-axis atom owners

Bug #46 covers model pointer roles that reject verified derived classes. A
populated reference-axis composition exposed another case: the inherited atom
chooser's owner pointer rejected `moRefAxis_c`, although the documented axis
constructor inherits `moRefGeom_c`. The existing owner family already admitted
the reference-geometry base and reference planes. Empty chooser fixtures had
not exercised the missing axis entry.

The fix admits exactly `moRefAxis_c` to the model atom-owner family. It retains
the same shared role validator, exact class-name checks and display-role policy.
Two focused regressions failed before the change: a new axis used as an owner
and a populated axis containing cached self owners. Both now pass. Unrelated
new objects, loaded aliases and a class-name suffix still fail. Every truncated
prefix and trailing byte remains rejected.

Four complete independently decoded axes cover the minimum modern and current
versions, each with empty and populated chooser bodies. Their 678 primitive
observations partition every byte. The source reader verifies that all four
populated chooser owners refer to the enclosing axis object, preserving the
full `65535` presence words. Three owned streams import the actual blank-part
model and configuration-manager state with 97 slots. Whole-document tests
reconstruct both owner variants and match a raw-flag edit against separately
constructed expected bytes, preserving owners, history and input documents.
Three Python tests verify regeneration, the matrix and invalid initial slots.

The staged `cargo xtask pr` passed on Windows x64 with Rust 1.99.0: 474 library
tests, four xtask tests, all integration suites and 265 doctests. Three external
corpus gates remain ignored by default. Two fresh native replays preserve all
906 outcomes and failure causes, including all 588 complete reconstructions.
`frontier-checkpoint.json` binds the earlier independent accepted-stream audit;
`local-verification.json` records the before/after regression and executed scope.

This bug exists in the unreleased interop runtime. `main` contains only
`.gitignore`, `LICENSE` and `README.md`, so there is no corresponding main-line
runtime to patch. The fix targets the existing interop candidate and remains a
separate commit from the unfinished component-axis/point readers that exposed
it. Bug #46 stays open for its broader remaining role audit.

The reference-axis constructor and common geometry layout are retained in
`schema-snapshot.md` with hashes in `schema-reference.json`. `reference.json`
binds the fixtures to their generators, builders and independent readers.
This change validates archive storage; native geometry, topology and application
acceptance remain outside its scope. No dependency or package version changed.
The humanizer skill is unavailable; its prose pass remains with the maintainer.
