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

Typed local request/background arithmetic projections and remaining feature166
models are still outstanding, as are exact publication gates. Humanizer is
unavailable in this checkout; maintainer prose review remains pending.
