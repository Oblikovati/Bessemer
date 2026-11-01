# Hidden-parts and model-tessellation triangle selectors

Section14.507 and C-889 identify two selector words and six RGB roles. The
original complete RGBA groups remain authoritative, including the alpha roles
already documented in14.465. These controls select local helper arguments;
they do not establish a UI option name, invocation or successful rendering.

The evidence replay retains all276 original identities, candidate versions,
selector bytes, selector offsets, full RGBA bits and conditional observations.
It reparses complete parent/style bodies through exact EOF and matches the
original record multiset. Word1F0 is0 in276 records; word230 is1 in221 and0 in55.
These frequencies do not impose file validity restrictions. All425 owned
version contexts also parse through exact EOF:405 contain both selectors and20
contain neither. Missing selectors are recorded as missing without defaults.

The historical color-report hash changed from
`0bde69f7e89ffa46838afddb2b31eafb615f7e5a5a1f53b8a4bf16542e67f63d` to
`97f4f83c3dad71727f9d6edb209acc776753680d74ea770c2b7edc349a1e415d`.
Only that exact transition is accepted. Every original selector/color input
and derived observation is independently replayed from complete bytes. All
other eight original source pins must match. Original caller/helper bodies
and instruction listings are retained in the C889 reference artifact; this
source replay alone does not rematch the installed machine code.

The final strict targeted suite passed11 checks in5.741seconds, including all
current catalog tests. Its process exited successfully after snapshot recording.
The [evidence proof](triangle-selector-evidence-verification.json) records exact
revision, log and snapshot hashes, scope and outstanding gates.

## Caller and helper contract

The hidden-parts caller requires a nonzero result from named
`HiddenPartsOptionOn`, unsigned1F0 below2, and core678 equal0 for this helper
branch. It passes third argument1 and fourth argument1F0 equal exactly1.
The other core678 branch follows different rendering paths.

The model-tessellation caller first excludes renderer type5 and requires a
zero result from helper1800FF240. Its nonzero21C branch passes third argument0
and fourth argument230 equal exactly1. Other230 values use supplied RGB on
that branch; other caller branches remain separate.

Helper1801578E0 uses core2520, group count40C and array410. It filters group
flag8 by third-argument zero/nonzero class. Runtime flag8C8 adds a lookup/end
comparison and chooses group color+C or+1C when per-group RGB is selected.
Only positive signed triangle counts at+2C reach slot58; payloads+30/+38
remain opaque. The group stride is184 bytes. Raw high-bit group counts can
pass the nonzero outer check while admitting no signed iterations.

This helper does not inherit the auxiliary pass's plot51 or renderer3-through5
rejections. Its MINSS alpha cap at180157981 also has a broader possible caller
domain. Before publishing a full plan, explicitly specify floating-point
assumptions and build an independent oracle covering signed zero, subnormals,
infinities and NaNs.
Do not extend the auxiliary caller's admitted-alpha shortcut to this helper.

The [Intel instruction reference](https://cdrdv2-public.intel.com/835752/253667-sdm-vol-2b.pdf),
order253667-085US, pages4-31 and4-32, specifies selection of the second operand
when a source is NaN. Here that second operand is the stored constant1. With
floating-point exceptions masked and DAZ/FTZ disabled, the planned model must
therefore return1 for either-sign signaling or quiet NaN, retain negative
infinity and values below1, and cap positive infinity and values above1. The
model must retain signed zeros and original subnormal patterns. Those are
declared operation assumptions; the file bytes do not establish native MXCSR.

Independent Ghidra12.1.4 captures retain all81 hidden-caller,132 helper and92
model-caller instruction locations and the four constant bytes. Evidence IDs
and provider limitations are preserved in the `triangle-selector-rea-*.json`
records. The source binary was not executed or edited, and the ephemeral
analysis session was closed. The reproducible GNU verifier additionally checks
the complete function bytes and instruction operands, named accessor, exact
color imports and concrete triangle vtable slots. Both new native/capture checks passed in the strict11-check gate. The earlier
full suite passed168 checks in504.026seconds at the evidence-only cut2063B1B7;
its denominator excludes these two new checks. The updated full gate remains
required.

`hidden_parts_triangle_controls()` and `model_tessellation_triangle_controls()`
read canonical parent/style snapshots without initializing missing older fields.
`SimulationTriangleColorControls` retains the caller kind, original selector,
RGBA and, for the model path, the stored21C override. All425 owned and276 native
complete projections retain raw archive bytes;20 older owned groups are missing.

`plan_masked_daz_ftz_off` accepts matching typed caller facts, explicit helper
runtime facts, complete group snapshots and a maximum-group budget. Unknown
external results remain `None`. It requires each result only when reached.
Lookup runs after the group-class test and before the positive triangle-count
test. A missing lookup outcome is therefore required even for a nonpositive
triangle count when the class matches and runtime8C8 is nonzero. An earlier
class rejection needs no lookup result.

Plans distinguish disabled/bypassed branches, other caller paths, helper
rejections, admitted setup and completed empty/call results. They own the
inspected original groups, controls, runtime context and ordered local requests.
Each request retains the group index, opaque payload identities and exact RGB
origin. Supplied alpha is independent of the selected group alpha. Coverage
records signed iteration, reached lookup count, budget, exact pattern error and
explicit floating-point assumptions. Native execution, payload validity, other
caller paths and scene traversal exclusions accompany every successful plan.

Budget overruns, unavailable required groups, unknown reached external results
and mismatched caller kinds return typed errors without partial successful
output. A high-bit raw count can admit setup while inspecting no groups.
No plan dereferences payload identities or publishes geometry.

An independent pointer-offset oracle covers25,436 scenarios with both caller
gates, zero/nonzero and unsigned selector boundaries, class/lookup/count order,
both RGB origins, resource limits, missing facts and1,547 alpha patterns. Tests
also cover source mutation after publication, both typed caller mismatches,
eight-worker replay, public Send/Sync traits and complete archive projections.
All7 Rust integration checks and the allocation-error regression passed.
Clippy passed with warnings denied. The strict Source suite passed13 checks.
Two fresh processes matched26,137 contexts and1,505,328 output bytes against
independent expectations. The [operation proof](triangle-color-operation-verification.json)
records revisions, log hashes, exclusions and remaining gates. The expanded
full Source suite and publication gates remain pending. Complete scene rendering remains outside this documented local
binding; all other named schema models and publication gates remain required
for feature166.
