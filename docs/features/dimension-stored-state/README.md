# Stored dimension state

[Feature #179](https://github.com/Oblikovati/Bessemer/issues/179) covers specification
clauses 11.436 and 11.437. The [archive-backed API](api.md) exposes documented
roles while preserving the existing signed dimension-base `mask` and
`unknown_5c` words and raw field paths.

The view exposes the bend-angle range flag (`mask & 2`), the stored
fillet reference-point change number (`(mask >> 2) & 7`), unresolved mask bits
(`mask & 0xFFFFFFE1`) and the signed redundant identifier. Stored zero through
seven are valid counter inputs. Every signed identifier remains valid. Native
range enforcement, increment transitions, units and handle resolution are
outside this stored-value operation.

The existing field paths and wire layout remain unchanged. Projection identifies
the dimension-base scope and physical typed neighbors in the selected
archive revision; unrelated tolerance masks and other `unknown_5c` fields must
fail selection. Edits use the existing field-edit validation and history,
preserve unselected bits and fields, and reject stale or foreign views. No-op
edits must preserve identity. Both configuration and resolved-feature entry
points need tests using complete archive carriers.

`freeze_dimension_state.py` copies the complete current source tree before
importing any original reader. It runs all nine original related test modules,
plus separate checks for the twelve current reader/test pins, both complete
schema clauses, five complete historical reports and the owned complete-carrier
oracle. Native report dependency
pins remain strict. Historical replay readers and accounting totals remain
historical evidence; the harness does not invoke report writers, patch original
tests or adopt their old source revisions as current behavior.

The asset manifest pins the original tests' file-relative reports, blank-part
fixture, CFB adapter and existing key cache. These copies keep test behavior
unchanged when the Python sources move into the temporary root. The harness
uses the existing local PE reader for the original installed-corpus tests.

The installed-image check matches the complete 45,766-name export map, nine
recorded ranges totaling 3,341 bytes and the typed redundant-ID resolver slot.
Seven ranges have named entries; the constructor and copy ranges are internal.
Both the export-count cap and the default 512-byte symbol-name preview need
explicit handling: names are read from their physical PE offsets with an
8,192-byte termination bound. These are ordinary static observations using the
existing PE reader, not fresh REA Evidence. No CAD code is executed.

The owned corpus contains 5,440 complete feature streams across 17 dimension
classes: direct, geometric, reference, valued-sketch and offset dimensions.
Each stream inherits the original 97-slot configuration map, stores two physical
definitions through new and loaded class tags, and ends with an instance alias.
The original reader confirms all 10,880 definitions, 5,440 aliases and final
100-slot maps. Cases cover all sixteen flag/counter combinations, four unrelated
bit patterns and five signed identifier limits. Optional Unicode text retains
an embedded NUL and a noncanonical presence byte. Geometric bodies retain
negative zero and a NaN payload. Reference dimensions exercise a nested base
scope. Six additional reader pins protect these derived bodies and contexts.

The literal 1,264,320-byte packet includes 1,242,560 stream bytes. The original
primitive observer independently partitions every stream byte into 318,400
reads and checks 54,400 signed scalar spans at literal offsets. Its mask reads
pass through the original unknown-field helper; the oracle preserves that
attribution. `dimensions.tsv` records class, raw unsigned mask, signed redundant
identifier, both physical base offsets and stream length. `primitives.tsv`
records case index, byte start/end, primitive kind and exact bytes. The binary
packet prefixes each complete stream with its little-endian u32 length.

The [owned verification record](owned-verification.json) retains the completed
114-check initialization and strict rerun, their logs and the final immutable
source snapshot. Both processes exited successfully and were reaped. The strict
run checked all existing fixture bytes without baseline-write mode. The earlier
113-check foundation record identifies its artifacts by Git revision so later
source snapshots do not silently replace that evidence.

The complete configuration fixture adds two dimension definitions and an alias
to the original absent object map, updating the resolved section extent. Its
independently read primitive partition covers all 24,137 bytes. Adding objects
changes inherited slots, so its companion feature stream is an independently
validated ten-byte empty list with the same 100-slot context. Document tests
read both archives, preserve unrelated streams and reverse edits byte for byte.

The [operation record](operation-verification.json) records ten passing
integration tests, four projection guards, nine documentation examples and clean
all-target Clippy. All 163,200 selected-value requests match independently
calculated complete images, unselected fields and ordered history: 131,648
change a value and 31,552 are exact no-ops. Every remaining u8 counter request
fails explicitly. Both public types pass compile-time Send/Sync checks. Eight
workers share immutable snapshots through 43,520 accepted edits and 43,520
expected failures. Serial and eight-worker projections match in full; two fresh
processes produce identical 66,235,220-byte view/field/edit/history exports.

The native gate projects 5,920 configuration dimensions and 76,318 feature
dimensions while reconstructing both carriers for all 906 cases. The original
private/public reconstruction gate also passes. Every feature count binds by
stream identity and complete hash to its original per-stream C-1923 identifier
gain; [native bindings](native-bindings-verification.json) retain every row.
Historical accounting is unchanged. Local repository and exact-publication-head
platform/ThreadSanitizer gates remain required; this operation does not claim
whole-format parity or native application/geometry acceptance.

Run the source checks from the repository root:

```powershell
python -B -X utf8 docs/features/dimension-stored-state/freeze_dimension_state.py
```

The humanizer skill is unavailable in this checkout; its prose pass remains for
maintainer review.
