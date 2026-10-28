# Weldment finishing

Feature #54 implements the ordinary modern `moGusset_c`, `moWeldBeadFeat_c` and
`weldBeadSideData_c` bodies from 11.349 / C-391. Behavior version:
`C391-weldment-finishing-v1`.

The gusset reuses the complete extrusion serializer, including its tagged end
specification, double and signed-word suffix. Its continuation retains twelve
typed pointers and 52 raw scalar bytes, giving 76 bytes when all pointers are
null. The packed words preserve their upper bits as well as runtime bits 0/1.

The bead feature reuses the complete body-feature parent. In the explicitly
declared ordinary current-runtime dialect, helper `+834` is clear and the cabinet
pointer is serialized. The reader always consumes that field; it does not infer
another mode from the next bytes. Its remaining fields include a CString,
display-item reference, existing transform, packed word, shared side data, list,
five distance dimensions and two raw words. Side data has no additional
inherited body: three tagged list pointers precede two signed words and a double.
With null pointers it occupies 22 bytes.

Independent fixtures cover 396 graphs, 191,366 primitive observations and 21
owned streams across twenty-two inherited, cabinet and display version
boundaries. They exercise signed extrema, exact IEEE bits, packed upper bits,
all existing display-dimension classes, full extrusion suffixes, transform
operators, null/new/cached typed pointers, shared sides and lists, and
distinct/repeated roots. Three edit goldens change only a selected scalar.
Tests reject every truncated prefix, appended bytes, old contexts, missing
serialized cabinets and unrelated cached classes; public checks cover input
isolation after accepted and rejected edits.

Two fresh Bessemer processes matched all 906 outcomes and failure causes, with
no prior success lost. Together with the absolute-plane dependency, this
completed both targeted streams: 351,010 bytes / 43 roots and 371,730 bytes /
52 roots. The complete count increased from 819 to 821; 85 inputs remain explicit
failures. The independent frozen reader reached exact EOF for the same 821
inputs, totaling 55,590,941 feature bytes. Native evidence retains eight gussets,
eight bead features and sixteen side-data bodies.

Run `cargo test --lib weldment_finishing_tests`,
`cargo test --test resolved_weldment_finishing` and `cargo xtask pr`.
Regenerate with
`python -B -X utf8 docs/features/weldment-finishing/run_pinned_tests.py`.
This uses the shared immutable reference runner and checks actual reader origins;
it does not replace serializer functions.

Nonzero helper overrides, earlier/internal contexts, geometry validity, scalar
meanings, application acceptance and whole-schema parity remain incomplete.
Humanizer is unavailable; maintainer prose review remains open.
