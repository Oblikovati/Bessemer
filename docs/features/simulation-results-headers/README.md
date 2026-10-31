# Results Property hierarchy interoperability (#164)

This feature covers the three distinct Property levels under a study's Results
storage: root, category and plot. Specification 14.382–14.384 and the complete
C-762/C-763/C-764 reports are retained here. The reference foundation precedes
the Rust codec; no Results header API is published at this stage.

The original inventory contains 450 occurrences and 10,956 logical bytes:

| Level | Streams | Bytes | Arguments | Accepted | Rejected |
|---|---:|---:|---:|---:|---:|
| Root | 53 | 2,192 | 58 | 53 | 5 |
| Category | 185 | 1,652 | 196 | 185 | 11 |
| Plot | 212 | 7,112 | 227 | 212 | 15 |
| Total | 450 | 10,956 | 481 | 450 | 31 |

Fresh replay recovers each source envelope and nested payload before requiring
every complete original result, including values, spans and failed traces.
Parsers and shared adapters are copied before imports; the final snapshot checks
all imported origins and hashes. Each stream is assigned by its complete study
storage path. Named studies such as `SimStudy-7` are supported without deriving a
study-array index, and category/plot names equal to `Results` remain ordinary
names. Unrelated, empty-segment and deeper Property shapes reject before parser
selection.

Installed evidence is read without loading target code. Checks bind the complete
image hash, recorded function spans, version constants, storage literals, the
root primary +48 serializer slot, exact category/plot constructor call targets,
and the plot comparison's inclusive 830035 immediate. The complete source and
decompilation pins have no hash exceptions.

Root and category have documented current-writer branches that differ from
historical reader layouts. Plot has a recovered reader without equivalent writer
evidence. Future profiles must keep these distinctions explicit. The category
parent's opening gate is also separate from the standalone record layout; a
valid record does not prove that its parent opened Property at that version.

Run the foundation with:

```powershell
python -B -X utf8 docs/features/simulation-results-headers/freeze_results_headers.py
```

Initialization and strict replay each passed all 17 checks: nine original tests
and eight foundation checks. Both processes exited successfully after cleanup.
[Foundation verification](foundation-verification.json) records exact counts,
source identity, native packet and log hashes, and exclusions.

Independent reader literals cover 660 contexts across 22 versions, five scalar
and text rotations, all three levels and separate null controls. Their complete
physical spans and reconstruction hashes match the reference. Twenty current
root/category writer controls pass compatible readers and reject version-zero
reader layouts. Invalid writer levels reject; no plot writer is inferred.

Owned input, complete primitive projection and ordered first-word edit history
packets are now retained under `tests/fixtures/simulation-results-headers`.
The 660 owned contexts and all 450 accepted native arguments are reparsed after
literal replacement; every unaffected field and copied parent remains in the
expected output. Native failed arguments remain in the original input packet.
[Packet references](operation-packets-reference.json) bind every byte and hash.
These are reference oracles; they do not establish Rust operation acceptance.

The typed Rust codec, malformed/cap/resource/isolation/concurrency checks, process exports
and full local/platform gates remain pending. Dialog, graph and auxiliary
streams are separate serializers and receive no coverage from this inventory.
Native application opening, historical writer symmetry, business meanings,
geometry validation and whole-format parity remain unproven.

Humanizer was unavailable; maintainer prose review remains pending.
