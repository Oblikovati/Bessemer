# Historical MaterialList investigation (#175)

The legacy `Cosworks/<named-study>/MaterialList/<enumerated-name>` carrier is
separate from study-manager and AWDocument Material. Its current dispatcher
route is bound, but all70 retained full-reader attempts fail at the next
`text08` boundary. The verified header stops after `text60` and remains
explicitly incomplete.

The candidate body partitions all20,426 bytes into6,902 header bytes,196 fixed
records/4,704 bytes and1,470 trailing texts/8,820 bytes. Exact candidate EOF does
not bind the historical serializer. The observed21 trailing texts per stream
are not yet a verified format invariant. All16 identifier bytes and raw64
patterns must survive;182 identifiers have nonzero suffix bytes after NUL.
Installed spellings do not establish property IDs, units or historical order.

`freeze_legacy_material.py` copies the actual Python models before imports,
runs every original modern-material/header/candidate test, and checks all
C838/C894/C895 report/dependency hashes without exceptions. It retains the
complete14.458/14.512/14.513 clauses, including their noncompletion qualifications:

```powershell
python -B -X utf8 docs/features/simulation-legacy-material/freeze_legacy_material.py
```

Initialization and strict replay both passed all11 checks, including all nine
current original tests. [Foundation verification](foundation-verification.json)
records the complete clauses, source snapshot and logs. Native extraction and
installed-function verification remain separate from this initial source gate.

The investigation must establish the historical producer/consumer or context
before claiming a complete codec. Current-reader failures, original versions,
candidate-success flags and zero completed-schema credit remain required
outcomes. No Rust legacy codec or production fallback is introduced here.
Native writer equivalence and whole-format parity remain unproven.

Humanizer is unavailable; maintainer prose review remains pending.
