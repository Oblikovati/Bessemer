# Stored ellipse-backup rotation snapshots

Feature [#192](https://github.com/Oblikovati/Bessemer/issues/192) follows clauses
11.1686/C-3370 and 11.1688/C-3372. The named producer stores the complete
getRotationDir result in ellipse backup member20. The existing wire field is
signed `word_20`; all 32 bits must survive reading and isolated edits.

The native compare/copy phase does not establish a Boolean, closed enum,
clockwise convention, coordinate frame or successful geometric restoration.
The planned API names the stored snapshot and reuses complete archive edits,
revision guards, coverage and history. It must admit the actual physical
ellipse class, preserve aliases and reject the same field name in arc backups.
The local implementation now provides `ellipse_backup_rotation_states()`,
`ellipse_backup_rotation_state()` and `edit_ellipse_backup_rotation()`.
The state exposes the raw signed snapshot and separate physical-owner/word
revision handles. The existing complete edit result supplies completion,
coverage, fidelity, diagnostics and field history. Native and publication
acceptance remain open.

All 20 direct publication pins matched across 16 distinct files. Both raw
reports are retained byte for byte. Historical charges and global accounting
remain unchanged. The duplicate native capture is not treated as new evidence.

Independent literals cover 108 complete contexts across three supported
versions, nine signed words and four null/shared/distinct point profiles.
Each contains two physical ellipses, a first-owner alias and an unrelated arc
with the same scalar field name. One profile uses a cached ellipse class.
The 41,030-byte packet and all scalar offsets regenerate exactly.

The first fixture attempt imported live reference modules indirectly through
the old byte-builder imports. Its import guard rejected the attempt. The
literal object-map helpers are being extracted into a reader-free module;
existing fixture defaults remain schema7 and the new canonical fixtures select
schema1 explicitly. Three closed-form helper tests pass. Both existing Source
fixture tests passed, preserving all 174 graphs, 3,653 primitive observations
and 12 owned streams byte for byte. The strict copy/import guards passed.
A first copy attempt rejected a concurrently changed research file before any
test ran; a fresh complete snapshot passed without a hash exception.
The [extraction proof](byte-helper-refactor-verification.json) retains both
attempts. New ellipse Source validation also passed all four tests: 108 complete
contexts, 216 physical ellipses, 108 aliases, 108 unrelated arcs, 324 scalar
checks and 7,569 primitive reads. The frozen file/import guards passed and
the process exited successfully.
No assertion, original fixture or reader hash has been relaxed.

Native cohort binding, complete typed edits/document reversal, provenance,
concurrency, fresh-process identity and full local/remote gates remain required.
Publication stays serial behind PR #191. No dependency or version changed.
The humanizer skill is unavailable; its prose pass remains for review.

The first six Rust tests passed all 36 current-version contexts and 360
independent literal edit/reverse requests. They check full fields and history,
changed/unchanged completion, coverage/exclusions, foreign/stale revisions,
arc/alias/nonowner rejection, empty role lists, generic-valid noncanonical
schema rejection and compile-time Send/Sync. An initial schema-control
attempt changed only the class definition metadata and was correctly rejected
by the existing alias-consistency check. The corrected control changes the
literal class-header bytes and reparses the complete archive, preserving that
check and the role-rejection assertions. Historical Rust contexts, native
cohort operations and wider acceptance remain pending.

Regenerate the independent literals with
`python -B -X utf8 docs/features/ellipse-backup-rotation/freeze_owned_ellipse.py`.
Run the frozen original readers with
`python -B -X utf8 docs/features/ellipse-backup-rotation/verify_owned_source.py`.
