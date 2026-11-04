# Intersection reference matching state

Feature [#184](https://github.com/Oblikovati/Bessemer/issues/184) follows
[clause 11.1623](schema-11.1623.md), published as C-3307. Bessemer already reads
the three signed words in each embedded edge reference. This feature will give
raw24 and raw28 their documented stored roles and provide isolated edits through
the existing model archive operations.

The [preparation record](preparation-verification.json) pins the complete clause,
the [published report](reports/part-sketch-intersection-reference-match-completion-evidence.json),
four current readers and two original test modules. All fourteen direct report
dependencies match their published hashes. This is preparation: transitive
dependencies, native bindings, original Source replay and Rust operations have
not been verified here.

The [unchanged original test run](original-tests-verification.json) subsequently
passed all six tests without skips, including their existing native fixture
checks. Frozen-source and import checks passed for 5,661 copied Python files.
This does not rerun the separate C-3307 populated accounting experiment.

The [transitive inventory](dependency-inventory-verification.json) reached 615
reports and 30,306 source references. Of these, 4,660 do not match current files;
the graph includes earlier point-in-time reader revisions. Those historical
pins remain distinct from current requirements. Full dependency and native
binding admission remains incomplete.

The [unchanged populated replay](original-roots-verification.json) accepted all
eight retained complete streams without an accounting adapter. Complete graph
fingerprints, object maps and aliases, root spans, EOF and original unknown-byte
totals match the unadapted baselines. Direct container slices also match all
136 signed scalars and their state/raw20 prefixes. All six prepared reader/test
pins and the replay report's 31 direct dependencies match. This establishes
baseline reader acceptance; native bindings and typed operations remain open.

The [owned controls](owned-controls-verification.json) prove two complete sketch
definitions, eight distinct reference groups and a real alias. All 451 physical
primitive reads partition the 1,518-byte stream, including 32 independently
framed scalar spans. Six maintained regression tests pass against the retained
Source-accepted literal and invalid builder inputs. Run them through frozen
Source imports from the repository root:

```powershell
python -B -X utf8 docs/features/intersection-reference-match/freeze_owned_controls.py
```

An [ordinary PE-file check](native-fragments-verification.json) matches all
thirteen retained native intervals, totalling 21,607 bytes. This is exact fragment
verification, not REA instruction or dataflow Evidence. The
[REA qualification](native-tool-qualification.json) records the successful
artifact identity check, timed-out byte request and session cleanup. Complete
native bindings and the wider owned matrix remain pending.

The [ordinary instruction replay](native-instructions-verification.json)
subsequently matched all 5,395 retained instructions across thirteen captures
and both named consumer exports, using the existing Capstone 5.0.7 installation.
Instruction text matches exactly, including the original trailing spaces on
operand-free instructions. The mixed code/table update interval retains its
incomplete control-flow scope. Import, RTTI and bounded dataflow admission remain
separate; this replay supplies no REA Evidence or target execution.

The [bounded binding replay](native-bindings-verification.json) subsequently
matched all 76 imported call sites, the constructor's table/RTTI connection and
all 67 selected instruction assertions. The two receiver definitions and single
spill definition match the retained direct-predecessor review. All direct jumps
in the 95-instruction stackless helper stay within its decoded instructions;
its selected input operands read raw24/raw28 and do not read raw20. This retains
the published branch scope, opaque provider contracts and incomplete update CFG.
The wider owned matrix, typed operations and publication gates remain pending.

The [matrix-input tests](matrix-input-verification.json) passed all nine checks.
They cover 1,344 unique sorted Cartesian inputs, 10,752 generated physical groups
and 43,008 literal scalar values. A separate literal check covers every u16 state
word. The existing complete Source control still matches its original bytes.
The full matrix's complete roots and primitive oracles remain to be generated
and accepted through Source; the input tests do not establish that acceptance.

The [first complete matrix run](matrix-initial-verification.json) subsequently
passed all 1,344 declared cases and all ten tests. It retains 4,032 roots, 10,752
physical reference groups, 43,008 scalar values and 606,144 physical primitive
reads. Complete Source readers preserve definitions, aliases, map sizes, null
siblings and EOF. Every constructor output also matches an independent literal
patch. A separate stdlib checker verified the complete 2,045,568-byte packet and
every primitive range and raw byte without importing Source. The strict second
fresh-process replay is running; typed operations remain pending.

The [strict second process](matrix-source-verification.json) subsequently passed
the same 1,344 cases and all ten tests. It rebuilt every root, word row and
primitive row and matched all retained artifacts and metadata byte for byte.
All 5,686 common frozen Source files match; six added research files are listed
separately. Typed operations and the remaining caller/version, document,
concurrency and publication gates stay open.

The first [typed stored operation](api.md) is implemented for complete ordinary
part resolved-feature archives. The
[Rust operation record](typed-operation-verification.json) covers 43,008 changed
requests and 32,256 no-ops per complete matrix run, including paired requests
with only one actual change. All fields, literal bytes, original/replacement
history, completion, coverage, diagnostics, stale revisions and input isolation
pass. Two fresh Rust exports match at 67,503,748 bytes and independently match
every expected signed-word patch. Eight focused projection checks, all four
integration tests and standalone Clippy pass. Cargo/platform, fresh Source
changed-output, native edit, broader caller/version, whole-document and
concurrency gates remain required; this does not complete Feature #184.

The [first fresh typed-output Source replay](typed-export-source-initial-verification.json)
accepted all 44,352 actual exported roots, including 43,008 changed outputs and
1,419,264 scalar projections. Complete unchanged readers retained aliases, map
sizes, all physical groups and EOF. Every output also matched an independent
literal patch. The corrected run passed its pre-import and final frozen-source
guards; the rejected import-order and cast attempts remain excluded. A second
fresh-process replay is running. No accounting adapter or native target ran.

The [second complete typed-output replay](typed-export-source-verification.json)
subsequently accepted the same 44,352 roots and 1,419,264 scalar projections.
All 5,757 common Source files match across snapshots; nine added research files
are listed separately. Both corrected processes passed frozen-import and file
integrity checks. Native edits, caller/version boundaries, complete documents,
concurrency and publication gates remain open.

Regenerate the matrix only during explicit oracle initialization:

```powershell
python -B -X utf8 docs/features/intersection-reference-match/freeze_owned_controls.py --write-matrix
```

Without `--write-matrix`, the command performs strict byte comparisons with every
retained artifact and never updates them.

The published cohort contains eight complete streams, seventeen edges and
sixty-eight reference groups. Its 544-byte isolated accounting reduction is not
global coverage credit. Raw20 remains separate and uninterpreted by this feature.
No geometry, cache resolution, native matcher or topology lifecycle runs offline.

See the [design and acceptance scope](design.md). Publication remains serial
behind PR #182 and the accepted local queue. This preparation creates no new
publication branch or PR. Humanizer is unavailable; prose review remains pending.
