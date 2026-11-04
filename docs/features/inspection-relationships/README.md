# Inspection relationship projections

Feature #197 covers the persisted dictionary in §§14.19–14.20. This directory
contains independent preparation fixtures and reference tests alongside the first
Rust read/preserve/reopen operation. `InspectionRecords::relationship_views`
requires an explicit retained-node or element-only policy. It retains every load
attempt, first-insertion map entry and writer-filtered entry separately, together
with immutable source identity, completion, lexical coverage, history and semantic
exclusions. It does not serialize a new relationship dictionary.

The first focused operation passed nine contract tests. Two fresh processes each
passed ten tests including the manual exporter; their complete packets were
byte-identical. The unchanged original Source helper matched every attempt and
selected entry for 12 cases under both policies. Exact limits and every one-below rejection passed for both cipher profiles.
Eight workers matched 384 mixed-policy results against serial results. The
focused metadata and Composer regressions passed 33 tests, retaining one existing
exclusion. The full repository and publication gates remain required.

A matched local probe found additional read time and memory use in the first
storage implementation. Ordered capture is now opt-in at the shared-parser
boundary: validation and Composer keep the lightweight element model, while
Inspection record indexing requests the content needed by relationships. The
complete result packets remain unchanged. The optimized implementation still
needs a repeated cost measurement and complete repository validation.

The selected native helper distinguishes ordered insertion attempts, insertion
into an initially empty map, and the writer's empty-key filter. A retained text,
comment or processing-instruction node can insert an empty key before a later
explicit empty-key entry. An element-only input has a different result. The
native caller's DOM whitespace-retention policy remains unknown and must not be
silently chosen by the library.

The owned fixture also distinguishes exact DOM node names from expanded
namespace equality. An unprefixed name in a default namespace can match;
`x:RelationshipsDictionary` and `x:string` do not match the unprefixed selector.
The first exact dictionary wins, repeated key fields replace the local key,
repeated value fields append, and descendant text includes nested elements and
CDATA in source order. Map insertion keeps the first exact UTF-16 key. Case and
Unicode normalization are not changed. Output inspection order does not establish
native map iteration order.

The pure Source insertion helper accepts a wider string domain than valid XML,
including NUL and explicit surrogate code units. Those helper checks must remain
separate from strict XML admission. The populated tutorial supplies 48 nonempty
writer keys and 120 reciprocal links; those counts and graph properties are
fixture observations, not universal validity requirements.

The Rust interface reuses the existing bounded XML parser and retains explicit
node-policy assumptions through the complete read/preserve/reopen operation.
Further gates include unknown-node preservation, descendant-text order, exact Qt
whitespace, duplicate handling, source isolation, limits, concurrency, fresh
processes and unchanged Source agreement. No native application or geometry
acceptance follows from the preparation tests.
