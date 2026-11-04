# Legend height cell count and local product

Specification14.469 and C-849 bind serialized `legend.word_14` as a signed32
cell-count multiplier. Its absolute wire span follows the selected versioned
legend profile. The raw count remains unchanged for every signed value; older
absent words remain absent. Runtime font metrics and requested tick counts are
separate inputs to their respective native consumers.

The native total-height method copies its supplied LOGFONTW and calls the
cell-height helper before executing `imul eax, [rbx+14]`. The returned result
uses the low32 product bits. The helper reads the supplied font's first signed
member, converts it to binary32, clears its sign, doubles it and truncates to
signed32. That helper's exceptional conversions depend on the SSE environment.
The stored count alone supplies no font metric or valid pixel-height claim.

The title method calls the previously bound upper-left consumer, preserves its
horizontal result and adds a truncated font-derived vertical offset using exact
binary32-2.0. The tick method receives its requested count through a separate
argument. Counts below2 and allocation failure return null. Otherwise it builds
a preliminary range array; nonzero D4 then calls `AdjustValuesAtTick`. That
preliminary array does not establish final adjusted tick values. The regular
color mapper also has a conditional256-level use of word14+1, after earlier
mapping guards. These relations do not establish a universal tick-count identity
or a count-sized wire palette.

The installed verifier checks all152 instructions across the complete tick,
title, total-height and cell-height ranges,583 bytes in total. It verifies exact
exports, helper targets, decompilation pins, all16 sign-mask bytes and the title
scale. Independent Ghidra captures match every instruction location and all four
contiguous extents. The temporary analysis session closed; no target code
executed. Original source reports and probes remain unchanged.

The corpus retains all284 original rows,276 accepted count observations and
eight rejected streams with their selected-version and prefix/EOF scopes.
Ninety-two native observations store10 and184 store12. All425 owned complete
contexts are retained, including twenty older opaque count absences. No new
stored-byte coverage is claimed.

## Typed views and final-instruction projection

`SimulationPostDialog::legend_height_inputs` and
`SimulationPostStyle::legend_height_inputs` return
`SimulationLegendHeightInputs`. `total_height_from_cell_height` requires an
explicit optional signed32 helper result, then the stored count, matching the
native dependency order. An unknown helper result fails even when the count
is zero. An unknown count fails even when the helper result is zero.

Completed `SimulationLegendHeightPlan` values retain both original scalar
parents, a derived history relation, zero/nonzero product completion, exact
coverage and zero achieved local pattern error. `total_height_raw` returns the
signed32 pattern without clamping, including negative and overflow results.
Plans expose no mutation path. Font conversion, title coordinates, tick arrays,
pixel validity and actual native execution remain explicitly outside this local
integer operation.

Independent expectations use unbounded integer multiplication followed by a
32-bit mask. They cover8,864 signed-boundary/generated/missing-input cases,
425 owned and276 native complete parents and268 full-body count mutations.
Every parent receives five explicitly hypothetical helper contexts. Archive
bytes, repeated getters, field absence and source isolation remain checked.

All seven Rust checks passed, including native inputs and eight-worker
comparisons; all six public types satisfy `Send + Sync`. Clippy passed with
warnings denied. The strict Source suite passed18 checks in6.275seconds and
exited successfully after snapshot recording. Two fresh processes matched all
13,709 independent expected results, totaling219,344bytes.
The full Source suite passed248 checks in525.047seconds and exited successfully
after snapshot recording.
The [verification record](legend-height-operation-verification.json) and
[process proof](legend-height-process-verification.json) retain exact hashes,
coverage and remaining full/publication gates. Remaining documented feature166
models and exact platform integration remain outstanding. Humanizer is
unavailable in this checkout; maintainer prose review remains pending.
