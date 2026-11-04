# Ordered Inspection records

Feature #193 adds ordered lexical views and isolated edits for the standalone
Inspection layout documented in §14.16. The existing encrypted project envelope
and XML validator remain the starting point. This directory currently contains
preparation and independent reference tests; no new Rust record API is shipped.

`ordered.xml` is an owned literal fixture. It distinguishes the DOCUMENT container
from its nested records, preserves duplicate identifiers and property names, and
separates missing values from empty strings. Namespace and nested-child decoys
check that literal record views use the documented scope.

The selected native load projections in §§14.18 and 14.16.24 have a different contract:
they accept all direct element children and default absent selected attributes to
empty strings. Tests keep that projection separate from named record views. No
relationship dictionary insertion policy (§§14.19–14.20), numeric conversion, tolerance calculation, source
path access or application acceptance follows from these checks.

The populated tutorial test reuses the already accepted decoded historical XML.
It exercises the original `inspectionrecords.read` without requiring the current
AES decoder's optional Python dependency. Caller-supplied source comparison uses
the published `abc` MD5 vector as a checksum example, not authentication.

Before implementing the record API, extract the shared UTF-8 XML facade in a
separate behavior-preserving change and retain all Composer baselines. Then prove
one measurement-value edit through encryption, reopen and exact reversal before
extending the remaining record edits. Required gates include stale and foreign
identities, explicit limits, input isolation, concurrent reads and edits, fresh
process byte equality, unchanged Source acceptance, and the complete local and
remote checks. Application reload remains a separate requirement.
