# SolidManager solid-body Property

Feature [#150](https://github.com/Oblikovati/Bessemer/issues/150) covers the direct
solid-body Property in specification 14.355/14.357, C-731/C-734. The expanded
reference reports 680 streams /855,786 bytes, with 745 original version
arguments: 691 accepted and 54 failed alternatives. Those alternatives must
remain explicit; a successful candidate does not establish runtime dispatch.

All 15 strict reference checks now pass. The initial source guard correctly
stopped on a changed `solidworks/modelboundary.py`. A full replay trace recorded
zero calls to that file, and frozen import origins confirm it is not imported.
The scoped embedded readers install their own instance dispatch tables. The
[reconciliation](source-reconciliation.json) pins exactly this unused file's
new hash, preserves the original C-734 hash and failed check, and rejects other
files/hashes or future imports of that module. Every other original source hash
and all 745 original outcomes remain unchanged. This does not authorize using
the newer general boundary readers in this callback. See the
[foundation ledger](foundation-verification.json).

The foundation replays unchanged actual body and embedded-archive readers from
an immutable source copy. It retains the complete earlier and expanded reports,
all original outcomes, earlier successful reconstructions, installed image
hashes, function spans, threshold bytes and four embedded-ID primary/suffix
bindings. Independent owned literals cover every historical gate and neighbor,
raw text variants, signed extremes, floating patterns, duplicate list entries,
and null/populated archives under their distinct inner versions.

Run `python -B -X utf8 docs/features/simulation-solid-body/freeze_body.py` for
strict replay. `--write` initializes evidence for review. Generated native
binary packets stay under `target/`; original research reports retain decoded
values and raw spans as provenance. No target code executes.

`SimulationSolidBodyProperty` reads and constructs one complete direct body
under an explicit outer version and caller limits. `edit_field` returns the
immutable source, complete replacement, validated result, completion/coverage
and every ordered outer/nested history relation. Existing embedded layouts
serve the four exact C-734 identities at inner versions 4307 and above; the
separate 4281 profile remains explicit and rejects those four classes.

All 21 focused tests, including three explicit native inputs, pass. Complete
fields/construction and scalar-edit fields/history match independent actual
source expectations for 240 owned profiles and all 691 accepted native
arguments, with all 54 failed alternatives preserved. Two fresh processes match
complete field and edit/history exports. The five stored value kinds, exact
no-ops, foreign/stale IDs, fabricated shapes, raw text parent ownership,
CString framing, source isolation, all 15 public types' Send/Sync bounds and
eight workers x 32 mixed edits are checked. Native writer/application,
field meanings and geometric acceptance remain separate.

All 5,454 targeted framing outcomes match: 240 unchanged accepted controls and
5,214 rejections. Every owned context passes seven independent exact/one-below
outer/embedded budgets and rejects every proper prefix/trailing byte. Fixed
blob/list caps have sufficient payload controls at and above each boundary.
The [operation ledger](interop-verification.json) records full exports and scope.
The [full local gate](full-gate-verification.json) passed 3,326 tests
with zero failures and 66 ignored. Exact-head platform integration remains
pending. Sheet bodies and
sibling ComponentPersistID streams are separate operations. The earlier
[basic read ledger](basic-read-verification.json) remains retained as an initial
checkpoint rather than complete operation acceptance.
