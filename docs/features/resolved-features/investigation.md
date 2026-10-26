# Separate ordinary part resolved features

Tracked in #54, as part of the schema parity work in #2. This investigation
precedes implementation. It does not establish Bessemer support for separate
`Contents/Config-N-ResolvedFeatures` streams.

The approved reference is `../inventor-interface/solidworks/`. Its
`test_resolvedcoverage.py` currently expects 873 completed modern part streams
and 49,781,219 bytes of known structures. That source regression is a future
corpus target, not a Bessemer result. Source unknown meanings remain explicitly
raw fields; completing framing does not establish geometric validity.

The newer `research/resolved-frontier-audit-0.370.json` reports 880 complete
streams and 26 explicit stops across the same 906 configurations. It assigns
zero coverage credit. A fresh run of the complete current reader, repeated in a
second process, confirmed 880 complete streams and 26 rejections over 842
document paths. Both processes produced identical outcome rows. Exact EOF checks
accounted for 50,486,789 bytes of known structures.

Disabling the new frontier dependency branch reproduced the older regression's
873 complete streams and 49,781,219 known bytes exactly. All 873 previously
complete rows stayed identical with the branch enabled. Seven newly complete
streams contributed 705,570 additional known bytes. `current-reference.json`
binds every outcome to document/stream hashes and a stable source inventory;
`current-reference-verification.json` records the comparison and remaining
rejections. These are reference-reader measurements, not Bessemer support.

## Owned baseline

The read-only probe used `tests/fixtures/blank-part.SLDPRT`, SHA-256
`b52e080ba2cc64d1d07ed28306f27281b07521aef550bc0c7fea8cd54be2b276`.
It supplied document bytes and an empty lookup table directly to the reference
container reader, avoiding the user's cached signature table. The source
inventory and fixture bytes were unchanged after the probe.

The reference then added an optional frontier dependency branch. A second fresh
process repeated the owned checks against that revision with identical results.
`owned-context.json` binds both hashes of the changed reader and the repeated
source inventory; wider corpus coverage still requires a fresh measurement.

`Contents/Config-0-ResolvedFeatures` occupied 5,558 bytes, SHA-256
`d7ba8160aafa1e390850ba9beceadf00fb3ef6b5b3cf9fc0b7cee7be2d6000b0`.
The reference consumed it through EOF: raw prefix 97, 17 feature roots, an MBSM
tail, and 2,724 bytes whose meanings remain unknown. The completed configuration
provided 97 object-map slots; the separate stream finished with 161 slots.

`Contents/CMgr` occupied 1,968 bytes, SHA-256
`f6387400850ae80dedbc5d2f5dc77fb1dd38ad4b8afb1e6fb2ffc2ad35edb27d`.
The reference consumed it through EOF, with 1,117 bytes of unknown meaning.
Its root was `moConfigurationMgr_c`, selecting `moPartConfiguration_c` through
two aliases to the same object. Its root list and validated postload graph each
contained one configuration. The saved helper state was 1 at byte offset 1,886.

Three independent mutations rejected the complete resolved operation:

- Appending one byte to CMgr failed its EOF check.
- Replacing the saved helper state with 2 failed its allowed-state check.
- Replacing the state with 0 while retaining the MBSM tail failed the separate
  stream's EOF check.

These checks show why reading the state at a fixed offset without completing
CMgr would be insufficient. The offset above describes this fixture only.

## Implementation requirements

At public-API baseline `bf8d61c`, the configuration reader discarded its
operation-local object reader after parsing. The current private work retains
that completed map as an immutable snapshot and lets subsequent decoders borrow
the completed configuration's field facts. A new builder must inherit those identities without
mutating the published configuration. A fresh null-only map or a map padded to
the prefix value would lose class and object evidence.

The owned CMgr loads only five runtime classes: `moConfigurationMgr_c`,
`moPartConfiguration_c`, `moNodeName_c`, `moVisualProperties_c` and `suObList`.
The last class has two new objects; each other class has one. Existing name,
visual-property and list codecs are candidates for reuse, but their context and
framing must be checked before reuse. `owned-class-loads.json` records separate
manager, configuration and feature-stream load counts. The feature stream's
class counts exclude inherited objects.

Complete the selected CMgr framing, configuration graph and EOF checks before
using its saved state. The verified part configuration and helper state 1 allow
the MBSM map; state 0 omits it. Other states and missing context fail the whole
operation. The feature list is direct: its prefix and compact/extended count
are followed by object pointers, without an enclosing object tag.

## Context foundation checks

The snapshot and inherited-fact changes remain uncommitted under #54; no
resolved-feature API is published. At the initial checkpoint, thirteen focused tests passed: two snapshot
tests, two owned configuration tests and nine object-fact tests. They check
class/object/null slot distinctions, every owned loaded identity, independent
builder growth and failure, short/extended counts, missing identities and
invalid local facts. An invalid local definition fails without consulting an
inherited body as a fallback. The ordinary public API was subsequently integrated
by PR #55 into candidate merge `f54815af5ce868958f0ae0eed7bc58e44c016c9a`
after all five required checks passed. Its local and remote branches were deleted.

## Complete owned manager checkpoint

The private CMgr reader now completes the owned 1,968-byte manager and forces
exact typed reconstruction. It reuses the existing feature, name, visual,
mass-property and object-list codecs. The factory's absent concept child list
is explicit; ordinary feature callers retain their existing child-list layout.

This first manager scope is version 19000 with one selected part configuration,
matching selected aliases, a one-root list, no parent or children, no conversion
state and no optional feature text. Required unsupported nonempty branches fail
explicitly. Wider documented graphs, populated currently unsupported maps and
older manager versions remain work toward schema parity; the singleton scope
does not redefine the goal.

`tests/fixtures/resolved-features/owned-manager-primitives.tsv` records 504 independent reference reads covering
every byte. The Rust oracle compares field boundaries, scalar kinds and exact
bit patterns against that partition. The generator's injected-root Python test
repeats it independently. Clauses 11.254–11.263 and 11.327 are preserved with raw
byte hashes in `manager-schema-snapshot.md` and its manifest.

The current focused suite has 26 passing tests, including every one of the
manager's 1,968 truncated prefixes, appended data, both allowed helper states,
invalid helper states, null/missing selected roots, parent/self-child graphs,
all five resource budgets and compact/extended collection framing. Impossible
64-bit map counts fail as invalid before a feasible unimplemented payload fails
as unsupported. Three existing body-feature tests and all ten public
configuration regressions also passed after the shared feature change.

The public result/history contracts and current reference corpus gate remain
ahead. This work stays uncommitted until its full public operation is complete;
unused private foundation code will not be shipped on its own.

## Owned feature read, construction and edit

The private feature reader now consumes the complete owned 5,558-byte stream,
including its direct 17-root list and required MBSM tail. It begins with the
configuration's actual 97-slot map and finishes with 161 slots. The raw prefix
is preserved without assigning it a meaning or deriving map slots from it.
Manager and configuration versions must agree. The manager's verified saved
state selects the tail; EOF rejects a state/tail mismatch.

The browser folder and origin-profile additions reuse existing feature, body,
list, sketch and reference-plane codecs. An independent 1,518-read primitive
partition checks the complete original stream's field boundaries, scalar kinds
and bit patterns. Typed construction reparses the entire output and requires
the supplied field paths, types and identities to match it exactly.

An independent Python edit changes the unique stored `Front Plane` CString at
offset 2,964 to UTF-16 `Bessemer Plane Ω`. The complete reference reader accepts
the 5,568-byte result and checks that semantic name. Rust construction matches
those expected bytes exactly. Reopening the complete edited document parses
its configuration and manager afresh, preserves every unrelated stream and
restores the original document byte for byte when reverted.

The current focused Rust suite has 34 passing tests; the Python suite has three.
It includes all 5,558 truncated feature prefixes, trailing bytes, empty results,
all five budgets, wrong context versions, missing inherited map slots, source
byte overwrite, failed construction and four concurrent deterministic edits.
Operation/result types have compile-time Send/Sync checks. These focused checks
do not replace the full local gate, sanitizer or native application acceptance.
The native check then opened both owned copies with zero errors/warnings in
SOLIDWORKS 2026 revision 34.2.1, retaining 17 features, zero bodies and one
configuration. However, the edited file still exposed `Front Plane`, and lookup
of `Bessemer Plane Ω` returned no feature. The complete 17-name native tree
matched the baseline. Bug #56 records this semantic acceptance failure.

Both file hashes and the three pre-existing open document paths/order were
unchanged. Neither file was saved or rebuilt. The normalized report is
`native-plane-name-failure.json`; it omits existing-document paths. The temporary
Rust output-emission test was removed after producing the native input.

The old name also occurs in LWDATA, both headers, DisplayLists, KeyWords and
3DExperienceExchange2. In the headers it is revision-log text, which does not
prove naming authority. No speculative cache/header/global replacement was
made. Raw LWDATA inspection found a candidate `moLWPlaneNodeData_c` class record;
the installed model DLL exports that class. Those observations do not establish
its serialization or load policy. REA bound the installed DLL by SHA-256; its
first string query timed out after 300 seconds without returning analysis
evidence. The subsequent naming investigation below resolved the owned-case
load policy without guessing a cache layout.

`owned-feature-verification.json` binds this checkpoint's sources, fixtures and
logs. The public API, immutable result/history surface, wider manager branches,
current reference corpus gate and native semantic acceptance remain unfinished.
Current work is on `codex/resolved-features`, based on the merged candidate;
there is no new open PR for this uncommitted feature.

In one optimized Windows test process, 10,000 deep builder copies of the owned
97-slot map took 35,843,600 ns in total. Snapshot clones took 27,100 ns in total.
These are one-process measurements without allocator counts or peak-memory
measurement, not a performance guarantee. The temporary test was removed after
recording `snapshot-copy-measurement.json`. Keep the simple immutable snapshot
and operation-local copy until broader workloads justify a storage change.

Prove one owned read, typed construction, edit and write before expanding the
corpus. Include independent expected bytes, whole-document reopen and revert,
unrelated-stream preservation, malformed inherited identities, aggregate
budgets, revision-bound history and concurrent input isolation. Keep unsupported
bodies and incomplete tails out of successful results. Measure the cost of
copying the baseline's 97 map slots before changing storage policy.

Integration order remains the private configuration codec, public configuration
operations, then this feature. Keep one PR active and remove each merged branch.
No version bump or release is part of this work.

Humanizer was unavailable; prose remains for maintainer review.

### Native naming policy regression (#56)

The nine-case native matrix is recorded in `native-plane-name-matrix.json`.
Every file opened with zero errors and warnings, 17 features, zero bodies and
one configuration. Read-only probes saved and rebuilt nothing; the three
pre-existing open documents retained their paths and order.

Changing the resolved feature alone, LWDATA alone, both together, or the global
Header2 alone left the displayed name as `Front Plane`. Changing the selected
configuration's ModelHeader log changed `IFeature.Name` to `Bessemer Plane Ω`.
Changing both headers, or both headers together with the resolved feature,
also succeeded. A separate positive control renamed the feature through the
native API and saved only an owned copy. Its source remained unchanged.

The matching identity is the persisted logical node ID 2, not the archive's
object-map index. The application regression composes the existing typed header
and resolved-feature codecs using this identity. It matches the complete
native-accepted owned fixture, preserves unrelated streams, isolates the input,
and reverts exactly using the operation's original compressed spans. Generic
field editing retains its declared stream scope; document naming policy belongs
to the application. Other cached views and exports have not been validated.

REA's longer Ghidra query completed successfully. The observed pointer reader at
`0x4c79a4d0` dispatches to the database or ordinary archive ReadObject using the
`moLWPlaneNodeData_c` runtime class. The preserved evidence is
`native-lw-plane-read-evidence.json`. This establishes pointer dispatch, not
the class body layout or naming authority, and adds no speculative LWDATA codec.

The earlier 34-test report and failed native check remain historical checkpoints.
This regression adds two Rust tests. Bug #56 stays open until the changes pass
the integration gates. Public resolved-feature operations, wider manager shapes
and the current reference corpus gate remain unfinished.

### First sample frontier measurement

The complete inherited configuration, selected ModelHeader, CMgr and separate
resolved-feature bytes were bound for all 906 existing sample configurations.
The explicit installed kernel schema was supplied. Exact feature reconstruction
was required for acceptance. All 906 cases stopped at an unsupported dependency;
none reached acceptance. This does not contradict the owned fixture result.

`native-frontier-checkpoint.json` retains all first failures, grouped causes,
packet and log hashes, and the supplied schema hash. Populated linked-atom maps
were the most frequent first failure. Other blockers include custom properties,
Swift schema references, toolbox arrays and face-color configuration objects.
This diagnostic is not a passing corpus gate. Its throwaway packet generator
and Rust probe were removed; the bound local packet remains under target for
the next investigation. No sample file was changed.

### Configuration dependencies and repeatable gate

The next implementation reads complete recursive linked-atom maps and direct
DWORD arrays, including shared nodes, self references and nulls. An independent
136-byte graph and 37 primitive observations pin its exact bytes and identities.
Complete custom-property records now work both directly and through tagged
`moCustomPropertyList` objects. Their independent 130-byte fixture and Unicode
edit retain duplicate names, signed limits and unsigned high bits. Prefix,
trailing-byte, count, identity, ownership and budget tests cover these readers.

Bug #57 records the configuration body-state role that rejected implemented
solid and surface body classes. The fix reuses the existing three-class body
family and retains rejection of unrelated classes. It affects the unfinished
configuration-manager feature; no deployed configuration-body policy is widened.
The common configuration-table folder, native configuration-table payload and
design-table model-feature body now use their documented complete serializers.
Independent reference tests confirm the shared body sizes, duplicate pair keys,
Unicode representation and four-byte time.

The repeatable external gate lives in `tests/resolved_features/native_corpus.rs`.
`prepare_resolved_corpus.py` binds all four input streams without filtering
failures; its packet matches the earlier diagnostic SHA-256 exactly. The gate
requires exact typed reconstruction of both CMgr and resolved features, and
fails until all 906 cases complete. The latest run completed **41**, rejecting
**865**. `dependency-frontier-checkpoint.json` retains the sample outcomes and
first failures. The separate owned packet unit test is excluded from those
sample counts. The next leading dependencies are per-body chooser objects,
populated scene maps, manager graphs and history-item records. These are stored
archive checks; no broader native or geometry acceptance is claimed.

To repeat the external check, first prepare the ordinary configuration corpus
and its `target/model-native-corpus-manifest.json` using the model-archive
workflow. The resolved packet generator verifies those configuration hashes
before reading each matching header, CMgr and resolved-feature stream. Run:

```powershell
python -B -X utf8 docs/features/resolved-features/prepare_resolved_corpus.py
$env:BESSEMER_KERNEL_SCHEMA = 'C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS/data/pschema/sch_13006.s_t'
cargo test --lib model_archives::resolved_corpus_tests::complete_native_resolved_corpus_requires_exact_typed_reconstruction -- --ignored --exact --nocapture
```

Use the exact test selection when collecting `RESOLVED` outcome rows. The owned
packet unit test also exercises the reporting helper; running both tests with
`--include-ignored` adds its two diagnostic rows to the console log. Those rows
are not sample cases. The explicit corpus test's total and failure assertion
remain the acceptance authority.

### Chooser, browser and graph completion

Complete `moPerBodyChooserData_c`, its filename subclass and the bounding-center
record now reuse the existing direct pointer-list, DWORD-array and typed wrapper
readers. The independent fixtures retain root self-aliases, extended counts,
unsigned high bits, signed limits, Unicode filenames and exact floating bits.
Their primitive partitions contain 18, 24 and 9 observations. An independent
raw-word edit pins the writer; malformed identity, filename-target and count
tests reject incomplete objects.

The manager now applies the documented multi-configuration postload policy from
11.284. Matching selected pointers must reach a nonempty distinct root list.
Every reachable node must be a part configuration with null serialized parent,
zero conversion and absent optional feature text. An iterative active/complete
walk preserves shared children and rejects cycles. It invents no reciprocal
parent pointers and consumes no extra postload bytes. Rust and independent
reference tests cover shared descendants, multiple roots, selected descendants,
a 1,501-node chain and each rejected predicate. Existing owned manager prefix,
budget and saved-helper-state tests still pass.

Final history/favorite item serializers now read their node body and, for feature
items, the documented component-feature pointer. The pointer keeps the existing
finite component-feature class family. Live/plane section folders add their raw
word to the common folder body; configuration-comment folders reuse that common
body unchanged. Seven independently parsed fixtures pin their complete layouts,
with all prefixes, appended bytes, ownership and unrelated-class rejection
covered. The schema snapshot and section hashes are retained in
`graph-browser-schema-snapshot.md` and `graph-browser-schema-reference.json`.

The same complete 906-case gate advanced from 41 through 118 and 162 to **190**
accepted reconstructions. The other **716** remain explicit failures, led by
populated scene maps, chamfers, context-chain components and attributes. This is
progress toward documented parity, not a passing broad gate or a native geometry
acceptance claim. The public resolved-feature operation surface remains open.

`graph-browser-frontier-checkpoint.json` binds two fresh processes with identical
906-case outcome rows. The independent reference reader also completed every
Bessemer-accepted case against matching hashes for all four context/feature
streams. Those 190 resolved streams total 5,787,213 bytes; their independent
root counts, byte lengths and raw-semantic unknown-byte totals are retained in
`accepted-reference-checkpoint.json`. This comparison proves reference framing
completion for that accepted subset, not per-field semantic or native geometry
equivalence across the entire sample corpus.

The full local `cargo test` run passed, including 366 library tests and 247
doctests; three external-fixture gates remained ignored by that command. The
explicit 906-case gate was run separately and failed as reported above. Fifteen
independent Python tests passed. `graph-browser-verification.json` binds this
checkpoint's sources, fixtures and logs. No PR, push, version change or release
was part of this still-uncommitted checkpoint.

### Ordinary configuration scenes

The PW scene-data, wall and floor serializers now share their existing
display-list implementations through the named-primitive interface. The display
adapters retain their object and transform policies. Existing owned modern and
earlier display-scene regressions passed before configuration integration;
neither their expected fields nor their expected bytes changed.

The current CMgr reader now decodes populated owned scene maps and both concrete
scene classes. It retains signed counts, ordered duplicate keys, shared object
identities, all three wall shapes and exact scalar/transform bits. A separate
typed operation-local context selects the documented condition-0x200 scope.
Ordinary model reads cannot gain scene permission by encountering a class name,
and nested scope selection restores the previous context. Generic concept
helper slots accept only null or previously loaded pointers; fresh object bodies
are rejected before decoding. Older configuration-scene layouts remain explicit
unsupported paths; shared display readers retain their existing version gates.

Five independent owned cases pin complete cube/sphere scenes, negative empty
counts, duplicate-key maps and an exposure edit. The reference performs 23
speculative PW-prefix reads before its real decoder. The primitive observer
excludes exactly those call-site-identified probe reads and requires the remaining
reads to partition every byte without overlaps or gaps. Exclusion counts and
all five reused wire-builder hashes are retained in
`configuration-scenes-reference.json`. Six complete scene/map tests plus the
explicit context regression cover prefixes, appended bytes, class/identity
policy, input isolation, counts and all five operation budgets. The budget
checks require actual limit failures inside the selected scene context.

The same 906-case sample gate now reconstructs **203** cases, leaving **703**
explicit failures. Its remaining first dependencies include sensors, chamfers,
planar surfaces and context-chain components. The independent reference reader
also completed all 203 accepted cases against matching context/feature hashes,
covering 6,155,753 resolved-feature bytes. That confirms reference framing of the
accepted subset; it does not establish per-field semantics or native geometry
acceptance. The ordinary-scene schema clauses and hashes are retained separately
in `configuration-scenes-schema-snapshot.md` and its reference manifest.

One Python suite run rejected reference changes during an owned-feature check;
the owned fixture retained its original hash. A fresh full run passed all 16
Python tests. `scene-reference-drift.json` identifies the two changed research
drivers, and the interrupted log remains under target. Earlier successful
source-bound checkpoints were preserved.

The completed local run passed 373 library tests and 247 doctests, with three
external-fixture gates ignored by the default command. The explicit 906-case
gate still fails at 203/703, with identical outcome rows from two fresh
processes. `scene-sharing-frontier-checkpoint.json` retains those outcomes;
`scene-sharing-verification.json` binds the sources, fixtures, structural
refactor regressions and final logs. This checkpoint remains uncommitted on the
existing feature branch. No new PR, push, dependency, version or release was
introduced.

### Sensors, chamfers and finite wrapper roles

The sensor reader now consumes the complete inherited model feature, inline data
and all three stored reference vectors. It preserves the two independently
serialized +0x88 values, the +0x34-before-+0x30 point order, the binary32/binary64
switch at 0x3765, both tail gates and the type-0x47 string group. Nonpositive
string counts retain their raw values and represent empty stored arrays.
Native type 2's postload removal of one vector does not discard its wire content.
Thirty-two independent data cases cover every gate boundary and conditional
string mode. Complete sensor objects, a loaded-reference payload, all prefixes
and an independent repeated-member edit also pass their primitive partitions.

The chamfer reader reuses the complete body-feature base and direct surface-ID
keeper. Its suffix retains byte-width flags, packed DWORD arrays, signed
presence values, typed dimensions, shared references and the optional wrapper.
Independent populated direct and complete tagged fixtures pin 35 and 124
primitive observations, respectively. Every truncated prefix, appended byte,
negative keeper count and wrong dimension class is rejected. An independent
raw-word edit preserves the original owned fields.

The first combined gate reached 229 accepted cases and exposed 38 point-reference
stops in the new sensor wrapper vectors. Bug #58 records that narrower-than-base
role. A finite family assembled from twelve existing documented concrete wrapper
readers now serves those vectors and the chamfer's optional base-wrapper pointer.
Tests accept every listed loaded class and reject unrelated display/component
objects and suffix lookalikes. The display-object sensor vector and specific
dimension roles keep their own constraints. The role fix advances those cases to
their next dependencies; it does not add another accepted case at this checkpoint.
This corrects unfinished feature code and remains open until green integration.

The exact 906-case gate currently accepts **229** cases and rejects **677**.
The independent reference reader completed every accepted case against matching
four-stream hashes, totaling 7,879,900 resolved-feature bytes. That proves
reference framing completion for the accepted subset, not complete semantic or
native geometry acceptance. Sensor/chamfer schema clauses and section hashes
are retained in their own snapshot. Planar surfaces, attributes, context-chain
components and detail folders remain leading first dependencies.

The full local run passed 384 library tests and 247 doctests; three external
fixture gates remained ignored by that default command. Eighteen independent
Python tests passed. The explicit sample gate was run separately and failed
with identical 906-case rows in two fresh processes. Outcomes and verification
hashes are retained in `sensor-chamfer-frontier-checkpoint.json` and
`sensor-chamfer-verification.json`. Public resolved operations and complete
documented parity remain unfinished; this checkpoint adds no PR, push or release.

### Planar surfaces and attributes

Attribute and definition readers now share the constructor-disabled child-list
path. Their direct parameter arrays preserve aliases and self-referencing
definitions. Base, scalar and string parameter bodies retain both name pointers,
negative values, override-presence bytes and UTF-16 values. Independent complete
graphs, parameter modes and an edited scalar value pin their primitive partitions,
prefix failures, class constraints and source ownership.

Reference planes and planar surfaces now reuse the same reference-geometry
prefix without changing the plane field paths. Planar surface bodies remain
inline even when the backend marker is nonzero, as required by this class's
virtual branch. Sheet attributes are read for suppressed sheet bodies; extra-body
attributes are read only after a complete serialized body. Present kernel data
requires the caller's explicit schema and limits. The word-vector version gate,
mixed packed/plain keeper counts, unsigned high bits, all kernel-prefix failures
and an independent planar-word edit are covered by owned fixtures.

The complete planar fixture exposed bug #59: the inherited nonnull ID-keeper role
omitted the concrete planar keeper. The fix admits only the base and the newly
implemented keeper and retains explicit null/unrelated/lookalike rejection. The
sample gate then exposed bug #60 in the atom-feature chooser's owner role. Its
finite existing family now includes the implemented planar and chamfer classes,
while unrelated model features remain rejected. Both tickets stay open through
green integration; the implementations remain unshipped.

The exact gate advanced from 229 through 266 to **294 accepted / 612 failures**
among the same 906 sample configurations. Two fresh processes produced identical
outcomes. The independent reference reader completed every accepted case against
matching four-stream hashes, covering 9,655,678 resolved-feature bytes. These are
stored-grammar checks, not per-field semantic or native geometry acceptance.
Context-chain components, detail folders, scale features and schema-reference
objects remain among the leading dependencies.

Full local `cargo test` passed 396 library tests and 247 doctests; its three
external-fixture gates remained ignored by the default command. The explicit
sample gate was executed separately and still fails as reported above. Twenty
independent Python tests passed. The reference `xtbin.load_schema` emitted its
existing unclosed-file ResourceWarning; its source was left unchanged and the
warning is retained in the logs. Verification, source/helper/fixture hashes,
schema snapshots and outcomes are retained in the planar/attribute checkpoint
files. No PR, push, dependency, version change or release was introduced.

## Context-chain framing checkpoint

Five context-chain class bodies now have private readers and closed-form tests.
The independent reference completed each body through EOF; its source hashes
and results are in `context-chain-reference.json`. The corrected focused suite
passed three tests. The 906-sample gate still accepts 294 configurations and
explicitly rejects 612; fresh external feature and reference objects remain
unsupported. See `context-chain-checkpoint.md` for validation scope and logs.

## External-feature reconstruction checkpoint

The complete ordinary `moExtFeature_c` body now reuses the existing shared-file
rules and validates its required string handle. Six independent fixtures cross
the PDM-text and additional-byte version boundaries. Four focused tests passed,
including scalar expectations, truncations, trailing bytes and null-name
rejection. Schema clauses 11.121, 11.122 and 11.143 were retained with verified
raw hashes; the full `moReference_c` body remains unsupported.

Exact CMgr and feature-stream reconstruction increased from 294 to 323 of 906
configurations. Two fresh processes produced identical outcomes for all 906.
The independent reference completed every accepted case against matching
four-stream hashes, covering 12,019,332 feature bytes. The other 583 cases still
fail explicitly. The leading dependencies are `moPLine_c` (41),
`moDetailFolder_c` (35) and `moScale_c` (34). These results establish stored
framing, not native geometry acceptance or complete schema parity.

Full `cargo test` passed 400 library tests, the integration suites and 247
doctests; three external gates remained ignored by the default command. The
explicit 906-case gate was run twice and still failed as reported above.
Additional scalar assertions passed in the final focused run. Evidence is in
the `external-feature-*` reports and logs. No PR, push, dependency, version
change or release was introduced.

## Polyline reconstruction checkpoint

Complete ordinary polyline, base data, project data and tagged WORD-array
readers now reuse the existing body-feature and entity-ID codecs. Four owned
fixtures have independently verified primitive partitions; the expected edit
was generated separately. Five focused Rust tests cover exact reconstruction,
truncations, required pointers, invalid counts and input isolation. Three fast
independent Python polyline tests passed.

Exact sample reconstruction increased from 323 to 357 of 906 configurations,
preserving every previously accepted case. Two fresh processes produced
identical outcomes. The independent reference completed all 357 accepted cases
against matching four-stream hashes, covering 14,152,243 feature bytes. The
549 remaining failures stay explicit. Leading dependencies are detail folders
(35), scale features (34) and schema-reference objects (32). Draft/intersection
polyline data remain unsupported; native application and geometry acceptance
have not been established by this checkpoint.

Full `cargo test` passed 405 library tests, the integration suites and 247
doctests; three external gates stayed ignored by default. The explicit sample
gate was run twice and still failed as reported above. Clause 11.145 and the
source/fixture evidence are retained in the polyline checkpoint artifacts.
No dependency, version, release, PR or push changed.

## Scale and schema-reference reconstruction checkpoint

Complete ordinary scale features, coordinate-system wrappers and Swift schema
references now reuse existing codecs. Five owned fixtures have independent
primitive partitions, including a separately generated scale edit, a shared
coordinate-system pointer, a UTF-16 name and signed extrema. The raw schema
value is retained without applying the native helper's runtime remap. Four
focused Rust tests and four fast independent Python tests passed.

Exact sample reconstruction increased from 357 to 386 of 906 configurations,
preserving every prior accepted case. Two fresh processes produced identical
outcomes. The independent reference completed all 386 accepted cases against
matching four-stream hashes, covering 15,168,199 feature bytes. The remaining
520 cases fail explicitly. The leading dependencies are detail folders (45),
cut-list sort options (30) and face-color configurations (27). These checks do
not establish native application or geometry acceptance.

Full `cargo test` passed 409 library tests, the integration suites and 247
doctests; three external gates remained ignored by default. The explicit
sample gate ran twice and still failed as reported above. Clauses 11.150 and
11.264, fixture observations, source hashes and corpus evidence are retained
in the scale/schema checkpoint artifacts. No dependency, version, release,
PR or push changed.

## Cut-list sort and face-color configuration checkpoint

Complete cut-list sort-option and face-color configuration readers now preserve
their signed raw values, packed lists and typed pointers. Three owned fixtures
have independent primitive partitions, including a separately generated edit
and repeated self-reference. Four focused Rust tests and two fast independent
Python tests passed, covering truncations, invalid counts, missing references,
pointer roles and completed empty lists.

Exact sample reconstruction increased from 386 to 388 of 906 configurations,
preserving every prior accepted case. Two fresh processes produced identical
outcomes. The independent reference completed all 388 accepted cases against
matching four-stream hashes, covering 15,305,243 feature bytes. The other 518
cases still fail explicitly. Leading dependencies are detail folders (48),
mirror patterns (37) and hole-wizard features (28). Native application and
geometry acceptance remain outside this stored-framing checkpoint.

Full `cargo test` passed 413 library tests, the integration suites and 247
doctests; three external gates remained ignored by default. The explicit
sample gate ran twice and still failed as reported above. The build retains
20 dead-code warnings from the unfinished private CMgr/resolved-feature API.
The next integration work is recorded in `publication-plan.md`: connect a
complete operation and actual caller before extending or publishing that API.
These warnings are not suppressed, and no green PR gate is claimed.
Clauses 11.132 and 11.265 and the fixture/corpus evidence are retained in the
folder/configuration checkpoint artifacts. No dependency, version, release,
PR or push changed.

## Public resolved-feature read/write checkpoint

The public `PartResolvedFeaturesArchive` and reconstruction example now use the
previously private CMgr and feature path. All 20 dead-code warnings are gone
without suppression. Four public integration tests cover owned exact bytes,
root/map counts, input isolation, concurrent reconstruction, Send/Sync, failure
boundaries and completed empty results. The example's whole-document test and
actual CLI run reproduce the original owned document byte for byte.

The corpus probe now also requires public-API field and byte reconstruction.
Two fresh processes retain the same 388 accepted and 518 rejected outcomes.
Independent reference reads completed every accepted case against matching
four-stream hashes, covering 15,305,243 feature bytes. These checks remain
stored-schema evidence, not native application or geometry acceptance.

`cargo xtask pr` passed its Clippy/build/test pipeline, including 413 library
tests and 255 doctests. Later test-only public empty-result and corpus-call
assertions passed focused checks and final all-target Clippy. See
`public-resolved-checkpoint.md` for precise execution scope. Editing, field
history and the full CI matrix remain required by #54 before integration;
`publication-plan.md` tracks that work. No PR, push, dependency, version or
release changed.

## Public resolved-feature edits and history

The existing configuration field-history implementation was shared in separate
behavior-preserving commit `d9d4147`, after all ten public configuration API tests
passed. Resolved-feature edits now use it to return completion, coverage,
diagnostics, exact stored-field fidelity and revision-bound structural history.
Nine public tests cover independent expected bytes, all three field-relation
kinds, handle ownership, unchanged and failed edits, concurrency, whole-document
reopen/revert and unrelated streams. Both example tests passed, and two actual
edit CLI processes produced identical documents. The independent reader verified
the edited feature bytes and exact EOF against the expected fixture.

`cargo xtask pr` passed Clippy/build/tests without warnings, including 413 library
tests and 265 doctests. Both explicit corpus runs retain 388 accepted and 518
rejected outcomes, preserving every prior accepted case. Independent reference
reads completed all accepted cases against matching four-stream hashes, covering
15,305,243 feature bytes. See `public-edit-checkpoint.md` for scope and evidence.
The full CI matrix and remaining schema work are still required; no complete
parity or new native application acceptance is claimed. No dependency, version
or release changed.
