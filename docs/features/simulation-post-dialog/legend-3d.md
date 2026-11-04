# Legend 3D body and background consumers

Specification14.472 and C-852 add consumers for existing signed words14 and BC.
Word14 controls body cells and boundary positions in `RenderLegend3DBody`;
wordBC affects background width in `RenderLegend3DBackground`. This evidence
adds no wire records or stored-byte coverage.

For ordinary valid positive cell counts, the body constructs count+1 pairs of
three-component positions spanning supplied width and using supplied height/count
for vertical spacing. The native method allocates temporary buffers and rereads
the count, including inside its vertex loop and before the mapping request.
Stored count alone establishes neither a safe allocation nor valid geometry for
zero, negative, excessive or overflowing counts. A future projection must keep
those read/allocator boundaries and floating-point assumptions explicit.

A separate plain integer rendering argument selects the mapping request and
initial drawing branch. Exactly2 requests count+1, uses smooth shading1D01 and
quad-strip primitive8. Other values request count, use flat shading1D00 and
quad primitive7. That argument is not assigned to a stored ContourType without
a bound caller. The later body code also calls `glShadeModel(1D01)` after its
label loop; initial branch selection does not establish final GL state.

The background builds a local COGLColor from its supplied packed argument,
passes that object to `GetColorfv` and supplies the resulting RGB buffer to
`glColor3fv`. Exact caller IATs, provider identity, constructor bytes and both
named exports are bound. The float getter's body is not independently modeled
by this evidence. Serialized EC or100 is not the color source read by this
routine; neither acquires a background-color label from the method's name.

PrecisionBC+10 multiplies a runtime character-width estimate before further
floating-point dimension conversion. Font metrics, view dimensions and other
arguments remain runtime facts. No actual pixels or floating-point output are
reproduced here. The background disables lighting for polygon primitive9 and
reenables it only when its earlier `glIsEnabled` result was nonzero. This does
not establish stored alpha-compositing semantics or restoration of all GL state.

GNU decoding matches all185 background and247 body instructions across1,831
bytes. Independent Ghidra captures match every instruction location and both
complete contiguous function extents. Exact exports, two original decompilation
hashes, packed-color calls and named OpenGL imports remain pinned. No decoder
qualification was needed. The temporary native analysis session closed without
executing target code.

All284 original rows remain retained:276 accepted contexts with276 count and276
precision observations, plus eight rejected streams. Original identities,
multiplicity, selected versions, modern prefix-only qualifications, absolute
offsets and raw signed bytes remain explicit. All425 complete owned contexts
retain actual spans, including twenty older opaque contexts lacking both named
words. Signed extrema and malformed scalar widths are tested without adding
rendering-validity rules to the wire reader.

The strict Source suite passed16 checks in5.189seconds and exited successfully
after snapshot recording and cleanup. All eight original catalog/style/color
unit tests remain included. The
[verification record](legend-3d-evidence-verification.json) retains exact logs,
snapshot hashes and remaining gates.

## Typed local requests and background inputs

`SimulationLegendHeightInputs::project_3d_body_requests` uses the entry count
snapshot for the earlier temporary allocation calculation. It requires an
explicit `SimulationLegend3DBodyRuntime::cell_count_at_mapping_raw` for the
later read; that value never falls back to the snapshot. The plain by-value
rendering argument is also explicit and supplies no stored contour identity.

The result retains low32 pair/float counts, the raw requested mapping count,
saturated byte requests and the bound initial/later GL-call arguments. Signed32
float counts are sign-extended before unsigned64 multiplication by4. Every
negative result overflows that multiply and requests u64::MAX; nonnegative
values multiply without overflow. These are computed request patterns only.
Zero, negative, excessive or wrapped counts do not acquire allocation or
rendering-validity approval.

`SimulationLegend3DBackgroundRuntime::project_background_nearest_even` requires
the packed argument, actual precision at its width read and raw GLboolean
lighting result in that order. It reuses the explicit nearest-even packed
constructor, computes low32 precision+10 and records the polygon/lighting
decisions. The constructed color is not an emulated float-getter buffer or a
stored-background-color view. All raw lighting bytes remain in history; every
nonzero value requests the later lighting-enable call.

Both immutable plans retain every supplied parent, derived history, completion,
exact local coverage, zero achieved pattern error and explicit native/geometry
exclusions. No buffer is allocated. Caller phase facts, native/helper completion,
float-getter output, geometry, floating dimensions and actual GL state remain
unvalidated.

The operation corpus covers26,784 boundary/presence/generated cases. Complete
parent replay retains425 owned and276 native contexts plus452 full count/precision
mutants, each with sixteen explicitly hypothetical body/background contexts.

All fourteen Rust checks passed, including complete native replay and eight-worker
comparisons. All eleven public types satisfy `Send + Sync`. Clippy passed with
warnings denied after the native unsigned overflow expression was written with
`saturating_mul`. The strict Source suite passed19 checks in7.375seconds and
exited successfully after snapshot recording and cleanup. Two fresh processes
matched all45,232 independent expected results, totaling2,171,136bytes each.
The [operation record](legend-3d-operation-verification.json) and
[process proof](legend-3d-process-verification.json) retain exact hashes and
remaining publication gates. The full integrated Source replay passed277 checks
in493.863seconds and exited with code0 after snapshot recording and cleanup.

Independent positive-count geometry and floating background-dimension behavior,
other required feature166 models and exact publication gates remain outstanding.
Humanizer is unavailable in this checkout; maintainer prose review remains pending.
