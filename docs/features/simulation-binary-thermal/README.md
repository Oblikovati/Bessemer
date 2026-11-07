# Binary thermal collections

Feature [#134](https://github.com/Oblikovati/Bessemer/issues/134) covers complete
Temperature, Convection, HeatFlux, HeatPower and Radiation collections. Their
reference containers require an explicit Simulation embedded archive context.
The ordinary part and assembly archive guards must retain their existing behavior.

The [initial acceptance ledger](acceptance.json) maps the feature requirements
to their evidence and records the code/test fingerprints. The focused corpus
and independent replay passed. The full local gate passed 3,183 tests with zero
failures; platform integration remains pending.
Earlier verification files retain their original implementation-stage results.

Bug [#137](https://github.com/Oblikovati/Bessemer/issues/137) exposed an additional
wire-framing case after that initial gate. Two embedded CStrings can decode to
the same text while retaining different prefix widths. Edit no-op checks now
compare the complete requested embedded wire image, leaving semantic text
equality unchanged. Both valid forms are independently replayed by the actual
reference reader. Focused regressions check the framing change and an authentic
byte-identical no-op. The fixed candidate requires a fresh full validation gate.

## Reference baseline

`freeze_thermal.py` copies the complete approved SolidWorks Python source tree
and the external `reader/cfb.py` adapter before importing the actual readers.
It copies the existing document signature cache into that temporary directory;
the replay does not load or execute a SolidWorks DLL or COM object.

The actual document and CFB adapters recover all 68 C-716 streams, totaling
267820 bytes. Each outer envelope and thermal payload must match its retained
SHA-256. The actual additive archive callback reproduces all 68 successful and
seven failed candidate attempts, including complete spans, archive class counts,
body counts and transform traces. Successful attempts contain 242 embedded
archives and 19 component transform wrappers. Native payloads remain under
ignored `target/`; committed evidence records their identities.

The owned baseline adds 198 complete transform wrapper archives across all
historical wrapper thresholds and their neighbors. Both optional transforms
are populated; both direct suffix transforms cover matrix presence and the
base perspective, extended perspective and 5×5 matrix payloads. Literal double
patterns include signed zero, infinities and a NaN payload. The actual direct
codec independently verifies the boundary between the two suffix transforms.
Every truncated prefix of a populated matrix case must fail, along with
unsupported archive versions. Complete relevant schema clauses are frozen
separately so later documentation changes are visible in strict replay.

Install the reference-only import dependency and run the strict replay from
the repository root:

```powershell
python -m pip install --target target/reference-python --only-binary=:all: --no-deps --require-hashes -r docs/features/simulation-binary-thermal/reference-requirements.txt
python -B -X utf8 docs/features/simulation-binary-thermal/freeze_thermal.py
```

`--write` creates initial baselines for review. Normal verification compares
them without rewriting them. The source snapshot records every copied Python
file and imported reader origin. The external CFB adapter appears as `cfb.py`
in that snapshot and originates from `../inventor-interface/reader/cfb.py`.

## Remaining implementation

The Rust draft provides reads, construction and isolated field edits for the
complete declared embedded registry in an explicit context. Its fields, immutable
snapshots, object-map framing, transforms, coverage and history reuse the existing
model archive implementation. Unknown embedded classes fail explicitly; they
never fall through to ordinary model readers. Full independent projections,
malformed-case coverage and verification gates remain open, so this draft is
not ready to merge.

The registry also reads historical component holders, null-owner unit components,
reference chains, external-feature lists, external features and external-file
fields. The holder fixtures cover 66 complete archives across every word and text
gate; 140 additional owned archives cover nested chains and shared-file gates.
All reconstruct exactly against the actual reference callback. Unsupported
old-reference, reference-info and direct component-transform sub-bodies remain
explicit errors, as do nonnull component owners and external unit models.

The native dependency gate uses 242 complete embedded roots detached from the
68 thermal streams. Each detached image first replays with the actual full
reference registry, with rebased boundaries and unchanged class/body counts.
Rust reconstructs, constructs and performs unchanged edits on all 242 roots
exactly, including all 19 transform wrappers. The actual callback's declarations
and real registry dictionaries are inspected without changing the readers;
every one of the 61 declared classes has an owned root case. Another 1191 owned
images cover reference families, including shared object aliases, cached classes
and historical suffix gates. The depth fixtures accept 128 nested representations
and reject 129 independently of larger operation budgets. The reference Python
process temporarily raises its interpreter recursion limit to exercise that
declared format boundary, then restores the prior limit.
The ignored test requires `BESSEMER_EMBEDDED_NATIVE_INPUT` to name the extracted
packet explicitly. Normal fixture tests do not substitute for that native gate.

The outer reader and constructor cover the full shared load base and all five
derived thermal grammars. All 68 native streams reconstruct exactly, while the
seven retained alternate-version candidates remain rejected. Another 295 owned
collection cases cover every common and derived threshold and its neighbors,
exact selector one, populated optionals, repeated pairs and table entries, raw
text frames and two nonnull embedded archives with distinct inner profiles.
Every actual reference-parser span boundary is checked against the typed outer
fields. Constructors check the complete selected shape through EOF. The new
prefix reader consumes one embedded root without absorbing the outer suffix;
its byte budget applies to that root, independently of the outer input extent.

Thermal edits use field identities tied to immutable collection revisions.
They return the retained source and complete replacement, a validated result,
completion, exact coverage, diagnostics, zero achieved pattern error and every
outer field relation. Embedded archive history composes source and replacement
parents, including deleted and created fields when an archive shape changes.
No-op edits preserve both outer and nested identities. Result getters expose
immutable references; changing a returned coverage copy cannot alter the result.
The tests compare complete scalar-edited images against independent actual-reader
expectations for all 295 owned contexts and 68 native streams. Eight threads also
perform 32 isolated edits each and compare bytes and ordered histories.

The actual reference callback and Rust reader both reject all 36,569 targeted
count, text, optional, mode and inner-version mutations. Exact byte splices keep
the complete mutant corpus in a 1.6 MB fixture. Every proper prefix and one
trailing byte are rejected for all 295 owned contexts. Independent source spans
also establish exact aggregate-entry, text, embedded-root and object budgets;
the exact limits pass and a limit reduced by one fails without source mutation.

Complete independent outer projections cover every path, type discriminant,
offset and raw value in all 295 owned contexts and 68 native streams. The
reference callback supplies complete spans; the projection expands their
documented mixed scalar groups without reading Rust output. Two fresh processes
produce identical 5,953,224-byte exports matching the reference byte for byte.
Embedded archives are retained as complete atomic images in this outer export.
The native projection test requires `BESSEMER_THERMAL_NATIVE_PROJECTION_INPUT`
to name `target/simulation-thermal-native-projection.bin`; set
`BESSEMER_THERMAL_PROJECTION_OUTPUT` to an explicit destination to export it.

Complete scalar-edit exports also cover every edited outer field and every
ordered outer history relation with all source and replacement parents. The
actual reference callback validates each expected edited image before the
independent projection is generated. All 295 owned and 68 native contexts match;
two fresh processes produce identical 8,780,754-byte exports matching that oracle.
Set `BESSEMER_THERMAL_NATIVE_HISTORY_INPUT` to
`target/simulation-thermal-native-history.bin` for the native test, and optionally
set `BESSEMER_THERMAL_HISTORY_OUTPUT` to export the complete checked packet.

Independent nested-pointer projections cover all 1,595 owned roots and all 242
native roots. A Python profile callback observes calls to the original
`ObjectArchive.object` implementation without replacing it or its readers.
Every pointer's framing, extension, resolved kind, map index, class name and
schema matches, as does the final map extent. Aliases and cached classes retain
their original identities. The prior Python profile callback is restored after
each operation. Two fresh processes produce identical 822,634-byte exports
matching the independently observed source records.
Set `BESSEMER_EMBEDDED_NATIVE_MAP_INPUT` to
`target/simulation-embedded-native-maps.bin` for the native test, and optionally
set `BESSEMER_EMBEDDED_MAP_OUTPUT` to export the checked packet.

Nested primitive projections also match every field type, exact byte range and
raw value across all 1,595 owned and 242 native roots, including CString framing
and complete compact transforms. Two fresh processes match the independent
1,382,060-byte oracle. The native test requires
`BESSEMER_EMBEDDED_NATIVE_FIELD_INPUT=target/simulation-embedded-native-fields.bin`;
`BESSEMER_EMBEDDED_FIELD_OUTPUT` optionally exports the checked packet.

The complete scalar-edit history export now includes every copied nested
occurrence. Independent source primitive calls establish its physical ordinal;
tests validate both revision-bound handles and both containing outer identities.
The path must match the original and result occurrence. All 295 owned and 68
native contexts match. Two fresh processes match the complete 9,687,810-byte
oracle, including original images, edited fields, outer history and nested
parents. The native test requires `BESSEMER_THERMAL_NATIVE_NESTED_HISTORY_INPUT`
to name `target/simulation-thermal-native-nested-history.bin`;
`BESSEMER_THERMAL_NESTED_HISTORY_OUTPUT` optionally exports it.

The full local PR gate passed; the platform integration gates remain pending. Inner archive
versions and outer Simulation versions use separate typed acceptance rules.
Legacy 4281 requires its documented separate reader, while modern embedded
archives accept 4307 through 19000. Unknown bodies and unsupported optional
reference states must produce explicit errors.

Acceptance also requires owned populated transforms, threshold and malformed
cases, complete field and history projections, source isolation, concurrency
checks, two fresh-process exports, the local PR gate and all five CI checks.
The reference replay alone establishes neither native application acceptance
nor complete SolidWorks schema parity.
