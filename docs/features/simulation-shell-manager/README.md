# ShellManager header and group evidence

Feature [#156](https://github.com/Oblikovati/Bessemer/issues/156) tracks complete
typed interoperability for specification 14.363/C-741 and 14.368/C-747.
This checkpoint is a verified reference foundation. No production API is added yet.

Fresh recovery matches every original argument dictionary and payload hash.
C-741 contains 430 streams /3,164 bytes and 474 arguments: 444 accepted and
30 failed. These include 361 direct headers and 69 empty groups. C-747 separately
checks the same 69 groups /276 bytes through the recovered record grammar,
with all 79 arguments accepted. The groups occur in both reports; adding the
report counts would inflate the unique denominator.

All ten checks pass, including the six original tests, exact original source
hashes, complete retained clauses, installed DLL function/threshold/literal
bindings and fresh envelope/CFB recovery. No target code executes. Original
reports retain decoded values and complete spans as provenance; generated native
packets remain under `target/`.

Direct header reading includes +C8 and adds +10C only above version 800027.
The current writer always emits both. Group records use four MFC CStrings,
two signed words, a raw double, three signed words and +84 from reader version
890016 or under the explicit current writer profile. CwPersist text framing,
object tags and embedded archive-version words do not belong to this group format.
Thirty independent header profiles and 72 group profiles match known literal
values, including empty/multiple records, all three MFC length widths, embedded
NUL, Latin-1/UTF-16, signed extremes and raw floating-point patterns. The original
group tests also cover the profile boundary, narrow/wide text and raw NaN.
Native groups remain empty; populated-record coverage is static and synthetic.

Run `python -B -X utf8 docs/features/simulation-shell-manager/freeze_shell.py`
for strict replay. `--write` initializes evidence for review. Complete independent
primitive-field/edit/history oracles, typed APIs, resource/identity/ownership and
concurrency checks, full local and exact-head platform gates remain pending.
Shell/midsurface Property, inline Material, composites, runtime dispatch, field
meanings, geometry and native CAD writer/application acceptance are separate work.
Humanizer was unavailable; maintainer prose review remains pending.
