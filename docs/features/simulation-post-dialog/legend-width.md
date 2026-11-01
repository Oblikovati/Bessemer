# Legend eDrawings width evidence

Feature166's C854 evidence binds specification14.474 to the complete installed
eDrawings legend renderer and the separate2D title routine. GNU decoding and
independent Ghidra captures agree on all1,235 instructions and4,974 body bytes.
The exact exports, complete decompilation hashes and three width constants are
retained. No target code executes.

The renderer converts signed word68 from its supplied font object to binary32,
then multiplies by constant bits`0x3a83126f`. It multiplies that result by
`0x3f200000` to form the initial width. Stored B4=1 adds this width to itself;
B4=0 multiplies it by`0x40800000`; other raw values keep it. The evidence records
these native operations without assigning enum names, pixel units or a rule for
other renderers. Floating output depends on the supplied font and SSE behavior.

The font object's word64 supplies the packed-color constructor. The title
routine likewise receives its packed color and font arguments explicitly.
Neither path establishes an identity with unresolved stored legend color100.

All284 original candidate rows remain intact:276 accepted contexts and eight
rejections. There are262 stored B4 words equal to1 and fourteen equal to2. No
native sample reaches the zero branch; its role is established statically.
All425 complete owned parents remain in replay, including twenty earlier opaque
contexts without a B4 view. Unknown values and absence receive no normalization
or defaults. Original prefix-only and candidate-version qualifications remain;
this evidence adds no stored-byte coverage credit.

The [verification record](legend-width-verification.json) records targeted and
full replay gates and exact captures. The typed views and local numerical
projection are described below. Renderer execution, geometric validation and
the remaining feature166 obligations remain pending.

The targeted freeze passed16 checks in5.557seconds. Strict replay passed the
same16 checks in5.142seconds and exited successfully after snapshot recording
and cleanup. Both retain all eight original catalog/style/color tests. Full
integrated replay and publication gates remain separate obligations.

## Numerical projection

`legend_width_inputs` preserves optional signed B4 on complete dialog and
PostStyle snapshots. The separate
`SimulationLegendWidthRuntime::project_width_nearest_even` operation requires
actual signed font68 and B4 reads. Font conversion and the two initial
multiplications occur before B4 is read, with no intervening helper call.
Missing facts fail in that order, including a missing B4 when the font is zero.
The stored view supplies no font state or earlier-profile selector default.

For signed32 font inputs, every nonzero intermediate magnitude is normal and
finite. The [domain proof](legend-width-domain-reference.json) retains closed
representable bounds and the 48-bit significand-product bound. The final
doubling and multiplication by four are exact power-of-two scalings in this
domain. Results retain their distinct native operation identities and every
intermediate binary32 pattern. Completion distinguishes zero from signed
nonzero output; a negative width is a local numerical result, without physical
or geometric validation. Immutable history retains both supplied parents.

The independent oracle rounds exact rational values between neighboring
binary32 encodings, including ties to an even significand. Production reuses
the existing integer nearest-even significand primitive and performs no host
floating arithmetic. The corpus covers 18,040 cases, including 498 signed font
boundaries around powers of two, conversion midpoints and signed32 limits.
All presence combinations and three B4 branches remain distinct. Complete
parent replay retains 425 owned contexts, 135 full selector mutations and
276 native parents with eight explicit hypothetical entry contexts each.

The [operation record](legend-width-operation-verification.json) and
[process proof](legend-width-process-verification.json) retain exact checks and
remaining publication gates. The declared nearest-even policy does not establish
the live process's SSE state, pixel dimensions or a universal physical unit.

The strict operation suite passed 19 checks in 8.377 seconds and exited
successfully after snapshot recording and cleanup. Nine owned Rust tests and
the complete native-parent test passed, including eight-worker comparisons.
The existing color-conversion and preference suites passed unchanged after
reuse of their rounding primitive. Clippy passed for all targets and features
with warnings denied. Two fresh Rust processes matched all 199,496 independent
expected words byte for byte. Full integrated replay and publication validation,
including ThreadSanitizer, remain separate gates.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
