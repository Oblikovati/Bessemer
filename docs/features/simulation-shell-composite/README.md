# Separate shell composite streams

Feature [#157](https://github.com/Oblikovati/Bessemer/issues/157) tracks complete
CompositeFlipTurn and CompositeExtra interoperability for specification 14.364,
C-742. This checkpoint is a verified reference foundation; no production API is
added yet.

Fresh recovery matches all 20 identities /736 bytes and every original named
profile argument: 40 attempts, 30 accepted and 10 failed. Eleven FlipTurn streams
/220 bytes contain a signed count and four signed words per record. Eight Extra
streams /512 bytes complete under the writer profile. Their selector-zero reader
attempts retain the expected 52-byte remainder. The remaining four-byte Extra
at candidate 840030 fails both profiles. It remains in the denominator without
padding, a guessed short form or successful prefix classification.

Extra reading starts with three signed words. Selector exactly 1 enables mode
and six raw doubles; mode exactly 4 requires a delegated persist ID. The current
writer always writes all four words and six doubles, making a 64-byte prefix.
For mode 4 it can omit the ID under its native null-pointer condition. The bounded
writer framing profile parses a tail when bytes follow; this does not establish
the original runtime pointer. Reader and writer are explicit choices.

All 16 checks pass, including all ten original tests, the exact scoped
constant-surface/ID callback, installed DLL/model slots and function hashes,
stream literals, every original source pin and the narrowly guarded unused
model-boundary reconciliation. No target code executes. The native input packet
contains all 40 arguments; original reports retain full decoded fields and errors.
Generated native packets stay under `target/`.

All 270 independently constructed owned profiles match complete actual-reader
values, raw spans and reconstruction. They cover signed four-word records, exact
non-1/1 selectors, negative/non-4/4 modes, unconditional writer prefixes,
mode-4 absent/present writer tails and the 840001/840002/840003 archive boundary.
Delegated IDs include empty/populated raw blobs and distinct inner/null/shared
archive profiles. Raw double patterns include NaN, signed zero, infinity and a
subnormal. These cases establish stored framing, without native pointer claims.

Run `python -B -X utf8 docs/features/simulation-shell-composite/freeze_composite.py`
for strict replay. `--write` initializes evidence for review. Independent owned
fields/edit/history oracles, typed APIs, malformed/budget/ownership/concurrency
checks and full local/exact-head platform gates remain pending.

Field meanings, native runtime/pointer selection, the historical short layout,
native writer/application and geometry acceptance are unproven. Shell Property
and inline Material remain separate work. Humanizer was unavailable; maintainer
prose review remains pending.
