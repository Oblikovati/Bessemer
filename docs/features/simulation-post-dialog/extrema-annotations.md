# Min/Max annotation controls

Specification section14.505 and C-887 bind style234 to the minimum annotation
and style238 to the maximum. Both are signed raw words tested against zero.
Every nonzero value requests its own annotation; the file grammar does not
restrict either control to0/1.

Parent and PostStyle `extrema_annotation_controls()` queries return
`SimulationExtremaAnnotationControls` through their canonical snapshots.
Missing older fields return a typed missing-field error, with no initializer
or legacy migration fallback. Original words and archive bytes remain intact.

`minimum_requested()` and `maximum_requested()` preserve the two independent
branches. `needs_value_position_sets()` models the request for the helper that
supplies both sets. It does not compute extrema, select coordinates or equate
the results with user-selected legend ranges.

`glyph_material_adjustment_requested(render_argument, plot_object_present)`
models the separate material block: the argument must be nonzero, the supplied
plot pointer must be present, and at least one annotation control must be
nonzero. The two following glyph helper calls are outside that block. A false
material predicate does not establish that glyph rendering is disabled, nor
does a true predicate establish rendering success or current OpenGL state.

Static evidence retains seven complete consumer/helper functions, the named
glyph export, independent guard paths and all four UTF16LE literals and actual
RIP-relative references. These bind `Min`, `Max`, `Min: ` and `Max: ` to the
corresponding branches. Resource-derived labels and complete number formatting
remain separate from these fixed fallback observations.

The pinned report has three COMISD qualifier discrepancies recorded in
[bug169](https://github.com/Oblikovati/Bessemer/issues/169). The verifier retains
the original report and records exact original/canonical spellings and bytes.
Only those three byte-checked cases are qualified. Arithmetic-equivalent zero
displacement and unit-scale address spellings use an explicit comparison mode;
nonzero displacements, other scales, method hashes and complete bytes stay
strict. Focused checks reject changed captures and unknown qualifier addresses.

Native replay preserves all276 original pairs:250 both zero,24 both nonzero
and2 maximum-only. The absence of minimum-only pairs does not prohibit them.
Complete view fixtures include425 owned contexts with405 present and20 missing
pairs, plus276 native contexts with both fields present. Reviewed truth tables
cover43,350 policy cases across signed control boundaries, five signed render
arguments and both plot-pointer states. The explicit C-885 preference-copy
model composes with these later consumer roles without changing stored bytes.

Seven Rust tests, all-target Clippy and thirteen focused reference checks passed.
Eight workers matched serial policies and all425 owned views. Two fresh
processes matched all44,051 policy/archive contexts and177,606 output words
against the independent expectations. All156 integrated reference checks also
passed. [Verification](annotation-operation-verification.json) records exact
evidence, the decoder qualification and remaining integration/publication gates.
No native execution, extrema computation or global semantic coverage credit is
claimed.
