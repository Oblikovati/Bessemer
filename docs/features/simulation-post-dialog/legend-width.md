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
full replay gates and exact captures. Typed width views, explicit floating-point
projections, renderer execution and the remaining feature166 obligations remain
pending.

The targeted freeze passed16 checks in5.557seconds. Strict replay passed the
same16 checks in5.142seconds and exited successfully after snapshot recording
and cleanup. Both retain all eight original catalog/style/color tests. Full
integrated replay and publication gates remain separate obligations.

## Next numerical projection

An explicit nearest-even projection can use the actual signed font68 and B4
reads as separate inputs. Font conversion and the two initial multiplications
occur before B4 is read, with no intervening helper call. Missing facts must
follow that order. The stored view must not manufacture font state or an earlier
profile's missing selector.

For signed32 font inputs, every nonzero intermediate magnitude is normal and
finite. The final doubling and multiplication by four are exact power-of-two
scalings in this domain. A projection should still retain their distinct native
operation identities. It should return each intermediate binary32 pattern,
explicit rounding policy, complete input history and local error/coverage,
without claiming pixel dimensions or the live process's SSE rounding mode.

The independent oracle should round exact rational values between neighboring
binary32 encodings, including ties to an even significand. The production model
can reuse the existing integer nearest-even significand primitive. That keeps
the oracle independent of its integer bit calculation and avoids ambient
floating-point state. Boundary cases should include signed extrema, integer
rounding transitions around powers of two, generated signed patterns and the
three distinct B4 branches.

Humanizer is unavailable in this checkout; maintainer prose review remains pending.
