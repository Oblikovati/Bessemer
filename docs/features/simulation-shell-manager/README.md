# ShellManager header and group evidence

Feature [#156](https://github.com/Oblikovati/Bessemer/issues/156) tracks complete
typed interoperability for specification 14.363/C-741 and 14.368/C-747.
Direct header reading, complete construction and isolated edits are implemented.
The populated group API remains pending.

Fresh recovery matches every original argument dictionary and payload hash.
C-741 contains 430 streams /3,164 bytes and 474 arguments: 444 accepted and
30 failed. These include 361 direct headers and 69 empty groups. C-747 separately
checks the same 69 groups /276 bytes through the recovered record grammar,
with all 79 arguments accepted. The groups occur in both reports; adding the
report counts would inflate the unique denominator.

All fourteen reference checks pass, including the six original tests, exact original source
hashes, complete retained clauses, installed DLL function/threshold/literal
bindings and fresh envelope/CFB recovery. No target code executes. Original
reports retain decoded values and complete spans as provenance; generated native
packets remain under `target/`.

Direct header reading includes +C8 and adds +10C only above version 800027.
The current writer always emits both. Group records use four MFC CStrings,
two signed words, a raw double, three signed words and +84 from reader version
890016 or under the explicit current writer profile. CwPersist text framing,
object tags and embedded archive-version words do not belong to this group format.
Sixty independent reader/current-writer header profiles and 72 group profiles match known literal
values, including empty/multiple records, all three MFC length widths, embedded
NUL, Latin-1/UTF-16, signed extremes and raw floating-point patterns. The original
group tests also cover the profile boundary, narrow/wide text and raw NaN.
Native groups remain empty; populated-record coverage is static and synthetic.

`SimulationShellManagerProperty` requires a normalized version, explicit
`SimulationShellWireProfile::Reader` or `Writer`, and caller budgets. Reader
version 800027 has one word; Writer retains two even at that supplied version.
Coverage retains both the version and selected grammar. Construction revalidates
the complete shape; edits retain immutable source/replacement/result, exact
surrounding bytes and all ordered source/replacement field parents.

All 13 focused header tests pass, including explicit native inputs. Both fresh
processes match every owned/native field and history byte against independent
expectations. Fourteen public types have Send/Sync assertions; eight workers
perform 32 mixed edits each against three serial options. Exact/one-below byte
and field budgets, zero entry/text requirements, all proper prefixes/trailing
bytes, typed shape/kind errors, no-ops, revision ownership and input isolation
are checked. See [header verification](header-interop-verification.json).

Run `python -B -X utf8 docs/features/simulation-shell-manager/freeze_shell.py`
for strict replay; `--write` initializes evidence for review. Independent owned
group primitive boundaries and complete scalar-edit/history expectations match
the actual reader. Populated groups change one known word; empty groups retain
a no-op control. Group Rust parity, native group projections, group resource/
identity/ownership/concurrency checks and full local/exact-head platform gates
remain pending.
Shell/midsurface Property, inline Material, composites, runtime dispatch, field
meanings, geometry and native CAD writer/application acceptance are separate work.
Humanizer was unavailable; maintainer prose review remains pending.
