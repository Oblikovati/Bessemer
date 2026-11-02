# Stored model node state (#178)

Feature [#178](https://github.com/Oblikovati/Bessemer/issues/178) covers the
stored node update-state bits and parameter-only flag roles in specification
11.435, 11.448, 11.449 and 11.572. The existing archive codecs preserve these
words. Archive-integrated named views and isolated edits remain to be implemented.

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

Next, bind complete archive object identities and finite parameter class scope,
then add views and edits through the existing validated operation/history paths.
Independent literals must cover all128 update-bit combinations, parameter-bit
combinations, unrelated bits, exact version gates, generic-node exclusion,
no-ops, failures, isolation and ordered provenance. Existing906-case corpus
acceptance, deterministic process/concurrency checks, the immutable-head local
gate and all five platform gates remain required. No Rust API or stub was added
by this foundation. Whole-schema, native application and geometry acceptance
remain unproven.

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
the next implementation; these fixtures do not establish Rust acceptance yet.
