# Stored model node state (#178)

Feature [#178](https://github.com/Oblikovati/Bessemer/issues/178) covers the
stored node update-state bits and parameter-only flag roles in specification
11.435, 11.448, 11.449 and 11.572. The existing archive codecs preserve these
words. [Archive-integrated named views and isolated edits](api.md) are now
implemented locally; publication acceptance remains pending.

The foundation retains all four complete clauses, nine original native/replay
reports and nine current reader/test dependencies. Historical replay reports
keep their original source revisions and unknown-byte counters. They are not
adopted as current accounting baselines. Bug
[#177](https://github.com/Oblikovati/Bessemer/issues/177) tracks the separate
current-source accounting discrepancy; this feature assigns no new byte credit.

Initialization and strict replay each passed all nine checks, including all five
unchanged focused source tests from an immutable source copy.
[Foundation verification](foundation-verification.json) records the terminal
logs, hashes and scope.
Four additional checks bind current sources, complete clauses, original reports
and installed native evidence. The native check verifies the entire DLL hash,
23 recorded intervals totaling847 bytes, their exact exported entries and all
45,766 declared export names. It reuses the frozen source RTTI locator and
checks the complete recorded primary parameter table and zero-offset,
nonvirtual base list. These are ordinary PE checks, not fresh REA Evidence or
CAD execution. REA's objdump provider is unavailable on this host, as recorded
in the historical MaterialList investigation.

```powershell
python -B -X utf8 docs/features/model-node-state/freeze_node_state.py
```

`--write` initializes evidence for review. Original report bytes retain their
CRLF endings through a scoped Git attribute so their hashes remain exact.
The current source pins are separate from historical replay source pins;
there is no hash override or weakened native check.

The implementation must preserve signed update state, every flag bit and the
absence of the second word through version0x143A. Stored bits do not reproduce
effective native predicates: needsUpdate can consult virtual state, and
dimension sharing can consult a runtime reference count. Parameter meanings
must not appear on generic nodes. Native setter cascades, pointer allocation
and state normalization are outside the stored-word edit operation.

The implementation binds revision-scoped state fields and finite parameter
class scope, then reuses the existing validated archive edit/history paths.
Focused tests cover every owned context, literal byte edits, complete unselected
field preservation, exact no-ops, foreign/stale/type/scope failures, missing-word
guards, ownership, serial/parallel agreement and eight-worker shared-snapshot
stress. Existing906-case corpus retention, fresh-process checks, the immutable-head
local gate and all five platform gates remain separate acceptance requirements.
Whole-schema, native application and geometry acceptance remain unproven.

Humanizer is unavailable; maintainer prose review remains pending.

The complete owned-input stage passed all10 checks in final initialization and
strict replay. Its992 feature streams contain1,984 definitions across seven
parameter and two generic classes, each with a loaded-class definition and an
instance alias. They use the unchanged owned part's verified97-slot map and
complete required MBSM tail. All128 update-bit combinations, three unrelated
state patterns and all16 parameter-role combinations are represented.

[Owned verification](owned-verification.json) records the exact context and
fixture hashes. Independent literals retain91,776 packet bytes, including
87,808 wire bytes. The node oracle records raw words and offsets;23,872 original
reader primitives partition every wire byte and corroborate all7,936 node-word
spans. Generic classes retain the same upper bits without parameter roles.
Raw scalar NaNs, negative zero, alternate-presence bytes, text and all other
fields stay in the full inputs. The earlier nine-check snapshot remains at its
recorded Git revision. The [archive integration contract](design.md) describes
the implementation contract; fixture acceptance remains distinct from Rust and
platform acceptance.

The [local operation ledger](operation-verification.json) records seven passing
integration tests, five projection-guard tests, twelve documentation examples,
nine prior public archive regressions and clean all-target Clippy. All35,328
owned bit requests match closed-form full images, complete unselected fields
and ordered changed-field history; half are exact no-ops. The configuration
entry points also reopen an edited document, preserve unrelated streams and
revert the whole document byte for byte.

The new native gate inspects15,542 configuration nodes and72,113 feature nodes
across all906 retained contexts, including the original39,487 parameter
definitions and zero observed feature global-variable bits. Both complete
carriers reconstruct exactly. The original private/public906-case gate also
passes. Two fresh processes produce identical6,315,152-byte full view/field/edit
history exports, matching serial and eight-worker projections. Eight workers
also exercise shared immutable snapshots with23,552 accepted edits and4,096
expected scope rejections. The [repository PR gate](repository-pr-verification.json)
also passed: 3,839 tests passed, zero failed and 195 were ignored across
305 summaries. Exact-publication-head platform and ThreadSanitizer gates
remain pending in the serial queue.
