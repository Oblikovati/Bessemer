# Feature166 acceptance audit

This index maps the original issue requirements to inspected implementation and
independent per-slice proofs. It is not a completion claim. The broader issue2
goal retains all documented carriers and unresolved cohorts.

Runtime source cut: `c6eff1f517c2b5dca6a19bec700d8e37c5371b0c`. The fresh full
Source replay passed338 checks in471.309seconds and exited after cleanup.
The consolidated Rust/native-fixture run passed355 tests across37 harnesses,
with zero failures or ignored tests. All34 exports matched their independent
process-proof hashes; [consolidated verification](consolidated-native-verification.json)
records exact scope,logs and the corrected exporter configuration. Repository-wide
`cargo xtask pr` also passed locally:3802 passed,zero failed,192 ignored across303
test summaries. The [repository gate proof](repository-pr-verification.json)
records the unchanged Rust/test trees and explicitly separates the ignored
cases from the complete native-fixture run. Publication gates remain separate.

| Specification | Local implementation and bounded scope | Evidence |
|---|---|---|
|14.386–390 |Complete parent grammar, caller/provider bindings and all staged original outcomes; [post_dialog](../../../src/simulation/persistence/post_dialog) |[proof](parent-operation-verification.json) |
|14.460 |Distinct PostInfo/PostStyle carriers and explicit helper/caller contexts; [post_info](../../../src/simulation/persistence/post_info), [post_style](../../../src/simulation/persistence/post_style) |[proof](poststyle-operation-verification.json) |
|14.461–463 |Fixed slots, primitive/direction views, ContourType policies and four-word comparison; [post_info](../../../src/simulation/persistence/post_info) |[proof](postinfo-named-view-verification.json) |
|14.464–465 |Raw RGBA channels, alpha roles and unnormalized option words; [rgba](../../../src/simulation/persistence/rgba), [post_render](../../../src/simulation/persistence/post_render) |[proof](rgba-operation-verification.json) |
|14.466 |Primary/alternate format construction with explicit runtime facts; [post_legend_format](../../../src/simulation/persistence/post_legend_format) |[proof](legend-format-operation-verification.json) |
|14.467 |Raw position factors and bounded pixel conversion; [post_legend_position](../../../src/simulation/persistence/post_legend_position) |[proof](legend-position-operation-verification.json) |
|14.468 |Palette counts, reversal and local dispatcher; [post_legend_palette](../../../src/simulation/persistence/post_legend_palette) |[proof](legend-palette-operation-verification.json) |
|14.469 |Height/cell/title/tick consumers; [post_legend_height](../../../src/simulation/persistence/post_legend_height) |[proof](legend-height-operation-verification.json) |
|14.470 |Yield-color selection and explicit split facts; [post_legend_yield](../../../src/simulation/persistence/post_legend_yield) |[proof](legend-yield-operation-verification.json) |
|14.471 |Step availability and local wrapping layout; [post_legend_step](../../../src/simulation/persistence/post_legend_step), [post_legend_step_layout](../../../src/simulation/persistence/post_legend_step_layout) |[proof](legend-step-layout-operation-verification.json) |
|14.472 |3D local geometry/call requests with explicit external inputs; [post_legend_3d](../../../src/simulation/persistence/post_legend_3d) |[proof](legend-3d-operation-verification.json) |
|14.473 |End-cell activation and raw RGB selection; [post_legend_end_cell](../../../src/simulation/persistence/post_legend_end_cell) |[proof](legend-end-cell-operation-verification.json) |
|14.474 |Width selector and exact nearest-even stages; [post_legend_width](../../../src/simulation/persistence/post_legend_width) |[proof](legend-width-operation-verification.json) |
|14.475 |Preserved counted text and separately assigned prefix; [post_text_controls](../../../src/simulation/persistence/post_text_controls) |[proof](text-selection-operation-verification.json) |
|14.476 |Explicit legacy hidden-parts migration stores; [post_hidden_migration](../../../src/simulation/persistence/post_hidden_migration) |[proof](hidden-migration-operation-verification.json) |
|14.477 |Version-gated derived zeros distinct from wire absence; [post_legend_resets](../../../src/simulation/persistence/post_legend_resets) |[proof](legend-resets-operation-verification.json) |
|14.478 |Optional raw locale-grouping control; locale rendering excluded by schema; [post_legend_grouping.rs](../../../src/simulation/persistence/post_legend_grouping.rs) |[proof](legend-grouping-verification.json) |
|14.479 |Exact role catalog, unknown purposes and original byte accounting; [post_style](../../../src/simulation/persistence/post_style), [post_info](../../../src/simulation/persistence/post_info) |[proof](field-model-verification.json) |
|14.480 |Lossy A8 zero tests and guarded local assignment; [post_legend_inverse](../../../src/simulation/persistence/post_legend_inverse) |[proof](legend-inverse-operation-verification.json) |
|14.481–482;14.524 |Packed-color projection, construction and alpha-preserving assignment; [rgba](../../../src/simulation/persistence/rgba) |[proof](colorref-import-operation-verification.json) |
|14.483 |Complete conditional style comparator, physical aliases and native-order checks; [post_style/comparison](../../../src/simulation/persistence/post_style/comparison) |[proof](style-comparison-operation-verification.json) |
|14.484 |Explicit masked-exception/DAZ-off color comparison; [rgba](../../../src/simulation/persistence/rgba) |[proof](rgba-comparison-operation-verification.json) |
|14.485–487 |Detailed raw legacy partitions and non-overlapping byte accounting; [post_style/legacy.rs](../../../src/simulation/persistence/post_style/legacy.rs) |[proof](poststyle-operation-verification.json) |
|14.489 |Bounded loaded legend comparison without inferred old runtime state; [post_style/legend_comparison.rs](../../../src/simulation/persistence/post_style/legend_comparison.rs) |[proof](legend-comparison-operation-verification.json) |
|14.490–491 |Layer suppression and model-tessellation request policies; [post_render](../../../src/simulation/persistence/post_render) |[proof](render-policy-operation-verification.json) |
|14.492–493 |Stored range/deformation views with raw float patterns; [post_dialog/views.rs](../../../src/simulation/persistence/post_dialog/views.rs) |[proof](parent-views-verification.json) |
|14.494–495 |Offset decision and fringe-option local mutation; [post_render/offset_fringe.rs](../../../src/simulation/persistence/post_render/offset_fringe.rs) |[proof](offset-fringe-operation-verification.json) |
|14.496–498 |Vector outer dispatch, complete tables and provider outputs; [post_vector](../../../src/simulation/persistence/post_vector) |[proof](vector-operation-verification.json) |
|14.499–501 |Unit-type model and explicit runtime/global flag facts; [post_units](../../../src/simulation/persistence/post_units) |[proof](unit-operation-verification.json) |
|14.502 |Local rendering predicate only; [post_render/local_predicate.rs](../../../src/simulation/persistence/post_render/local_predicate.rs) |[proof](local-render-operation-verification.json) |
|14.503 |Explicit raw initialization and local preference transfers/copy; [post_style_preferences](../../../src/simulation/persistence/post_style_preferences) |[proof](raw-preferences-operation-verification.json) |
|14.504 |Vector size/density/color controls; [post_vector_style](../../../src/simulation/persistence/post_vector_style) |[proof](vector-style-operation-verification.json) |
|14.505 |Extrema annotation controls; [post_annotations](../../../src/simulation/persistence/post_annotations) |[proof](annotation-operation-verification.json) |
|14.506 |Auxiliary triangle gates and colors; [post_aux_triangles](../../../src/simulation/persistence/post_aux_triangles) |[proof](aux-triangle-operation-verification.json) |
|14.507 |Hidden/model triangle color selectors; [post_triangle_colors](../../../src/simulation/persistence/post_triangle_colors) |[proof](triangle-color-operation-verification.json) |
|14.508 |Primary text and runtime seven-line controls; [post_text_controls](../../../src/simulation/persistence/post_text_controls) |[proof](text-selection-operation-verification.json) |
|14.509 |Plot54 callback color path; [post_plot54](../../../src/simulation/persistence/post_plot54) |[proof](plot54-operation-verification.json) |
|14.510 |Element-segment RGB requests; [post_segment_color](../../../src/simulation/persistence/post_segment_color) |[proof](segment-rgb-operation-verification.json) |
|14.511 |Mode3 RGB geometry candidates; [post_mode3_rgb](../../../src/simulation/persistence/post_mode3_rgb) |[proof](mode3-rgb-operation-verification.json) |
|14.523;14.525 |Bound initialized color100 literals and conditional preference color routes; [post_preference_colors](../../../src/simulation/persistence/post_preference_colors) |[proof](preference-color-operation-verification.json) |

The catalog preserves118 labels, including unresolved purposes, rather than
assigning meanings to every raw word. Original modern prefix qualifications,
standalone helper/caller distinctions, exact supplied versions and known
failure candidates remain part of the acceptance scope.

Required whole-feature checks:

- All224 original dialog identities,all120 standalone carriers and every original candidate remain in the fresh Source replay.
- Read/construction/edit coverage includes every supported value kind,atomic text,exact offsets,ordered nested history and unchanged unselected bytes.
- Independent owned literals,prefix/remainder rejection,exact/one-below budgets,configuration caps,no-op identity and every-field mutation expectations remain frozen.
- Input isolation,public Send/Sync assertions,eight-worker replay and two fresh-process comparisons remain required; a narrower consumer check does not replace the complete codec gates.
- The consolidated37-harness suite includes native fixture tests and supplies every exporter an explicit workspace-local destination. Its first attempt failed because a required exporter variable was absent; the configured rerun retains all tests.
- Exact publication-head `cargo xtask pr` and all five platform checks,including ThreadSanitizer,remain pending for feature166. Feature163 merged through PR173 after all five checks; feature166 remains queued behind164/165.

Clause14.488 is a separate unresolved DocumentInfo dispatch qualification. Its
known primary-version suffix uses the existing explicit full/partial
`SimulationPrimaryVersion` decoder in [version.rs](../../../src/simulation/persistence/version.rs).
This supplies no new handler for the short contact/mesh carriers.

The additional251 legacy-location dialog streams/275,470bytes remain separate
from the original224-stream denominator. Their complete retained candidates
are replayed by both Source and the ignored native Rust regression in
[simulation_post_dialog.rs](../../../tests/simulation_post_dialog.rs). All remain
rejected; no version is lowered to manufacture acceptance.

The distinct legacy plot carrier14.459/14.526 now has its own typed codec in
feature174, with [local operation acceptance](../simulation-legacy-plot/operation-verification.json)
and [process proof](../simulation-legacy-plot/process-pair-verification.json).
Its publication/platform gates remain separate from feature166. The
[EGR scope audit](../simulation-egr/scope-audit.md) confirms the existing bounded
implementation of14.514–522 and retains its native-fixture/cache/save limits.
MaterialList14.458/14.512–513 still lacks a bound historical body grammar and
is tracked in feature175. Neither is counted as completed by this feature166 audit.
Native application/writer equivalence,universal runtime version selection and
whole-format parity remain unproven.

Humanizer is unavailable; maintainer prose review remains pending.
