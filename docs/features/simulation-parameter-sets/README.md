# Complete Simulation ParameterSets framing

Feature issue: [#133](https://github.com/Oblikovati/Bessemer/issues/133).
Behavior profile: `simulation-parameter-sets-v1`. The implementation, complete
owned/native corpus, fresh-process checks and final seven-check reference
replay and full local gate passed. Platform integration gates remain pending.

## Declared complete grammar

Use the actual `cosworkspersist.read_parameter_sets` reader and complete
clauses 14.278, 14.279, 14.280 and 14.283 from the approved reference.
Require explicit normalized version context and record/table/text budgets.
Preserve five ordered signed header words, optional four-byte words above
versions 700013 and 700018, the signed record count and the entire integer
table. Before version 810026 the table has exactly one hundred entries,
independent of header word zero. Later versions use that header word as their
nonnegative table count. Preserve the ignored earlier header value exactly.

Each complete record contains one persisted raw text frame, three four-byte
words, another four-byte word above version 700012, a raw eight-byte value
above version 700013 and one raw eight-byte value per table entry. Preserve
the documented signed/unsigned views without interpreting double patterns.
Retain the trailing word and additional words above versions 820015 and
830009. Require exact EOF, including when both counts are zero.

Native retention and wire consumption differ. The installed runtime retains
at most ten thousand table values while consuming the entire declared table.
The library must preserve every consumed byte when explicit budgets allow
larger tables. Test both ten thousand and ten thousand and one entries, with
complete record boundaries and values following them. Reject negative,
over-limit and impossible extents before allocation or field iteration.

## Construction, edits and ownership

Reuse the existing counted-text codec and normalized version type.
Construction must supply complete matching tables and records and derive the
record count. Preserve the independent legacy header word zero; modern table
counts must match the complete supplied table. Count/structure changes require
complete reconstruction. Allow isolated complete text replacement and
same-width scalar/raw-pattern edits, with authentic no-op identity retention.

Publish immutable owned archives and normalized revision-bound field IDs.
Return full source/replacement parents, every ordered copy/derivation relation,
completion, diagnostics, explicit exact check scope and achieved pattern
error. Preserve source isolation after success, failure, caller/drop and
returned-value mutation. Assert Send/Sync and compare complete threaded edits
and history against serial results.

## Native evidence and limits

C-656 retains all five complete payloads and their candidate contexts.
Four streams reconstruct under retained matching versions, totaling 5,468
bytes. The separate 2,944-byte BasicStudy stream has no matching stored
version; C-659 rejects its retained versions 700021 and 890116. Replay every
candidate and compare all accepted spans, text frames, raw patterns and full
byte images. Keep every mismatch explicit. Candidate success does not prove
runtime dispatch, storage-name/index identity or a general legacy fallback.

Pin the full clauses, complete actual reader modules, C-656 native inventory
and C-659 decoder-validation identities. Use independent literal grammar and
splice goldens at every threshold and neighbor, including null/empty/odd/raw
UTF-16 text, signed extrema and raw NaN/infinity/zero patterns. Check every
representative truncated prefix, trailing bytes, reduced budgets, checked
extent overflow and complete constructions/edits. Compare full outputs from
two fresh processes and record revisions, options, hashes and exclusions.

ResultLocations, nested persistent-ID archives, native runtime retention,
domain meanings, native writer equivalence and geometry/application
acceptance remain separate work. Run `cargo xtask pr` and require all five
platform checks before integration. Keep one PR open and merge when green.
No dependency, package version or release is planned. The maintainer humanizer
skill is unavailable; its prose pass remains for review.

## Reference preflight checkpoint

Six checks passed during initial evidence creation and an unchanged-baseline
replay. All forty-three retained candidate outcomes matched the actual frozen
reader: eight accepted contexts and thirty-five rejected contexts. Accepted
results matched every retained span label, offset, endpoint and complete byte
payload. Four stored-version matches reconstructed all 5,468 native bytes.
The C-659 report's source-inventory identity, four rows and span counts matched;
BasicStudy remained rejected at both retained versions.

Independent literal sections covered every threshold and neighbor, legacy
ignored header word zero, zero table/record cases, null/empty/odd/raw UTF-16
text and complete signed-word/double-bit patterns. Both ten-thousand and
ten-thousand-and-one-entry table/record arrays were consumed in full when
allowed; the latter was rejected under default limits. Every representative
prefix, trailing bytes, negative/huge counts and reduced record/table/text
budgets failed explicitly. The complete relevant clauses, source modules and
native-report hashes are retained here. The function/file-size audit passed.
No installed target code executed.

## Implementation and complete corpus

The library retains every signed/unsigned primitive and raw double pattern as
a typed field with its physical offset, documented role and revision-bound ID.
All record texts share one owned complete archive parent. Complete construction
checks counts, conditional presence, text budgets and matching value arrays
before allocating the encoded image. The legacy first header word remains
independent of its hundred-entry table. Changed structural counts require full
reconstruction; authentic no-ops retain identities.

Isolated text/primitive edits retain complete source/replacement parents and
one ordered copy/derivation relation per field. Full reparsing and literal
prefix/suffix checks establish exact framing and preservation. Returned scope
distinguishes complete empty-record archives from populated records and records
all budgets, counts, native/semantic exclusions and zero achieved pattern error.

All fifteen focused tests passed with the injected native export, as did both
private checked-extent/count tests. All eighteen public types have Send/Sync
assertions; eight threads repeated complete edits and checked every relation.
Tests covered malformed input, typed/count rejection, caller/returned-data
isolation, independent text parents, raw NaN/signed-zero bits and every version
threshold. Clippy and the function/file-size audit passed.

The complete corpus contains 14,531 owned and 3,255 native read/edit contexts.
Each accepted input was reconstructed through the complete constructor, and
every emitted field role, type, offset, pattern, history parent/result and
coverage value matched independent reference packets. All retained candidate
mismatches remained errors. Large arrays retain every field and history entry;
edits probe indices 0, 1, 9,998, 9,999, 10,000 and last where present, plus all
non-array fields. Smaller fixtures exercise every field edit.

The full export is 742,113,027 bytes. Two fresh processes ran all fifteen tests
and produced byte-identical outputs. The owned fixture losslessly stores all
600,410,109 golden bytes in 8,298,097 compressed bytes. Complete baselines and
fixed-length XOR patterns reduce fixture storage; every original expected byte
is reconstructed and checked. No case, field or history relation was dropped.

## Test-harness memory measurement

The initial harness retained whole expanded packets and measured roughly
3.2 GB working set in each fresh run. Streaming one complete case at a time
reduced observed peaks to about 302 MB. All 742,113,027 export bytes remained
identical before and after that test-only change. Production library code and
the accepted workload were unchanged. `harness-observations.json` records both
runs before and after, executable/output hashes and elapsed observations.
Working sets were sampled at roughly one millisecond and are lower bounds;
the host was not isolated. These observations are not statistical speed claims
or a comparison with a previous published release.

## Full local verification

`cargo xtask pr` passed 3,130 tests with zero failures and thirty opt-in tests
ignored by the standard gate. The ParameterSets native export ran explicitly
in the focused and fresh-process checks. All ten preceding manager-header
tests also passed with their injected native export; its complete
5,257,724-byte packet remained unchanged. `complete-verification.json` records
the full gate, exact indexed sources/fixtures, actual reader identity, every
corpus category, process observations and acceptance exclusions. All five
platform checks remain required before integration.
