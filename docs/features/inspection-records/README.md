# Ordered Inspection records

Feature #193 adds ordered lexical views and isolated edits for the standalone
Inspection layout documented in §14.16. The existing encrypted project envelope
and XML validator provide the carrier boundary. `InspectionRecords` retains the
envelope and indexes sheets, nested document records, groups, characteristics,
measurements and custom properties in physical XML order.

Each occurrence has a revision-bound identity and its nearest named owner.
Stored Uid and Name strings remain ordinary attributes; duplicate values do not
merge occurrences. `edit_attribute` replaces an existing unqualified attribute
through the shared XML lexical engine. Its result retains both immutable
snapshots, completion, exact XML coverage, semantic exclusions and one source
parent for every result record. Only the selected record's relation is derived.

`ordered.xml` is an owned literal fixture. It distinguishes the DOCUMENT container
from its nested records, preserves duplicate identifiers and property names, and
separates missing values from empty strings. Namespace and nested-child decoys
check that literal record views use the documented scope.

The selected native load projections in §§14.18 and 14.16.24 have a different contract:
they accept all direct element children and default absent selected attributes to
empty strings. Independent reference tests keep that projection separate from
named record views; a Rust native-load projection remains unfinished. Relationship
dictionary insertion (§§14.19–14.20), numeric conversion, tolerance calculation,
source path access and application acceptance remain separate requirements.

The populated tutorial test reuses the already accepted decoded historical XML.
It exercises the original `inspectionrecords.read` without requiring the current
AES decoder's optional Python dependency. Caller-supplied source comparison uses
the published `abc` MD5 vector as a checksum example, not authentication.

The UTF-8 facade extraction was committed separately, with unchanged function
bodies and diagnostics. All ten owned Composer XML tests and 38 installed project
indexes passed; the complete 89,092,608-byte native edit packet matched the
accepted baseline byte for byte.

The focused record suite checks one measurement-value edit through encryption,
reopen and exact reversal under both current and historical cipher profiles. It
also checks the populated tutorial's 158 named records, exact escaping, all record
parents, stale and foreign identities, failed-edit isolation, empty completed
indexes and eight concurrent edits against the serial result. Explicit limit
boundaries, wider native templates, fresh-process equality and unchanged Source
acceptance of actual Rust outputs still need their feature gates. The complete
local and exact-head remote checks remain required before integration.
