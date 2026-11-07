# Direct sketch curves, hatches and surfaces

Feature #44 now reads direct hatch, arc, ellipse and conic collections, plus
the five documented surface collections. The shared entity prefix returns both
stored flag words: one controls hatch suppression and the other controls line
tails. Earlier entity, handle and direct-geometry fixtures remain unchanged.

Active hatches retain four unsigned index arrays, tagged references, contour
records and tagged list/DWORD-array containers. The reference source documents
the `suObList` to `su_CObList` serializer forwarding; both use the existing typed
pointer-list reader. DWORD arrays reuse the existing unsigned-word reader.
The hatch's two list classes are explicit; no name-based role relaxation was
added. Missing hatch-data or chooser body readers still stop at those tagged
dependencies when encountered.

Arcs preserve short or escaped centers and signed auxiliary indices. Ellipses
exercise every combination of their five escaped indices. Conics retain raw
eccentricity/rho bits and their stored geometry word. Surface records preserve
their entity prefixes, coordinates, radii and other doubles. Planes also retain
their border, optional string and optional point. No radius, axis, angle or mask
is normalized.

The independent packet has 1,254 cases and 46,844 scalar expectations. Cases
cover suppressed and active hatches, populated arrays, cyclic list references,
compact/extended counts, noncanonical flags, signed extremes, negative zero and
NaN payloads. All reconstruct exactly and reject every truncated prefix and
trailing byte. The reference's derived surfaces-completion marker is excluded
from the scalar oracle because it consumes no bytes; every stored field remains
covered.

The populated hatch uses exactly 15 aggregate array entries: 15 passes and 14
fails. Negative nested lengths and oversized container counts fail before
iteration. An arc edit matches separately assembled, reference-validated bytes,
reparses to its edited fields and leaves the source unchanged. Source-overwrite
checks cover every case; concurrent writes retain the populated hatch's shared
identities and unsigned values. The entity flag type has a Send/Sync assertion.

Nine new tests, all 27 sketch-focused tests, all 200 model tests, all-target
Clippy and all 50 Python tests passed. Three new wire fixtures reproduced
exactly in fresh processes. All 906 native outcomes matched the preceding run.
Checks, schema/source hashes and the native corpus outcome
are bound in `sketch-curve-verification.json`. Complete sketch composition still
needs splines, dimension collections, patterns and the tail. There is no partial
`sgSketch` root reader. Native acceptance remains 866/906 and its full assertion
still fails.

These checks establish archive structure and stored values, not geometry or
native application acceptance. Public model operation/result/history contracts
remain open. The full PR gate belongs to the earlier geometric/kernel source
state and must rerun before push. Humanizer was unavailable; prose remains for
maintainer review.
