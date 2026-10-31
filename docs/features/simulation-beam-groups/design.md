# Joint and group operation boundaries

JointGroup, Joint, JointList and BeamFolder are separate complete bodies. Public
operations need an explicit body kind and normalized stored version. JointList
also needs an optional independently supplied expected child count. Coverage
must state whether that extra count agreement was checked; complete byte
framing alone cannot establish adjacency or enumeration order.

Reuse the existing reference-container implementation. Its current caller
constructs count/tail paths below `record0.references` and reference paths below
`record0.referenceN`. These new Source bodies put the reference paths below
`group.references.referenceN` or `joint.references.referenceN`. A private helper
can accept separate container and reference roots while retaining existing
paths and bytes exactly. Before changing it, preserve complete existing beam
projections and compare them unchanged against the independent expectations.
Keep this extraction in a separate structural commit.

Use the shared complete scalar/text/blob/archive value types; none of these
four bodies adds a physical short. Preserve signed beam IDs, duplicate pairs,
raw words/double bits and explicit embedded archive contexts. Joint count zero
is a completed list body. Missing or mismatched required count context must
never become reconstructed adjacency or a partial successful body.

The inclusive Joint reference interval is 810000 through 810030. At 810030,
both the old reference container and final double are present. At 810031, the
reference container is absent. Group children are separate storage payloads,
not extra fields inserted before the final group double. Header words do not
establish child counts.

BeamFolder parsing covers only its word/text header. The native storage method
requires a Beams child storage before opening that header, but the header codec
does not itself validate a whole storage tree. No native folder fixture exists.
Keep static/synthetic header acceptance distinct from native carrier acceptance.

Prove complete read, construction and isolated edits with full outer/nested
history before exposing the API. Required evidence includes all retained native
outcomes, owned threshold neighbors and count contexts, full field/edit/history
oracles, malformed and exact budget cases, fresh-process exports, source
isolation, concurrency and full local/platform gates. Runtime native lookup,
repair, geometry and physical meanings remain outside stored-profile acceptance.
