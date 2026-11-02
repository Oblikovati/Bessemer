# Historical named-plot interoperability (#174)

Specification sections [14.459](schema-14.459.md) and [14.526](schema-14.526.md) describe the historical named plot
stream, separate from `cwPostDlgParam`, `DspPostInfo` and `DspPostStyle`.
`SimulationLegacyPlot` reads, constructs and edits every documented helper
layout under an explicit saved version. The complete operation is implemented;
final local checks and publication acceptance remain in progress.

`freeze_legacy_plot.py` copies the reference Python sources before importing
the plot model and its four original tests. It also pins the non-Python
46-read layout table, checks all recorded source dependencies, verifies both
schema clauses and installed byte ranges, and replays the complete native set.
Module origins and source hashes are checked after replay.
The [schema snapshot](schema-snapshot.json) identifies the original clauses;
the generated `source-snapshot.json` records the tested Python source cut.
The initial four-test replay is recorded in
[foundation verification](foundation-verification.json). The expanded gate
contains fourteen checks, including populated owned layouts and every proper
prefix. Native evidence is retained in [native reference](native-reference.json).
The earlier initialization and strict rerun passed eleven foundation checks;
[native foundation verification](native-foundation-verification.json)
records the 120 candidate attempts, exact fixture packet and executed sources.
Run it from the repository root:

```powershell
python -B docs/features/simulation-legacy-plot/freeze_legacy_plot.py
```

The codec covers both historical bulk layouts, all versioned fixed
fields and counted raw wide texts. Helper completion and caller compatibility
must remain separate. Of the 60 retained records, 48 parse completely; only
40 also have a compatible caller version. The eight incompatible contexts and
twelve short records remain required outcomes, with no invented saved version.

The five fixed wide buffers gain bounded prefix views from version 700004.
Every view must preserve the original buffer and its suffix starting at the
first zero unit. Exact version 700003 has a different conversion contract.
The 240 retained buffers all have nonzero bytes after their first terminator.

`from_bytes` requires exact EOF. `from_fields` checks physical order, paths,
value kinds and the complete version-selected shape. `edit_field` accepts a
revision-bound field identity and retains the immutable source, result,
replacement, exact coverage and every ordered history relation. Unchanged
requests retain the original identities. All unselected bytes must match.

Words and eight-byte values are raw patterns; fixed buffers and the historical
bulk forms remain complete byte blocks. Counted texts reuse the existing
raw-preserving character-text type. Counts are signed and nonnegative, with
an independent 512-character profile cap, per-text character/payload budgets,
aggregate character budget and complete input/field budgets. A zero count
describes stored framing and does not claim to clear native runtime state.

`fixed_wide_view` returns an owned immutable buffer view from version700004;
earlier versions return no view. It exposes exact source offsets, scan limit,
pre-NUL code units, terminator presence and the complete remainder. Unterminated
buffers retain their full bounded prefix, and lone surrogates remain unchanged.

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

The owned set contains52 cases across13 saved-version boundaries and four
payload profiles. Source checked all92,944 proper prefixes, trailing bytes,
negative/over-limit counts, raw NUL suffixes and unterminated buffers. Its
[operation packets](operation-packets-reference.json) preserve every expected
typed field, span and byte, all96 successful native arguments and all240
original native wide views. The complete native packet also retains all24
failed attempts and their original endpoints.

The final Rust integration run passed all15 tests, including both native tests
with zero failures or ignored cases. All-target/all-feature Clippy passed.
The strict Source rerun passed all14 checks without updating baselines.
Two fresh processes produced identical30,307,140-byte field/history exports,
also matching the concurrent full-suite export. Runs took0.328 and0.266 seconds
with peak working sets46,477,312 and46,211,072 bytes. The
[process proof](process-pair-verification.json) records the binary and hashes;
[operation verification](operation-verification.json) records all gates and
their limits. No previous release provides this operation for comparison.

Remaining acceptance work includes the exact publication-head repository
gate and all five platform checks in
[issue #174](https://github.com/Oblikovati/Bessemer/issues/174). Native writer
equivalence and whole-format parity remain unproven.

Humanizer is unavailable; maintainer prose review remains pending.
