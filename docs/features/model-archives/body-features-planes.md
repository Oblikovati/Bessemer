# Body features and reference planes

Feature #44 adds the documented inherited body-feature layout and extrusion,
boss, cut, ICE, revolution, revolved-cut and rib suffixes. The shared reader
retains model-feature fields, nullable reference arrays, direct chooser bodies,
partition records, maps, appearance/name records and surface-ID children. It
preserves duplicate keys, chooser cycles, signed sentinel values and raw presence
words. End specifications reuse the existing direction and display-dimension
readers, with version-gated fields and conditional point/reference branches.

Reference-plane roots reuse that inherited layout without the body-feature
suffix. Nine registered plane-data classes cover common/fixed/default data,
face and face/point data, line/point and perpendicular data, angled data and
three-point data. Their direct plane references and transforms allocate no
outer map slot. Required curve references and present ID keepers reject null.
Unsupported new nested classes still fail the complete containing operation.

The silhouette-edge reference now has its complete documented body. It shares
the boundary/face/flag prefix with the existing silhouette-vertex reader while
retaining the two serializers' distinct field names and vertex-only final flag.
Earlier owned fixtures remain unchanged across this extraction.

The independent packet has 576 cases and 37,239 scalar expectations across
twelve versions. Every exact reconstruction, truncated prefix and trailing-byte
check passes. A body-word edit matches independently assembled bytes, reparses
with the expected fields and preserves the original. All cases survive source
overwrite; four parallel writers reproduce the same populated owned body.
Negative/impossible counts, present-null keepers and 48 null-required-curve
cases have focused checks.

The populated owned body charges 53 aggregate entries: four in its header,
nine reference entries, eight chooser entries, eight map entries, twelve late
entries, two body-tail entries and ten end-specification entries. The independent
53-entry budget passes and 52 fails. Fixed scalar members and object-map slots
have their separate field/object limits. All ten new tests, all 263 model tests,
all-target Clippy and all 51 Python tests passed. Three wire fixtures reproduce
exactly in two fresh reference processes.

Two final native runs agree at 893/906 across 829 document paths. Newly accepted
indices are 505 and 559. All thirteen remaining first stops are
`moProfileFeature_c`, replacing the earlier extrusion/boss/revolution stops.
This does not prove those containing feature objects completed. The complete
corpus gate still fails. Complete profile geometry, contours and nested
dependencies are next; public operation/history contracts, selected-configuration
diagnostics and native application/platform gates remain open. No geometry
evaluation or validity claim is added by these serialization readers.

Fourteen edition 0.356 clauses, fixture/source hashes and actual test logs are
bound in `body-feature-verification.json`. The full PR gate remains the earlier
geometric/kernel checkpoint and must rerun before push. Humanizer was unavailable;
prose remains for maintainer review.
