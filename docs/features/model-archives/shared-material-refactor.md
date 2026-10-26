# Shared material serializers

The owned blank part's model configuration and display cache store the same
845-byte `moVisualProperties_c` body. The independent reference locates it at
configuration bytes 108..953 and display bytes 3731..4576, both with schema 1.
Its SHA-256 is `f68056a29791658c34809a8b0a0f208a19a0996d8a0b056c8f6136542a541ac6`.
The reference completes both entire streams before accepting this comparison.
The second display appearance has a different body and remains a separate case.

Feature #44 therefore reuses the existing material serializers. The material,
DSPBR and user-property layouts moved from `display_lists` into `archive_codec`.
Their private `MaterialRead` trait supplies named primitives, unsigned version,
scope entry, collection budgets, remaining bytes and archive-specific failures.
The display adapter delegates each operation to the existing `DisplayDecoder`.
Field names, order, stored types, version predicates, limits and error context
remain unchanged. The serializers do not depend on display document flags or
external tessellation state. Implementors must be `Send`, matching operation
ownership requirements without adding shared mutable state.

This is a structural change. The display archive's public types and wire layouts
remain the earlier behavior; frozen owned roots and independent scalar tables
must pass unchanged. No expected-result file is regenerated to accommodate the
refactor. The next complete model reader will implement the same private trait
with its own field storage, budgets and model context.

The independent native comparison and source manifest are recorded in
`shared-material-native-reference.json` and `shared-material-source-hashes.json`.
Before/after hashes matched across 572 reference Python files. That evidence
establishes shared stored layout for the owned body, not complete model archive
support, geometry validity or native application acceptance. The full local PR
gate runs after the extraction; the model feature will also need its own final
gate and CI matrix before integration.

The extraction's full local PR gate passed, including all 204 doctests and the
frozen display field/byte comparisons. Final formatting, clippy and the native
header import test also passed after the `Send` bound was added. The original
configuration oracle contains 219 primitive reads covering the shared 845-byte
body exactly. A token comparison confirms all three serializer groups were
mechanically extracted with only reader, module and context-access substitutions.

The humanizer skill was unavailable; prose remains subject to maintainer review.
