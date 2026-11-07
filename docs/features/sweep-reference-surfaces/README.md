# Sweep-reference surfaces and edge data

Issue #54 implements clause 11.233 / C-275 in ordinary external-XML part
archives above version `0x27CB`. The complete reference-geometry and inline
surface bases precede the existing direct sweep-data serializer. Its versioned
final word still follows the `0x27D9` gate. The surface parent then retains five
raw bytes, two angular pointers only for selector 8 or 9, a full signed i32 edge
count and an optional radial pointer when its independent selector is nonzero.
The ordinary sweep feature's suffix is not reused for this parent.

Negative edge counts retain their bits and consume no entries. Positive counts
are bounded before reading tagged `moEdgeData_c` pointers. Null entries and
repeated references remain in the consumed raw sequence; no native runtime
filtering or writer-side nonnull recount silently changes it. Edge-data bodies
contain exactly three endpoint pointers, including the established loft-sync
subclass. The empty sweep-surface keeper adds no body bytes. Native RTTI proves
its inherited keeper role and the surface's atom-owner and reference-geometry
roles. The source reader's expected-class argument is not a role oracle.

Independent fixtures cover 162 complete graphs across nine versions, 44,798
primitive observations and 19 owned streams. They include both sides of the
sweep-data, surface and reference-geometry gates; signed count extremes; null,
fresh and cached entries; shared dimensions and endpoints; complete loft-sync
bodies; real inherited keepers; distinct equal roots; a typed component-surface
caller; and an isolated direct-data word edit. No new positive kernel framing
is introduced by these fixtures; existing inline-surface framing is reused.

Rust tests check every truncated prefix, appended bytes, unsupported old context,
minimum widths, exact object slots, negative-count behavior, impossible positive
counts and unrelated cached-pointer rejection. Public tests verify whole-document
byte reconstruction, preservation of untouched storage and input isolation after
successful and rejected edits. Run `cargo test --lib model_archives::sweep_surface_tests`,
`cargo test --test resolved_sweep_surfaces` and `cargo xtask pr`. Independent
regeneration uses `python -B -X utf8 -m unittest discover -s docs/features/sweep-reference-surfaces`.

Two fresh native processes agree on all 906 outcomes and failure causes.
Case 269 completes, raising reconstruction from 764 to 765 inputs; case 342
reaches the later `moExtendRefSurface_c` stop. No prior success is lost, and the
remaining 141 stops receive no prefix credit. The independent pinned-source
audit verifies all 765 accepted streams and four stream hashes per input,
covering 49,094,182 feature-stream bytes. The same snapshot reproduces both
original C-275 root spans, their raw flags and selectors, totaling 6,340 bytes.
Results are in `accepted-reference.json` and `native-sentinel-reference.json`.

Source/native bindings and executed checks are recorded beside this document.
Historical migrations, alternate backends, raw option meanings, geometry validity,
SolidWorks application acceptance and broader all-schema parity remain open.
The humanizer skill is unavailable; prose review remains a maintainer task.
