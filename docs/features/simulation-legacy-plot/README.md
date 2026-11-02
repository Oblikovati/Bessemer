# Historical named-plot interoperability (#174)

Specification sections [14.459](schema-14.459.md) and [14.526](schema-14.526.md) describe the historical named plot
stream, separate from `cwPostDlgParam`, `DspPostInfo` and `DspPostStyle`.
This feature is in development. No Rust codec or native acceptance is claimed.

`freeze_legacy_plot.py` copies the reference Python sources before importing
the plot model and its four original tests. It also pins the non-Python
46-read layout table, checks all recorded source dependencies, verifies both
schema clauses and installed byte ranges, and replays the complete native set.
Module origins and source hashes are checked after replay.
The [schema snapshot](schema-snapshot.json) identifies the original clauses;
the generated `source-snapshot.json` records the tested Python source cut.
The initial four-test replay is recorded in
[foundation verification](foundation-verification.json). The expanded gate
contains eleven checks; its native evidence is retained in
[native reference](native-reference.json). Both initialization and the strict
rerun passed all eleven checks; [native foundation verification](native-foundation-verification.json)
records the 120 candidate attempts, exact fixture packet and executed sources.
Run it from the repository root:

```powershell
python -B docs/features/simulation-legacy-plot/freeze_legacy_plot.py
```

The required codec covers both historical bulk layouts, all versioned fixed
fields and counted raw wide texts. Helper completion and caller compatibility
must remain separate. Of the 60 retained records, 48 parse completely; only
40 also have a compatible caller version. The eight incompatible contexts and
twelve short records remain required outcomes, with no invented saved version.

The five fixed wide buffers gain bounded prefix views from version 700004.
Every view must preserve the original buffer and its suffix starting at the
first zero unit. Exact version 700003 has a different conversion contract.
The 240 retained buffers all have nonzero bytes after their first terminator.

The installed check matches all 1,898 instructions and contiguous bytes from
the original dispatcher, reader, conversion and wide-helper probes. The fresh
[REA wide-helper capture](wide-helper-rea.json) independently confirms its
complete 171-byte body and 46 instruction locations. A fresh caller-image REA
request timed out; its larger body remains bound through the original C839
decompilation hashes and the complete installed-byte replay. No fresh caller
REA result is claimed.

REA displays the later MFC call as `TrimRight`, while the PE import is ordinal
14039 at `0x180010588`. Its actual runtime provider remains unresolved. These
checks establish the bounded raw-prefix scan, not the final native CString
value or its trimming rules. The first verifier attempt incorrectly required
a named import; the corrected check binds the observed ordinal and keeps the
runtime qualification explicit.

Next acceptance steps are independent owned layout expectations, typed
read/construction/edit operations, isolation and process checks, and the publication gates listed in
[issue #174](https://github.com/Oblikovati/Bessemer/issues/174). Reference tests
alone do not prove any of those steps or native writer equivalence.

Humanizer is unavailable; maintainer prose review remains pending.
