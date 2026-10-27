# Structural members and contour records

Bessemer reads, edits and writes the complete ordinary modern structural-member
feature and its documented profile, contour, index and orientation dependencies
above version `0x27CB` in non-DB external-XML model archives. The feature reuses
the complete body-feature base before its own suffix. The profile-reference
reader reuses the complete wrapper and appends its single unsigned word.

Signed curve, point and contour counts remain distinct from packed direct-array
counts. Point records retain both full i32 metadata words before native u16
truncation. The original point list is read before the later contour mode;
offline decoding does not suppress it for native store mode one. Only a strictly
positive mode consumes the tagged contour pointer. Negative and zero modes omit
the tag, while the complete raw i32 remains observable.

The existing finite curve and point-reference families supply the declared roles.
The curve family's visibility changes for this caller without changing its names
or existing users. Newly encountered unsupported classes remain explicit stops.
Member property lists are tagged; advanced-property and segment-index containers
delegate to packed direct collections under their own tagged identities. Equal
and null entries retain their original order and map references.

The single-contour serializer does not prepend the pointer-array serializer from
its runtime ancestry. It reads one orientation pointer, one direct array and its
scalar suffix. Orientation retains separate curve, point and direction pointers,
including repeated points, without inferring a vector from adjacent doubles.
Segments retain their direct +50 array and signed scalar order. Index leaves
contain exactly three signed words, without a curve pointer or count.

Independent readers verified 90 complete cases and 25,171 primitive observations
across nine classes and three versions. Additional complete member cases vary all
three existing point-reference classes, two curve subtypes and six modes including
signed extrema. Populated graphs share curves, points, profile components,
directions, properties, segments, contours and index leaves. Negative zero, raw
words, Unicode, newlines and out-of-u16 metadata values remain unchanged.

Six owned streams import the blank part's actual 97 slots and finish with
101, 125, 133, 133, 125 and 133 slots. An independently constructed edit changes
only the first point record's metadata word. Public tests reconstruct complete
documents, compare that edit and verify source isolation after rejection. Five
private tests compare every field/byte/slot, reject every truncated prefix and
trailing byte, check malformed counts and unrelated cached references, and prove
the direct leaf minima without additional inheritance arrays or object slots.
Three Python tests regenerate every fixture and check the explicit matrix.

`reference.json` binds stable before/after sibling reader hashes, generators,
reused complete builders, primitive inventories and object slots. The schema
snapshot retains the native field-order and constructor-type clauses. Native
contour construction, native point-list suppression, arbitrary synthetic native
writer validity, geometric validity, native application acceptance, DB/deferred
archives, historical layouts and full documented schema parity remain unproven.
No dependency or package version changed. The humanizer skill is unavailable;
its prose pass remains with the maintainer.

Two fresh native processes preserve the previous 686 complete configurations and
add ten, reaching 696/906 with identical outcomes and causes. Six other failures
advance to later unsupported records; 210 configurations still stop explicitly.
The independent reader audit verifies all 696 accepted streams to exact EOF,
totaling 40,804,987 feature bytes and matching all four original input hashes per
case. It used a verified immutable copy of 1,124 approved sibling Python files,
checked before and after decoding, then removed after the audit process exited.

The staged Windows x64 `cargo xtask pr` gate passed with Rust 1.99.0: 540 library
tests, four xtask tests, every integration suite and 265 doctests. Three external
corpus gates remain ignored by default; both native replays above ran explicitly.
