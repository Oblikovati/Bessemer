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
snapshots, completion, exact XML coverage, semantic exclusions and a source
parent for every result record. Only the selected record's relation is derived.

Edits retain the initial immutable lexical checkpoint (#194). Restoring a
checkpoint value replays that attribute's original token, including quote style,
spacing and character-reference spelling, while retaining unrelated later edits.
Replay history includes both the current occurrence and the checkpoint occurrence;
the result exposes the checkpoint snapshot so both parents can be resolved.
Intermediate snapshots share the initial checkpoint rather than forming a chain.
A project parsed from a reopened edited envelope starts with its own observed
checkpoint; it cannot reconstruct an earlier file's unobserved lexical spelling.

`ordered.xml` is an owned literal fixture. It distinguishes the DOCUMENT container
from its nested records, preserves duplicate identifiers and property names, and
separates missing values from empty strings. Namespace and nested-child decoys
check that literal record views use the documented scope.

The selected native load projections in §§14.18 and 14.16.24 have a different contract:
they accept all direct element children and default absent selected attributes to
empty strings. `loaded_measurements` and `loaded_custom_properties` expose those
ordered input projections for a revision-bound characteristic. Each keeps the
source element's expanded name and selected strings, including empty defaults.
Independent reference tests keep that projection separate from named record views. Relationship
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
indexes and eight concurrent edits against the serial result. The load-view tests
also check repeated exact containers, qualified and differently named children,
comments, processing instructions, nested descendants, empty characteristics and
rejection of wrong-role, foreign and stale identities. A literal eight-element
document independently fixes the exact limits at 16 lexical events, three
attributes and depth eight. Both cipher profiles accept those bounds and reject
each axis one below; the byte boundary includes required cipher padding. Every
proper XML prefix and an unexplained trailing suffix are rejected. Oversized
edits and caller mutation preserve the original snapshot and identities.

Both owned current template variants have a sheet and nested document with no
characteristic records. Their title edits preserve every other XML byte, reopen
through encryption and reverse to the exact original ciphertext. An explicit
installed-root test repeats these checks against all 42 original templates,
first requiring their bytes to equal the accepted owned variants (28 and 14).

Two fresh processes each passed 22 tests and exported all 1,349 unqualified
attribute edits across six fixtures. Every edit reversed to its original lexical
bytes, including numeric references and alternate quotes. Their 467,509,853-byte
packets were byte-identical. Two original Source tests accepted every actual Rust
output, comparing complete XML trees against independently changed expectations.
Independent Expat byte spans and fixed escaped-value spellings also checked each
output's exact byte patch. The frozen reader and source-integrity guard passed.
The complete
local and exact-head remote checks remain required before integration.
