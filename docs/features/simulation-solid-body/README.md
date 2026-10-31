# SolidManager solid-body Property

Feature [#150](https://github.com/Oblikovati/Bessemer/issues/150) covers the direct
solid-body Property in specification 14.355/14.357, C-731/C-734. The expanded
reference reports 680 streams /855,786 bytes, with 745 original version
arguments: 691 accepted and 54 failed alternatives. Those alternatives must
remain explicit; a successful candidate does not establish runtime dispatch.

All 13 strict reference checks now pass. The initial source guard correctly
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

Production typed operations, complete field/edit/history oracles, adversarial
and resource tests, ownership/concurrency checks and full local/platform gates
remain pending. Sheet bodies, sibling ComponentPersistID streams, field business
meanings, native writer/application and geometric acceptance are separate.

The local read/construction prototype matches complete independent fields for
240 owned profiles and all 691 accepted native arguments, with all 54 failed
alternatives preserved. Four added identity suffixes retain the 4307 minimum
and match 24 independent version cases. Two fresh processes match the full
3,113,446-byte field export. The [basic read ledger](basic-read-verification.json)
records these checks and the remaining operation gates; this is not complete
production acceptance.
