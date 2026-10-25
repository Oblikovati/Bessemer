# Display-list archives

Development for #28 is on `codex/display-lists`. It is not ready for integration.
The goal remains complete clause 13 part and assembly archive reading, writing,
construction and isolated edits, with all documented branches and all 1,086 native
streams verified. No prefix-only archive API or opaque remainder fallback will
be merged. Drawing `Contents/DisplayLists` and unresolved payloads must fail explicitly.

## Verified foundation

`DisplayTransform` implements both distinct overloads in
[clause 11.6](specification-11.6.md). Compact restore retains its raw matrix
presence byte, optional matrix, translation, scale, raw flags and flag-selected
perspective. Fixed extraction retains all 120 bytes as typed stored values,
including the two raw word pairs. Neither performs native in-memory corrections
or claims geometry validity. `StoredDouble` retains NaNs, signed zero and all
other bit patterns. Validated results are immutable; owned content edits are
checked before publication. Matrix5 payload storage is owned and clone-isolated.

The independent generator produces 1,025 complete vectors: all 256 flags with
presence bytes 0, 1, 2 and 255, plus the fixed form. Values exercise signed zero,
infinities, quiet/signaling NaN payloads, a subnormal and the largest finite double.
Tests check all wire fields, forced reconstruction, every truncation, trailing
bytes, overload confusion, invalid flags/payload combinations, source isolation,
Send/Sync and concurrent deterministic encodings.

On Windows x64 with Rust 1.99.0 MSVC, the reference reader completed all 1,086 core
part/assembly display-list streams. Instrumented transform boundaries identified
20,209 compact and 6,277 fixed transforms (1,804,378 bytes). Bessemer forced byte-
identical reconstruction of all 3,467 distinct transform encodings. The SHA-256
of sorted corpus-relative path, NUL and display-list payload was
`94047d85dc01ed0f391378374f2c15f5464301bb26990d4214eecdd600723a87`.
This proves transform codec fidelity, not Bessemer's complete-stream support.

```text
python -X utf8 docs/features/display-lists/generate_transform_vectors.py
cargo test --test display_transforms
cargo build --release --example rebuild_display_transforms
python -X utf8 docs/features/display-lists/verify_transform_corpus.py target/pinned-1.99.0/release/examples/rebuild_display_transforms.exe
```

The verifier imports the approved sibling reader, observes each serializer's exact
input span during complete reference parsing, and deduplicates by overload plus
exact bytes. Its private executable copy prevents build collisions. Native sample
bytes are read in place and are not redistributed. The owned wire vectors use
independent Python struct primitives and require no installed application.

## Verified named-view component

`DisplayVersion` normalizes exact `_DL_VERSION_<n>/DLUpdateStamp` markers once.
Signed negative names become their unsigned 32-bit values; malformed, missing
and conflicting markers fail explicitly. Model/file versions and add-in version
strings do not substitute for this context. Equal numeric marker aliases agree.

`NamedViews` reads, constructs, edits and writes the complete named-view body in
13.4.1. Its [reference snapshot](reference-named-views.md) records all conditional
fields. Inactive versions have no body, distinct from a present empty collection.
Typed records retain both compact transforms, CString encodings and prefix widths,
section planes, raw selection flags and names, and unknown fields and bitsets.
Version-gated fields must have the exact required shape before publication.

`DisplayViewLimits` bounds bytes, view records and the cumulative nested planes
and selection-name entries. Counts are checked before allocation. Encoded CString
sizes are checked without allocating a temporary string. Every prefix width and
both character encodings have exact-boundary tests. Regression #29 checks that
equivalent CString edits cannot use shorter canonical prefix sizes to return
longer preserved source bytes beyond the requested budget.

Sixty independent vectors cover every named-view version boundary, absent and
empty collections, noncanonical DWORD CString prefixes, wide and Latin-1 text,
embedded NULs, raw nonboolean flags, planes and selected names. Eleven focused
tests cover complete field expectations and trace output, every truncation,
malformed counts, all budgets, invalid shapes, equivalent edits, authentic reverts,
source isolation, Send/Sync and repeated concurrent edits against serial bytes.

On Windows x64, Rust 1.99.0 MSVC, complete reference parsing of all 1,086 native
part/assembly streams identified 1,086 named-view collections containing 99 views
and 30,432 bytes. All 41 distinct versioned collections force-reconstructed byte
for byte. Native collections and all 60 owned vectors also matched independent
reference field traces: text, array shapes, signed/unsigned field values and exact
floating-point bits. The trace covers fields exposed by the reference decoder;
raw transform presence bytes and discarded Matrix5 integers have separate complete
wire-vector coverage. [named-verification.json](named-verification.json) records
manifest and fixture identities. This is component evidence, not full Bessemer
display-list archive acceptance.

```text
python -B -X utf8 docs/features/display-lists/generate_named_vectors.py
cargo test --test display_named_views --test display_named_trace
cargo build --release --example rebuild_named_views
python -B -X utf8 docs/features/display-lists/verify_named_corpus.py target/pinned-1.99.0/release/examples/rebuild_named_views.exe
```

## Verified annotation-view component

`AnnotationViews` implements the complete annotation-view body in 13.4.3, using
its [captured serializer](reference-annotation-views.md). The outer version gate,
the per-view unknown flag and the collection-level trailing bitset have distinct
conditions. An active collection can have zero views and still require the bitset.
It is not the same layout as named views.

`AnnotationViewId` wraps the stored signed ID once without inventing a restricted
domain or uniqueness rule. Repeated IDs, raw shown/current flags, both compact
transforms, second-name and unknown strings, section-plane fields and bitsets
remain unchanged. Native suffixes and visibility flags are not reconstructed or
normalized. `DisplayViewLimits` bounds physical/rebuilt bytes, view records and
cumulative planes. The shared immutable view envelope retains equivalent bytes
and authentic reverts, including the #29 preserved-prefix budget check.

Twenty-seven independent vectors exercise each version boundary, absent and
present-empty bodies, collection tails, duplicate IDs, nonboolean flags, all
conditional fields, noncanonical CString prefixes, embedded NULs, wide text and
exact special floating-point bits. Seven focused tests cover complete field and
trace expectations, every truncation, malformed counts, invalid shapes, budgets,
clone/source isolation, authentic reverts, Send/Sync and concurrent edits.

On Windows x64, Rust 1.99.0 MSVC, 2026-10-25, all 1,086 native annotation collections
matched independent byte and field oracles: 3,260 views and 874,416 bytes. Every
one of the 597 distinct versioned bodies force-reconstructed byte for byte; the
reference field traces also matched all 27 owned vectors. The parent display-
stream manifest is the same as the transform and named-view corpus above.
[annotation-verification.json](annotation-verification.json) records fixture,
reference-snapshot and corpus identities. These are component checks; Bessemer's
complete display-list archive remains unimplemented.

```text
python -B -X utf8 docs/features/display-lists/generate_annotation_vectors.py
cargo test --test display_annotation_views --test display_annotation_trace
cargo build --release --example rebuild_annotation_views
python -B -X utf8 docs/features/display-lists/verify_annotation_corpus.py target/pinned-1.99.0/release/examples/rebuild_annotation_views.exe
```

The named and annotation verifiers share framing, scalar/transform tracing and
complete-reference-stream scanning. Each still invokes its own native serializer
and compares the exact semantic fields of that distinct body.

## Verified 3D-view component

`SpatialViews` implements the complete 3D-view body in 13.4.2 using its
[captured serializer](reference-spatial-views.md). It preserves both transforms,
version-gated active flag and strings, all section planes, the data-dependent
after-plane field, paired late integer/bitset fields and the final text. Named
and 3D planes share typed `ViewSectionPlane` fields with distinct explicit gates;
`NamedSectionPlane` remains an alias for compatibility within this development.

`ViewBitmap` is the serialized Win32 BITMAP body, not a packed DIB. It retains
width, height, row-byte count, plane count, bits per pixel and raw bytes without
pixel decoding. The native framing test uses the signed height-times-row-bytes
product, accepting only 0 through 64 MiB inclusive. The product is computed in
64 bits before conversion or copying. Payload lengths must match exactly; width,
individual signs and other header values do not introduce inferred constraints.
Global byte budgets still apply across the whole collection.

Fifty-one independent vectors cover every version boundary, absent and empty
collections, zero-length and populated bitmaps, both-negative dimension products,
noncanonical strings, embedded NULs, raw flags and every conditional plane/tail
field. Ten focused tests cover decoded fields and traces, every truncation,
malformed/oversized bitmap headers and counts, the inclusive size boundary without
a large allocation, every optional slot, cumulative budgets, the preserved-prefix
budget regression, bitmap/input/source isolation, authentic reverts and repeated
parallel edits against serial bytes.

On Windows x64, Rust 1.99.0 MSVC, 2026-10-25, all 1,086 native collections matched
byte reconstruction and independent field traces: 22 views and 435,030 bytes.
Every one of the 11 distinct versioned bodies reconstructed exactly, including
its raw bitmap bytes. All 51 owned vectors matched the independent reference
field traces too. [spatial-verification.json](spatial-verification.json) records
identities. This does not establish pixel semantics, complete Bessemer stream
acceptance, native application acceptance or geometric validity.

```text
python -B -X utf8 docs/features/display-lists/generate_spatial_vectors.py
cargo test --test display_spatial_views --test display_spatial_trace
cargo build --release --example rebuild_spatial_views
python -B -X utf8 docs/features/display-lists/verify_spatial_corpus.py target/pinned-1.99.0/release/examples/rebuild_spatial_views.exe
```

## First complete part-stream checkpoint

The unmerged `DisplayArchive` now consumes the owned blank part's complete
6,316-byte display stream at display/file version 19000. Construction from owned
typed fields forces exact reconstruction. Fields include the shared class/object
map, environment and splitters, fonts, lights, visual materials and DSPBR, view
collections, empty body lists, reference planes and origin, units and empty
configuration tails. Other versions and unresolved populated bodies return typed
failures. This checkpoint does not establish complete clause 13 support.

The derived [shared-font fixture](shared-font-fixture.json) aliases its second
font to the first font's map identity. The independent reader confirms shared
identity; Rust reconstructs all 6,292 bytes exactly. Pointer roles validate both
new objects and aliases, preventing an unrelated mapped object from replacing a
font (#31). Ink configuration dimensions remain independent of absent view rows
(#30). Archive collection limits include entries inside view packets.

Twelve focused tests cover full reconstruction, shared aliases and invalid roles,
whole-document edit/reopen/revert, input and snapshot isolation, every byte
truncation, trailing bytes, schema/path/order mismatches, explicit contexts,
identity boundaries, all resource limits and concurrent deterministic writes.
The [independent scalar oracle](blank-scalar-oracle.json) checks 535 font, light
and material fields against `../inventor-interface`; the existing view oracles
cover their respective packets. Whole-stream independent field coverage and the
remaining corpus are still required. Native application and geometry acceptance
have not been established.

```text
python -B -X utf8 docs/features/display-lists/generate_alias_fixture.py
python -B -X utf8 docs/features/display-lists/generate_blank_scalars.py
cargo test --test display_archive
```

## Complete blank-assembly checkpoint and corpus audit

The same archive reader and writer now consume and reconstruct the owned blank
assembly's complete 5,446-byte stream. Its component records retain explicit
identity, transform and bounds, display state, references, dependency lists,
reference geometry, tessellation numbers, owner/child pointers, hidden-body atoms
and envelope options. Populated shaded bodies and external face references are
covered below; decals and full cross-stream validation remain implementation work.

Six assembly tests cover exact forced reconstruction, 40 independent component
and envelope fields, setting edit/reopen/revert with other streams preserved,
every truncation, impossible counts and a shared-map owner cycle. The
[cycle fixture](owner-cycle-fixture.json) changes only the original null owner
pointer to a reference to the object currently being decoded. The independent
reader verifies identity; this tests archive graph handling, not valid model
ownership or native application acceptance. Two independently built
[component fixtures](component-vectors.json) exercise both display-mode layouts,
populated dependency/tessellation-number/hidden-body arrays, extended collection
counts, UTF-16 strings, raw flags and transform bits. They match 112 independent
scalar values and complete byte reconstruction.

The [whole-stream audit](archive-audit.json) records missing work explicitly.
Its first assembly checkpoint reconstructed 2 of the 1,086 reference-complete
native streams. The current result after shaded-body support is recorded below.
The corpus manifest matches the component oracles above. An audit
failure is incomplete support, not a passing corpus case. The audit does not
establish complete-field semantics, geometric validity or application acceptance.

```text
python -B -X utf8 docs/features/display-lists/generate_owner_cycle.py
python -B -X utf8 docs/features/display-lists/generate_component_vectors.py
cargo test --test display_assembly
cargo build --release --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/release/examples/rebuild_display_archives.exe
```

## Populated shaded bodies and facet reuse

Version-19000 body, face and body-property records now use the shared object map.
Inline faces reuse the complete clause-12 `FacetContainer` codec. Variable facet
array entries share the outer archive's aggregate limit; a sizing pass checks the
remaining byte budget before encoding each facet payload. The earlier standalone
facet layouts remain unchanged.

Assembly context now owns the exact component IDs declared by the external face
directory. Matching components omit inline facets and expose explicit component
and zero-based face references. These references preserve archive consumption
order; resolving and validating their external blocks remains a separate required
cross-stream check. Unknown directory context is not inferred from stream bytes.
External faces share one immutable operation-owned ID buffer. The allocation
regression observed two separate six-byte face-ID buffers before the fix and one
shared buffer afterward; caller and snapshot edits remain isolated (#33). This
removes the face-count times ID-length copy amplification without changing bytes.

Nine tests use [independently built shaded vectors](shaded-vectors.json) in owned
part and assembly roots. They cover two inline or external faces, reused class
tags, raw metadata and transform/facet bits, complete forced reconstruction,
external-context and reference-index mismatches, aggregate quotas, facet preflight
byte limits, malformed field shape, equivalent-string preservation budgets,
every truncation, whole-document edit/reopen/revert, input/context ownership and
concurrent deterministic writes. The inline fixture has 37 variable entries per
facet and 60 outer entries, giving an exact aggregate boundary of 134. The external
fixture's boundary is 65.

On Windows x64, Rust 1.99.0 MSVC, 2026-10-25, the shaded-body
checkpoint reconstructed 404 of 1,086 complete native
streams. Two processes produced identical bytes and failures; all remaining cases
returned explicit unsupported layouts. The major remaining gaps include display
items, scenes, additional reference geometry and earlier versions. The geometry
extension and current audit are recorded below. This is byte
reconstruction evidence, not complete-field, geometric or application acceptance.
Successful paths are retained in bounded manifest files referenced by the report.
The audit rejects lost earlier successes before replacing its evidence; a forced
failure test confirmed that the prior report remained unchanged.

The [facet reuse regression](reused-facet-verification.json) reconstructed the
earlier 170 directories and 193 blocks, covering 35,242 faces and 150,411,068 bytes.
Its manifest exactly reproduces the frozen clause-12 baseline. The verifier keeps
the original sorted Windows file order and each container's central-directory
stream order; sorting stream names had caused a false mismatch (#32). No baseline
bytes, expected results or tolerances were changed.

```text
python -B -X utf8 docs/features/display-lists/generate_shaded_vectors.py
cargo test --test display_shaded
cargo build --release --example rebuild_display_archives --example rebuild_tessellations
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/release/examples/rebuild_display_archives.exe
python -B -X utf8 docs/features/display-lists/verify_reused_facets.py target/pinned-1.99.0/release/examples/rebuild_tessellations.exe
```

## All modern reference-geometry kinds

At display/file version 19000, the shared collector now reads and writes all seven
documented kinds: origin, point, axis, plane, coordinate system, CPoint and route
point. The discriminator is checked once before its body is read. Plane data keeps
the fixed 120-byte transform overload. The other records retain their raw vectors,
integers, extra CPoint fields and CString representations without geometric
normalization. Earlier version-dependent layouts still require implementation.

The [independent reference vectors](reference-vectors.json) replace the collector
inside an owned complete part stream. They cover every kind, reordered duplicate
kinds, signaling/quiet NaNs, signed zero, UTF-16 supplementary characters, embedded
NULs and noncanonical string prefixes. A third vector independently encodes the
expected nested CPoint edit. The three cases match 388 scalar values and 445 full
geometry trace fields, including every fixed-transform field.

Eight tests cover complete reconstruction, discriminator failures, high-word-first
counts, exact aggregate limits, every new collector/footer truncation, missing
fields, equivalent-text byte limits, independent expected edit bytes, whole-file
edit/reopen/revert, input isolation and eight concurrent edits against the same
expected result. The reference module was extracted separately; its existing
owned fixtures and all 1,025 transform vectors remained unchanged.

The reference-geometry checkpoint reconstructed 453 of 1,086
native streams and matches 90,067 independent reference-geometry fields within
those complete streams. Two processes return identical reconstruction, diagnostics
and field traces. Failed streams publish no partial trace, and the audit preserves
all earlier accepted paths. This extends geometry-field verification; complete
field verification for the rest of each archive, geometry validity and native
application acceptance remain separate requirements.

```text
python -B -X utf8 docs/features/display-lists/generate_reference_vectors.py
cargo test --test display_reference_geometry
cargo build --release --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/release/examples/rebuild_display_archives.exe
```

## Modern scenes and generic object slots

Version-19000 cubical and spherical scenes now read and write their complete
shared body: concept/node metadata, PhotoWorks scene settings, wall information,
all three wall shapes, bounds, floor/environment transforms, exposure and field of
view. Other scene selector values retain their documented absent-body behavior.
The two wall counts preserve raw nonpositive values while reading empty lists,
matching the native loading branch. Declared font/material pointers still enforce
their runtime classes; generic `CObject` slots accept heterogeneous registered
objects without treating unknown bodies as opaque successes.

Five [independent scene vectors](scene-vectors.json) cover both selectors, all wall
shapes, duplicate wall keys, noncanonical counts and strings, heterogeneous aliases,
empty/negative counts, absent scene bodies and an independently encoded exposure
edit. They match 747 semantic fields. Raw presence/count framing is checked
separately and every complete stream reconstructs exactly.

Nine tests cover those fields and bytes, object roles and aliases, unknown class
rejection, exact entry/object budgets, malformed counts and wall kinds, every new
scene/footer truncation, conditional field shape, equivalent-string byte budgets,
whole-document edit/reopen/revert, source ownership and eight concurrent edits.
These generated graph fixtures establish archive behavior; they do not establish
model ownership or application validity.

The [current native audit](archive-audit.json) reconstructs 552 of 1,086 streams,
with 117,118 independently checked reference-geometry fields and 80,284 scene
fields. Both processes return identical bytes, diagnostics and combined field
traces, and all earlier accepted streams remain accepted. Scene traces follow
first object declarations so shared aliases do not invent duplicate bodies.
Earlier scene versions, remaining archive fields, cross-stream validity and
native application acceptance remain required work.

```text
python -B -X utf8 docs/features/display-lists/generate_scene_vectors.py
cargo test --test display_scenes
cargo build --release --example rebuild_display_archives
python -B -X utf8 docs/features/display-lists/audit_archive_corpus.py target/pinned-1.99.0/release/examples/rebuild_display_archives.exe
```

## Remaining implementation and integration gate

The [modern display-item checkpoint](display-items.md),
[view-bucket checkpoint](view-buckets.md), [light checkpoint](lights.md) and
[version-range checkpoint](version-range.md), [older shaded checkpoint](old-shaded.md)
and [early shaded checkpoint](early-shaded.md) reconstruct all 1,086 complete
reference-readable part/assembly streams in this corpus. Their fixtures
and field-oracle counts supersede the earlier totals
above. This corpus milestone does not establish complete schema parity.

The [legacy bucket checkpoint](legacy-buckets.md) extends the unexercised schema
coverage for the four older supported snapshots while preserving that corpus gate.

- Graphics-reference wire coverage and its independent plane evidence are recorded
  in [graphics-references.md](graphics-references.md); unresolved faceted-data bodies
  remain unsupported.
- Populated ink grids, empty dimensions and isolated state edits are covered in
  [markup-ink.md](markup-ink.md).
- Per-configuration bodies, view matrices, visibility pairs and high-word-first
  tails for admitted contexts are covered in [configurations.md](configurations.md).
- Decal lists, keyed maps, masks, face associations and reused appearance records
  are covered in [decals.md](decals.md).
- Textured-plane records with null bitmap pointers and complete transforms are
  covered in [textured-planes.md](textured-planes.md); bitmap bodies remain unresolved.
- The middle range extends admitted contexts to 16000–19000 with complete facet,
  component, material, scene and bucket boundary fixtures in
  [middle-range.md](middle-range.md).
- The lower range extends admitted contexts to 14000–19000 with texture, thread,
  face, component-number and detailing boundaries in [lower-range.md](lower-range.md).
- The 13000 range adds older scalar widths, retained silhouette words, appearance,
  annotation and ink boundaries in [transition-range.md](transition-range.md).
- Context resolution and exact external face-cache links now have owned and native
  checks; broader geometry and native application validation remain unfinished.
- Reuse existing MFC strings, exact numeric values, class/object maps and face
  tessellation records. Preserve cycles, aliases and object-tag spelling; do not
  reset inherited state between nested serializers.
- Extend environments, splitters, fonts, lights, visual properties, buckets and
  display items to the remaining version and caller contexts.
- Add earlier scene and unit layouts, remaining shaded-data layouts,
  textured-plane bitmap bodies and earlier configuration layouts. Unknown-meaning
  fields remain explicit raw typed values; unresolved bodies are unsupported.
- Complete the immutable archive graph, validated owned construction/edits,
  revision-scoped identities, authentic reverts, ordered coverage and history.
- Preserve the complete 1,086-stream forced-byte gate and expand independent field
  oracles to the remaining schema branches. Add their owned version/degeneracy
  vectors, malformed input/budget checks, whole-document isolation and process/
  concurrency comparisons, then run all five CI checks.
- Update the public README and final CHANGELOG entry, review this evidence, create
  one PR, merge when green and remove the finished branch. Native application
  acceptance stays separate from archive validity and geometric validity.

[Clause 13](specification-13.md) is a working-tree snapshot from the approved
`../inventor-interface` repository, whose reference commit is
`dd0537b3a142d8dda00ef846891efc736a6aac46`; its documentation is evolving. Corliss's
typed decoded fields and isolated edits inform the public API design. The
maintainer-side humanizer skill is unavailable; its prose pass remains for review.

- The 12000 range retains discarded edge objects, float coordinates and caller-specific
  collector gates; see [earlier-range.md](earlier-range.md).

- The remaining 11000 interval adds configuration and component boundaries,
  including inline configuration tessellation; see [precollector-range.md](precollector-range.md).

- The 10000 interval retains transient viewer graphs and earlier facet/bucket
  boundaries; see [component-transition.md](component-transition.md).

- The 9000 interval covers grouped components, word-key bucket maps and the early
  assembly configuration/decal boundaries; see [grouped-components.md](grouped-components.md).

- Explicit native file context now controls tangent-list omission independently
  of display-list version; see [file-contexts.md](file-contexts.md).

- The 8000 interval covers earlier annotation strides, environment/body/unit
  omissions and the documented uncounted-window exclusion; see [window-boundaries.md](window-boundaries.md).

Early complete-root view transitions and Bug #37 are recorded in [early view roots](early-view-roots.md).

- The 6000 interval retains historical scene, wall, component, pen and halo-list layouts; see [historical roots](historical-roots.md).

- The 5000 interval covers early appearance, face-identity and display-item fields; see [early appearances](early-appearances.md).

- The 4850 range covers exploded-view and scene background-fit gates; see [earliest scenes](earliest-scenes.md).

- The 4800 range covers component metadata, property fonts, BOS flags and populated decal boundaries; see [earliest components](earliest-components.md).

- The 4600 range covers face-pair arrays, scene flags, 24-slot property fonts and polyline layouts; see [earliest faces](earliest-faces.md).

- The 4500 range covers all five older light kinds, appearance words, hidden atoms, weld lists and decal options; see [earliest lights](earliest-lights.md).

- The 4400 range covers earlier appearance payloads, face words and null legacy wall materials; see [earliest walls](earliest-walls.md). Non-null legacy wall-material bodies remain unresolved and explicitly unsupported.

- The lower bound is now 4300, with direct body-property maps, earlier state words and versioned OLE table lists; see [earliest bodies](earliest-bodies.md).

- Archive object lookup now checks revision-scoped handles; see [revision identities](revision-identities.md). Operation history and whole-document coverage remain unfinished.

- Shared line-style and layer-manager CObject bodies retain aliases and versioned map framing; see [UI objects](ui-objects.md).

- Component properties and holders preserve version-sized font arrays and shared cyclic graphs; see [component properties](component-properties.md).

- Typed document context resolution reads exact markers and bounded face-directory declarations; whole-document byte reconstruction is covered in [document context](document-context.md). Named blocks and requested external facets are checked in [external face links](external-face-links.md).

- The lower bound is now 4200, including implicit face, body and model visual pointers and adjacent viewer, table and material gates; see [earliest visuals](earliest-visuals.md).
- The 4100 checkpoint adds the pre-`0x1052` appearance layout, with absent rendering fields and the retained old appearance word; the expanded [earliest visuals](earliest-visuals.md) corpus contains 76 roots and four complete containers.
- The 4000 checkpoint adds the optional root model-size double at `0xFCB` and face/decal records at `0xFD8`. The expanded [earliest visuals](earliest-visuals.md) corpus contains 110 roots and six complete containers, retaining all prior fixtures.
- The 3905 checkpoint covers the earlier environment, 23-font properties, omitted scenes and assembly shrink-wrap words. The [earliest visuals](earliest-visuals.md) corpus now contains 168 roots and eight complete containers; pre-scene property graphs use bucket CObject keys.
- The 3890 checkpoint adds caller-specific legacy line widths before `0xF41`. The [earliest visuals](earliest-visuals.md) corpus now covers 196 roots and ten complete containers, preserving the previous cases.
- The 3800 checkpoint adds 15-font properties and pre-`0xF1E` material and texture layouts. The [earliest visuals](earliest-visuals.md) corpus now covers 224 roots and twelve complete containers, retaining the previous cases.
- The 3555 checkpoint covers independently gated appearance words and assembly reference IDs. The [earliest visuals](earliest-visuals.md) corpus now contains 258 roots and fourteen complete containers, retaining the previous cases.
- The 3508 checkpoint adds independently gated visibility, configuration, pattern and hierarchy records. The [earliest visuals](earliest-visuals.md) corpus contains 298 roots and sixteen complete containers, preserving the previous cases.
- The 3330 checkpoint adds the independently gated facet-display and shader words. The [earliest visuals](earliest-visuals.md) corpus contains 326 roots and eighteen complete containers, retaining the previous cases.
- The 3238 checkpoint adds early PV and keep-scene light words, with all five old-light classes. The [earliest visuals](earliest-visuals.md) corpus contains 404 roots and twenty complete containers, retaining the previous cases.

The 3196 checkpoint covers front-view, font-cap, pen-cap and bucket-pointer gates. The [earliest visuals](earliest-visuals.md) corpus now contains 548 roots and twenty-two complete containers, retaining all previous cases.

Edition 0.206 adds complete modern drawing `Contents/VBLists` framing. Its separate
[implementation and evidence](../drawing-buckets/README.md) reuse the shared
codecs while preserving the drawing backend facet context. Drawing
`Contents/DisplayLists` remains unresolved.

The [unsigned-version checkpoint](unsigned-versions.md) removes the artificial
19000 ceiling, with 85 rich part/assembly roots, 25 drawing streams and three
signed-alias documents. All earlier unresolved intervals remain excluded.

The [earlier resolved archive checkpoint](pre-facet-roots.md) admits contexts
from 3065, with 94 complete roots and six containers. Cache-backed parts retain
their non-null objects and exact external face references. Unresolved inline
facet objects remain explicit payload failures; empty geometry is preserved distinctly.

The [legacy pane, line and symbol checkpoint](legacy-panes.md) admits contexts
from 2999, with 66 additional complete roots and six containers. Old line flags
retain their full 32-bit words; omitted pane strings and symbol planes stay absent.

The [camera, viewer and text checkpoint](legacy-camera-layouts.md) admits contexts
from 2902, with 66 additional complete roots and 8192 independent field checks.

The [earlier appearance checkpoint](legacy-visual-layouts.md) admits contexts from
2891, with 50 complete roots and six containers. Lightweight-rendering words and
appearance names follow separate presence gates; cache-backed faces remain complete
through edits, reopen and revert.

The [earlier add-in and body-property checkpoint](legacy-addin-layouts.md) admits
contexts from 2796, with 50 complete roots and six containers. It preserves absent
add-in fields and body-property pointers without inserting default values.

The [implicit-width checkpoint](legacy-width-layouts.md) admits contexts from
2771, with 34 complete roots and six containers. Unicode text determines implicit
widths, while later stored counts remain independent of text length.

The [bucket thread-pattern checkpoint](legacy-thread-layouts.md) admits contexts
from 2749, with 94 complete roots and six containers. Populated 2D and 3D buckets
retain their aliases and complete shaded models across the thread-storage boundary.

The [polygon-region checkpoint](legacy-region-layouts.md) admits contexts from
2725, with 94 complete roots and six containers. Independent oracles retain the
raw region words as well as their derived flags; earlier layouts omit the words.

The [viewer-mode checkpoint](legacy-mode-layouts.md) admits contexts from 2711,
with 94 complete roots and six containers. Absent, one-word and three-word mode
records remain distinct, including complete shape-changing edits.

The [line-visibility checkpoint](legacy-visibility-layouts.md) admits contexts
from 2650, with 150 complete roots and six containers. It preserves omitted
visibility words and records why the older holder-name branch is unreachable.

The [temporary edge and triple checkpoint](legacy-edge-layouts.md) admits contexts
from 2606, with 206 complete roots and twelve containers. It retains discarded
edge words, nested view triples and the versioned absence of component string maps.

The [resumed inline-facet checkpoint](resumed-inline-layouts.md) admits contexts
from 2596 and proves complete inline part and assembly edits before the
unresolved facet-object interval. Its 130 roots and four containers preserve
empty and cached layouts across the 2597 boundary.

The [ordinary text checkpoint](ordinary-text-layouts.md) admits contexts from
2563, with separate text-width and material-classification boundaries, 158
complete roots and four inline part/assembly containers.

The [legacy rendering checkpoint](legacy-rendering-layouts.md) admits contexts
from 2388. Its 262 complete roots cover material rendering flags and auxiliary
facet textures on both sides of their bounds, with four whole-document fixtures.

The [legacy texture checkpoint](legacy-texture-layouts.md) admits contexts from
2359, covering absent material textures and per-view shaded-edge words with
190 complete roots and four whole-document fixtures.

The [legacy environment checkpoint](legacy-environment-layouts.md) admits contexts
from 2277, covering absent environment-tail and facet-coordinate fields with
190 complete roots, twenty independent facet records and four containers.

The [legacy model checkpoint](legacy-model-layouts.md) admits contexts from
2130. Its 226 complete roots cover absent body/part metadata and component atom
lists, including inline and externally cached models and four containers.

The [legacy boundary checkpoint](legacy-boundary-layouts.md) admits contexts
from 2078. Its 370 complete roots and forty independent facet records cover
polygon attributes, body metadata, facet stamps and cone-culling fields.

The [legacy font checkpoint](legacy-font-layouts.md) admits contexts from 2011,
with 262 complete roots, explicit font records, solid-fill list boundaries,
twenty-eight independent facet records and four containers.

The [legacy zone checkpoint](legacy-zone-layouts.md) admits contexts from 1875,
covering exploded lists, component zones and textured planes with 262 roots,
2112 independent plane-transform fields and eight whole-document fixtures.

The [discarded edge-word checkpoint](legacy-discard-layouts.md) admits contexts
from 1857. Its 262 complete roots cover hidden-silhouette list introduction and
the seven-version extra edge word, with eight whole-document fixtures.

The [early text checkpoint](legacy-text-layouts.md) admits contexts from 1645,
covering omitted text tails, facet edge-ID flags and physical node words despite
native logical resets, with 262 complete roots and four containers.

The [early UI checkpoint](legacy-ui-layouts.md) admits contexts from 1286, with
442 complete roots covering secondary splitters, ghost transforms, font/sketch
suffixes, body metadata and facet edge-type arrays, plus eight containers.
