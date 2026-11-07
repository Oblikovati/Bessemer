# UI bends

Bessemer reads and edits the three native bindings documented in section 11.244,
C-286: `moUiBend_c`, `moRuledBendLineData_c` and `moBendSketchLineData_c`.
The behavior version is `C286-ui-bends-v1`.

A UI bend retains its complete body-feature base. Saved helper state zero selects
one tagged single-bend pointer. State one selects a raw identifier, an i32 declared
line count, direct ruled-line bodies, a tagged sketch-line pointer and a direct
object array. Nonpositive line counts are preserved and read no lines. Positive
counts are bounded before iteration. Direct line records add no object tags or
map slots and do not serialize the native runtime handle.

Ruled-line bodies have twelve bytes. Sketch-line bodies have eight bytes through
archive version `0x42CB` and twelve bytes thereafter. The integer domains and the
engineering meaning of the stored double remain unresolved.

Independent fixtures contain 126 complete graphs across seven versions, with
33,215 primitive observations and 21 ordinary complete document streams. Twelve
alternate-state document streams cover minimum, negative, zero and positive
counts, selected through a fully decoded configuration manager. Scalar edits
have independent goldens, and rejected count edits preserve their input archives
and documents.

The independent reader's native regression test verified three containing flat
patterns with sixteen UI bends and two compressed raw kernel frames. Raw frames
continue to use Bessemer's existing strict kernel reader and explicit schema;
they have no suppression-byte prefix. No native alternate-helper fixture or
geometric-validity claim is made.

Two fresh native-corpus processes produced identical statuses and failure causes.
The complete-configuration count increased from 821 to 826, preserving every
previous success and leaving 80 explicit failures. Independent reference decoding
confirmed exact EOF and matching four-stream hashes for all 826 configurations,
totaling 56,589,343 resolved bytes. Native sample 407 now reaches an unsupported
`featureStateData_c` record and remains incomplete.

Run `cargo test --lib ui_bend_tests` for primitive oracles, exact line-body widths,
the version gate, finite reference roles and exhaustive truncation rejection.
Run `cargo test --test resolved_ui_bends` for complete documents, alternate saved
states, direct records, independent edits and count bounds. Run
`python -B docs/features/ui-bends/run_pinned_tests.py` to regenerate fixtures with
frozen copies of the actual `../inventor-interface/solidworks` readers.

The humanizer skill is unavailable in this environment; prose review remains
with the maintainer.
