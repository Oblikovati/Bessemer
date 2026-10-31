# Separate shell composite streams

Feature [#157](https://github.com/Oblikovati/Bessemer/issues/157) tracks complete
CompositeFlipTurn and CompositeExtra interoperability for specification 14.364,
C-742. `SimulationShellComposite` reads, constructs and edits complete bodies
under an explicit `SimulationShellCompositeProfile` and normalized outer version.
Results retain physical fields, byte spans, revision-scoped IDs, diagnostics,
completion, exact validation coverage and ordered outer/nested history.

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

All 21 reference checks pass, including all ten original tests, the exact scoped
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
fields/edit/history oracles cover all 270 owned cases and all 30 accepted original
native arguments. Rust checks complete read and construction bytes, every field,
all spans and all ordered outer/nested parents against those independent packets.

Behavior profile 1 follows the documented framing without inferring native
pointer values. The kernel decision is exact EOF for the selected grammar, fixed
count caps, independent caller budgets and the scoped constant-surface/ID archive
callback. Reader mode 4 requires its ID. Writer mode 4 permits exactly the 64-byte
tailless body or a complete ID tail. Two embedded roots are required at outer
version 840002 and above. The general modern archive version floor remains 4307;
the separately documented 4281 context remains explicit.

The extent gate checks all 80,300 shorter owned extents. Exactly 80 are complete
tailless writer bodies; all others fail. All 270 appended-byte cases fail.
Targeted count, selector, mode and archive mutations retain 2,400 accepted controls
and 2,070 failures. Seven independently measured resource axes pass at their exact
limits and fail one below each positive requirement, for both reads and complete
construction. The record and blob caps use sufficient complete payloads on each
side of the boundary. Composite bodies have no outer text payload, so a zero
outer text budget remains valid; nested archives retain their independent limits.

Tests also cover every field's exact no-op, raw double/blob changes, complete
embedded replacement with supplied-parent provenance, wrong value kinds,
malformed construction, foreign/stale IDs and caller-input isolation. All 16
public types satisfy `Send + Sync`. Eight workers cover every owned context with
three scalar edit options, comparing complete bytes/fields/spans and every ordered
outer/nested parent against serial results. ThreadSanitizer and application/native
writer acceptance have not been executed. The full local `cargo xtask pr` gate
passed on `6839d21985943878dd3703637ccfb8208ba182cd`: 3,426 tests, zero failures,
87 explicit ignores and 259 summaries. The process exited successfully and was
reaped; the log SHA-256 is
`52fa0c2f06fa6d6e009304ac01570caf19f7acc180339a0e6e14b5746eb1e817`.
Exact-head platform gates remain pending for this feature.

Field meanings, native runtime/pointer selection, the historical short layout,
native writer/application and geometry acceptance are unproven. Shell Property
and inline Material remain separate work. Humanizer was unavailable; maintainer
prose review remains pending.
