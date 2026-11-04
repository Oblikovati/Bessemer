# Inspection relationship projections

Feature #197 covers the persisted dictionary in §§14.19–14.20. This directory
currently contains independent preparation fixtures and reference tests; no Rust
relationship API is implemented yet.

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

Before adding a Rust interface, reuse the existing bounded XML parser and prove
one complete read/preserve/reopen operation with explicit retention assumptions.
Further gates include unknown-node preservation, descendant-text order, exact Qt
whitespace, duplicate handling, source isolation, limits, concurrency, fresh
processes and unchanged Source agreement. No native application or geometry
acceptance follows from the preparation tests.
