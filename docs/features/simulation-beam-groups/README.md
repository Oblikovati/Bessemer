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
