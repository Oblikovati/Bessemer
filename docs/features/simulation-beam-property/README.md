# Beam child Property

Feature [#142](https://github.com/Oblikovati/Bessemer/issues/142) covers the
complete direct beam child Property body in specification 14.349, C-725.
The retained reference reports 163 streams totaling 43,516 bytes. This initial
snapshot preserves the clause, full report and source hashes. Fresh replay now
matches every retained native outcome and all physical spans. Seven checks pass,
including original boundary tests, complete source hashes, installed function and
threshold bytes, and 104 independent owned profiles. This is a reference
foundation; no production beam Property API has been accepted yet.

The body starts with a direct persistent-reference container, followed by raw
words, two signed shorts, text, versioned section values, counted interleaved
triples, a linking word and historical helpers. Embedded references use explicit
archive dependencies. It has no outer collection count or common-load tail.
Signed and unsigned values must keep their original bit patterns.

The six beam-specific thresholds are 810029, 820007, 840014, 840024, 850035 and
850036. Shared reference thresholds also apply. Native range repairs and defaults
must not rewrite persisted values. Material streams are separate and covered by
[#140](https://github.com/Oblikovati/Bessemer/issues/140).

Complete construction and edit/history oracles, resource limits, broader
malformed-input cases, isolation,
concurrency and local/platform gates remain required. Units, link domains,
geometry generation, native writer and application acceptance remain unresolved.

Run `python -B -X utf8 docs/features/simulation-beam-property/freeze_beam.py`
for strict replay from an immutable source copy. `--write` initializes evidence
for review. The complete native input/span packet stays under `target/`; owned
literals and spans are committed under `tests/fixtures/simulation-beam-property/`.
No target code executes. The initial report and current source snapshot retain
the distinction between historical evidence and this fresh replay.

The complete local codec reads and reconstructs every owned/native case,
validates complete typed construction, and provides isolated edits with ordered
outer/nested history. Twenty Rust tests pass with four explicit native inputs;
nine strict copied-source checks pass. All fifteen published types/aliases have
Send/Sync assertions. Clippy, formatting and function/file audits pass.

Two fresh Rust processes match the complete independent field export (585,542
bytes), short-edit/outer-history export (872,118 bytes), and short-edit/nested-copy
history export (914,346 bytes). Every original accepted context is retained.
All 1,548 targeted malformed inputs match Source rejection. Every owned context
passes seven exact/one-below outer/embedded budgets and rejects every proper
prefix and trailing byte. Eight threads each run 32 mixed edits against seven
serial variants, matching bytes, fields, coverage and complete parent relations.

The [operation ledger](operation-checkpoint.json) records hashes, scope and
remaining gates. The full local gate passed; exact-head platform integration remains pending.
Native runtime/application, physical/link semantics and geometric acceptance
remain separate. No target code executes.

Set `BESSEMER_BEAM_NATIVE_INPUT`, `BESSEMER_BEAM_NATIVE_PROJECTION_INPUT`,
`BESSEMER_BEAM_NATIVE_HISTORY_INPUT` and
`BESSEMER_BEAM_NATIVE_NESTED_HISTORY_INPUT` to the corresponding freshly generated
`target/simulation-beam-property-native*.bin` packets. Optional complete exports
use `BESSEMER_BEAM_PROJECTION_OUTPUT`, `BESSEMER_BEAM_HISTORY_OUTPUT` and
`BESSEMER_BEAM_NESTED_HISTORY_OUTPUT`.

The [full local gate](full-local-verification.json) records the exact committed
implementation, platform, toolchain, test totals and complete log hash.
