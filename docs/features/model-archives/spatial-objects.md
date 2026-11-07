# Spatial model object bodies

The private dispatcher reads the complete fixed bodies of `gcCurvatureObject_c`
and `moTransRefPlaneData_c` from reference section 11.20. Curvature values retain
their double bits; timestamp words remain separate stored integers. Reference
plane fields retain their signedness, including the unsigned back-face color.

Independent wire cases cover signed zero, a signaling NaN payload, the smallest
positive double subnormal, negative integers, and high-bit unsigned words. The
sibling reference consumed both records to EOF and confirmed the scalar values.
`spatial-object-verification.json` records its source hash and the wire hashes.
Rust tests verify exact values, typed reconstruction, every truncated prefix,
trailing bytes, and failure publication after exhausting the field budget.

These are object-body checks. The surrounding lightweight section, resolved
section, complete model construction, and native application acceptance remain
unfinished under #44. No public model API is exported. The humanizer skill is
unavailable in this environment; its prose pass remains for maintainer review.
