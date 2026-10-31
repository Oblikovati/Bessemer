# Final LBC name-order and legacy map streams

Feature issue: [#130](https://github.com/Oblikovati/Bessemer/issues/130).
Behavior profile: `simulation-lbc-final-streams-v1`. Both complete profiles,
construction and isolated edits passed focused tests and the owned/native
byte corpus and full local gate. Platform integration gates remain pending.

The approved reference is the complete `cosworkspersist.py` module and clause
14.336 in `../inventor-interface`. C-712 retains the scoped native stream
payloads in `cosworks-lbc-final-native-evidence.json`.

## Separate complete profiles

LbcListOrder starts with one signed i32 item count, followed by exactly that
many persisted text frames and EOF. It has no leading selection string.
Preserve wire order, duplicate names, null versus allocated empty text, odd
payload bytes, embedded NULs and isolated UTF-16 units. Reject negative counts,
resource-limit violations, missing frames and surplus bytes. Do not sort,
deduplicate, normalize names or infer the native reordering helper's behavior.

Reuse the existing complete text codec and compatible checked sequence/edit
helpers. Any extraction from StudyListOrder must preserve its unsigned-count
profile, actual leading text, public identities and complete byte behavior.
Keep that structural change separate from new LBC behavior and prove existing
owned/native study fixtures unchanged. No synthetic leading string or invented
caller context may be supplied to adapt the formats.

Legacy MidsurfMap contains exactly three ordered groups. Each begins with a
signed i32 record count and then that many records of four signed i32 words.
Bound each group against its explicit limit and remaining bytes, retain every
word/count, and require exact EOF. Construction derives each group count from
its complete supplied records. Group and component ordinals are physical
positions, without assigned map-insertion or geometric meaning.

This legacy profile does not decode PersistMidsurfMap. Native selector values,
storage-open availability/fallback and repeated-key insertion are separate
runtime concerns and are not inferred from a successfully framed map.

## Publication and evidence

Publish immutable owned archives with normalized revision-bound IDs. Exact
construction and isolated text/cell edits must return full source/replacement
parents, complete ordered copy/derivation history, completion, diagnostics,
explicit check scope and achieved byte-pattern error. Authentic no-ops retain
identity; successful and failed edits leave inputs isolated.

Replay all 336 native lists, 1,137 strings and 32,396 bytes against the actual
frozen reader and retained text payloads. Native maps cover only five empty
three-group streams totaling 60 bytes. Preserve that limitation: populated
groups require independent synthetic signed-word goldens and must not be
reported as observed native records. The scoped inventory has no persistent-ID
map fixture; that does not complete its schema.

Include malformed prefixes, count/size/limit boundaries, duplicate/empty/odd
text, signed record extrema, caller/drop/returned-value isolation, compile-time
Send/Sync, concurrent serial equivalence and two fresh-process full-byte
comparisons. Record revisions, hashes, options, executed checks and exclusions.
Run `cargo xtask pr` and require all five platform gates before integration.
Keep one integration PR open and merge when green. No dependency, package
version or release change is planned. The maintainer humanizer skill is
unavailable; the prose pass remains for review.

## Reference preflight checkpoint

Seven checks passed against the complete actual frozen reader. All 336 native
name lists reconstructed exactly, including all 1,137 text payloads and 32,396
bytes. All five native legacy maps matched the exact twelve-byte empty layout;
no populated native map record is claimed. Independent synthetic cases covered
null/empty/odd/surrogate names, duplicates, populated signed four-word records,
empty groups, every representative truncated prefix, trailing bytes, negative
counts and reduced limits. Complete clause and native-inventory hashes are
retained alongside the actual reader snapshot. Bessemer implementation and
corpus/integration checks remain pending.

## Shared text extraction and name-list checkpoint

The private text sequence, replacement and preservation helpers were extracted
in commit `532204f91e1d10af80972b07da77042c786ad3d4`, separately from new LBC
behavior. All 31 existing persistence tests passed with the native export, and
the complete 4,486,980-byte owned/native projection remained byte-identical to
the pre-refactor reference. Public APIs, unsigned study counts, actual leading
study text and existing error entities were preserved. Clippy passed with
warnings denied. `text-extraction-verification.json` records the input revision,
code identities and exact comparison.

The new name list requires its own signed count at byte zero and reuses those
helpers without fabricated text or runtime context. Eight focused tests passed
for raw/null/empty/odd/surrogate frames, duplicates, complete empty lists,
negative/truncated/trailing/over-limit input, literal full-image edits, no-op
identity, foreign IDs, source/drop/returned-value isolation and concurrent reuse.
All eleven current public name/coverage types have Send/Sync assertions. The
function/file size audit and Clippy passed.

## Complete map and corpus checkpoint

All seventeen focused tests passed, including the injected native export.
The legacy map tests covered every physical word in populated synthetic
groups, signed extrema, exact three-count empty framing, per-group limits,
malformed prefixes, foreign identities and complete source isolation.
Each edit retained all three count headers and one ordered relation per word.
All nineteen public types have Send/Sync assertions; eight threads repeated
complete edits and checked every history relation against serial results.

The frozen reader passed eight checks. The corpus contains 1,931 owned
contexts and 6,026 native read/edit contexts, preserving every accepted byte,
physical field offset and pattern. Accepted archives were reconstructed
through the complete constructors. Edits were compared with independent
literal splices, including changed text widths and authentic no-ops.
The complete 6,599,636-byte Rust export matched the reference packet exactly.
Native maps still contain zero populated records. Clippy passed with warnings
denied. Two fresh processes each passed all seventeen tests with the native
export; their complete outputs matched the reference and each other byte for
byte. `complete-verification.json` records the full local gate, exact source
and fixture identities, process observations and acceptance exclusions.
The final workspace Clippy and formatting checks also passed after the test
helpers were split to meet the contributor function and nesting limits.
