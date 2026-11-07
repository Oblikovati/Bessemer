# Beam joint, group, list and folder headers

Feature [#144](https://github.com/Oblikovati/Bessemer/issues/144) implements the
four separate bodies documented in specification 14.350, C-726. The retained
native denominator is 414 payloads /50,872 bytes: 36 JointGroups, 11 JointLists
and 367 Joints. BeamFolder has zero retained native payloads; its header has
static and synthetic evidence only.

Joint references include both 810030 endpoints and disappear at 810031. Child
joint storages remain separate from the group stream. JointList preserves signed
neighbor pairs and their order, including duplicates. Optional expected-count
validation checks agreement with an independently enumerated child count; it
does not establish enumeration order or reconstruct runtime adjacency.

The initial clause and full report are retained. Fresh immutable-source replay
and strict comparison pass eight checks, covering all 414 native attempts and
560 independent owned bodies. The [reference ledger](reference-foundation-verification.json)
records exact packet hashes and exclusions. Complete read/construction/edit
operations, independent field/history oracles, malformed/resource budgets,
isolation/concurrency and full local/platform gates remain required. Runtime lookup, repair, physical
units, geometry and native application/writer acceptance remain unresolved.

Run `python -B -X utf8 docs/features/simulation-beam-groups/freeze_groups.py`
for strict frozen-source replay. `--write` initializes evidence for review. The
complete native input/span packet stays under `target/`; independent literals
and their complete spans are retained in `tests/fixtures/simulation-beam-groups/`.
No target code executes.

The complete local body API passes nineteen focused Rust tests with three
explicit native inputs; ten strict copied-source checks pass. All 560 owned and
414 native contexts match complete independent typed fields, forced construction,
header/pair edits and every ordered outer/nested-copy parent. Two fresh processes
match 570,022 bytes of fields and 1,002,418 bytes of full edited fields/history.
Explicit JointList count scope, completed empty lists and all sixteen published
types/aliases have direct checks and Send/Sync assertions.

All five physical value kinds have isolated edit checks. Every exact no-op keeps
source identities; wire-distinct embedded CString forms cause real edits. All
3,104 targeted mutations match Source rejection. Every owned context passes
seven exact/one-below outer/embedded budgets and rejects every proper prefix and
trailing byte. Eight threads each run 32 mixed edits across four body kinds and
eight serial requests, matching bytes, fields, coverage and all parent relations.
Clippy, formatting and size/function audits pass. See the
[operation ledger](operation-checkpoint.json). The full local gate passed; exact-head platform
integration remains pending; native runtime/application and geometric acceptance remain
separate.

Bug [#145](https://github.com/Oblikovati/Bessemer/issues/145) corrected the draft
span verifier: zero-byte empty-array spans may end at EOF without an actual
field starting there. Positive spans still require physical start/end alignment,
and full independent raw/typed field comparisons are unchanged. The
[correction record](empty-span-correction.json) retains the failed assertion and
focused regression. No production wire format changed.

Set `BESSEMER_BEAM_GROUPS_NATIVE_INPUT`,
`BESSEMER_BEAM_GROUPS_NATIVE_PROJECTION_INPUT` and
`BESSEMER_BEAM_GROUPS_NATIVE_HISTORY_INPUT` to the corresponding complete
`target/simulation-beam-groups-native*.bin` packets. Optional complete exports
use `BESSEMER_BEAM_GROUPS_PROJECTION_OUTPUT` and
`BESSEMER_BEAM_GROUPS_HISTORY_OUTPUT`.

The [full local gate](full-local-verification.json) records the exact committed
implementation, platform, toolchain, test totals and complete log hash.
