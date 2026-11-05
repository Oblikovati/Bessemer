# Direct boundary data

Feature #205 replaces two explicit ordinary-boundary stops with complete typed
bodies from specification clauses 11.393 and 11.394. It uses the existing model
and resolved-feature read, construction and edit APIs; it adds no public API.

Temporary chooser records contain two direct raw `u32` time words each. Negative
declared counts remain stored and skip the loop. No runtime time-object width,
calendar conversion or sentinel widening changes the wire values. The existing
suffix remains gated above `0x332C`.

Every nonzero direct-data presence word selects ordered signed key/value pairs,
an integer vector and a trailing signed scalar. Negative counts skip only their
loops. Duplicate keys and zero vector entries remain in their original order;
the parser does not apply the native loader's map/vector changes. Two additional
signed values occur from `0x3359`. Direct bodies allocate no MFC map slots.
Existing zero-body field paths, `unknown_ec` representation, archive modes and
version admission are unchanged.

The independent oracle contains 78 complete bare cases across both gate
transitions and 17 complete owned streams. It checks signed extremes, raw time
bits, duplicate negative keys, zero entries, noncanonical presence, exact EOF
and map identity. Every bare prefix and impossible temporary count fails. A
pre-change public regression failed at the explicit negative-count stop; its
log remains retained.

The focused Rust suites read every golden and force typed reconstruction. Seven
independent expected edits reopen complete documents and reverse from fresh
snapshots to the original file bytes. The private cases also check all prefixes,
extra bytes, old-context rejection, and exact field/positive-entry budgets.
The 48 historical boundary fixtures and their 412 expected fields remain
unchanged. Broader validation and exact revisions are recorded separately.

All three public tests and all six private boundary tests passed. Library and
integration Clippy, formatting and strict original-reader regeneration passed.
The corrected reader reconstructed all 906 retained native cases through forced
typed construction. `local-verification.json` binds the source pins, log hashes,
implementation files and exclusions. Full PR/platform validation and serial
integration remain pending.

These owned cases do not add installed positive-body population, native
application or geometry acceptance. Source reports zero temporary entries and
zero present downstream bodies in its retained installed checkpoint. Unknown
time and identifier meanings remain unknown. Full schema parity remains open.

```text
python -B -X utf8 docs/features/boundary-direct-data/freeze_reference.py
cargo test --test model_boundary_direct_data
```
