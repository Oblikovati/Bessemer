# Historical named-plot interoperability (#174)

Specification sections [14.459](schema-14.459.md) and [14.526](schema-14.526.md) describe the historical named plot
stream, separate from `cwPostDlgParam`, `DspPostInfo` and `DspPostStyle`.
This feature is in development. No Rust codec or native acceptance is claimed.

`freeze_legacy_plot.py` copies the reference Python sources before importing
the plot model and its four original tests. It also pins the non-Python
46-read layout table and checks module origins and source hashes after replay.
The [schema snapshot](schema-snapshot.json) identifies the original clauses;
the generated `source-snapshot.json` records the tested Python source cut.
The initial replay passed all four tests and exited successfully; the
[verification record](foundation-verification.json) lists its scope and exclusions.
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

Next acceptance steps are installed binding verification, independent layout
expectations, complete retained-candidate replay, typed read/construction/edit
operations, isolation and process checks, and the publication gates listed in
[issue #174](https://github.com/Oblikovati/Bessemer/issues/174). Reference tests
alone do not prove any of those steps or native writer equivalence.

Humanizer is unavailable; maintainer prose review remains pending.
