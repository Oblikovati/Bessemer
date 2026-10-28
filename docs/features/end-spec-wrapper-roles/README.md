# End-specification wrapper roles

The current exact family also includes the body reference described in
[stock-feature evidence](../stock-features/README.md). Additional complete end
caller fixtures prove its membership in the 21-class inventory while retaining
this feature's original corpus.

Bug #46 includes native case 353, `learn/Mouse/On Off Switch.SLDPRT`, which reaches
an implemented end specification and rejects `moSolidRef_w` at `reference_78`.
The new regression reproduces that exact error before the production fix.

Clauses 11.90 / C-105 and 11.91 / C-106 declare wrapper-base references at both
conditional extrusion end fields, the starting-end field for stored types 1/2,
and both unconditional revolution-end fields. These sites now reuse the existing
reviewed nineteen-class wrapper family. Its complete body readers and ancestry
are unchanged. No global assignability rule or class-name prefix matching is
introduced. Nulls remain permitted, and complete unrelated node-name objects
remain invalid for both fresh and cached tags.

The production change affects only role membership. It retains stored type words,
conditional occurrence rules, physical fields, bytes and object identities.
Starting-end types are not normalized. Revolution wrapper occurrences remain
independent of its stored side-type words. Historical contexts and alternate
archive masks retain their previous explicit scope restrictions.

Independent fixtures cover 1,140 graphs across ten versions, 169,376 primitive
observations and 133 owned documents. All nineteen wrappers pass through all
three caller layouts using complete fresh/cached bodies, distinct parents and
actual repeated identities. Owned starting-end records are reached through
complete boss/end parents, with revolution records using complete revolution
parents. Nineteen independent edits change only a common signed wrapper word.
Three null and six complete unrelated witnesses separately pin nullable and
negative-role behavior. The source reader verifies EOF/bytes/identity but ignores
expected-class mismatches; native declarations and reviewed ancestry supply the
role oracle.

Seven private tests cover the reproduced solid-reference bug, every independent
field/byte/map slot, the exact finite family at each caller, seeded identities,
nulls and unrelated classes. Prefix regressions cover every latest-version
caller/class/style; existing wrapper/body corpora retain their exhaustive version
boundaries. Three public tests cover all owned documents, independent edits and
source isolation after successful and rejected edits. Four Python tests verify
regeneration and the 180 pre-change default builder baselines. The original
end-spec point corpus also regenerated unchanged.

Run `cargo test --lib model_archives::end_spec_wrapper_role_tests`,
`cargo test --test resolved_end_spec_wrapper_roles` and `cargo xtask pr`.
Independent regeneration uses
`python -B -X utf8 -m unittest discover -s docs/features/end-spec-wrapper-roles`.
The fixture-only builder change is a separate commit; its rationale is in
`builder-reuse.md`.

The final local PR gate passes with 795 library tests, all integrations and 274
doctests. Two fresh native processes agree on every outcome and cause across
906 inputs. Cases 353, 480 and 481 now reconstruct completely, raising the
accepted set from 803 to 806 without losing any earlier success. The aggregate
assertion still fails explicitly for 100 unsupported inputs.

A verified immutable 1,559-module source snapshot independently checks all 806
accepted EOFs and four original stream hashes, covering 52,637,722 feature bytes.
The newly completed inputs retain recorded end/from/revolution target classes and
stored type words; case 353 contains the original solid-reference target at
`reference_78`. These source observations remain separate from the native
ancestry and negative-role oracle. Reports are in `frontier-checkpoint.json`,
`accepted-reference.json`, `native-wrapper-reference.json` and
`local-verification.json`.

This change does not assign engineering meanings to wrapper targets or execute
feature regeneration. Geometry validity, application acceptance and full schema
parity remain open. The humanizer skill is unavailable; maintainer prose review
remains open.
