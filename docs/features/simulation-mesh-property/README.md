# Mesh Property interoperability (#163)

This feature implements the complete documented study `Mesh/Property` stream
from specification 14.378–14.381. The reference foundation precedes the Rust
codec; no Mesh Property API is published at this stage.

The retained reports cover the initial static layout (C-758), complete native
replay and imported settings dispatch (C-759), populated nested direct records
(C-760), and the newest integer-set suffix (C-761). The complete clauses and
original reports are frozen here. Earlier tentative map wording remains in its
original clause beside the later correction.

The verifier copies the approved `../inventor-interface` Python sources before
imports, checks their origins and hashes after the run, and preserves all native
candidate outcomes. It binds full installed image hashes, reader and constructor
functions, version constants, the Mesh storage literal, four embedded model ID
serializers, and the primary settings vtable's +80 slot. The slot points through
the exact thunk and IAT entry to
`CosUtilities.dll!CSWPersistReferenceContainer::Serialization`. No target code
executes.

First behavior profile 1 adopts the complete C-761 parent reader. Only the exact
C-759 and C-760 parent-reader hashes may reconcile to that adopted hash. The
earlier native reports retain every original field and failure. Their empty
newest-map count is compared with the corrected set count, removing only the
new zero-byte key span and the new summary field. Populated keys and nonempty
spans cannot use this historical reconciliation. The already reviewed general
`modelboundary.py` pin remains unimported; it supplies no callback or fallback.

The native denominator is 357 distinct retained stream occurrences and 94,876
logical bytes, with 391 candidate arguments: 357 accepted and 34 rejected.
There are 353 direct records. All eight native occurrences of the newest suffix
have zero keys; populated nested extensions and set keys have static and owned
reference evidence only. Versions remain explicit caller inputs.

Run the foundation with:

```powershell
python -B -X utf8 docs/features/simulation-mesh-property/freeze_mesh_property.py
```

Both initialization and strict replay passed all 29 checks: the 20 original
reference tests and nine foundation checks. Both processes exited successfully
after cleanup. [Foundation verification](foundation-verification.json) records
the source revision, packet and log hashes, and remaining acceptance scope.

Complete independent owned fields, extents, edits and history, the typed Rust
codec, malformed/cap/resource checks, isolation, concurrency, process export
comparison and integration gates remain pending. Native writer symmetry,
application opening, physical meanings, geometry validation, universal runtime
version selection and whole-format parity remain unproven.

Humanizer was unavailable; maintainer prose review remains pending.
