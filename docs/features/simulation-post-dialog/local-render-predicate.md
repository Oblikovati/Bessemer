# Local rendering predicate

`SimulationLocalRenderInputs::passes_masked_exceptions_daz_off()` models the
C-884 IsSafeForRendering predicate from specification section14.502. It requires
both signed counts to be positive and rejects either position when it compares
less than zero. Under the declared masked-exception, DAZ-off assumptions, both
signs of zero, NaNs and positive infinity pass the position tests. Negative
nonzero values, including negative subnormals and negative infinity, fail.

The predicate has no palette allocation bound, finite-value check or upper limit
on either count. A true result establishes only this local predicate. It does
not validate an archive or establish that a renderer can safely consume it.

Parent and PostStyle `local_render_inputs()` methods read the exact four stored
words through the canonical field snapshot. They preserve position bits and do
not mutate bytes. Old styles without current position fields return a typed
missing-field result; no migration defaults are inferred.

The reference gate retains the original276 native input sets, their identities,
offsets and raw words. Independent cases cover18,177 signed-count and binary32
boundary combinations. Complete projections cover425 owned and276 native
parent/style contexts, including unavailable legacy state. Installed evidence
binds the full predicate and the parent reader's style-subobject call. These are
static bindings and model comparisons, not observed native execution.

Six Rust tests, all-target Clippy and all139 integrated reference checks passed.
Eight workers reproduced the serial predicate results. Two fresh processes
matched all18,453 owned/native results against Source byte for byte.
[Verification](local-render-operation-verification.json) records the exact
evidence, assumptions and exclusions. The local PR gate passed for the committed
predicate change. Publication still needs the required platform checks.
