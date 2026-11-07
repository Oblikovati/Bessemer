# Legend end-cell evidence

Feature166's C853 evidence binds specification14.473 to the complete installed
2D legend consumer. GNU decoding and an independent Ghidra capture each cover
all1,638 instructions and the complete6,873-byte body. Neither executes the DLL.

The renderer reads the runtime plot pointer at legend+1A8 and tests its word524
before reading stored E8. A zero runtime word skips the stored flag read. Both
extension branches copy the RGB words at EC, F0 and F4. This does not establish
alpha compositing, an allowed-color set or an identity with color100.

The original284 candidate rows remain intact:276 accepted contexts and eight
rejections. The109 version-gated E8/EC pairs all have a zero flag. Their stored
RGBA encodings comprise101 red and eight gray values; no native active extension
was observed. The remaining167 accepted contexts lack the pair. All425 complete
owned contexts are retained, including312 with no pair. Missing words receive no
default, and all four stored color words retain their original bits.

The targeted replay includes signed flag boundaries, malformed-width rejection,
complete installed bindings, independent capture validation, all original native
identities, owned parents and all eight original catalog/style/color checks.
The [verification record](legend-end-cell-verification.json) records executed
gates and exact evidence hashes.

`legend_end_cell_inputs` exposes the optional signed flag and full raw color on
complete dialog and PostStyle snapshots. It retains absence and supplies no live
initialization defaults.

`SimulationLegendEndCellRuntime::project_end_cell` requires actual read-time
plot524 first, E8 only after a nonzero plot word, and EC only after a nonzero E8.
A zero gate completes without an RGB result. An unknown reached fact returns a
typed missing-dependency error. Active results copy the first three raw color
words without floating arithmetic; all supplied parents, including unused alpha,
remain in immutable derived history. Coverage records the reached reads, exact
local pattern checks and zero pattern error. It excludes pointer, geometry and
renderer validation.

The operation corpus crosses every presence combination, signed boundaries and
raw color classes, with generated cases. Parent projections retain all425 owned
and276 native contexts plus184 complete flag/color mutations. Eight explicitly
hypothetical contexts per parent include different later colors; the operation
never assumes that a stored value survives earlier native helpers unchanged.
The [operation record](legend-end-cell-operation-verification.json) and
[process proof](legend-end-cell-process-verification.json) record execution and
remaining gates separately from the evidence checkpoint.

The strict Source replay passed18 checks in7.508seconds and exited successfully
after snapshot recording and cleanup. Nine owned Rust tests passed, followed by
the complete native-parent test. Eight-worker results matched serial results.
Two fresh Rust processes matched all91,182 independent expected words byte for
byte. Clippy passed across all targets and features with warnings denied.
The full integrated Source replay passed285 checks in536.873seconds and
exited successfully after snapshot recording and cleanup.

Run the owned Rust checks with `cargo test --test simulation_legend_end_cell`.
Native replay additionally requires fresh packets from the pinned Source gate;
its test is explicitly ignored in an ordinary checkout. The full publication
gate, including ThreadSanitizer, remains separate from these local checks.

Independent geometry and floating-point projections, broader UI activation and
the remaining feature166 obligations remain pending. This adds no stored-byte
coverage credit and does not expand the original prefix-only admission scope.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
