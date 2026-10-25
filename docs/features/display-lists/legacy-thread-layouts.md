# Earlier bucket thread-pattern layouts

Reference baseline: specification edition 0.231, dated 2026-10-25. Reader hashes
are recorded in the fixture manifest.

Complete part and assembly fixtures cover versions 2749, 2770 and 2771. The 94
fixtures provide 19392 independently expected fields. They include all five old
light classes, empty shaded parts, null component parts, complete external faces,
and populated or empty 2D and 3D buckets. The reference reader parses each final
stream to its end at the declared version.

Polygons store `thread_ptn_xform_id` only after `0xAD2`. Buckets store their four
`cthread_maps` only after that same boundary. Earlier layouts store neither the
polygon ID nor any map headers. Nonpositive polygon counts end the polygon
before either version's attributes. Line properties retain the older signed i32
width rather than the later short/float representation.

Populated cases contain nested buckets, a root-bucket alias as a thread-map key,
an edge-object key, repeated string-map keys, noncanonical flags and negative
empty-list counts. Main polygon lists use their enclosing 2D or 3D scope. Thread
map values restore the outer 3D scope even when the main lists were 2D. Float
coordinate oracles preserve signaling NaN payloads during widening.

The generator starts with an owned null view-bucket pointer. It inserts complete
bucket records and relocates subsequent short MFC references by the number of
new class and object slots. New-class records keep their implicit slot ordering.
This lets populated buckets precede complete shaded part and assembly models;
the resulting graph is checked again by the reference reader. The relocation
helper accepts only the short tags present in these owned fixtures.

Integration tests compare independent fields and exact forced bytes, check edits
against independently built streams, and verify source isolation and authentic
reverts. They check aliases, coordinate scopes, concurrent shaded-assembly
construction, truncation failures and incompatible contexts. Roots with null
buckets remain unchanged across this boundary because no changed payload exists.

Two owned unshaded containers check context resolution at version 2749. Four
complete shaded assembly containers combine populated buckets with non-null
models at versions 2749 and 2771, before and after edits. Both referenced faces
remain complete through bucket and text edits, replacement, reopen and revert.
Their cache version stays 19000 independently of the display version. Process
verifiers compare complete streams, containers, contexts and linked facet bytes
in two fresh processes.

This checkpoint expands archive support to version 2749. Earlier polygon-region
layouts still need their own coverage. Unresolved inline-facet payloads remain
explicitly unsupported. Byte reconstruction does not establish geometric validity
or native application acceptance. Complete schema parity remains tracked by #28.
