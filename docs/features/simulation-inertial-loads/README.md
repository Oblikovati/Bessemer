# Binary inertial and base-excitation collections

Feature [#136](https://github.com/Oblikovati/Bessemer/issues/136) covers complete
Gravity, CentriFugal and BaseExcitation collections. Implementation follows the
thermal acceptance gate in #134. No new public surface is present yet.

The initial baseline retains complete clauses 14.338 and 14.341, hashes of the
actual readers, and every native report row and candidate outcome from C-714
and C-717. Fresh extraction through the actual document/CFB adapters recovered
all 18 streams and 17,928 bytes, checking envelope and nested hashes. Every
retained outcome matches both the original callback and the full actual C-716
embedded registry. All 18 retained candidate attempts succeed; their complete
input/span packet is 24,304 bytes.

Another 135 independently built two-record cases cover every shared and derived
threshold and neighbor, populated optional bodies and both nonnull embedded
profiles. The full actual readers accept each complete image, including the
high-bit unsigned BaseExcitation header. Their input/span fixture is 481,704
bytes. These are reference checks; Rust implementation remains pending.

Reuse the shared load base and explicit embedded profiles from #134, preserving
every existing thermal field, identity, byte and public API. Keep any structural
extraction separate from the new derived grammars. Gravity and CentriFugal add
their word/string/double groups and the 800037 tail gate. BaseExcitation retains
its high-bit unsigned header word and the 820012 three/four-string and mixed-tail
gate. The older BaseExcitation branch has synthetic/static evidence only.

Run `python -B -X utf8 docs/features/simulation-inertial-loads/freeze_inertial.py`
for strict replay from a consistent copied source tree. The reference-only
dependency setup is shared with #134. `--write` creates initial baselines for
review; strict runs compare all existing evidence and fixture bytes.

Next verify Rust exact construction,
isolated edits, full physical fields and histories, malformed inputs, explicit
budgets, ownership, concurrency and fresh-process exports. Require the local
PR gate and all five exact-head platform checks before merging.

Native runtime dispatch, study-index association, model conversion, unit
semantics, DLL/COM execution and native application acceptance remain outside
the byte-framing evidence. This feature does not complete installed-schema parity.
