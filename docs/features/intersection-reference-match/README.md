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

The published cohort contains eight complete streams, seventeen edges and
sixty-eight reference groups. Its 544-byte isolated accounting reduction is not
global coverage credit. Raw20 remains separate and uninterpreted by this feature.
No geometry, cache resolution, native matcher or topology lifecycle runs offline.

See the [design and acceptance scope](design.md). Publication remains serial
behind PR #182 and the accepted local queue. This preparation creates no new
publication branch or PR. Humanizer is unavailable; prose review remains pending.
