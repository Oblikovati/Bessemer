# Containing node prefix

Section 11.245 selects the flatten-bend presence field from the containing
feature's inherited node flags. Looking up the last decoded node would allow a
nested object to select the wrong layout.

The existing `ModelNodePrefix` now returns through the feature, model-feature,
header and body readers. Existing callers discard it through their original
unit-returning interfaces. The same parsing functions read the same fields in
the same order; the refactor adds no archive reads or stored fields.

All owned fixtures were committed before this refactor. `cargo test --lib`
passed all 956 existing tests, with three external-fixture tests explicitly
ignored. No fixture or behavior baseline changed. The flatten-bend serializer
is added in a separate behavior commit.

The humanizer skill is unavailable; prose review remains with the maintainer.
