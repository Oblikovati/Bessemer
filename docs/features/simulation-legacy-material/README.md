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

The expanded installed-reader gate passed all12 checks in initialization and
strict replay. It verifies the full current image hash, ten recorded intervals
totaling19,007 bytes, all15 current-serializer direct calls and the exact
MaterialList literal. [Current-reader verification](current-reader-verification.json)
records the scope and logs. The later native replay below supersedes the pending extraction step; these
interval checks alone do not bind the historical candidate body.

The older modern-Material checkpoint supplies only its pinned native intervals.
Its obsolete pure-model source pins are not adopted. C838/C894/C895 retain
their current strict model/dependency pins, with no hash exception. REA's
objdump provider was unavailable on this host; the existing PE reader verified
the same installed image and intervals without executing target code. The
[tool qualification](native-tool-qualification.json) preserves that limitation.

The fresh replay passed all15 checks, including all nine original tests. It
recovered all70 complete payloads through frozen container readers and matched
the original headers, retained version contexts, current-model failures,
candidate partitions and196 identifier records exactly. The nine installed
identifier spellings were checked through their actual pointer slots; their
positions still do not establish wire property IDs or units.
[Native replay verification](native-replay-verification.json) records the logs
and current snapshot.

The installed reader delegates below version800001 to an older helper. Seven
contiguous chained unwind fragments identify its404-byte interval, excluding
the adjacent functions. Static import calls show16 identifier characters and
a double per property, followed by24 strings and an additional `text08` at
version800000 and above. This differs from the retained candidate's21 strings.
[Static helper capture](historical-helper-capture.json) and its
[assembly listing](historical-helper-objdump.txt) preserve the distinction.
These ordinary PE/objdump observations are not REA Evidence, native execution,
or proof of the historical producer. The current source model's failures stay
unchanged; its modeled profile is not promoted to native runtime equivalence.

The investigation must establish the historical producer/consumer or context
before claiming a complete codec. Current-reader failures, original versions,
candidate-success flags and zero completed-schema credit remain required
outcomes. No Rust legacy codec or production fallback is introduced here.
Native writer equivalence and whole-format parity remain unproven.

Humanizer is unavailable; maintainer prose review remains pending.
