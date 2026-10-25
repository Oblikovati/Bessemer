# Float32 NaNs in verification traces

Bug #35 affected the scalar trace helper on the unmerged #28 feature branch.
Native float32-to-float64 conversion could quiet a signaling NaN, producing a
false mismatch with the Windows reference reader in the new version-boundary
fixtures. The archive codec retained the stored float32 bits correctly.

The trace now widens NaN sign and payload bits explicitly, preserving the
quiet/signaling bit. Other values use the existing exact widening. Direct IEEE
oracles cover both NaN signs and kinds, payload extremes, finite values, signed
zero, subnormals and infinities. No historical fixture baseline changed.

This is verification behavior only; it does not change archive bytes or the
stored numeric API. The maintainer-side humanizer skill is unavailable; its prose
pass remains for review.
