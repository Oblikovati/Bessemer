# Study-manager Simulation Material

Feature [#140](https://github.com/Oblikovati/Bessemer/issues/140) covers the
complete shared Material body in specification 14.347 and its retained contexts
in 14.348. This is a direct CwPersistStream body. It has neither a tagged archive
header nor the common load base used by MassElement Property.

The initial baseline retains both complete clauses, actual reader hashes and
all original C-723/C-724 reports. The study-manager denominator is 1,226 streams
and 4,278,214 bytes, with all 1,297 candidate attempts retained: 1,230 successful
attempts and 67 failed alternatives. Six AWDocument Material streams totaling
150 bytes have a distinct unresolved carrier and stay outside this grammar.
The earlier two mass-child identities overlap this study-manager corpus.

Fresh extraction now matches every retained native attempt, including exact
failure-prefix hashes and both earlier mass-child results in full. All 300 owned
profiles pass the unchanged actual reader, with explicit prefix and standalone
EOF checks. Six reference checks passed, including installed module/function
and floating-threshold bytes. The representative totals remain 8,712 properties,
18,390 delegated bodies and 18/38 populated property/delegated tables. Those
totals use the original first-success convention, not inferred runtime dispatch.
See the [reference foundation](reference-foundation-verification.json).
Rust operations and their full acceptance gates remain pending.

Two boundary cases need explicit owned fixtures. A nonzero property-table
presence consumes positive counted pairs; zero presence retains the declared
count without consuming point payload bytes. Applying a remaining-byte-per-pair
check to that absent payload would reject valid source shapes. Direct global
and delegated tables consume every counted pair, interleaved on the wire.

The actual Material reader also requests a decoded text view for every string.
Unpaired UTF-16 surrogate units remain intact. Odd-byte payloads are accepted
as raw frames and have no UTF-16 view. Bug [#141](https://github.com/Oblikovati/Bessemer/issues/141)
corrected an earlier design/verifier assumption that odd frames would fail.
The owned corpus includes those frames; Rust must retain their bytes without
normalizing them or inventing decoded text.

The reference also exposes `require_eof=False` and tests a complete Material body
followed by unrelated bytes. Support that explicit prefix boundary alongside
the standalone exact-EOF operation. A prefix result covers only the consumed
body; it must preserve the following bytes and keep body-byte limits separate
from the size of the enclosing carrier.

Implement every property, table group, delegated helper and historical tail in
physical field order. Preserve raw words, double bits, duplicate entries and the
temporary compatibility word. Native clamping, enum/default repairs and
material-model or unit interpretation are separate runtime actions. A retained
successful candidate does not establish native runtime version selection.

Full independent field/edit/history exports, malformed and exact-limit cases,
ownership, concurrency, fresh-process comparisons and local/platform gates are
required before integration. No target DLL, COM method or native writer has run
for this feature. Whole-object, application and geometric acceptance remain open.


Run `python -B -X utf8 docs/features/simulation-material/freeze_material.py` for
strict fresh replay from an immutable copy of the actual sources. The shared
reference-only dependencies and approved document/CFB adapters match the earlier
Simulation features. `--write` initializes evidence for review. Native packet
bytes remain under `target/`; the committed input/span fixture is independently
built. The [initial baseline](initial-reference.json) retains complete historical
clauses and reports, while [the text correction](text-framing-correction.json)
records Bug #141 and why the initial owned draft gained odd auxiliary strings.
